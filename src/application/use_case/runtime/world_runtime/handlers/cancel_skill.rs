use crate::domain::skill::dtos::codes::SkillCode;
use crate::domain::user_player::calc::recalculate_user_player_state;
use crate::domain::user_player::state::UserPlayerId;

use super::super::world::World;

pub fn handle(world: &mut World, player_id: UserPlayerId, skill_code: SkillCode) {
    if let Some(player) = world.players.get_mut(&player_id) {
        if player.remove_active_continue_skill(skill_code).is_some() {
            recalculate_user_player_state(player, Some(&world.skill_state));
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::mpsc;
    use std::time::Instant;

    use crate::domain::field::dto::FieldId;
    use crate::domain::skill::dtos::codes::SKILL_VIRTUAL_LIFE;
    use crate::domain::skill::dtos::skills::{
        Levels, SkillClassConfig, SkillEntry, SkillValueType, VirtualLife,
    };
    use crate::domain::skill::state::SkillState;
    use crate::domain::user_player::dto::{
        ChangeJobTierState, PlayerJob, UserCalcOutput, UserPlayerData,
    };
    use crate::domain::user_player::state::UserPlayerState;

    use super::super::process_continue_skill;
    use super::*;

    fn mk_levels_i32(v: i32) -> Levels<i32> {
        Levels::new(SkillValueType::Fixed2, [v; 10])
    }

    fn mk_virtual_life_state() -> SkillState {
        let virtual_life = SkillEntry::VirtualLife(VirtualLife {
            name: "VirtualLife".to_string(),
            description: "test".to_string(),
            require_level: 1,
            use_stamina: None,
            require_mastery: None,
            element: None,
            use_weapon_code: vec![],
            skill_code: SKILL_VIRTUAL_LIFE,
            percent: mk_levels_i32(42),
            time: mk_levels_i32(300),
            use_mana: mk_levels_i32(0),
        });
        let mut by_code = HashMap::new();
        by_code.insert(SKILL_VIRTUAL_LIFE, virtual_life);
        SkillState {
            state: HashMap::from([(PlayerJob::Priestess, SkillClassConfig { by_code })]),
        }
    }

    #[test]
    fn cancel_skill_reverts_virtual_life_materialized_state() {
        let (tx_net, _rx_net) = mpsc::channel();
        let (_tx_world, rx_world) = mpsc::channel();
        let mut world = World::new(mk_virtual_life_state(), rx_world, tx_net);
        let now = Instant::now();
        let mut player_state = UserPlayerState::from_record_data(
            1,
            UserPlayerData::default(),
            UserCalcOutput::default(),
        );
        player_state.core.char_info.job_code = PlayerJob::Priestess;
        player_state.core.char_info.change_job = ChangeJobTierState::Tier4;
        player_state.core.char_info.level = 140;
        player_state.core.char_info.spirit = 820;
        player_state.core.char_info.health = 20;
        player_state.core.char_info.strength = 78;
        player_state.core.char_info.talent = 86;
        player_state.core.char_info.dexterity = 80;
        player_state.core.char_info.life = [1000, 0];
        player_state.progression.game_save.skill_info.b_skill_point[11] = 10;
        let baseline = crate::domain::user_player::calc::calculate_user_player(
            &crate::domain::user_player::calc::build_calc_input_from_state(&player_state),
            Some(&world.skill_state),
        )
        .updated_char_info
        .life[1];
        let player_id = UserPlayerId(654);
        world.players.insert(player_id, player_state);
        world
            .fields
            .player_field
            .insert(player_id, FieldId::Village1);

        process_continue_skill::handle(&mut world, player_id, SKILL_VIRTUAL_LIFE, false, now);
        handle(&mut world, player_id, SKILL_VIRTUAL_LIFE);

        assert_eq!(
            world
                .players
                .get(&player_id)
                .map(|player| player.core.char_info.life[0]),
            Some(1000)
        );
        assert_eq!(
            world
                .players
                .get(&player_id)
                .map(|player| player.core.char_info.life[1]),
            Some(baseline)
        );
        assert_eq!(
            world
                .players
                .get(&player_id)
                .map(|player| player.runtime.active_continue_skills.is_empty()),
            Some(true)
        );
    }
}
