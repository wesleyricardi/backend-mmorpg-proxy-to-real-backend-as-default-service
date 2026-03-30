use socket::Data;

use crate::application::use_case::runtime::world_runtime::world_command::{
    TargetBuffCast, TargetBuffContext, WorldCommand,
};
use crate::domain::skill::dtos::codes::SkillCode;
use crate::domain::user_player::state::UserPlayerId;
use crate::state::ServerState;

use super::helpers::player_id_for_connection;
use super::sender::world_sender;

#[derive(Debug, Clone, Copy)]
pub struct TargetBuffCommandPayload {
    pub skill_code: SkillCode,
    pub point: i32,
    pub context: TargetBuffContext,
    pub target_id: UserPlayerId,
}

pub async fn send_target_buff_from_connection(
    data: Data<ServerState>,
    connection_id: usize,
    payload: TargetBuffCommandPayload,
) {
    let Some(tx) = world_sender() else {
        log::error!("world sender not initialized");
        return;
    };

    if let Some(caster_id) = player_id_for_connection(&data, connection_id).await {
        log::debug!(
            "forwarding ProcessTargetBuff command to world, caster_id={:?} skill_code=0x{:X} point={} target_id={:?}",
            caster_id.0,
            payload.skill_code.raw(),
            payload.point,
            payload.target_id.0
        );
        let _ = tx.send(WorldCommand::ProcessTargetBuff {
            cast: TargetBuffCast {
                caster_id,
                skill_code: payload.skill_code,
                point: payload.point,
                context: payload.context,
                target_id: payload.target_id,
            },
        });
    }
}
