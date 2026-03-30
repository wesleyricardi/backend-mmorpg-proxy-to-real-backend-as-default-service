use crate::application::use_case::runtime::world_runtime::world_command::DisconnectReason;

use super::messages::{ClientWorldMessage, ServerWorldMessage};

pub enum WorldNetCommand {
    SendToClient {
        connection_id: usize,
        message: ClientWorldMessage,
    },
    SendToServer {
        connection_id: usize,
        message: ServerWorldMessage,
    },
    DisconnectClient {
        connection_id: usize,
        reason: DisconnectReason,
    },
}

impl WorldNetCommand {
    pub fn send_to_client(connection_id: usize, message: impl Into<ClientWorldMessage>) -> Self {
        Self::SendToClient {
            connection_id,
            message: message.into(),
        }
    }

    pub fn send_to_server(connection_id: usize, message: impl Into<ServerWorldMessage>) -> Self {
        Self::SendToServer {
            connection_id,
            message: message.into(),
        }
    }

    pub fn disconnect_client(connection_id: usize, reason: DisconnectReason) -> Self {
        Self::DisconnectClient {
            connection_id,
            reason,
        }
    }
}
