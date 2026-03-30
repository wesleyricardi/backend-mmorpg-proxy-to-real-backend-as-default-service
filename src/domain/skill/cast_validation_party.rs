use crate::domain::skill::cast_validation::{SkillCastValidationError, ValidatedSkillCast};
use crate::domain::user_player::state::UserPlayerId;

#[derive(Debug, Clone, Copy)]
pub struct PartySkillCastValidationContext<'a> {
    pub caster_position: Option<(i32, i32, i32)>,
    pub max_range_sq: Option<i64>,
    pub targets: &'a [PartySkillValidationTarget],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PartySkillValidationTarget {
    pub player_id: UserPlayerId,
    pub position: Option<(i32, i32, i32)>,
}

pub fn validate_party_skill_cast(
    context: PartySkillCastValidationContext<'_>,
    validated: ValidatedSkillCast,
) -> Result<ValidatedSkillCast, SkillCastValidationError> {
    let Some(max_range_sq) = context.max_range_sq else {
        return Ok(validated);
    };
    let Some(caster_position) = context.caster_position else {
        return Ok(validated);
    };

    for target in context.targets {
        let Some(target_position) = target.position else {
            continue;
        };
        if distance_sq(caster_position, target_position) > max_range_sq {
            return Err(SkillCastValidationError::PartyTargetOutOfRange);
        }
    }

    Ok(validated)
}

fn distance_sq(left: (i32, i32, i32), right: (i32, i32, i32)) -> i64 {
    let dx = i64::from(left.0) - i64::from(right.0);
    let dy = i64::from(left.1) - i64::from(right.1);
    let dz = i64::from(left.2) - i64::from(right.2);
    dx * dx + dy * dy + dz * dz
}

#[cfg(test)]
mod tests {
    use crate::domain::field::dto::FieldId;
    use crate::domain::skill::cast_validation::{
        validate_skill_cast, CommonSkillCastValidationInput, SkillCastValidationContext,
        SkillCastValidationError, SkillCastValidationInput,
    };
    use crate::domain::skill::dtos::codes::SKILL_CONCENTRATION;
    use crate::domain::skill::dtos::skills::{Concentration, Levels, SkillEntry, SkillValueType};
    use crate::domain::user_player::dto::{PlayerJob, UserCalcOutput, UserPlayerData};
    use crate::domain::user_player::state::{UserPlayerId, UserPlayerState};

    use super::{PartySkillCastValidationContext, PartySkillValidationTarget};

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
    fn party_validation_blocks_target_out_of_range_when_range_context_exists() {
        let player = mk_player();
        let result = validate_skill_cast(SkillCastValidationInput {
            common: CommonSkillCastValidationInput {
                player: &player,
                skill_code: SKILL_CONCENTRATION,
                point: 1,
                entry: &mk_entry(),
                field_id: Some(FieldId::Village1),
            },
            context: SkillCastValidationContext::PartyBuff(PartySkillCastValidationContext {
                caster_position: Some((0, 0, 0)),
                max_range_sq: Some(25),
                targets: &[PartySkillValidationTarget {
                    player_id: UserPlayerId(99),
                    position: Some((10, 0, 0)),
                }],
            }),
        });
        assert_eq!(result, Err(SkillCastValidationError::PartyTargetOutOfRange));
    }
}
