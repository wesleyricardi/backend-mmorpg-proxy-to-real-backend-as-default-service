use super::cast_validation::{SkillCastValidationError, ValidatedSkillCast};
use crate::domain::user_player::state::UserPlayerId;

#[derive(Debug, Clone, Copy)]
pub struct TargetBuffValidationTarget {
    pub player_id: UserPlayerId,
    pub position: Option<(i32, i32, i32)>,
}

#[derive(Debug, Clone, Copy)]
pub struct TargetBuffCastValidationContext {
    pub caster_position: Option<(i32, i32, i32)>,
    pub max_range_sq: Option<i64>,
    pub target: Option<TargetBuffValidationTarget>,
}

pub fn validate_target_buff_cast(
    ctx: TargetBuffCastValidationContext,
    validated: ValidatedSkillCast,
) -> Result<ValidatedSkillCast, SkillCastValidationError> {
    let Some(target) = ctx.target else {
        return Err(SkillCastValidationError::TargetBuffMissingTarget);
    };

    let _ = target.player_id;

    if let (Some(caster), Some(target), Some(max_range_sq)) =
        (ctx.caster_position, target.position, ctx.max_range_sq)
    {
        let dx = i64::from(caster.0) - i64::from(target.0);
        let dy = i64::from(caster.1) - i64::from(target.1);
        let dz = i64::from(caster.2) - i64::from(target.2);
        let dist_sq = dx * dx + dy * dy + dz * dz;
        if dist_sq > max_range_sq {
            return Err(SkillCastValidationError::TargetBuffOutOfRange);
        }
    }

    Ok(validated)
}
