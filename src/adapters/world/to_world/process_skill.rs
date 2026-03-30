use socket::Data;

use crate::application::use_case::runtime::world_runtime::world_command::WorldCommand;
use crate::domain::skill::dtos::codes::SkillCode;
use crate::state::ServerState;

use super::helpers::player_id_for_connection;
use super::sender::world_sender;

pub async fn send_process_skill_from_connection(
    data: Data<ServerState>,
    connection_id: usize,
    raw_skill_code: u32,
    from_party: bool,
) {
    let Some(tx) = world_sender() else {
        log::error!("world sender not initialized");
        return;
    };
    let Ok(skill_code) = SkillCode::try_from(raw_skill_code) else {
        log::info!("invalid skill code: {}", raw_skill_code);
        return;
    };

    if let Some(player_id) = player_id_for_connection(&data, connection_id).await {
        log::debug!(
            "forwarding ProcessSkill command to world, player_id={:?} skill_code=0x{:X} from_party={}",
            player_id.0,
            skill_code.raw(),
            from_party
        );
        let _ = tx.send(WorldCommand::ProcessSkill {
            player_id,
            skill_code,
            from_party,
        });
    }
}
