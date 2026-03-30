use std::time::{Duration, Instant};

use crate::domain::skill::dtos::codes::SkillCode;
use crate::domain::skill::runtime::apply_runtime_continue_skill;
use crate::domain::user_player::calc::recalculate_user_player_state;
use crate::domain::user_player::state::{ContinueSkillCode, ContinueSkillInstance, UserPlayerId};

use super::super::scheduler::SchedulerEvent;
use super::super::world::World;
use super::process_skill_common::PreparedSkillCast;

pub fn apply_continue_buff_to_targets(
    world: &mut World,
    target_ids: &[UserPlayerId],
    skill_code: SkillCode,
    prepared: &PreparedSkillCast,
    from_party: bool,
) -> Vec<UserPlayerId> {
    let mut applied_targets = Vec::new();

    for target_id in target_ids {
        let Some(player) = world.players.get_mut(target_id) else {
            continue;
        };

        let apply_result = apply_runtime_continue_skill(
            player,
            &prepared.entry,
            skill_code,
            prepared.point,
            1,
            from_party,
            0,
            [0, 0],
        );
        if let Err(_error) = apply_result {
            log::debug!(
                "continue_buff_target_apply_ignored target_id={:?} skill_code=0x{:X}",
                target_id.0,
                skill_code.raw()
            );
            continue;
        }

        recalculate_user_player_state(player, Some(&world.skill_state));
        applied_targets.push(*target_id);
    }

    applied_targets
}

pub fn schedule_continue_buff_for_targets(
    world: &mut World,
    target_ids: &[UserPlayerId],
    skill_code: SkillCode,
    prepared: &PreparedSkillCast,
    cooldown_ready_at: Instant,
    now: Instant,
) {
    let Some(duration_secs) = prepared.entry.duration_secs(prepared.level) else {
        return;
    };
    if duration_secs <= 0 {
        return;
    }

    let expires_at = now + Duration::from_secs(duration_secs as u64);
    for target_id in target_ids {
        let instance_id = world.allocate_instance_id();
        let skill_snapshot = world
            .players
            .get(target_id)
            .and_then(|player| {
                player
                    .runtime
                    .active_continue_skills
                    .get(&ContinueSkillCode(skill_code))
            })
            .map(|instance| instance.skill.clone())
            .unwrap_or_default();
        let instance = ContinueSkillInstance {
            skill: skill_snapshot,
            expires_at,
            instance_id,
            cooldown_ready_at,
        };
        if let Some(player) = world.players.get_mut(target_id) {
            player.upsert_continue_skill_instance(instance);
        }
        world.scheduler.push(
            expires_at,
            SchedulerEvent::BuffExpire {
                player_id: *target_id,
                continue_skill_code: ContinueSkillCode(skill_code),
                instance_id,
            },
        );
    }
}
