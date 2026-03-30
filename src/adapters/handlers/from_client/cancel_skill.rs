use socket::{Data, Request, RequestVecBody, Responder};

use super::skill_code::map_legacy_play_skill_code;
use crate::protocol::command::CancelSkillRuntimeCommand;
use crate::{adapters::world, error::AppError, proxy, state::ServerState};

fn raw_skill_code_from_wparam(w_param: i32) -> u32 {
    w_param as u32
}

pub async fn cancel_skill(
    original_request: RequestVecBody,
    req: Request<CancelSkillRuntimeCommand>,
    res: Responder,
    data: Data<ServerState>,
) -> Result<(), AppError> {
    let raw_skill_code = raw_skill_code_from_wparam(req.body.cancel_l_param);
    let Some(skill_code) = map_legacy_play_skill_code(raw_skill_code) else {
        log::info!("invalid legacy play skill code: {}", raw_skill_code);
        return proxy::forward_to_server(original_request, res, data).await;
    };
    proxy::forward_to_server(original_request, res.clone(), data.clone()).await?;
    world::send_cancel_skill_from_connection(data, res.id, skill_code.raw()).await;
    Ok(())
}
