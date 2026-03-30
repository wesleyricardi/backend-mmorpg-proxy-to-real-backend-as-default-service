use std::sync::Arc;

use tokio::runtime::Runtime;
use tokio::sync::Mutex;

use crate::state::Connected;
use crate::world_transport::WorldNetCommand;

use super::world_dispatch;

pub fn dispatch_command(
    runtime: &Runtime,
    connected: Arc<Mutex<Connected>>,
    command: WorldNetCommand,
) {
    match command {
        WorldNetCommand::SendToClient {
            connection_id,
            message,
        } => world_dispatch::send_to_client::handle(runtime, connected, connection_id, message),
        WorldNetCommand::SendToServer {
            connection_id,
            message,
        } => world_dispatch::send_to_server::handle(runtime, connected, connection_id, message),
        WorldNetCommand::DisconnectClient {
            connection_id,
            reason: _,
        } => world_dispatch::disconnect_client::handle(runtime, connected, connection_id),
    }
}
