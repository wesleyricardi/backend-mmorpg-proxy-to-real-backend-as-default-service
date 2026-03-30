use socket::{Client, Data, Responder, ServiceBuilder};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::Mutex;

pub mod legacy_sync;

use crate::{
    config::global::{SERVER_ADDR, SERVER_PORT},
    config::handlers::legacy_handlers_config,
    domain::skill::state::SkillState,
    domain::user_player::state::UserPlayerId,
};
pub use legacy_sync::{
    GameServerRuntimeSyncTracker, GameServerWarmupCollector, GameServerWarmupSnapshot,
};

#[derive(Debug, Default)]
pub struct PendingRecordDataMultipart {
    parts: HashMap<i32, Vec<u8>>,
    total: i32,
    updated_at: Option<Instant>,
}

impl PendingRecordDataMultipart {
    pub const EXPIRATION: Duration = Duration::from_secs(5);

    pub fn reset(&mut self) {
        self.parts.clear();
        self.total = 0;
        self.updated_at = None;
    }

    pub fn is_expired(&self, now: Instant) -> bool {
        self.updated_at
            .is_some_and(|updated_at| now.saturating_duration_since(updated_at) >= Self::EXPIRATION)
    }

    pub fn push_fragment(
        &mut self,
        count: i32,
        total: i32,
        trans_size: i32,
        chunk: &[u8],
        now: Instant,
    ) -> Option<Vec<u8>> {
        if count < 0 || total < 0 || trans_size < 0 {
            self.reset();
            return None;
        }

        if total > 0 && count >= total {
            self.reset();
            return None;
        }

        if self.is_expired(now) {
            self.reset();
        }

        if self.total != total {
            self.parts.clear();
            self.total = total;
        }

        self.updated_at = Some(now);

        self.parts.insert(
            count,
            chunk[..(trans_size as usize).min(chunk.len())].to_vec(),
        );

        let expected_parts = if total <= 0 { 1 } else { total as usize };
        if self.parts.len() < expected_parts {
            return None;
        }

        let mut assembled = Vec::new();
        for index in 0..expected_parts {
            let key = if total <= 0 { 0 } else { index as i32 };
            let Some(part) = self.parts.get(&key) else {
                return None;
            };
            assembled.extend_from_slice(part);
        }

        self.reset();
        Some(assembled)
    }
}

pub struct Connected {
    client_connected: HashMap<usize, Responder>,
    server_connection: HashMap<usize, Responder>,
    user_player_connected: HashMap<usize, UserPlayerId>,
}

impl Connected {
    pub async fn new(
        &mut self,
        client: Responder,
        data: Data<ServerState>,
    ) -> tokio::io::Result<()> {
        log::trace!("new connection {}", client.id);
        let legacy_addr = format!("{SERVER_ADDR}:{SERVER_PORT}");
        let mut services = ServiceBuilder::new();
        services
            .app_data(data.clone())
            .configure(legacy_handlers_config);

        let legacy_client = Client::new(services)
            .host(legacy_addr)
            .connect_with_id(client.id)
            .await?;

        log::trace!(
            "creating connection with the server for client {}",
            client.id
        );

        self.add(client, legacy_client.get_tx_socket());

        std::thread::spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build();

            match runtime {
                Ok(runtime) => {
                    runtime.block_on(async move {
                        legacy_client.run().await;
                    });
                }
                Err(error) => {
                    log::error!("failed to create legacy client runtime: {:?}", error);
                }
            }
        });

        Ok(())
    }

    fn add(&mut self, client: Responder, server_tx: Responder) -> &mut Self {
        let client_id = client.id;
        self.client_connected.insert(client_id, client);
        self.server_connection.insert(client_id, server_tx);
        self
    }

    pub async fn remove(&mut self, id: usize) {
        log::trace!(
            "removing client {} and the connection with the server from the list",
            id
        );

        if let Some(server) = self.server_connection.remove(&id) {
            if let Err(error) = server.close_connection().await {
                log::warn!(
                    "shutdown server for the client {} failed, error: {:?}",
                    id,
                    error
                );
            }
        }

        self.client_connected.remove(&id);
    }

    pub fn get_server(&self, id: usize) -> Option<Responder> {
        self.server_connection.get(&id).cloned()
    }

    pub fn get_client(&self, id: usize) -> Option<Responder> {
        self.client_connected.get(&id).cloned()
    }

    pub fn bind_player(&mut self, connection_id: usize, player_id: UserPlayerId) {
        self.user_player_connected.insert(connection_id, player_id);
    }

    pub fn unbind_player(&mut self, connection_id: usize) -> Option<UserPlayerId> {
        self.user_player_connected.remove(&connection_id)
    }

    pub fn get_player_id(&self, connection_id: usize) -> Option<UserPlayerId> {
        self.user_player_connected.get(&connection_id).copied()
    }
}

pub struct ServerState {
    pub servers_connected: Arc<Mutex<Connected>>,
    pub game_server_warmup: Arc<Mutex<GameServerWarmupCollector>>,
    pub game_server_warmup_order: Arc<Mutex<()>>,
    pub game_server_runtime_sync: Arc<Mutex<GameServerRuntimeSyncTracker>>,
    pub pending_record_data: Arc<Mutex<HashMap<usize, PendingRecordDataMultipart>>>,
}

pub async fn get_app_state(_skill_state: SkillState) -> Data<ServerState> {
    Data::new(ServerState {
        servers_connected: Arc::new(Mutex::new(Connected {
            client_connected: HashMap::new(),
            server_connection: HashMap::new(),
            user_player_connected: HashMap::new(),
        })),
        game_server_warmup: Arc::new(Mutex::new(GameServerWarmupCollector::default())),
        game_server_warmup_order: Arc::new(Mutex::new(())),
        game_server_runtime_sync: Arc::new(Mutex::new(GameServerRuntimeSyncTracker::default())),
        pending_record_data: Arc::new(Mutex::new(HashMap::new())),
    })
}
