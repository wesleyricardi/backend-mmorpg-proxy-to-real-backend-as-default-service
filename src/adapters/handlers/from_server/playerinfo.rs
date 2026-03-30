use std::time::Instant;

use socket::{Data, Request, RequestVecBody, Responder};

use crate::adapters::world;
use crate::domain::user_player::char_motion_state::CharMotionState;
use crate::error::AppError;
use crate::protocol::transcode::TransPlayerInfo;
use crate::proxy;
use crate::state::ServerState;

pub async fn playerinfo(
    original_request: RequestVecBody,
    request: Request<TransPlayerInfo>,
    responder: Responder,
    data: Data<ServerState>,
) -> Result<(), AppError> {
    proxy::forward_to_client(original_request, responder.clone(), data.clone()).await?;
    world::upsert_player_position_for_connection(
        data.clone(),
        responder.id,
        request.body.x,
        request.body.y,
        request.body.z,
        CharMotionState::from_raw(request.body.state),
        Instant::now(),
    )
    .await;
    Ok(())
}
