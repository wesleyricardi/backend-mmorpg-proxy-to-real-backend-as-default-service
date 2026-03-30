use std::time::Instant;

use socket::Data;

use crate::application::use_case::runtime::world_runtime::world_command::WorldCommand;
use crate::domain::user_player::char_motion_state::CharMotionState;
use crate::state::ServerState;

use super::helpers::player_id_for_connection;
use super::sender::world_sender;

pub async fn upsert_player_position_for_connection(
    data: Data<ServerState>,
    connection_id: usize,
    x: i32,
    y: i32,
    z: i32,
    state: CharMotionState,
    observed_at: Instant,
) {
    let Some(tx) = world_sender() else {
        return;
    };

    if let Some(player_id) = player_id_for_connection(&data, connection_id).await {
        let _ = tx.send(WorldCommand::UpsertPlayerPosition {
            player_id,
            x,
            y,
            z,
            state,
            observed_at,
        });
    }
}
