use std::time::Instant;

use crate::application::use_case::runtime::world_runtime::world_command::{
    PartySkillCast, PartySkillContext,
};
use crate::domain::skill::cast_validation::{
    validate_skill_cast, CommonSkillCastValidationInput, PartySkillCastValidationContext,
    PartySkillValidationTarget, SkillCastValidationContext,
};
use crate::domain::skill::classification::is_party_skill;
use crate::domain::skill::dtos::codes::{SkillCode, SKILL_HALL_OF_VALHALLA};
use crate::domain::user_player::state::UserPlayerId;

use super::super::world::World;
use super::process_continue_buff_effect::{
    apply_continue_buff_to_targets, schedule_continue_buff_for_targets,
};
use super::process_skill_common::{finalize_skill_cast, prepare_skill_cast};

pub fn handle(world: &mut World, cast: PartySkillCast, now: Instant) {
    log::debug!(
        "process_party_skill caster_id={:?} skill_code=0x{:X} point={} targets={} context={:?}",
        cast.caster_id,
        cast.skill_code.raw(),
        cast.point,
        cast.targets.len(),
        cast.context
    );

    if !is_party_skill(cast.skill_code) {
        log::debug!(
            "process_party_skill ignored: non-party skill skill_code=0x{:X}",
            cast.skill_code.raw()
        );
        return;
    }

    if !is_context_compatible(cast.skill_code, cast.context) {
        log::debug!(
            "process_party_skill ignored: incompatible context skill_code=0x{:X} context={:?}",
            cast.skill_code.raw(),
            cast.context
        );
        return;
    }

    let target_ids = normalize_targets(cast.caster_id, &cast.targets);
    let Some(prepared) = prepare_skill_cast(world, cast.caster_id, cast.skill_code) else {
        return;
    };

    if prepared.point != cast.point {
        log::debug!(
            "process_party_skill payload point differs from caster state skill_code=0x{:X} payload_point={} state_point={}",
            cast.skill_code.raw(),
            cast.point,
            prepared.point
        );
    }

    let Some(caster) = world.players.get(&cast.caster_id) else {
        return;
    };
    let field_id = world.fields.player_field.get(&cast.caster_id).copied();
    let caster_position = world.fields.player_position.get(&cast.caster_id).copied();
    let party_targets = target_ids
        .iter()
        .map(|player_id| PartySkillValidationTarget {
            player_id: *player_id,
            position: world.fields.player_position.get(player_id).copied(),
        })
        .collect::<Vec<_>>();

    if let Err(error) = validate_skill_cast(
        crate::domain::skill::cast_validation::SkillCastValidationInput {
            common: CommonSkillCastValidationInput {
                player: caster,
                skill_code: cast.skill_code,
                point: prepared.point,
                entry: &prepared.entry,
                field_id,
            },
            context: SkillCastValidationContext::PartyBuff(PartySkillCastValidationContext {
                caster_position,
                max_range_sq: None,
                targets: &party_targets,
            }),
        },
    ) {
        log::debug!(
            "process_party_skill blocked by domain validation skill_code=0x{:X} error={:?}",
            cast.skill_code.raw(),
            error
        );
        return;
    }

    let applied_targets =
        apply_continue_buff_to_targets(world, &target_ids, cast.skill_code, &prepared, true);

    let Some(cooldown_ready_at) =
        finalize_skill_cast(world, cast.caster_id, cast.skill_code, &prepared, now)
    else {
        return;
    };

    schedule_continue_buff_for_targets(
        world,
        &applied_targets,
        cast.skill_code,
        &prepared,
        cooldown_ready_at,
        now,
    );
}

fn is_context_compatible(skill_code: SkillCode, context: PartySkillContext) -> bool {
    match (skill_code, context) {
        (SKILL_HALL_OF_VALHALLA, PartySkillContext::HallOfValhalla { .. }) => true,
        (SKILL_HALL_OF_VALHALLA, PartySkillContext::StandardBuff) => false,
        (_, PartySkillContext::StandardBuff) => true,
        (_, PartySkillContext::HallOfValhalla { .. }) => false,
    }
}

fn normalize_targets(caster_id: UserPlayerId, raw_targets: &[UserPlayerId]) -> Vec<UserPlayerId> {
    let mut targets = Vec::new();
    for player_id in raw_targets {
        if !targets.contains(player_id) {
            targets.push(*player_id);
        }
    }
    if targets.is_empty() {
        targets.push(caster_id);
    }
    targets
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::mpsc;

    use crate::domain::field::dto::FieldId;
    use crate::domain::skill::dtos::codes::SKILL_VIRTUAL_LIFE;
    use crate::domain::skill::dtos::skills::{
        Levels, SkillClassConfig, SkillEntry, SkillValueType, VirtualLife,
    };
    use crate::domain::skill::state::SkillState;
    use crate::domain::user_player::dto::{
        ChangeJobTierState, PlayerJob, UserCalcOutput, UserPlayerData,
    };
    use crate::domain::user_player::state::{ContinueSkillCode, UserPlayerState};

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
            use_mana: mk_levels_i32(10),
        });
        let mut by_code = HashMap::new();
        by_code.insert(SKILL_VIRTUAL_LIFE, virtual_life);
        SkillState {
            state: HashMap::from([(PlayerJob::Priestess, SkillClassConfig { by_code })]),
        }
    }

    fn mk_player_state(point: u8) -> UserPlayerState {
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
        player_state.core.char_info.life = [1000, 1000];
        player_state.core.char_info.mana = [100, 100];
        player_state.progression.game_save.skill_info.b_skill_point[11] = point;
        player_state
    }

    #[test]
    fn process_party_skill_applies_to_caster_and_member() {
        let (tx_net, _rx_net) = mpsc::channel();
        let (_tx_world, rx_world) = mpsc::channel();
        let mut world = World::new(mk_virtual_life_state(), rx_world, tx_net);
        let now = Instant::now();

        let caster_id = UserPlayerId(321);
        let member_id = UserPlayerId(654);
        world.players.insert(caster_id, mk_player_state(10));
        world.players.insert(member_id, mk_player_state(0));
        world
            .fields
            .player_field
            .insert(caster_id, FieldId::Village1);
        world
            .fields
            .player_field
            .insert(member_id, FieldId::Village1);

        handle(
            &mut world,
            PartySkillCast {
                caster_id,
                skill_code: SKILL_VIRTUAL_LIFE,
                point: 10,
                context: PartySkillContext::StandardBuff,
                targets: vec![caster_id, member_id],
            },
            now,
        );

        assert_eq!(
            world.players.get(&caster_id).map(|player| player
                .runtime
                .active_continue_skills
                .contains_key(&ContinueSkillCode(SKILL_VIRTUAL_LIFE))),
            Some(true)
        );
        assert_eq!(
            world.players.get(&member_id).map(|player| player
                .runtime
                .active_continue_skills
                .contains_key(&ContinueSkillCode(SKILL_VIRTUAL_LIFE))),
            Some(true)
        );
        assert_eq!(
            world
                .players
                .get(&caster_id)
                .map(|player| player.core.char_info.mana[0]),
            Some(90)
        );
        assert_eq!(
            world
                .players
                .get(&member_id)
                .map(|player| player.core.char_info.mana[0]),
            Some(100)
        );
    }
}
