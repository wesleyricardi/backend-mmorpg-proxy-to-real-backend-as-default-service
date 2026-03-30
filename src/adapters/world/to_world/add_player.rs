use socket::Data;

use crate::application::use_case::runtime::world_runtime::world_command::WorldCommand;
use crate::domain::user_player::state::{UserPlayerId, UserPlayerState};
use crate::state::ServerState;

use super::sender::world_sender;

pub async fn upsert_player_for_connection(
    data: Data<ServerState>,
    connection_id: usize,
    player_id: UserPlayerId,
    mut player_state: UserPlayerState,
) {
    let Some(tx) = world_sender() else {
        return;
    };

    player_state.identity.connection_id = connection_id;

    data.servers_connected
        .lock()
        .await
        .bind_player(connection_id, player_id);

    let _ = tx.send(WorldCommand::AddPlayer {
        player_id,
        data: player_state,
    });
}
