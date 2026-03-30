use std::time::Instant;

use crate::debug_ui;
use crate::domain::user_player::char_motion_state::CharMotionState;
use crate::domain::user_player::dto::UserChar;
use crate::domain::user_player::state::UserPlayerId;

use super::super::scheduler::SchedulerEvent;
use super::super::world::World;
use super::cancel_skill;

const PLAYCOUNT_PER_SECOND: f32 = 70.0;

pub fn handle_event(world: &mut World, now: Instant, event: SchedulerEvent) {
    match event {
        SchedulerEvent::BuffExpire {
            player_id,
            continue_skill_code,
            instance_id,
        } => {
            let previous_version = world.player_state_version(player_id);
            let code_to_remove = world.players.get(&player_id).and_then(|player| {
                let should_remove = player
                    .runtime
                    .active_continue_skills
                    .get(&continue_skill_code)
                    .map(|instance| instance.instance_id == instance_id)
                    .unwrap_or(false);
                should_remove.then(|| {
                    player
                        .runtime
                        .active_continue_skills
                        .get(&continue_skill_code)
                        .map(|v| v.skill.code)
                })?
            });
            if let Some(code) = code_to_remove {
                cancel_skill::handle(world, player_id, code);
            }
            world.spawn_debug_player_dump_if_changed(player_id, previous_version, "buff_expired");
        }
        SchedulerEvent::CooldownReady {
            player_id,
            skill_code,
        } => {
            if let Some(player) = world.players.get_mut(&player_id) {
                player.clear_skill_cooldown_if_ready(skill_code);
            }
        }
        SchedulerEvent::RegenTick { player_id } => {
            if let Some(player) = world.players.get_mut(&player_id) {
                let regen_per_tick = regen_per_tick(&player.core.char_info);
                let life_max = player.core.char_info.life[1];
                let mana_max = player.core.char_info.mana[1];
                let stamina_max = player.core.char_info.stamina[1];

                player.runtime.regen_acc_life += regen_per_tick.life;
                player.runtime.regen_acc_mana += regen_per_tick.mana;
                player.runtime.regen_acc_stamina += regen_per_tick.stamina;
                apply_resource_acc(
                    &mut player.core.char_info.life[0],
                    life_max,
                    &mut player.runtime.regen_acc_life,
                );
                apply_resource_acc(
                    &mut player.core.char_info.mana[0],
                    mana_max,
                    &mut player.runtime.regen_acc_mana,
                );
                apply_resource_acc(
                    &mut player.core.char_info.stamina[0],
                    stamina_max,
                    &mut player.runtime.regen_acc_stamina,
                );
            }

            world.schedule_regen_for_player(player_id, now);
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct RegenTickAmounts {
    life: f32,
    mana: f32,
    stamina: f32,
}

fn regen_per_tick(char_info: &UserChar) -> RegenTickAmounts {
    let level = char_info.level as f32;
    let strength = char_info.strength as f32;
    let health = char_info.health as f32;
    let spirit = char_info.spirit as f32;

    let life = if !char_info.state.is_dead() {
        ((((level + (strength / 2.0) + health) / 180.0) + char_info.life_regen) / 1.5)
            / PLAYCOUNT_PER_SECOND
    } else {
        0.0
    };

    let mana = if !char_info.state.is_dead() {
        (((level + (spirit * 1.2) + (health / 2.0)) / 115.0) + char_info.mana_regen)
            / PLAYCOUNT_PER_SECOND
    } else {
        0.0
    };

    let stamina_increase = match char_info.state {
        CharMotionState::Stand => {
            (stamina_base(char_info.stamina_function, level) + char_info.stamina_regen)
                / PLAYCOUNT_PER_SECOND
        }
        CharMotionState::Walk => {
            ((stamina_base(char_info.stamina_function, level) + char_info.stamina_regen) * 0.6)
                / PLAYCOUNT_PER_SECOND
        }
        _ => 0.0,
    };

    let stamina_decrease = if char_info.state == CharMotionState::Run {
        (((1000.0 + char_info.weight[0] as f32)
            / (char_info.weight[1] as f32 + (strength / 2.0) + 500.0))
            + 0.4)
            / PLAYCOUNT_PER_SECOND
    } else {
        0.0
    };

    RegenTickAmounts {
        life,
        mana,
        stamina: stamina_increase - stamina_decrease,
    }
}

fn stamina_base(stamina_function: i32, level: f32) -> f32 {
    match stamina_function {
        1 => 3.8 + (level / 7.0),
        2 => 3.3 + (level / 7.0),
        3 => 2.9 + (level / 7.0),
        _ => 3.8 + (level / 7.0),
    }
}

fn apply_resource_acc(current: &mut i16, max: i16, acc: &mut f32) {
    if *acc > -1.0 && *acc < 1.0 {
        return;
    }

    let whole = if *acc >= 1.0 {
        acc.floor() as i16
    } else {
        acc.ceil() as i16
    };
    *acc -= whole as f32;
    let next = (*current as i32 + whole as i32).clamp(0, max as i32);
    *current = next as i16;
}
