use socket::Data;

use crate::application::use_case::runtime::world_runtime::world_command::{
    DisconnectReason, WorldCommand,
};
use crate::state::ServerState;

use super::sender::world_sender;

pub async fn remove_player_for_connection(data: Data<ServerState>, connection_id: usize) {
    let Some(tx) = world_sender() else {
        return;
    };

    let player_id = data
        .servers_connected
        .lock()
        .await
        .unbind_player(connection_id);

    if let Some(player_id) = player_id {
        let _ = tx.send(WorldCommand::RemovePlayer {
            player_id,
            reason: DisconnectReason::ClientDisconnected,
        });
    }
}
