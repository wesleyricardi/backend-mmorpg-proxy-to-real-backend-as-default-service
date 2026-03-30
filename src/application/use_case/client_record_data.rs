use socket::{Data, Responder};

use crate::{
    adapters::world,
    domain::user_player::{
        dto::{UserCalcOutput, UserPlayerData},
        state::{UserPlayerId, UserPlayerState},
    },
    error::AppError,
    state::ServerState,
};

pub async fn save_client_record_data_from_server(
    data: Data<ServerState>,
    responder: Responder,
    player_id: UserPlayerId,
    user_player_data: UserPlayerData,
) -> Result<(), AppError> {
    let connection_id = responder.id;

    let mut calc_output = UserCalcOutput::default();
    calc_output.updated_char_info = user_player_data.char_info.clone();

    let player_state =
        UserPlayerState::from_record_data(connection_id, user_player_data, calc_output);

    world::upsert_player_for_connection(data, connection_id, player_id, player_state).await;

    Ok(())
}
