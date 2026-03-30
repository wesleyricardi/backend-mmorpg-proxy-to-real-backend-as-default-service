use std::time::{Duration, Instant};

use crate::domain::skill::dtos::codes::SkillCode;
use crate::domain::user_player::state::{ContinueSkillCode, UserPlayerId};

use super::super::world::World;
use super::process_continue_buff_effect::{
    apply_continue_buff_to_targets, schedule_continue_buff_for_targets,
};
use super::process_skill_common::{finalize_skill_cast, prepare_skill_cast};

pub fn handle(
    world: &mut World,
    player_id: UserPlayerId,
    skill_code: SkillCode,
    from_party: bool,
    now: Instant,
) {
    log::debug!(
        "process_continue_skill player_id={:?} skill_code=0x{:X} from_party={}",
        player_id,
        skill_code.raw(),
        from_party
    );
    let Some(prepared) = prepare_skill_cast(world, player_id, skill_code) else {
        return;
    };

    let applied_targets =
        apply_continue_buff_to_targets(world, &[player_id], skill_code, &prepared, from_party);
    if applied_targets.is_empty() {
        log::debug!(
            "process_continue_skill ignored: validation failed skill_code=0x{:X}",
            skill_code.raw()
        );
        return;
    }

    log::debug!(
        "process_continue_skill applied player_id={:?} skill_code=0x{:X} level={} point={}",
        player_id.0,
        skill_code.raw(),
        prepared.level.get(),
        prepared.point
    );
    let Some(cooldown_ready_at) = finalize_skill_cast(world, player_id, skill_code, &prepared, now)
    else {
        return;
    };

    schedule_continue_buff_for_targets(
        world,
        &applied_targets,
        skill_code,
        &prepared,
        cooldown_ready_at,
        now,
    );
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::mpsc;
    use std::time::Instant;

    use crate::domain::field::dto::FieldId;
    use crate::domain::item::dto::EquippableItemType;
    use crate::domain::skill::dtos::codes::{SKILL_CONCENTRATION, SKILL_VIRTUAL_LIFE};
    use crate::domain::skill::dtos::skills::{
        Concentration, Levels, SkillClassConfig, SkillEntry, SkillValueType, VirtualLife,
    };
    use crate::domain::skill::state::SkillState;
    use crate::domain::user_player::dto::{
        ChangeJobTierState, PlayerJob, UserCalcOutput, UserPlayerData, UserRecordItem,
    };
    use crate::domain::user_player::state::UserPlayerState;

    use super::*;

    fn mk_levels_i32(v: i32) -> Levels<i32> {
        Levels::new(SkillValueType::Fixed2, [v; 10])
    }

    fn mk_skill_state(use_weapon_code: Vec<EquippableItemType>) -> SkillState {
        let concentration = SkillEntry::Concentration(Concentration {
            name: "Concentration".to_string(),
            description: "test".to_string(),
            require_level: 1,
            use_stamina: Some((5, 2)),
            require_mastery: Some((10, 0)),
            element: None,
            use_weapon_code,
            skill_code: SKILL_CONCENTRATION,
            attack_rate: mk_levels_i32(100),
            time: mk_levels_i32(1),
            use_mana: mk_levels_i32(20),
        });
        let mut by_code = HashMap::new();
        by_code.insert(SKILL_CONCENTRATION, concentration);
        SkillState {
            state: HashMap::from([(PlayerJob::Fighter, SkillClassConfig { by_code })]),
        }
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

    fn mk_world_and_player(
        use_weapon_code: Vec<EquippableItemType>,
    ) -> (World, UserPlayerId, Instant) {
        let (tx_net, _rx_net) = mpsc::channel();
        let (_tx_world, rx_world) = mpsc::channel();
        let mut world = World::new(mk_skill_state(use_weapon_code), rx_world, tx_net);
        let now = Instant::now();
        let mut player_state = UserPlayerState::from_record_data(
            1,
            UserPlayerData::default(),
            UserCalcOutput::default(),
        );
        player_state.core.char_info.job_code = PlayerJob::Fighter;
        player_state.core.char_info.level = 100;
        player_state.core.char_info.strength = 100;
        player_state.core.char_info.spirit = 100;
        player_state.core.char_info.talent = 100;
        player_state.core.char_info.dexterity = 100;
        player_state.core.char_info.health = 100;
        player_state.core.char_info.mana = [100, 100];
        player_state.core.char_info.stamina = [100, 100];
        player_state.progression.game_save.skill_info.b_skill_point[8] = 1;
        let player_id = UserPlayerId(123);
        world.players.insert(player_id, player_state);
        (world, player_id, now)
    }

    #[test]
    fn process_continue_skill_blocks_local_apply_in_village_safe_zone() {
        let (mut world, player_id, now) = mk_world_and_player(vec![]);
        world
            .fields
            .player_field
            .insert(player_id, FieldId::Village2);

        handle(&mut world, player_id, SKILL_CONCENTRATION, false, now);

        assert_eq!(
            world
                .players
                .get(&player_id)
                .map(|player| player.core.char_info.mana[0]),
            Some(100)
        );
        assert_eq!(
            world
                .players
                .get(&player_id)
                .map(|player| player.core.char_info.stamina[0]),
            Some(100)
        );
        assert_eq!(
            world
                .players
                .get(&player_id)
                .map(|player| player.runtime.cooldowns.contains_key(&SKILL_CONCENTRATION)),
            Some(false)
        );
    }

    #[test]
    fn process_continue_skill_applies_when_non_village_field_allows() {
        let (mut world, player_id, now) = mk_world_and_player(vec![]);
        world
            .fields
            .player_field
            .insert(player_id, FieldId::Village1);

        handle(&mut world, player_id, SKILL_CONCENTRATION, false, now);

        assert_eq!(
            world
                .players
                .get(&player_id)
                .map(|player| player.core.char_info.mana[0]),
            Some(80)
        );
        assert_eq!(
            world
                .players
                .get(&player_id)
                .map(|player| player.core.char_info.stamina[0]),
            Some(95)
        );
        assert_eq!(
            world
                .players
                .get(&player_id)
                .map(|player| player.runtime.cooldowns.contains_key(&SKILL_CONCENTRATION)),
            Some(true)
        );
    }

    #[test]
    fn process_continue_skill_blocks_when_weapon_requirement_is_not_met() {
        let (mut world, player_id, now) = mk_world_and_player(vec![EquippableItemType::WA1]);
        world
            .fields
            .player_field
            .insert(player_id, FieldId::Village1);
        if let Some(player) = world.players.get_mut(&player_id) {
            player.inventory.record_items.push(UserRecordItem {
                position: 1,
                item: crate::domain::user_player::dto::UserItem {
                    code: EquippableItemType::WS1 as u32,
                    ..Default::default()
                },
                ..Default::default()
            });
        } else {
            assert!(false, "missing player");
            return;
        }

        handle(&mut world, player_id, SKILL_CONCENTRATION, false, now);

        assert_eq!(
            world
                .players
                .get(&player_id)
                .map(|player| player.core.char_info.mana[0]),
            Some(100)
        );
        assert_eq!(
            world
                .players
                .get(&player_id)
                .map(|player| player.core.char_info.stamina[0]),
            Some(100)
        );
        assert_eq!(
            world
                .players
                .get(&player_id)
                .map(|player| player.runtime.active_continue_skills.is_empty()),
            Some(true)
        );
        assert_eq!(
            world
                .players
                .get(&player_id)
                .map(|player| player.runtime.cooldowns.contains_key(&SKILL_CONCENTRATION)),
            Some(false)
        );
    }

    #[test]
    fn process_continue_skill_materializes_virtual_life_into_char_state() {
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
        let player_id = UserPlayerId(321);
        world.players.insert(player_id, player_state);
        world
            .fields
            .player_field
            .insert(player_id, FieldId::Village1);

        handle(&mut world, player_id, SKILL_VIRTUAL_LIFE, false, now);

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
                .map(|player| player.core.char_info.life[1] > baseline),
            Some(true)
        );
        assert_eq!(
            world
                .players
                .get(&player_id)
                .and_then(|player| player
                    .runtime
                    .active_continue_skills
                    .get(&ContinueSkillCode(SKILL_VIRTUAL_LIFE)))
                .map(|skill| skill.skill.plus_state),
            Some([0, 0])
        );
    }

    #[test]
    fn process_continue_skill_refresh_keeps_newer_buff_when_older_expire_fires() {
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
        let player_id = UserPlayerId(321);
        world.players.insert(player_id, player_state);
        world
            .fields
            .player_field
            .insert(player_id, FieldId::Village1);

        handle(&mut world, player_id, SKILL_VIRTUAL_LIFE, false, now);

        let first_instance = world
            .players
            .get(&player_id)
            .and_then(|player| {
                player
                    .runtime
                    .active_continue_skills
                    .get(&ContinueSkillCode(SKILL_VIRTUAL_LIFE))
            })
            .cloned()
            .expect("first virtual life instance should exist");

        if let Some(player) = world.players.get_mut(&player_id) {
            player.runtime.cooldowns.clear();
        }

        let refreshed_at = now + Duration::from_secs(1);
        handle(
            &mut world,
            player_id,
            SKILL_VIRTUAL_LIFE,
            false,
            refreshed_at,
        );

        let second_instance = world
            .players
            .get(&player_id)
            .and_then(|player| {
                player
                    .runtime
                    .active_continue_skills
                    .get(&ContinueSkillCode(SKILL_VIRTUAL_LIFE))
            })
            .cloned()
            .expect("refreshed virtual life instance should exist");

        assert_ne!(first_instance.instance_id, second_instance.instance_id);
        assert!(second_instance.expires_at > first_instance.expires_at);

        world.tick(first_instance.expires_at);

        assert_eq!(
            world
                .players
                .get(&player_id)
                .and_then(|player| {
                    player
                        .runtime
                        .active_continue_skills
                        .get(&ContinueSkillCode(SKILL_VIRTUAL_LIFE))
                })
                .map(|instance| instance.instance_id),
            Some(second_instance.instance_id)
        );

        world.tick(second_instance.expires_at);

        assert_eq!(
            world.players.get(&player_id).map(|player| player
                .runtime
                .active_continue_skills
                .contains_key(&ContinueSkillCode(SKILL_VIRTUAL_LIFE))),
            Some(false)
        );
    }
}
