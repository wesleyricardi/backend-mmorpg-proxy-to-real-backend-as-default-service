use socket::{Client, Data, ServiceBuilder};
use tokio::time::{sleep, Duration, Instant};

use crate::config::{
    global::{SERVER_ADDR, SERVER_PORT},
    handlers::warmup_handlers_config,
};
use crate::error::{AppError, ProtocolErrorKind, StateErrorKind};
use crate::protocol::game_server::{self, GetGameServerAllDataRequest};
use crate::state::{GameServerWarmupCollector, ServerState};
use crate::{protocol_error, state_error, unexpected_error};

pub const WARMUP_CONNECTION_ID: usize = usize::MAX - 1;
pub const LEGACY_CONNECT_RETRY_DELAY: Duration = Duration::from_secs(2);
pub const WARMUP_COMPLETION_TIMEOUT: Duration = Duration::from_secs(90);

pub async fn wait_for_warmup_completion(data: Data<ServerState>) -> Result<(), AppError> {
    let started_at = Instant::now();

    loop {
        {
            let collector = data.game_server_warmup.lock().await;
            if collector.is_completed() {
                return Ok(());
            }

            if collector.is_disconnected() {
                return Err(state_error!(
                    StateErrorKind::LegacyWarmupIncomplete,
                    "warmup connection {} closed before warmup completion",
                    WARMUP_CONNECTION_ID
                ));
            }
        }

        if started_at.elapsed() >= WARMUP_COMPLETION_TIMEOUT {
            return Err(state_error!(
                StateErrorKind::LegacyWarmupTimeout,
                "warmup timed out after {:?} waiting for GameServerWarmupDone on connection {}",
                WARMUP_COMPLETION_TIMEOUT,
                WARMUP_CONNECTION_ID
            ));
        }

        sleep(Duration::from_millis(50)).await;
    }
}

pub async fn perform_warmup(data: Data<ServerState>) -> Result<(), AppError> {
    {
        let mut collector = data.game_server_warmup.lock().await;
        *collector = GameServerWarmupCollector::default();
    }

    let legacy_addr = format!("{SERVER_ADDR}:{SERVER_PORT}");

    loop {
        log::info!(
            "waiting for legacy server warmup connection: connection_id={} addr={}",
            WARMUP_CONNECTION_ID,
            legacy_addr
        );

        let mut services = ServiceBuilder::new();
        services
            .app_data(data.clone())
            .channel_capacity(4096)
            .service_concurrency(1)
            .configure(warmup_handlers_config);

        match Client::new(services)
            .host(legacy_addr.clone())
            .connect_with_id(WARMUP_CONNECTION_ID)
            .await
        {
            Ok(client) => {
                log::info!(
                    "warmup connection established: connection_id={}",
                    WARMUP_CONNECTION_ID
                );

                let responder = client.get_tx_socket();
                responder
                    .send(game_server::Message::GetGameServerAllData(
                        GetGameServerAllDataRequest { request_id: 1 },
                    ))
                    .await
                    .map_err(|error| {
                        protocol_error!(
                            ProtocolErrorKind::ForwardFailed,
                            error,
                            "failed to send GetGameServerAllData on warmup connection {}",
                            WARMUP_CONNECTION_ID
                        )
                    })?;

                log::info!("startup warmup request GetGameServerAllData sent to legacy server");

                let builder = std::thread::Builder::new()
                    .name("game-server-warmup-run".to_string())
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
                                        "failed to create runtime for thread game-server-warmup-run"
                                    )
                                );
                            }
                        }
                    })
                    .map_err(|error| {
                        unexpected_error!(
                            error,
                            "failed to spawn client run thread game-server-warmup-run"
                        )
                    })?;

                return wait_for_warmup_completion(data).await;
            }
            Err(error) => {
                log::warn!(
                    "legacy server unavailable for warmup connection {}: {:?}. retrying in {:?}",
                    WARMUP_CONNECTION_ID,
                    error,
                    LEGACY_CONNECT_RETRY_DELAY
                );
                sleep(LEGACY_CONNECT_RETRY_DELAY).await;
            }
        }
    }
}
