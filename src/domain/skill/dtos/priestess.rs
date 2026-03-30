use core::convert::TryFrom;

use crate::domain::skill::dtos::codes::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PriestessSkills {
    Tier1(PriestessTier1Skills),
    Tier2(PriestessTier2Skills),
    Tier3(PriestessTier3Skills),
    Tier4(PriestessTier4Skills),
    Tier5(PriestessTier5Skills),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PriestessTier1Skills {
    Healing,
    HolyBolt,
    Multispark,
    HolyMind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PriestessTier2Skills {
    Meditation,
    DivineLightning,
    HolyReflection,
    GrandHealing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PriestessTier3Skills {
    VigorBall,
    Resurrection,
    Extinction,
    VirtualLife,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PriestessTier4Skills {
    GlacialSpike,
    RegenerationField,
    ChainLightning,
    SummonMuspell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PriestessTier5Skills {
    SImpact,
    PIce,
    Ramiel,
    Krishna,
}

impl PriestessSkills {
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

impl From<PriestessTier1Skills> for SkillCode {
    fn from(v: PriestessTier1Skills) -> SkillCode {
        match v {
            PriestessTier1Skills::Healing => SKILL_HEALING,
            PriestessTier1Skills::HolyBolt => SKILL_HOLY_BOLT,
            PriestessTier1Skills::Multispark => SKILL_MULTISPARK,
            PriestessTier1Skills::HolyMind => SKILL_HOLY_MIND,
        }
    }
}

impl From<PriestessTier2Skills> for SkillCode {
    fn from(v: PriestessTier2Skills) -> SkillCode {
        match v {
            PriestessTier2Skills::Meditation => SKILL_MEDITATION,
            PriestessTier2Skills::DivineLightning => SKILL_DIVINE_LIGHTNING,
            PriestessTier2Skills::HolyReflection => SKILL_HOLY_REFLECTION,
            PriestessTier2Skills::GrandHealing => SKILL_GRAND_HEALING,
        }
    }
}

impl From<PriestessTier3Skills> for SkillCode {
    fn from(v: PriestessTier3Skills) -> SkillCode {
        match v {
            PriestessTier3Skills::VigorBall => SKILL_VIGOR_BALL,
            PriestessTier3Skills::Resurrection => SKILL_RESURRECTION,
            PriestessTier3Skills::Extinction => SKILL_EXTINCTION,
            PriestessTier3Skills::VirtualLife => SKILL_VIRTUAL_LIFE,
        }
    }
}

impl From<PriestessTier4Skills> for SkillCode {
    fn from(v: PriestessTier4Skills) -> SkillCode {
        match v {
            PriestessTier4Skills::GlacialSpike => SKILL_GLACIAL_SPIKE,
            PriestessTier4Skills::RegenerationField => SKILL_REGENERATION_FIELD,
            PriestessTier4Skills::ChainLightning => SKILL_CHAIN_LIGHTNING,
            PriestessTier4Skills::SummonMuspell => SKILL_SUMMON_MUSPELL,
        }
    }
}

impl From<PriestessTier5Skills> for SkillCode {
    fn from(v: PriestessTier5Skills) -> SkillCode {
        match v {
            PriestessTier5Skills::SImpact => SKILL_S_IMPACT,
            PriestessTier5Skills::PIce => SKILL_P_ICE,
            PriestessTier5Skills::Ramiel => SKILL_RAMIEL,
            PriestessTier5Skills::Krishna => SKILL_KRISHNA,
        }
    }
}

impl From<PriestessSkills> for SkillCode {
    fn from(v: PriestessSkills) -> SkillCode {
        match v {
            PriestessSkills::Tier1(s) => s.into(),
            PriestessSkills::Tier2(s) => s.into(),
            PriestessSkills::Tier3(s) => s.into(),
            PriestessSkills::Tier4(s) => s.into(),
            PriestessSkills::Tier5(s) => s.into(),
        }
    }
}

impl TryFrom<SkillCode> for PriestessSkills {
    type Error = ();

    fn try_from(code: SkillCode) -> Result<Self, Self::Error> {
        match code {
            c if c == SKILL_HEALING => Ok(Self::Tier1(PriestessTier1Skills::Healing)),
            c if c == SKILL_HOLY_BOLT => Ok(Self::Tier1(PriestessTier1Skills::HolyBolt)),
            c if c == SKILL_MULTISPARK => Ok(Self::Tier1(PriestessTier1Skills::Multispark)),
            c if c == SKILL_HOLY_MIND => Ok(Self::Tier1(PriestessTier1Skills::HolyMind)),
            c if c == SKILL_MEDITATION => Ok(Self::Tier2(PriestessTier2Skills::Meditation)),
            c if c == SKILL_DIVINE_LIGHTNING => {
                Ok(Self::Tier2(PriestessTier2Skills::DivineLightning))
            }
            c if c == SKILL_HOLY_REFLECTION => {
                Ok(Self::Tier2(PriestessTier2Skills::HolyReflection))
            }
            c if c == SKILL_GRAND_HEALING => Ok(Self::Tier2(PriestessTier2Skills::GrandHealing)),
            c if c == SKILL_VIGOR_BALL => Ok(Self::Tier3(PriestessTier3Skills::VigorBall)),
            c if c == SKILL_RESURRECTION => Ok(Self::Tier3(PriestessTier3Skills::Resurrection)),
            c if c == SKILL_EXTINCTION => Ok(Self::Tier3(PriestessTier3Skills::Extinction)),
            c if c == SKILL_VIRTUAL_LIFE => Ok(Self::Tier3(PriestessTier3Skills::VirtualLife)),
            c if c == SKILL_GLACIAL_SPIKE => Ok(Self::Tier4(PriestessTier4Skills::GlacialSpike)),
            c if c == SKILL_REGENERATION_FIELD => {
                Ok(Self::Tier4(PriestessTier4Skills::RegenerationField))
            }
            c if c == SKILL_CHAIN_LIGHTNING => {
                Ok(Self::Tier4(PriestessTier4Skills::ChainLightning))
            }
            c if c == SKILL_SUMMON_MUSPELL => Ok(Self::Tier4(PriestessTier4Skills::SummonMuspell)),
            c if c == SKILL_S_IMPACT => Ok(Self::Tier5(PriestessTier5Skills::SImpact)),
            c if c == SKILL_P_ICE => Ok(Self::Tier5(PriestessTier5Skills::PIce)),
            c if c == SKILL_RAMIEL => Ok(Self::Tier5(PriestessTier5Skills::Ramiel)),
            c if c == SKILL_KRISHNA => Ok(Self::Tier5(PriestessTier5Skills::Krishna)),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tier1_maps_to_codes() {
        let code: SkillCode = PriestessTier1Skills::Healing.into();
        assert_eq!(code, SKILL_HEALING);
    }

    #[test]
    fn try_from_classifies_skill_and_tier() {
        let result = PriestessSkills::try_from(SKILL_RESURRECTION);
        assert!(result.is_ok());
        let skill = match result {
            Ok(skill) => skill,
            Err(error) => {
                assert!(false, "unexpected priestess skill failure: {error:?}");
                return;
            }
        };
        assert_eq!(skill.tier(), ChangeJobTier::Tier3);
    }
}
