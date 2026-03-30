use core::convert::TryFrom;

use crate::domain::skill::dtos::codes::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KnightSkills {
    Tier1(KnightTier1Skills),
    Tier2(KnightTier2Skills),
    Tier3(KnightTier3Skills),
    Tier4(KnightTier4Skills),
    Tier5(KnightTier5Skills),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KnightTier1Skills {
    SwordBlast,
    HolyBody,
    PhysicalTraning,
    DoubleCrash,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KnightTier2Skills {
    HolyValor,
    Brandish,
    Piercing,
    DrasticSpirit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KnightTier3Skills {
    SwordMastery,
    DivineInhalation,
    HolyIncantation,
    GrandCross,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KnightTier4Skills {
    SwordOfJustice,
    GodlyShield,
    GodBless,
    DivinePiercing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KnightTier5Skills {
    SBreaker,
    CMoon,
    SBlade,
    HBenedic,
}

impl KnightSkills {
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

impl From<KnightTier1Skills> for SkillCode {
    fn from(v: KnightTier1Skills) -> SkillCode {
        match v {
            KnightTier1Skills::SwordBlast => SKILL_SWORD_BLAST,
            KnightTier1Skills::HolyBody => SKILL_HOLY_BODY,
            KnightTier1Skills::PhysicalTraning => SKILL_PHYSICAL_TRANING,
            KnightTier1Skills::DoubleCrash => SKILL_DOUBLE_CRASH,
        }
    }
}

impl From<KnightTier2Skills> for SkillCode {
    fn from(v: KnightTier2Skills) -> SkillCode {
        match v {
            KnightTier2Skills::HolyValor => SKILL_HOLY_VALOR,
            KnightTier2Skills::Brandish => SKILL_BRANDISH,
            KnightTier2Skills::Piercing => SKILL_PIERCING,
            KnightTier2Skills::DrasticSpirit => SKILL_DRASTIC_SPIRIT,
        }
    }
}

impl From<KnightTier3Skills> for SkillCode {
    fn from(v: KnightTier3Skills) -> SkillCode {
        match v {
            KnightTier3Skills::SwordMastery => SKILL_SWORD_MASTERY,
            KnightTier3Skills::DivineInhalation => SKILL_DIVINE_INHALATION,
            KnightTier3Skills::HolyIncantation => SKILL_HOLY_INCANTATION,
            KnightTier3Skills::GrandCross => SKILL_GRAND_CROSS,
        }
    }
}

impl From<KnightTier4Skills> for SkillCode {
    fn from(v: KnightTier4Skills) -> SkillCode {
        match v {
            KnightTier4Skills::SwordOfJustice => SKILL_SWORD_OF_JUSTICE,
            KnightTier4Skills::GodlyShield => SKILL_GODLY_SHIELD,
            KnightTier4Skills::GodBless => SKILL_GOD_BLESS,
            KnightTier4Skills::DivinePiercing => SKILL_DIVINE_PIERCING,
        }
    }
}

impl From<KnightTier5Skills> for SkillCode {
    fn from(v: KnightTier5Skills) -> SkillCode {
        match v {
            KnightTier5Skills::SBreaker => SKILL_S_BREAKER,
            KnightTier5Skills::CMoon => SKILL_C_MOON,
            KnightTier5Skills::SBlade => SKILL_S_BLADE,
            KnightTier5Skills::HBenedic => SKILL_H_BENEDIC,
        }
    }
}

impl From<KnightSkills> for SkillCode {
    fn from(v: KnightSkills) -> SkillCode {
        match v {
            KnightSkills::Tier1(s) => s.into(),
            KnightSkills::Tier2(s) => s.into(),
            KnightSkills::Tier3(s) => s.into(),
            KnightSkills::Tier4(s) => s.into(),
            KnightSkills::Tier5(s) => s.into(),
        }
    }
}

impl TryFrom<SkillCode> for KnightSkills {
    type Error = ();

    fn try_from(code: SkillCode) -> Result<Self, Self::Error> {
        match code {
            c if c == SKILL_SWORD_BLAST => Ok(Self::Tier1(KnightTier1Skills::SwordBlast)),
            c if c == SKILL_HOLY_BODY => Ok(Self::Tier1(KnightTier1Skills::HolyBody)),
            c if c == SKILL_PHYSICAL_TRANING => Ok(Self::Tier1(KnightTier1Skills::PhysicalTraning)),
            c if c == SKILL_DOUBLE_CRASH => Ok(Self::Tier1(KnightTier1Skills::DoubleCrash)),
            c if c == SKILL_HOLY_VALOR => Ok(Self::Tier2(KnightTier2Skills::HolyValor)),
            c if c == SKILL_BRANDISH => Ok(Self::Tier2(KnightTier2Skills::Brandish)),
            c if c == SKILL_PIERCING => Ok(Self::Tier2(KnightTier2Skills::Piercing)),
            c if c == SKILL_DRASTIC_SPIRIT => Ok(Self::Tier2(KnightTier2Skills::DrasticSpirit)),
            c if c == SKILL_SWORD_MASTERY => Ok(Self::Tier3(KnightTier3Skills::SwordMastery)),
            c if c == SKILL_DIVINE_INHALATION => {
                Ok(Self::Tier3(KnightTier3Skills::DivineInhalation))
            }
            c if c == SKILL_HOLY_INCANTATION => Ok(Self::Tier3(KnightTier3Skills::HolyIncantation)),
            c if c == SKILL_GRAND_CROSS => Ok(Self::Tier3(KnightTier3Skills::GrandCross)),
            c if c == SKILL_SWORD_OF_JUSTICE => Ok(Self::Tier4(KnightTier4Skills::SwordOfJustice)),
            c if c == SKILL_GODLY_SHIELD => Ok(Self::Tier4(KnightTier4Skills::GodlyShield)),
            c if c == SKILL_GOD_BLESS => Ok(Self::Tier4(KnightTier4Skills::GodBless)),
            c if c == SKILL_DIVINE_PIERCING => Ok(Self::Tier4(KnightTier4Skills::DivinePiercing)),
            c if c == SKILL_S_BREAKER => Ok(Self::Tier5(KnightTier5Skills::SBreaker)),
            c if c == SKILL_C_MOON => Ok(Self::Tier5(KnightTier5Skills::CMoon)),
            c if c == SKILL_S_BLADE => Ok(Self::Tier5(KnightTier5Skills::SBlade)),
            c if c == SKILL_H_BENEDIC => Ok(Self::Tier5(KnightTier5Skills::HBenedic)),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tier1_maps_to_codes() {
        let code: SkillCode = KnightTier1Skills::SwordBlast.into();
        assert_eq!(code, SKILL_SWORD_BLAST);
    }

    #[test]
    fn try_from_classifies_skill_and_tier() {
        let result = KnightSkills::try_from(SKILL_DIVINE_INHALATION);
        assert!(result.is_ok());
        let skill = match result {
            Ok(skill) => skill,
            Err(error) => {
                assert!(false, "unexpected knight skill failure: {error:?}");
                return;
            }
        };
        assert_eq!(skill.tier(), ChangeJobTier::Tier3);
    }
}
