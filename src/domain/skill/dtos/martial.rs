use core::convert::TryFrom;

use crate::domain::skill::dtos::codes::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MartialSkills {
    Tier1(MartialTier1Skills),
    Tier2(MartialTier2Skills),
    Tier3(MartialTier3Skills),
    Tier4(MartialTier4Skills),
    Tier5(MartialTier5Skills),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MartialTier1Skills {
    LowKick,
    SMastery,
    Dbblow,
    HStraight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MartialTier2Skills {
    RageUp,
    Patriot,
    RElbow,
    SMastery2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MartialTier3Skills {
    IBulkup,
    TCannon,
    WarCry,
    JHeelkick,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MartialTier4Skills {
    Combination,
    Steelers,
    BCheck,
    Typhoon,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MartialTier5Skills {
    DMastery2,
    HHawk,
    LBreaking,
    HTraining,
}

impl MartialSkills {
    #[must_use]
    pub const fn tier(self) -> ChangeJobTier {
        match self {
            Self::Tier1(_) => ChangeJobTier::Tier1,
            Self::Tier2(_) => ChangeJobTier::Tier2,
            Self::Tier3(_) => ChangeJobTier::Tier3,
            Self::Tier4(_) => ChangeJobTier::Tier4,
            Self::Tier5(_) => ChangeJobTier::Tier5,
        }
    }
}

impl From<MartialTier1Skills> for SkillCode {
    fn from(v: MartialTier1Skills) -> SkillCode {
        match v {
            MartialTier1Skills::LowKick => SKILL_LOW_KICK,
            MartialTier1Skills::SMastery => SKILL_S_MASTERY,
            MartialTier1Skills::Dbblow => SKILL_DBBLOW,
            MartialTier1Skills::HStraight => SKILL_H_STRAIGHT,
        }
    }
}

impl From<MartialTier2Skills> for SkillCode {
    fn from(v: MartialTier2Skills) -> SkillCode {
        match v {
            MartialTier2Skills::RageUp => SKILL_RAGE_UP,
            MartialTier2Skills::Patriot => SKILL_PATRIOT,
            MartialTier2Skills::RElbow => SKILL_R_ELBOW,
            MartialTier2Skills::SMastery2 => SKILL_S_MASTERY2,
        }
    }
}

impl From<MartialTier3Skills> for SkillCode {
    fn from(v: MartialTier3Skills) -> SkillCode {
        match v {
            MartialTier3Skills::IBulkup => SKILL_I_BULKUP,
            MartialTier3Skills::TCannon => SKILL_T_CANNON,
            MartialTier3Skills::WarCry => SKILL_WAR_CRY,
            MartialTier3Skills::JHeelkick => SKILL_J_HEELKICK,
        }
    }
}

impl From<MartialTier4Skills> for SkillCode {
    fn from(v: MartialTier4Skills) -> SkillCode {
        match v {
            MartialTier4Skills::Combination => SKILL_COMBINATION,
            MartialTier4Skills::Steelers => SKILL_STEELERS,
            MartialTier4Skills::BCheck => SKILL_B_CHECK,
            MartialTier4Skills::Typhoon => SKILL_TYPHOON,
        }
    }
}

impl From<MartialTier5Skills> for SkillCode {
    fn from(v: MartialTier5Skills) -> SkillCode {
        match v {
            MartialTier5Skills::DMastery2 => SKILL_D_MASTERY2,
            MartialTier5Skills::HHawk => SKILL_H_HAWK,
            MartialTier5Skills::LBreaking => SKILL_L_BREAKING,
            MartialTier5Skills::HTraining => SKILL_H_TRAINING,
        }
    }
}

impl From<MartialSkills> for SkillCode {
    fn from(v: MartialSkills) -> SkillCode {
        match v {
            MartialSkills::Tier1(s) => s.into(),
            MartialSkills::Tier2(s) => s.into(),
            MartialSkills::Tier3(s) => s.into(),
            MartialSkills::Tier4(s) => s.into(),
            MartialSkills::Tier5(s) => s.into(),
        }
    }
}

impl TryFrom<SkillCode> for MartialSkills {
    type Error = ();

    fn try_from(code: SkillCode) -> Result<Self, Self::Error> {
        match code {
            c if c == SKILL_LOW_KICK => Ok(Self::Tier1(MartialTier1Skills::LowKick)),
            c if c == SKILL_S_MASTERY => Ok(Self::Tier1(MartialTier1Skills::SMastery)),
            c if c == SKILL_DBBLOW => Ok(Self::Tier1(MartialTier1Skills::Dbblow)),
            c if c == SKILL_H_STRAIGHT => Ok(Self::Tier1(MartialTier1Skills::HStraight)),
            c if c == SKILL_RAGE_UP => Ok(Self::Tier2(MartialTier2Skills::RageUp)),
            c if c == SKILL_PATRIOT => Ok(Self::Tier2(MartialTier2Skills::Patriot)),
            c if c == SKILL_R_ELBOW => Ok(Self::Tier2(MartialTier2Skills::RElbow)),
            c if c == SKILL_S_MASTERY2 => Ok(Self::Tier2(MartialTier2Skills::SMastery2)),
            c if c == SKILL_I_BULKUP => Ok(Self::Tier3(MartialTier3Skills::IBulkup)),
            c if c == SKILL_T_CANNON => Ok(Self::Tier3(MartialTier3Skills::TCannon)),
            c if c == SKILL_WAR_CRY => Ok(Self::Tier3(MartialTier3Skills::WarCry)),
            c if c == SKILL_J_HEELKICK => Ok(Self::Tier3(MartialTier3Skills::JHeelkick)),
            c if c == SKILL_COMBINATION => Ok(Self::Tier4(MartialTier4Skills::Combination)),
            c if c == SKILL_STEELERS => Ok(Self::Tier4(MartialTier4Skills::Steelers)),
            c if c == SKILL_B_CHECK => Ok(Self::Tier4(MartialTier4Skills::BCheck)),
            c if c == SKILL_TYPHOON => Ok(Self::Tier4(MartialTier4Skills::Typhoon)),
            c if c == SKILL_D_MASTERY2 => Ok(Self::Tier5(MartialTier5Skills::DMastery2)),
            c if c == SKILL_H_HAWK => Ok(Self::Tier5(MartialTier5Skills::HHawk)),
            c if c == SKILL_L_BREAKING => Ok(Self::Tier5(MartialTier5Skills::LBreaking)),
            c if c == SKILL_H_TRAINING => Ok(Self::Tier5(MartialTier5Skills::HTraining)),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tier1_maps_to_codes() {
        let code: SkillCode = MartialTier1Skills::LowKick.into();
        assert_eq!(code, SKILL_LOW_KICK);
    }

    #[test]
    fn try_from_classifies_skill_and_tier() {
        let result = MartialSkills::try_from(SKILL_T_CANNON);
        assert!(result.is_ok());
        let skill = match result {
            Ok(skill) => skill,
            Err(error) => {
                assert!(false, "unexpected martial skill failure: {error:?}");
                return;
            }
        };
        assert_eq!(skill.tier(), ChangeJobTier::Tier3);
    }
}
