use core::convert::TryFrom;

use crate::domain::skill::dtos::codes::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArcherSkills {
    Tier1(ArcherTier1Skills),
    Tier2(ArcherTier2Skills),
    Tier3(ArcherTier3Skills),
    Tier4(ArcherTier4Skills),
    Tier5(ArcherTier5Skills),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArcherTier1Skills {
    ScoutHawk,
    ShootingMastery,
    WindArrow,
    PerfectAim,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArcherTier2Skills {
    DionsEye,
    Falcon,
    ArrowOfRage,
    Avalanche,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArcherTier3Skills {
    ElementalShot,
    GoldenFalcon,
    BombShot,
    Perforation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArcherTier4Skills {
    RecallWolverin,
    EvasionMastery,
    PhoenixShot,
    ForceOfNature,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArcherTier5Skills {
    EShot,
    SRope,
    NSplash,
    CTrap,
}

impl ArcherSkills {
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

impl From<ArcherTier1Skills> for SkillCode {
    fn from(v: ArcherTier1Skills) -> SkillCode {
        match v {
            ArcherTier1Skills::ScoutHawk => SKILL_SCOUT_HAWK,
            ArcherTier1Skills::ShootingMastery => SKILL_SHOOTING_MASTERY,
            ArcherTier1Skills::WindArrow => SKILL_WIND_ARROW,
            ArcherTier1Skills::PerfectAim => SKILL_PERFECT_AIM,
        }
    }
}

impl From<ArcherTier2Skills> for SkillCode {
    fn from(v: ArcherTier2Skills) -> SkillCode {
        match v {
            ArcherTier2Skills::DionsEye => SKILL_DIONS_EYE,
            ArcherTier2Skills::Falcon => SKILL_FALCON,
            ArcherTier2Skills::ArrowOfRage => SKILL_ARROW_OF_RAGE,
            ArcherTier2Skills::Avalanche => SKILL_AVALANCHE,
        }
    }
}

impl From<ArcherTier3Skills> for SkillCode {
    fn from(v: ArcherTier3Skills) -> SkillCode {
        match v {
            ArcherTier3Skills::ElementalShot => SKILL_ELEMENTAL_SHOT,
            ArcherTier3Skills::GoldenFalcon => SKILL_GOLDEN_FALCON,
            ArcherTier3Skills::BombShot => SKILL_BOMB_SHOT,
            ArcherTier3Skills::Perforation => SKILL_PERFORATION,
        }
    }
}

impl From<ArcherTier4Skills> for SkillCode {
    fn from(v: ArcherTier4Skills) -> SkillCode {
        match v {
            ArcherTier4Skills::RecallWolverin => SKILL_RECALL_WOLVERIN,
            ArcherTier4Skills::EvasionMastery => SKILL_EVASION_MASTERY,
            ArcherTier4Skills::PhoenixShot => SKILL_PHOENIX_SHOT,
            ArcherTier4Skills::ForceOfNature => SKILL_FORCE_OF_NATURE,
        }
    }
}

impl From<ArcherTier5Skills> for SkillCode {
    fn from(v: ArcherTier5Skills) -> SkillCode {
        match v {
            ArcherTier5Skills::EShot => SKILL_E_SHOT,
            ArcherTier5Skills::SRope => SKILL_S_ROPE,
            ArcherTier5Skills::NSplash => SKILL_N_SPLASH,
            ArcherTier5Skills::CTrap => SKILL_C_TRAP,
        }
    }
}

impl From<ArcherSkills> for SkillCode {
    fn from(v: ArcherSkills) -> SkillCode {
        match v {
            ArcherSkills::Tier1(s) => s.into(),
            ArcherSkills::Tier2(s) => s.into(),
            ArcherSkills::Tier3(s) => s.into(),
            ArcherSkills::Tier4(s) => s.into(),
            ArcherSkills::Tier5(s) => s.into(),
        }
    }
}

impl TryFrom<SkillCode> for ArcherSkills {
    type Error = ();

    fn try_from(code: SkillCode) -> Result<Self, Self::Error> {
        match code {
            c if c == SKILL_SCOUT_HAWK => Ok(Self::Tier1(ArcherTier1Skills::ScoutHawk)),
            c if c == SKILL_SHOOTING_MASTERY => Ok(Self::Tier1(ArcherTier1Skills::ShootingMastery)),
            c if c == SKILL_WIND_ARROW => Ok(Self::Tier1(ArcherTier1Skills::WindArrow)),
            c if c == SKILL_PERFECT_AIM => Ok(Self::Tier1(ArcherTier1Skills::PerfectAim)),
            c if c == SKILL_DIONS_EYE => Ok(Self::Tier2(ArcherTier2Skills::DionsEye)),
            c if c == SKILL_FALCON => Ok(Self::Tier2(ArcherTier2Skills::Falcon)),
            c if c == SKILL_ARROW_OF_RAGE => Ok(Self::Tier2(ArcherTier2Skills::ArrowOfRage)),
            c if c == SKILL_AVALANCHE => Ok(Self::Tier2(ArcherTier2Skills::Avalanche)),
            c if c == SKILL_ELEMENTAL_SHOT => Ok(Self::Tier3(ArcherTier3Skills::ElementalShot)),
            c if c == SKILL_GOLDEN_FALCON => Ok(Self::Tier3(ArcherTier3Skills::GoldenFalcon)),
            c if c == SKILL_BOMB_SHOT => Ok(Self::Tier3(ArcherTier3Skills::BombShot)),
            c if c == SKILL_PERFORATION => Ok(Self::Tier3(ArcherTier3Skills::Perforation)),
            c if c == SKILL_RECALL_WOLVERIN => Ok(Self::Tier4(ArcherTier4Skills::RecallWolverin)),
            c if c == SKILL_EVASION_MASTERY => Ok(Self::Tier4(ArcherTier4Skills::EvasionMastery)),
            c if c == SKILL_PHOENIX_SHOT => Ok(Self::Tier4(ArcherTier4Skills::PhoenixShot)),
            c if c == SKILL_FORCE_OF_NATURE => Ok(Self::Tier4(ArcherTier4Skills::ForceOfNature)),
            c if c == SKILL_E_SHOT => Ok(Self::Tier5(ArcherTier5Skills::EShot)),
            c if c == SKILL_S_ROPE => Ok(Self::Tier5(ArcherTier5Skills::SRope)),
            c if c == SKILL_N_SPLASH => Ok(Self::Tier5(ArcherTier5Skills::NSplash)),
            c if c == SKILL_C_TRAP => Ok(Self::Tier5(ArcherTier5Skills::CTrap)),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tier1_maps_to_codes() {
        let code: SkillCode = ArcherTier1Skills::ScoutHawk.into();
        assert_eq!(code, SKILL_SCOUT_HAWK);
    }

    #[test]
    fn try_from_classifies_skill_and_tier() {
        let result = ArcherSkills::try_from(SKILL_GOLDEN_FALCON);
        assert!(result.is_ok());
        let skill = match result {
            Ok(skill) => skill,
            Err(error) => {
                assert!(false, "unexpected archer skill failure: {error:?}");
                return;
            }
        };
        assert_eq!(skill.tier(), ChangeJobTier::Tier3);
    }
}
