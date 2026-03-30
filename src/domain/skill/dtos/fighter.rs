use core::convert::TryFrom;

use crate::domain::skill::dtos::codes::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FighterSkills {
    Tier1(FighterTier1Skills),
    Tier2(FighterTier2Skills),
    Tier3(FighterTier3Skills),
    Tier4(FighterTier4Skills),
    Tier5(FighterTier5Skills),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FighterTier1Skills {
    MeleeMastery,
    FireAttribute,
    Raving,
    Impact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FighterTier2Skills {
    TripleImpact,
    BrutalSwing,
    Roar,
    RageOfZecram,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FighterTier3Skills {
    Concentration,
    AvangingCrash,
    SwiftAxe,
    BoneCrash,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FighterTier4Skills {
    Detoryer,
    Berserker,
    CycloneStrike,
    BoostHealth,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FighterTier5Skills {
    DHit,
    PDash,
    MBlow,
    BBerserker,
}

impl FighterSkills {
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

impl From<FighterTier1Skills> for SkillCode {
    fn from(v: FighterTier1Skills) -> SkillCode {
        match v {
            FighterTier1Skills::MeleeMastery => SKILL_MELEE_MASTERY,
            FighterTier1Skills::FireAttribute => SKILL_FIRE_ATTRIBUTE,
            FighterTier1Skills::Raving => SKILL_RAVING,
            FighterTier1Skills::Impact => SKILL_IMPACT,
        }
    }
}

impl From<FighterTier2Skills> for SkillCode {
    fn from(v: FighterTier2Skills) -> SkillCode {
        match v {
            FighterTier2Skills::TripleImpact => SKILL_TRIPLE_IMPACT,
            FighterTier2Skills::BrutalSwing => SKILL_BRUTAL_SWING,
            FighterTier2Skills::Roar => SKILL_ROAR,
            FighterTier2Skills::RageOfZecram => SKILL_RAGE_OF_ZECRAM,
        }
    }
}

impl From<FighterTier3Skills> for SkillCode {
    fn from(v: FighterTier3Skills) -> SkillCode {
        match v {
            FighterTier3Skills::Concentration => SKILL_CONCENTRATION,
            FighterTier3Skills::AvangingCrash => SKILL_AVANGING_CRASH,
            FighterTier3Skills::SwiftAxe => SKILL_SWIFT_AXE,
            FighterTier3Skills::BoneCrash => SKILL_BONE_CRASH,
        }
    }
}

impl From<FighterTier4Skills> for SkillCode {
    fn from(v: FighterTier4Skills) -> SkillCode {
        match v {
            FighterTier4Skills::Detoryer => SKILL_DETORYER,
            FighterTier4Skills::Berserker => SKILL_BERSERKER,
            FighterTier4Skills::CycloneStrike => SKILL_CYCLONE_STRIKE,
            FighterTier4Skills::BoostHealth => SKILL_BOOST_HEALTH,
        }
    }
}

impl From<FighterTier5Skills> for SkillCode {
    fn from(v: FighterTier5Skills) -> SkillCode {
        match v {
            FighterTier5Skills::DHit => SKILL_D_HIT,
            FighterTier5Skills::PDash => SKILL_P_DASH,
            FighterTier5Skills::MBlow => SKILL_M_BLOW,
            FighterTier5Skills::BBerserker => SKILL_B_BERSERKER,
        }
    }
}

impl From<FighterSkills> for SkillCode {
    fn from(v: FighterSkills) -> SkillCode {
        match v {
            FighterSkills::Tier1(s) => s.into(),
            FighterSkills::Tier2(s) => s.into(),
            FighterSkills::Tier3(s) => s.into(),
            FighterSkills::Tier4(s) => s.into(),
            FighterSkills::Tier5(s) => s.into(),
        }
    }
}

impl TryFrom<SkillCode> for FighterSkills {
    type Error = ();

    fn try_from(code: SkillCode) -> Result<Self, Self::Error> {
        match code {
            c if c == SKILL_MELEE_MASTERY => Ok(Self::Tier1(FighterTier1Skills::MeleeMastery)),
            c if c == SKILL_FIRE_ATTRIBUTE => Ok(Self::Tier1(FighterTier1Skills::FireAttribute)),
            c if c == SKILL_RAVING => Ok(Self::Tier1(FighterTier1Skills::Raving)),
            c if c == SKILL_IMPACT => Ok(Self::Tier1(FighterTier1Skills::Impact)),
            c if c == SKILL_TRIPLE_IMPACT => Ok(Self::Tier2(FighterTier2Skills::TripleImpact)),
            c if c == SKILL_BRUTAL_SWING => Ok(Self::Tier2(FighterTier2Skills::BrutalSwing)),
            c if c == SKILL_ROAR => Ok(Self::Tier2(FighterTier2Skills::Roar)),
            c if c == SKILL_RAGE_OF_ZECRAM => Ok(Self::Tier2(FighterTier2Skills::RageOfZecram)),
            c if c == SKILL_CONCENTRATION => Ok(Self::Tier3(FighterTier3Skills::Concentration)),
            c if c == SKILL_AVANGING_CRASH => Ok(Self::Tier3(FighterTier3Skills::AvangingCrash)),
            c if c == SKILL_SWIFT_AXE => Ok(Self::Tier3(FighterTier3Skills::SwiftAxe)),
            c if c == SKILL_BONE_CRASH => Ok(Self::Tier3(FighterTier3Skills::BoneCrash)),
            c if c == SKILL_DETORYER => Ok(Self::Tier4(FighterTier4Skills::Detoryer)),
            c if c == SKILL_BERSERKER => Ok(Self::Tier4(FighterTier4Skills::Berserker)),
            c if c == SKILL_CYCLONE_STRIKE => Ok(Self::Tier4(FighterTier4Skills::CycloneStrike)),
            c if c == SKILL_BOOST_HEALTH => Ok(Self::Tier4(FighterTier4Skills::BoostHealth)),
            c if c == SKILL_D_HIT => Ok(Self::Tier5(FighterTier5Skills::DHit)),
            c if c == SKILL_P_DASH => Ok(Self::Tier5(FighterTier5Skills::PDash)),
            c if c == SKILL_M_BLOW => Ok(Self::Tier5(FighterTier5Skills::MBlow)),
            c if c == SKILL_B_BERSERKER => Ok(Self::Tier5(FighterTier5Skills::BBerserker)),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tier1_maps_to_codes() {
        let code: SkillCode = FighterTier1Skills::MeleeMastery.into();
        assert_eq!(code, SKILL_MELEE_MASTERY);
    }

    #[test]
    fn try_from_classifies_skill_and_tier() {
        let result = FighterSkills::try_from(SKILL_AVANGING_CRASH);
        assert!(result.is_ok());
        let skill = match result {
            Ok(skill) => skill,
            Err(error) => {
                assert!(false, "unexpected fighter skill failure: {error:?}");
                return;
            }
        };
        assert_eq!(skill.tier(), ChangeJobTier::Tier3);
    }
}
