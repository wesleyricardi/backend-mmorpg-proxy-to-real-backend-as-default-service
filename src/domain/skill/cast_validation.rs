use crate::domain::field::dto::FieldId;
pub use crate::domain::skill::cast_validation_party::{
    validate_party_skill_cast, PartySkillCastValidationContext, PartySkillValidationTarget,
};
pub use crate::domain::skill::cast_validation_target_buff::{
    validate_target_buff_cast, TargetBuffCastValidationContext, TargetBuffValidationTarget,
};
use crate::domain::skill::dtos::codes::SkillCode;
use crate::domain::skill::dtos::skills::{SkillEntry, SkillLevel};
use crate::domain::skill::runtime::{
    is_weapon_requirement_satisfied, validate_resource_requirements, RuntimeSkillApplyError,
};
use crate::domain::user_player::state::UserPlayerState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillCastValidationError {
    MissingFieldContext,
    FieldPolicyBlocked,
    CooldownNotReady,
    WeaponRequirement,
    PartyTargetOutOfRange,
    TargetBuffMissingTarget,
    TargetBuffOutOfRange,
    InvalidPoint,
    InsufficientMana,
    InsufficientStamina,
}

#[derive(Debug, Clone, Copy)]
pub struct CommonSkillCastValidationInput<'a> {
    pub player: &'a UserPlayerState,
    pub skill_code: SkillCode,
    pub point: i32,
    pub entry: &'a SkillEntry,
    pub field_id: Option<FieldId>,
}

#[derive(Debug, Clone, Copy)]
pub struct SkillCastValidationInput<'a> {
    pub common: CommonSkillCastValidationInput<'a>,
    pub context: SkillCastValidationContext<'a>,
}

#[derive(Debug, Clone, Copy)]
pub enum SkillCastValidationContext<'a> {
    ContinueSelfBuff,
    TargetBuff(TargetBuffCastValidationContext),
    PartyBuff(PartySkillCastValidationContext<'a>),
    SingleTargetAttack,
    AreaAttack,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedSkillCast {
    pub level: SkillLevel,
}

pub fn validate_common_skill_cast(
    input: CommonSkillCastValidationInput<'_>,
) -> Result<ValidatedSkillCast, SkillCastValidationError> {
    validate_field_policy(input.field_id)?;

    if input.player.is_skill_on_cooldown(input.skill_code) {
        return Err(SkillCastValidationError::CooldownNotReady);
    }

    if !is_weapon_requirement_satisfied(input.player, input.entry) {
        return Err(SkillCastValidationError::WeaponRequirement);
    }

    let level = validate_resource_requirements(input.player, input.entry, input.point)
        .map_err(map_runtime_error)?;

    Ok(ValidatedSkillCast { level })
}

pub fn validate_skill_cast(
    input: SkillCastValidationInput<'_>,
) -> Result<ValidatedSkillCast, SkillCastValidationError> {
    let validated = validate_common_skill_cast(input.common)?;

    match input.context {
        SkillCastValidationContext::ContinueSelfBuff => {
            unimplemented!("continue/self buff specific validation not implemented yet")
        }
        SkillCastValidationContext::TargetBuff(ctx) => validate_target_buff_cast(ctx, validated),
        SkillCastValidationContext::PartyBuff(ctx) => validate_party_skill_cast(ctx, validated),
        SkillCastValidationContext::SingleTargetAttack => {
            unimplemented!("single target attack validation not implemented yet")
        }
        SkillCastValidationContext::AreaAttack => {
            unimplemented!("area attack validation not implemented yet")
        }
    }
}

fn validate_field_policy(field_id: Option<FieldId>) -> Result<(), SkillCastValidationError> {
    let Some(field_id) = field_id else {
        return Err(SkillCastValidationError::MissingFieldContext);
    };
    if !field_id
        .get_catalog_entry()
        .policy
        .allows_skill_restrictive()
    {
        return Err(SkillCastValidationError::FieldPolicyBlocked);
    }
    Ok(())
}

fn map_runtime_error(error: RuntimeSkillApplyError) -> SkillCastValidationError {
    match error {
        RuntimeSkillApplyError::InvalidPoint => SkillCastValidationError::InvalidPoint,
        RuntimeSkillApplyError::InsufficientMana => SkillCastValidationError::InsufficientMana,
        RuntimeSkillApplyError::InsufficientStamina => {
            SkillCastValidationError::InsufficientStamina
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::domain::field::dto::FieldId;
    use crate::domain::skill::dtos::codes::SKILL_CONCENTRATION;
    use crate::domain::skill::dtos::skills::{
        Concentration, Levels, SkillClassConfig, SkillEntry, SkillValueType,
    };
    use crate::domain::user_player::dto::{PlayerJob, UserCalcOutput, UserPlayerData};
    use crate::domain::user_player::state::UserPlayerState;

    use super::*;

    fn mk_levels_i32(v: i32) -> Levels<i32> {
        Levels::new(SkillValueType::Fixed2, [v; 10])
    }

    fn mk_player() -> UserPlayerState {
        let mut player = UserPlayerState::from_record_data(
            1,
            UserPlayerData::default(),
            UserCalcOutput::default(),
        );
        player.core.char_info.job_code = PlayerJob::Fighter;
        player.core.char_info.level = 100;
        player.core.char_info.strength = 100;
        player.core.char_info.spirit = 100;
        player.core.char_info.talent = 100;
        player.core.char_info.dexterity = 100;
        player.core.char_info.health = 100;
        player.core.char_info.mana = [100, 100];
        player.core.char_info.stamina = [100, 100];
        player.progression.game_save.skill_info.b_skill_point[8] = 1;
        player
    }

    fn mk_entry() -> SkillEntry {
        SkillEntry::Concentration(Concentration {
            name: "Concentration".to_string(),
            description: "test".to_string(),
            require_level: 1,
            use_stamina: Some((5, 2)),
            require_mastery: Some((10, 0)),
            element: None,
            use_weapon_code: vec![],
            skill_code: SKILL_CONCENTRATION,
            attack_rate: mk_levels_i32(100),
            time: mk_levels_i32(1),
            use_mana: mk_levels_i32(20),
        })
    }

    #[test]
    fn validation_blocks_safe_field() {
        let player = mk_player();
        let result = validate_common_skill_cast(CommonSkillCastValidationInput {
            player: &player,
            skill_code: SKILL_CONCENTRATION,
            point: 1,
            entry: &mk_entry(),
            field_id: Some(FieldId::Village2),
        });
        assert_eq!(result, Err(SkillCastValidationError::FieldPolicyBlocked));
    }

    #[test]
    fn validation_accepts_allowed_field_and_resources() {
        let player = mk_player();
        let result = validate_common_skill_cast(CommonSkillCastValidationInput {
            player: &player,
            skill_code: SKILL_CONCENTRATION,
            point: 1,
            entry: &mk_entry(),
            field_id: Some(FieldId::Village1),
        });
        assert!(result.is_ok());
    }
}
