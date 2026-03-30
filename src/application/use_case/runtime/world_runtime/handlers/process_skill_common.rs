use std::time::Instant;

use crate::domain::skill::cast_validation::{
    validate_common_skill_cast, CommonSkillCastValidationInput, SkillCastValidationError,
};
use crate::domain::skill::dtos::codes::SkillCode;
use crate::domain::skill::dtos::skills::{SkillEntry, SkillLevel};
use crate::domain::skill::runtime::{
    apply_resource_costs, compute_skill_cooldown, skill_point_from_player,
};
use crate::domain::user_player::state::UserPlayerId;

use super::super::scheduler::SchedulerEvent;
use super::super::world::World;

#[derive(Debug, Clone)]
pub struct PreparedSkillCast {
    pub point: i32,
    pub level: SkillLevel,
    pub entry: SkillEntry,
}

pub fn prepare_skill_cast(
    world: &World,
    player_id: UserPlayerId,
    skill_code: SkillCode,
) -> Option<PreparedSkillCast> {
    let Some(player_ro) = world.players.get(&player_id) else {
        log::debug!("process_skill ignored: missing player_id={:?}", player_id.0);
        return None;
    };

    let Some(point) = skill_point_from_player(player_ro, skill_code) else {
        log::debug!("process_skill ignored: no point for {:?}", skill_code);
        return None;
    };

    let Some(entry) = world
        .skill_state
        .get_skill(player_ro.core.char_info.job_code, skill_code)
        .cloned()
    else {
        log::debug!(
            "process_skill ignored: missing config skill_code=0x{:X}",
            skill_code.raw()
        );
        return None;
    };

    let field_id = world.fields.player_field.get(&player_id).copied();
    let level = match validate_common_skill_cast(CommonSkillCastValidationInput {
        player: player_ro,
        skill_code,
        point,
        entry: &entry,
        field_id,
    }) {
        Ok(validated) => validated.level,
        Err(error) => {
            log_skill_cast_validation_failure(player_id, skill_code, point, error);
            return None;
        }
    };

    Some(PreparedSkillCast {
        point,
        level,
        entry,
    })
}

pub fn finalize_skill_cast(
    world: &mut World,
    player_id: UserPlayerId,
    skill_code: SkillCode,
    prepared: &PreparedSkillCast,
    now: Instant,
) -> Option<Instant> {
    let Some(player) = world.players.get_mut(&player_id) else {
        return None;
    };

    apply_resource_costs(player, &prepared.entry, prepared.level, prepared.point);

    let cooldown_duration =
        compute_skill_cooldown(player, &prepared.entry, skill_code, prepared.point);
    let cooldown_ready_at = now + cooldown_duration;
    player.set_skill_cooldown(skill_code, cooldown_ready_at);
    world.scheduler.push(
        cooldown_ready_at,
        SchedulerEvent::CooldownReady {
            player_id,
            skill_code,
        },
    );
    Some(cooldown_ready_at)
}

fn log_skill_cast_validation_failure(
    player_id: UserPlayerId,
    skill_code: SkillCode,
    point: i32,
    error: SkillCastValidationError,
) {
    match error {
        SkillCastValidationError::MissingFieldContext => {
            log::debug!("field_missing_player_context player_id={:?}", player_id.0);
        }
        SkillCastValidationError::FieldPolicyBlocked => {
            log::debug!(
                "field_policy_blocked player_id={:?} skill_code=0x{:X}",
                player_id.0,
                skill_code.raw()
            );
        }
        SkillCastValidationError::CooldownNotReady => {
            log::debug!(
                "cooldown_not_ready player_id={:?} skill_code=0x{:X}",
                player_id.0,
                skill_code.raw()
            );
        }
        SkillCastValidationError::WeaponRequirement => {
            log::debug!(
                "process_skill blocked by weapon requirement player_id={:?} skill_code=0x{:X}",
                player_id.0,
                skill_code.raw()
            );
        }
        SkillCastValidationError::PartyTargetOutOfRange => {
            log::debug!(
                "party_target_out_of_range player_id={:?} skill_code=0x{:X}",
                player_id.0,
                skill_code.raw()
            );
        }
        SkillCastValidationError::TargetBuffMissingTarget => {
            log::debug!(
                "target_buff_missing_target player_id={:?} skill_code=0x{:X}",
                player_id.0,
                skill_code.raw()
            );
        }
        SkillCastValidationError::TargetBuffOutOfRange => {
            log::debug!(
                "target_buff_out_of_range player_id={:?} skill_code=0x{:X}",
                player_id.0,
                skill_code.raw()
            );
        }
        SkillCastValidationError::InsufficientMana => {
            log::debug!(
                "insufficient_mana player_id={:?} skill_code=0x{:X}",
                player_id.0,
                skill_code.raw()
            );
        }
        SkillCastValidationError::InsufficientStamina => {
            log::debug!(
                "insufficient_stamina player_id={:?} skill_code=0x{:X}",
                player_id.0,
                skill_code.raw()
            );
        }
        SkillCastValidationError::InvalidPoint => {
            log::debug!(
                "invalid_point player_id={:?} skill_code=0x{:X} point={}",
                player_id.0,
                skill_code.raw(),
                point
            );
        }
    }
}
