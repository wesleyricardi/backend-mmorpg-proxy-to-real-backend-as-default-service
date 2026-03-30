use std::time::Instant;

use socket::{Data, Request, RequestVecBody, Responder};

use crate::adapters::world;
use crate::domain::user_player::char_motion_state::CharMotionState;
use crate::error::AppError;
use crate::protocol::transcode::TransPlayPos;
use crate::proxy;
use crate::state::ServerState;

fn decode_legacy_position_area(area: u32) -> i32 {
    i32::from_ne_bytes(area.to_ne_bytes())
}

pub async fn position(
    original_request: RequestVecBody,
    request: Request<TransPlayPos>,
    responder: Responder,
    data: Data<ServerState>,
) -> Result<(), AppError> {
    proxy::forward_to_server(original_request, responder.clone(), data.clone()).await?;

    let play_pos = request.body.play_pos;
    let observed_at = Instant::now();

    world::upsert_player_field_for_connection(
        data.clone(),
        responder.id,
        decode_legacy_position_area(play_pos.area),
        observed_at,
    )
    .await;

    world::upsert_player_position_for_connection(
        data,
        responder.id,
        play_pos.x,
        play_pos.y,
        play_pos.z,
        CharMotionState::Unknown(0),
        observed_at,
    )
    .await;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::decode_legacy_position_area;

    #[test]
    fn decode_legacy_position_area_preserves_field_id_values() {
        assert_eq!(decode_legacy_position_area(9), 9);
        assert_eq!(decode_legacy_position_area(55), 55);
    }

    #[test]
    fn decode_legacy_position_area_preserves_negative_one_sentinel() {
        assert_eq!(decode_legacy_position_area(u32::MAX), -1);
    }
}
