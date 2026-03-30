use std::time::Instant;

use socket::{Data, Request, RequestVecBody, Responder};

use crate::adapters::world;
use crate::error::AppError;
use crate::protocol::transcode::TransPlayData;
use crate::proxy;
use crate::state::ServerState;

pub async fn transplaydata(
    original_request: RequestVecBody,
    request: Request<TransPlayData>,
    responder: Responder,
    data: Data<ServerState>,
) -> Result<(), AppError> {
    proxy::forward_to_client(original_request, responder.clone(), data.clone()).await?;
    let field_id = request.body.area[0] as i32;
    if field_id >= 0 {
        world::upsert_player_field_for_connection(
            data.clone(),
            responder.id,
            field_id,
            Instant::now(),
        )
        .await;
    }
    Ok(())
}
