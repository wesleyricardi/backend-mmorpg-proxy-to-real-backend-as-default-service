use crate::domain::item::dto::EquippableItemType;
use crate::domain::skill::dtos::codes::{ChangeJobTier, SkillCode};
use crate::domain::skill::dtos::skills::{SkillEntry, SkillLevel};
use crate::domain::user_player::dto::{PlayerJob, UserChar, UserContinueSkill, UserItem};
use crate::domain::user_player::state::UserPlayerState;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeSkillApplyError {
    InvalidPoint,
    InsufficientMana,
    InsufficientStamina,
}

fn skill_class_slot_index(code: SkillCode) -> Option<usize> {
    let tier_base = match code.tier_enum()? {
        ChangeJobTier::Tier1 => 0_usize,
        ChangeJobTier::Tier2 => 4,
        ChangeJobTier::Tier3 => 8,
        ChangeJobTier::Tier4 => 12,
        ChangeJobTier::Tier5 => 16,
    };
    let slot = code.slot().0 as usize;
    if (1..=4).contains(&slot) {
        Some(tier_base + (slot - 1))
    } else {
        None
    }
}

pub fn skill_point_from_player(player: &UserPlayerState, code: SkillCode) -> Option<i32> {
    let idx = skill_class_slot_index(code)?;
    let point = player.progression.game_save.skill_info.b_skill_point[idx] as i32;
    if point <= 0 {
        None
    } else {
        Some(point)
    }
}

pub fn is_weapon_requirement_satisfied(player: &UserPlayerState, entry: &SkillEntry) -> bool {
    let use_weapon_codes = entry.use_weapon_codes_array();
    if use_weapon_codes.iter().all(|v| v.is_none()) {
        return true;
    }

    let Some(weapon_code) = equipped_weapon_code(player) else {
        return false;
    };

    use_weapon_codes
        .iter()
        .flatten()
        .any(|code| *code == weapon_code)
}

pub fn compute_skill_cooldown(
    player: &UserPlayerState,
    entry: &SkillEntry,
    code: SkillCode,
    point: i32,
) -> Duration {
    let Some(idx) = skill_class_slot_index(code) else {
        return Duration::from_millis(0);
    };

    let (base, step) = entry.require_mastery().unwrap_or((0, 0));
    let talent_component = (player.core.char_info.talent / 3).clamp(0, 50);
    let use_skill_mastery = (talent_component * 100
        + player.progression.game_save.skill_info.w_skill_mastery[idx] as i32)
        .clamp(0, 10000);

    let mut mastery = base + (step * point.max(1)) - (use_skill_mastery / 100);
    mastery = mastery.clamp(1, 70);

    let frames = mastery_frames(mastery);
    Duration::from_millis((frames as u64 * 1000) / 70)
}

fn equipped_weapon_code(player: &UserPlayerState) -> Option<EquippableItemType> {
    player
        .inventory
        .record_items
        .iter()
        .find(|record| {
            record.position == 1 && can_use_equipped_item(&player.core.char_info, &record.item)
        })
        .and_then(|record| EquippableItemType::try_from(record.item.code & 0xFFFF_0000).ok())
}

fn can_use_equipped_item(player: &UserChar, item: &UserItem) -> bool {
    if item.not_use_flag != 0 {
        return false;
    }
    if item.level > player.level {
        return false;
    }
    if item.dexterity > player.dexterity {
        return false;
    }
    if item.strength > player.strength {
        return false;
    }
    if item.talent > player.talent {
        return false;
    }
    if item.spirit > player.spirit {
        return false;
    }
    if item.health > player.health {
        return false;
    }
    true
}

fn mastery_frames(mastery: i32) -> i32 {
    let mut gage_length = 0;
    let mut gage_length2 = 0;
    let mut skill_count_time2 = 0;
    let temp_length = ((35.0f32 / (mastery as f32 / 2.0f32)) as i32).max(1);
    let threshold = ((35.0f32 / temp_length as f32) as i32).max(1);
    let mut frames = 0;

    while gage_length < 35 {
        gage_length2 += temp_length;
        if gage_length < gage_length2 {
            skill_count_time2 += 1;
            if skill_count_time2 >= threshold {
                gage_length += 1;
                skill_count_time2 = 0;
            }
        }
        frames += 1;
        if frames > 70 * 30 {
            break;
        }
    }

    frames
}

