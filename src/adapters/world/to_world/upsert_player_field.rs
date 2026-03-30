use std::time::Instant;

use socket::Data;

use crate::application::use_case::runtime::world_runtime::world_command::WorldCommand;
use crate::domain::field::dto::FieldId;
use crate::state::ServerState;

use super::helpers::player_id_for_connection;
use super::sender::world_sender;

pub async fn upsert_player_field_for_connection(
    data: Data<ServerState>,
    connection_id: usize,
    raw_field_id: i32,
    observed_at: Instant,
) {
    let Some(tx) = world_sender() else {
        return;
    };
    let Some(field_id) = FieldId::from_raw(raw_field_id) else {
        log::trace!(
            "field_id_unknown_dropped connection_id={} field_id={}",
            connection_id,
            raw_field_id
        );
        return;
    };

    if let Some(player_id) = player_id_for_connection(&data, connection_id).await {
        let _ = tx.send(WorldCommand::UpsertPlayerField {
            player_id,
            field_id,
            observed_at,
        });
    }
}
