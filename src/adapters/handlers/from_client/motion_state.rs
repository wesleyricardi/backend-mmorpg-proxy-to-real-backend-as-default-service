use std::time::Instant;

use socket::{Data, Request, Responder};

use crate::adapters::world;
use crate::domain::user_player::char_motion_state::CharMotionState;
use crate::error::AppError;
use crate::protocol::transcode::MotionState;
use crate::state::ServerState;

pub async fn motion_state(
    req: Request<MotionState>,
    res: Responder,
    data: Data<ServerState>,
) -> Result<(), AppError> {
    let body = req.body;
    let motion_state = CharMotionState::from_raw(body.motion_state);
    world::sync_char_motion_state_from_connection(data, res.id, motion_state, Instant::now()).await;

    Ok(())
}