fn stamina_gate(
    entry: &SkillEntry,
    point: i32,
    current_stamina: i32,
) -> Result<(), RuntimeSkillApplyError> {
    let Some(required) = stamina_cost_at(entry, point) else {
        return Ok(());
    };
    if current_stamina < required {
        return Err(RuntimeSkillApplyError::InsufficientStamina);
    }
    Ok(())
}

fn stamina_cost_at(entry: &SkillEntry, point: i32) -> Option<i32> {
    let Some((base, step)) = entry.use_stamina() else {
        return None;
    };
    if base <= 0 && step <= 0 {
        return None;
    }
    Some(base + step * (point.max(1) - 1))
}

fn mana_cost_at(entry: &SkillEntry, level: SkillLevel) -> Option<i32> {
    entry.use_mana(level)
}

fn mana_gate(
    entry: &SkillEntry,
    level: SkillLevel,
    current_mana: i32,
) -> Result<(), RuntimeSkillApplyError> {
    let Some(required) = mana_cost_at(entry, level) else {
        return Ok(());
    };
    if required <= 0 {
        return Ok(());
    }
    if current_mana < required {
        return Err(RuntimeSkillApplyError::InsufficientMana);
    }
    Ok(())
}

pub fn validate_resource_requirements(
    state: &UserPlayerState,
    entry: &SkillEntry,
    point: i32,
) -> Result<SkillLevel, RuntimeSkillApplyError> {
    let Some(level) = SkillLevel::new(point.clamp(1, 10) as u8) else {
        return Err(RuntimeSkillApplyError::InvalidPoint);
    };
    if point <= 0 {
        return Err(RuntimeSkillApplyError::InvalidPoint);
    }

    let current_mana = state.core.char_info.mana[0] as i32;
    let current_stamina = state.core.char_info.stamina[0] as i32;
    mana_gate(entry, level, current_mana)?;
    stamina_gate(entry, point, current_stamina)?;

    Ok(level)
}

pub fn apply_resource_costs(
    state: &mut UserPlayerState,
    entry: &SkillEntry,
    level: SkillLevel,
    point: i32,
) {
    if let Some(required_mana) = mana_cost_at(entry, level) {
        if required_mana > 0 {
            let current = state.core.char_info.mana[0] as i32;
            let next = (current - required_mana).max(0).min(i16::MAX as i32) as i16;
            state.core.char_info.mana[0] = next;
        }
    }
    if let Some(required_stamina) = stamina_cost_at(entry, point) {
        if required_stamina > 0 {
            let current = state.core.char_info.stamina[0] as i32;
            let next = (current - required_stamina).max(0).min(i16::MAX as i32) as i16;
            state.core.char_info.stamina[0] = next;
        }
    }
}

