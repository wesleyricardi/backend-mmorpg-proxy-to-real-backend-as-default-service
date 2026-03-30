use socket::Data;

use crate::application::use_case::runtime::world_runtime::world_command::WorldCommand;
use crate::domain::skill::dtos::codes::SkillCode;
use crate::state::ServerState;

use super::helpers::player_id_for_connection;
use super::sender::world_sender;

pub async fn send_cancel_skill_from_connection(
    data: Data<ServerState>,
    connection_id: usize,
    raw_skill_code: u32,
) {
    let Some(tx) = world_sender() else {
        return;
    };
    let Ok(skill_code) = SkillCode::try_from(raw_skill_code) else {
        return;
    };

    if let Some(player_id) = player_id_for_connection(&data, connection_id).await {
        let _ = tx.send(WorldCommand::CancelSkill {
            player_id,
            skill_code,
        });
    }
}
