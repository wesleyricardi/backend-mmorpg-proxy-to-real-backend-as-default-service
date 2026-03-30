use socket::{Data, Request, Responder};

use crate::protocol::transcode::TransAttackData2;
use crate::{error::AppError, proxy, state::ServerState};

pub async fn attack_damage(
    req: Request<TransAttackData2>,
    res: Responder,
    data: Data<ServerState>,
) -> Result<(), AppError> {
    log::trace!("smTRANSCODE_ATTACK_DAMAGE: {:?}", req.body);
    proxy::forward_to_server(req, res, data).await
}