pub fn apply_runtime_continue_skill(
    state: &mut UserPlayerState,
    entry: &SkillEntry,
    code: SkillCode,
    point: i32,
    flag: i32,
    party_flag: bool,
    element_index: i32,
    plus_state: [i32; 2],
) -> Result<UserContinueSkill, RuntimeSkillApplyError> {
    let Some(_level) = SkillLevel::new(point.clamp(1, 10) as u8) else {
        return Err(RuntimeSkillApplyError::InvalidPoint);
    };
    if point <= 0 {
        return Err(RuntimeSkillApplyError::InvalidPoint);
    }

    let skill = UserContinueSkill {
        code,
        point,
        flag,
        party_flag,
        element_index,
        plus_state,
        use_weapon_codes: entry.use_weapon_codes_array(),
    };
    state.upsert_active_continue_skill(skill.clone());
    Ok(skill)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::skill::dtos::codes::SKILL_CONCENTRATION;
    use crate::domain::skill::dtos::skills::{
        Concentration, Levels, SkillClassConfig, SkillValueType,
    };
    use crate::domain::skill::state::SkillState;
    use std::collections::HashMap;

    fn mk_levels_i32(v: i32) -> Levels<i32> {
        Levels::new(SkillValueType::Fixed2, [v; 10])
    }

    fn mk_state(
        mana: i16,
        stamina: i16,
        mastery: u16,
        require_mastery: Option<(i32, i32)>,
    ) -> (UserPlayerState, SkillState) {
        let mut player_state = UserPlayerState::from_record_data(
            1,
            crate::domain::user_player::dto::UserPlayerData::default(),
            crate::domain::user_player::dto::UserCalcOutput::default(),
        );
        player_state.core.char_info.job_code = PlayerJob::Fighter;
        player_state.core.char_info.mana = [mana, mana];
        player_state.core.char_info.stamina = [stamina, stamina];
        player_state
            .progression
            .game_save
            .skill_info
            .w_skill_mastery[8] = mastery;

        let concentration = SkillEntry::Concentration(Concentration {
            name: "Concentration".to_string(),
            description: "test".to_string(),
            require_level: 1,
            use_stamina: Some((5, 2)),
            require_mastery,
            element: None,
            use_weapon_code: vec![],
            skill_code: SKILL_CONCENTRATION,
            attack_rate: mk_levels_i32(100),
            time: mk_levels_i32(1),
            use_mana: mk_levels_i32(20),
        });
        let mut by_code = HashMap::new();
        by_code.insert(SKILL_CONCENTRATION, concentration);
        let skill_state = SkillState {
            state: HashMap::from([(PlayerJob::Fighter, SkillClassConfig { by_code })]),
        };
        (player_state, skill_state)
    }

    #[test]
    fn runtime_skill_blocks_for_low_mana() {
        let (player_state, skill_state) = mk_state(10, 100, 0, None);
        let entry = skill_state.get_skill(PlayerJob::Fighter, SKILL_CONCENTRATION);
        assert!(entry.is_some());
        let entry = match entry {
            Some(entry) => entry,
            None => {
                assert!(false, "missing skill entry");
                return;
            }
        };
        let result = validate_resource_requirements(&player_state, entry, 1);
        assert_eq!(result, Err(RuntimeSkillApplyError::InsufficientMana));
    }

    #[test]
    fn runtime_skill_blocks_for_low_stamina() {
        let (player_state, skill_state) = mk_state(100, 1, 0, None);
        let entry = skill_state.get_skill(PlayerJob::Fighter, SKILL_CONCENTRATION);
        assert!(entry.is_some());
        let entry = match entry {
            Some(entry) => entry,
            None => {
                assert!(false, "missing skill entry");
                return;
            }
        };
        let result = validate_resource_requirements(&player_state, entry, 2);
        assert_eq!(result, Err(RuntimeSkillApplyError::InsufficientStamina));
    }

    #[test]
    fn runtime_skill_upserts_when_all_specific_validations_pass() {
        let (mut player_state, skill_state) = mk_state(100, 100, 1200, Some((10, 0)));
        let entry = skill_state.get_skill(PlayerJob::Fighter, SKILL_CONCENTRATION);
        assert!(entry.is_some());
        let entry = match entry {
            Some(entry) => entry.clone(),
            None => {
                assert!(false, "missing skill entry");
                return;
            }
        };
        let result = apply_runtime_continue_skill(
            &mut player_state,
            &entry,
            SKILL_CONCENTRATION,
            1,
            1,
            false,
            0,
            [0, 0],
        );
        assert!(result.is_ok());
        assert_eq!(player_state.runtime.active_continue_skills.len(), 1);
        let has_skill = player_state
            .runtime
            .active_continue_skills
            .values()
            .any(|v| v.skill.code == SKILL_CONCENTRATION);
        assert!(has_skill);
        assert_eq!(player_state.core.char_info.mana[0], 100);
        assert_eq!(player_state.core.char_info.stamina[0], 100);
    }

    #[test]
    fn apply_resource_costs_consumes_mana_and_stamina_after_validation() {
        let (mut player_state, skill_state) = mk_state(100, 100, 1200, Some((10, 0)));
        let entry = skill_state.get_skill(PlayerJob::Fighter, SKILL_CONCENTRATION);
        assert!(entry.is_some());
        let entry = match entry {
            Some(entry) => entry,
            None => {
                assert!(false, "missing skill entry");
                return;
            }
        };
        let level = validate_resource_requirements(&player_state, entry, 1);
        assert!(level.is_ok());
        let level = match level {
            Ok(level) => level,
            Err(error) => {
                assert!(false, "unexpected valid costs failure: {error:?}");
                return;
            }
        };
        apply_resource_costs(&mut player_state, entry, level, 1);
        assert_eq!(player_state.core.char_info.mana[0], 80);
        assert_eq!(player_state.core.char_info.stamina[0], 95);
    }
}
