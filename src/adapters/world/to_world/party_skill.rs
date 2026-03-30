use socket::Data;

use crate::application::use_case::runtime::world_runtime::world_command::{
    PartySkillCast, PartySkillContext, WorldCommand,
};
use crate::domain::skill::dtos::codes::SkillCode;
use crate::domain::user_player::state::UserPlayerId;
use crate::state::ServerState;

use super::helpers::player_id_for_connection;
use super::sender::world_sender;

#[derive(Debug, Clone)]
pub struct PartySkillCommandPayload {
    pub skill_code: SkillCode,
    pub point: i32,
    pub context: PartySkillContext,
    pub targets: Vec<UserPlayerId>,
}

pub async fn send_party_skill_from_connection(
    data: Data<ServerState>,
    connection_id: usize,
    payload: PartySkillCommandPayload,
) {
    let Some(tx) = world_sender() else {
        log::error!("world sender not initialized");
        return;
    };

    if let Some(caster_id) = player_id_for_connection(&data, connection_id).await {
        log::debug!(
            "forwarding PartySkill command to world, caster_id={:?} skill_code=0x{:X} point={} targets={}",
            caster_id.0,
            payload.skill_code.raw(),
            payload.point,
            payload.targets.len()
        );
        let _ = tx.send(WorldCommand::ProcessPartySkill {
            cast: PartySkillCast {
                caster_id,
                skill_code: payload.skill_code,
                point: payload.point,
                context: payload.context,
                targets: payload.targets,
            },
        });
    }
}
