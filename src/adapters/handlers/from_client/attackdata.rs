use socket::{Data, Request, Responder};

use crate::protocol::transcode::TransAttackData;
use crate::{error::AppError, proxy, state::ServerState};

pub async fn attackdata(
    req: Request<TransAttackData>,
    res: Responder,
    data: Data<ServerState>,
) -> Result<(), AppError> {
    log::trace!("smTRANSCODE_ATTACKDATA: {:?}", req.body);
    proxy::forward_to_server(req, res, data).await
}
