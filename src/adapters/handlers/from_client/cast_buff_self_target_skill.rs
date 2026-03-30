use socket::{Data, Request, Responder};

use super::skill_code::map_legacy_play_skill_code;
use crate::application::use_case::runtime::world_runtime::world_command::TargetBuffContext;
use crate::domain::skill::classification::is_target_buff_skill;
use crate::domain::user_player::state::UserPlayerId;
use crate::protocol::command::ProcessSkillRuntimeCommand;
use crate::{adapters::world, error::AppError, proxy, state::ServerState};

fn raw_skill_code_from_wparam(w_param: i32) -> u32 {
    (w_param as u32) & 0xFF
}

pub async fn cast_buff_self_target_skill(
    req: Request<ProcessSkillRuntimeCommand>,
    res: Responder,
    data: Data<ServerState>,
) -> Result<(), AppError> {
    let wire_params = req.body.wire_params();
    let packed_w_param = wire_params.w_param;
    let raw_skill_code = raw_skill_code_from_wparam(packed_w_param);
    log::debug!(
        "received cast_buff_self_target_skill opcode=ProcessSkill2 raw_skill_code=0x{:X} packed_w_param=0x{:X}",
        raw_skill_code,
        packed_w_param
    );
    let Some(skill_code) = map_legacy_play_skill_code(raw_skill_code) else {
        log::info!("invalid legacy play skill code: {}", raw_skill_code);
        return proxy::forward_to_server(req, res, data).await;
    };
    if let Some(target_object_serial) = req.body.active_target_object_serial() {
        if is_target_buff_skill(skill_code) {
            world::send_target_buff_from_connection(
                data.clone(),
                res.id,
                world::TargetBuffCommandPayload {
                    skill_code,
                    point: req.body.packed_point(),
                    context: TargetBuffContext::StandardBuff,
                    target_id: UserPlayerId(target_object_serial),
                },
            )
            .await;
            return proxy::forward_to_server(req, res, data).await;
        }
    }
    world::send_process_skill_from_connection(data.clone(), res.id, skill_code.raw(), false).await;
    proxy::forward_to_server(req, res, data).await
}
