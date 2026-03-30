use core::convert::TryFrom;

use crate::domain::skill::dtos::codes::{
    ChangeJobTier, SkillCode, SKILL_AMPLIFIED, SKILL_ASSASSIN_EYE, SKILL_CHAIN_LANCE,
    SKILL_CHARGING_STRIKE, SKILL_CRITICAL_HIT, SKILL_CRITICAL_MASTERY, SKILL_D_REAPER,
    SKILL_EXPANSION, SKILL_F_SPEAR, SKILL_GROUND_PIKE, SKILL_ICE_ATTRIBUTE, SKILL_JUMPING_CRASH,
    SKILL_PIKE_WIND, SKILL_SHADOW_MASTER, SKILL_SS_ATTACK, SKILL_TORNADO, SKILL_VAGUE,
    SKILL_VANISH, SKILL_VENOM_SPEAR, SKILL_WEAPONE_DEFENCE_MASTERY,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PikemanSkills {
    Tier1(PikemanTier1Skills),
    Tier2(PikemanTier2Skills),
    Tier3(PikemanTier3Skills),
    Tier4(PikemanTier4Skills),
    Tier5(PikemanTier5Skills),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PikemanTier1Skills {
    PikeWind,
    IceAttribute,
    CriticalHit,
    JumpingCrash,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PikemanTier2Skills {
    GroundPike,
    Tornado,
    WeaponDefenseMastery,
    Expansion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PikemanTier3Skills {
    VenomSpear,
    Vanish,
    CriticalMastery,
    ChainLance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PikemanTier4Skills {
    AssassinEye,
    ChargingStrike,
    Vague,
    ShadowMaster,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PikemanTier5Skills {
    DReaper,
    FSpear,
    Amplified,
    SsAttack,
}

impl PikemanSkills {
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

impl From<PikemanTier1Skills> for SkillCode {
    fn from(v: PikemanTier1Skills) -> SkillCode {
        match v {
            PikemanTier1Skills::PikeWind => SKILL_PIKE_WIND,
            PikemanTier1Skills::IceAttribute => SKILL_ICE_ATTRIBUTE,
            PikemanTier1Skills::CriticalHit => SKILL_CRITICAL_HIT,
            PikemanTier1Skills::JumpingCrash => SKILL_JUMPING_CRASH,
        }
    }
}

impl From<PikemanTier2Skills> for SkillCode {
    fn from(v: PikemanTier2Skills) -> SkillCode {
        match v {
            PikemanTier2Skills::GroundPike => SKILL_GROUND_PIKE,
            PikemanTier2Skills::Tornado => SKILL_TORNADO,
            PikemanTier2Skills::WeaponDefenseMastery => SKILL_WEAPONE_DEFENCE_MASTERY,
            PikemanTier2Skills::Expansion => SKILL_EXPANSION,
        }
    }
}

impl From<PikemanTier3Skills> for SkillCode {
    fn from(v: PikemanTier3Skills) -> SkillCode {
        match v {
            PikemanTier3Skills::VenomSpear => SKILL_VENOM_SPEAR,
            PikemanTier3Skills::Vanish => SKILL_VANISH,
            PikemanTier3Skills::CriticalMastery => SKILL_CRITICAL_MASTERY,
            PikemanTier3Skills::ChainLance => SKILL_CHAIN_LANCE,
        }
    }
}

impl From<PikemanTier4Skills> for SkillCode {
    fn from(v: PikemanTier4Skills) -> SkillCode {
        match v {
            PikemanTier4Skills::AssassinEye => SKILL_ASSASSIN_EYE,
            PikemanTier4Skills::ChargingStrike => SKILL_CHARGING_STRIKE,
            PikemanTier4Skills::Vague => SKILL_VAGUE,
            PikemanTier4Skills::ShadowMaster => SKILL_SHADOW_MASTER,
        }
    }
}

impl From<PikemanTier5Skills> for SkillCode {
    fn from(v: PikemanTier5Skills) -> SkillCode {
        match v {
            PikemanTier5Skills::DReaper => SKILL_D_REAPER,
            PikemanTier5Skills::FSpear => SKILL_F_SPEAR,
            PikemanTier5Skills::Amplified => SKILL_AMPLIFIED,
            PikemanTier5Skills::SsAttack => SKILL_SS_ATTACK,
        }
    }
}

impl From<PikemanSkills> for SkillCode {
    fn from(v: PikemanSkills) -> SkillCode {
        match v {
            PikemanSkills::Tier1(s) => s.into(),
            PikemanSkills::Tier2(s) => s.into(),
            PikemanSkills::Tier3(s) => s.into(),
            PikemanSkills::Tier4(s) => s.into(),
            PikemanSkills::Tier5(s) => s.into(),
        }
    }
}

impl TryFrom<SkillCode> for PikemanSkills {
    type Error = ();

    fn try_from(code: SkillCode) -> Result<Self, Self::Error> {
        match code {
            c if c == SKILL_PIKE_WIND => Ok(Self::Tier1(PikemanTier1Skills::PikeWind)),
            c if c == SKILL_ICE_ATTRIBUTE => Ok(Self::Tier1(PikemanTier1Skills::IceAttribute)),
            c if c == SKILL_CRITICAL_HIT => Ok(Self::Tier1(PikemanTier1Skills::CriticalHit)),
            c if c == SKILL_JUMPING_CRASH => Ok(Self::Tier1(PikemanTier1Skills::JumpingCrash)),
            c if c == SKILL_GROUND_PIKE => Ok(Self::Tier2(PikemanTier2Skills::GroundPike)),
            c if c == SKILL_TORNADO => Ok(Self::Tier2(PikemanTier2Skills::Tornado)),
            c if c == SKILL_WEAPONE_DEFENCE_MASTERY => {
                Ok(Self::Tier2(PikemanTier2Skills::WeaponDefenseMastery))
            }
            c if c == SKILL_EXPANSION => Ok(Self::Tier2(PikemanTier2Skills::Expansion)),
            c if c == SKILL_VENOM_SPEAR => Ok(Self::Tier3(PikemanTier3Skills::VenomSpear)),
            c if c == SKILL_VANISH => Ok(Self::Tier3(PikemanTier3Skills::Vanish)),
            c if c == SKILL_CRITICAL_MASTERY => {
                Ok(Self::Tier3(PikemanTier3Skills::CriticalMastery))
            }
            c if c == SKILL_CHAIN_LANCE => Ok(Self::Tier3(PikemanTier3Skills::ChainLance)),
            c if c == SKILL_ASSASSIN_EYE => Ok(Self::Tier4(PikemanTier4Skills::AssassinEye)),
            c if c == SKILL_CHARGING_STRIKE => Ok(Self::Tier4(PikemanTier4Skills::ChargingStrike)),
            c if c == SKILL_VAGUE => Ok(Self::Tier4(PikemanTier4Skills::Vague)),
            c if c == SKILL_SHADOW_MASTER => Ok(Self::Tier4(PikemanTier4Skills::ShadowMaster)),
            c if c == SKILL_D_REAPER => Ok(Self::Tier5(PikemanTier5Skills::DReaper)),
            c if c == SKILL_F_SPEAR => Ok(Self::Tier5(PikemanTier5Skills::FSpear)),
            c if c == SKILL_AMPLIFIED => Ok(Self::Tier5(PikemanTier5Skills::Amplified)),
            c if c == SKILL_SS_ATTACK => Ok(Self::Tier5(PikemanTier5Skills::SsAttack)),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tier2_has_four_skills_and_maps_to_codes() {
        let s1: SkillCode = PikemanTier2Skills::GroundPike.into();
        let s2: SkillCode = PikemanTier2Skills::Tornado.into();
        let s3: SkillCode = PikemanTier2Skills::WeaponDefenseMastery.into();
        let s4: SkillCode = PikemanTier2Skills::Expansion.into();

        assert_eq!(s1, SKILL_GROUND_PIKE);
        assert_eq!(s2, SKILL_TORNADO);
        assert_eq!(s3, SKILL_WEAPONE_DEFENCE_MASTERY);
        assert_eq!(s4, SKILL_EXPANSION);
    }

    #[test]
    fn try_from_skill_code_classifies_pikeman_skill() {
        let skill = PikemanSkills::try_from(SKILL_VANISH);
        assert!(skill.is_ok());
        let skill = match skill {
            Ok(skill) => skill,
            Err(error) => {
                assert!(false, "must parse pikeman skill: {error:?}");
                return;
            }
        };
        assert_eq!(skill.tier(), ChangeJobTier::Tier3);
    }
}
