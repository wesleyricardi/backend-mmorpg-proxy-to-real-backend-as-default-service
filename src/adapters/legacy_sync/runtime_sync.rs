use socket::{Client, Data, ServiceBuilder};
use tokio::time::{sleep, Duration, Instant};

use crate::adapters::world;
use crate::application::use_case::runtime::world_runtime::world_command::WorldCommand;
use crate::config::{
    global::{SERVER_ADDR, SERVER_PORT},
    handlers::runtime_handlers_config,
};
use crate::error::{AppError, ProtocolErrorKind, StateErrorKind};
use crate::protocol::game_server::{
    self, RequestWorldRuntimeSnapshotRequest, SubscribeWorldRuntimeRequest,
};
use crate::state::ServerState;
use crate::{protocol_error, state_error, unexpected_error};

pub const RUNTIME_SYNC_CONNECTION_ID: usize = usize::MAX - 2;
pub const RUNTIME_SNAPSHOT_TIMEOUT: Duration = Duration::from_secs(30);

pub async fn wait_for_runtime_snapshot(data: Data<ServerState>) -> Result<(), AppError> {
    let started_at = Instant::now();

    loop {
        {
            let tracker = data.game_server_runtime_sync.lock().await;
            if tracker.snapshot_completed() {
                return Ok(());
            }

            if tracker.is_disconnected() {
                return Err(state_error!(
                    StateErrorKind::LegacyRuntimeDisconnected,
                    "runtime sync connection {} closed before initial snapshot completed",
                    RUNTIME_SYNC_CONNECTION_ID
                ));
            }
        }

        if started_at.elapsed() >= RUNTIME_SNAPSHOT_TIMEOUT {
            return Err(state_error!(
                StateErrorKind::LegacyRuntimeSnapshotTimeout,
                "runtime snapshot timed out after {:?} on connection {}",
                RUNTIME_SNAPSHOT_TIMEOUT,
                RUNTIME_SYNC_CONNECTION_ID
            ));
        }

        sleep(Duration::from_millis(50)).await;
    }
}

pub fn clear_runtime_scope() {
    let Some(tx) = world::world_sender() else {
        log::error!("world sender not initialized to clear runtime scope");
        return;
    };

    let _ = tx.send(WorldCommand::ResetRuntimeScope {
        field_id: None,
        include_monsters: true,
        include_npcs: true,
        observed_at: std::time::Instant::now(),
    });
}

pub async fn request_runtime_snapshot(
    data: Data<ServerState>,
    request_id: u32,
    field_id: i32,
    include_monsters: bool,
    include_npcs: bool,
) -> Result<u32, AppError> {
    let responder = {
        let tracker = data.game_server_runtime_sync.lock().await;
        let responder = tracker.runtime_responder().ok_or_else(|| {
            state_error!(
                StateErrorKind::LegacyRuntimeDisconnected,
                "runtime connection {} is unavailable to request snapshot",
                RUNTIME_SYNC_CONNECTION_ID
            )
        })?;
        responder
    };

    responder
        .send(game_server::Message::RequestWorldRuntimeSnapshot(
            RequestWorldRuntimeSnapshotRequest {
                request_id,
                field_id,
                include_monsters: include_monsters as i32,
                include_npcs: include_npcs as i32,
            },
        ))
        .await
        .map_err(|error| {
            protocol_error!(
                ProtocolErrorKind::ForwardFailed,
                error,
                "failed to send RequestWorldRuntimeSnapshot(field_id={}) on runtime connection {}",
                field_id,
                RUNTIME_SYNC_CONNECTION_ID
            )
        })?;

    Ok(request_id)
}

pub async fn run_runtime_sync_session(
    data: Data<ServerState>,
    require_snapshot: bool,
    retry_delay: Duration,
) -> Result<(), AppError> {
    data.game_server_runtime_sync
        .lock()
        .await
        .reset_for_new_session();

    let legacy_addr = format!("{SERVER_ADDR}:{SERVER_PORT}");

    loop {
        log::info!(
            "waiting for legacy runtime sync connection: connection_id={} addr={}",
            RUNTIME_SYNC_CONNECTION_ID,
            legacy_addr
        );

        let mut services = ServiceBuilder::new();
        services
            .app_data(data.clone())
            .channel_capacity(4096)
            .service_concurrency(1)
            .configure(runtime_handlers_config);

        match Client::new(services)
            .host(legacy_addr.clone())
            .connect_with_id(RUNTIME_SYNC_CONNECTION_ID)
            .await
        {
            Ok(client) => {
                log::info!(
                    "runtime sync connection established: connection_id={}",
                    RUNTIME_SYNC_CONNECTION_ID
                );

                let responder = client.get_tx_socket();
                data.game_server_runtime_sync
                    .lock()
                    .await
                    .bind_runtime_responder(responder.clone());
                responder
                    .send(game_server::Message::SubscribeWorldRuntime(
                        SubscribeWorldRuntimeRequest {
                            request_id: 1,
                            include_monsters: 1,
                            include_npcs: 1,
                            full_snapshot: 0,
                            incremental_updates: 1,
                        },
                    ))
                    .await
                    .map_err(|error| {
                        protocol_error!(
                            ProtocolErrorKind::ForwardFailed,
                            error,
                            "failed to send SubscribeWorldRuntime on runtime connection {}",
                            RUNTIME_SYNC_CONNECTION_ID
                        )
                    })?;

                log::info!("runtime subscribe request sent to legacy server");

                if require_snapshot {
                    responder
                        .send(game_server::Message::RequestWorldRuntimeSnapshot(
                            RequestWorldRuntimeSnapshotRequest {
                                request_id: 2,
                                field_id: -1,
                                include_monsters: 1,
                                include_npcs: 1,
                            },
                        ))
                        .await
                        .map_err(|error| {
                            protocol_error!(
                                ProtocolErrorKind::ForwardFailed,
                                error,
                                "failed to send RequestWorldRuntimeSnapshot on runtime connection {}",
                                RUNTIME_SYNC_CONNECTION_ID
                            )
                        })?;

                    log::info!("runtime snapshot request sent to legacy server");
                }

                let builder = std::thread::Builder::new()
                    .name("game-server-runtime-run".to_string())
                    .stack_size(16 * 1024 * 1024);

                builder
                    .spawn(move || {
                        let runtime = tokio::runtime::Builder::new_current_thread()
                            .enable_all()
                            .build();

                        match runtime {
                            Ok(runtime) => runtime.block_on(client.run()),
                            Err(error) => {
                                panic!(
                                    "{}",
                                    unexpected_error!(
                                        error,
                                        "failed to create runtime for thread game-server-runtime-run"
                                    )
                                );
                            }
                        }
                    })
                    .map_err(|error| {
                        unexpected_error!(
                            error,
                            "failed to spawn client run thread game-server-runtime-run"
                        )
                    })?;

                if require_snapshot {
                    wait_for_runtime_snapshot(data.clone()).await?;
                }

                loop {
                    {
                        let tracker = data.game_server_runtime_sync.lock().await;
                        if tracker.is_disconnected() {
                            log::warn!(
                                "runtime sync session disconnected after snapshot: connection_id={}",
                                RUNTIME_SYNC_CONNECTION_ID
                            );
                            return Ok(());
                        }
                    }

                    sleep(Duration::from_millis(250)).await;
                }
            }
            Err(error) => {
                log::warn!(
                    "legacy server unavailable for runtime connection {}: {:?}. retrying in {:?}",
                    RUNTIME_SYNC_CONNECTION_ID,
                    error,
                    retry_delay
                );
                sleep(retry_delay).await;
            }
        }
    }
}
