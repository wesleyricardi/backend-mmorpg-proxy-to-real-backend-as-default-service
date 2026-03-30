use std::time::Instant;

use socket::{Data, Request, RequestVecBody, Responder};

use crate::adapters::world;
use crate::error::AppError;
use crate::protocol::command::CharCommand;
use crate::state::ServerState;

pub async fn fail_record_data(
    _original_request: RequestVecBody,
    request: Request<CharCommand>,
    responder: Responder,
    _data: Data<ServerState>,
) -> Result<(), AppError> {
    world::mark_record_data_bootstrap_failed(
        responder.id,
        request.body.w_param as u32,
        request.body.l_param,
        Instant::now(),
    )
    .await;

    Ok(())
}
