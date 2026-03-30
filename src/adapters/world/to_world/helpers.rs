use socket::Data;

use crate::domain::user_player::state::UserPlayerId;
use crate::state::ServerState;

pub async fn player_id_for_connection(
    data: &Data<ServerState>,
    connection_id: usize,
) -> Option<UserPlayerId> {
    data.servers_connected
        .lock()
        .await
        .get_player_id(connection_id)
}
