use core::convert::TryFrom;

use crate::domain::skill::dtos::codes::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MagicianSkills {
    Tier1(MagicianTier1Skills),
    Tier2(MagicianTier2Skills),
    Tier3(MagicianTier3Skills),
    Tier4(MagicianTier4Skills),
    Tier5(MagicianTier5Skills),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MagicianTier1Skills {
    Agony,
    FireBolt,
    Zenith,
    FireBall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MagicianTier2Skills {
    MentalMastery,
    Watornado,
    EnchantWeapon,
    DeadRay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MagicianTier3Skills {
    EnergyShield,
    Diastrophism,
    SpiritElemental,
    DancingSword,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MagicianTier4Skills {
    FireElemental,
    FlameWave,
    Distortion,
    MMeteo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MagicianTier5Skills {
    Silraphim,
    Tenus,
    Ignis,
    Anima,
}

impl MagicianSkills {
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

impl From<MagicianTier1Skills> for SkillCode {
    fn from(v: MagicianTier1Skills) -> SkillCode {
        match v {
            MagicianTier1Skills::Agony => SKILL_AGONY,
            MagicianTier1Skills::FireBolt => SKILL_FIRE_BOLT,
            MagicianTier1Skills::Zenith => SKILL_ZENITH,
            MagicianTier1Skills::FireBall => SKILL_FIRE_BALL,
        }
    }
}

impl From<MagicianTier2Skills> for SkillCode {
    fn from(v: MagicianTier2Skills) -> SkillCode {
        match v {
            MagicianTier2Skills::MentalMastery => SKILL_MENTAL_MASTERY,
            MagicianTier2Skills::Watornado => SKILL_WATORNADO,
            MagicianTier2Skills::EnchantWeapon => SKILL_ENCHANT_WEAPON,
            MagicianTier2Skills::DeadRay => SKILL_DEAD_RAY,
        }
    }
}

impl From<MagicianTier3Skills> for SkillCode {
    fn from(v: MagicianTier3Skills) -> SkillCode {
        match v {
            MagicianTier3Skills::EnergyShield => SKILL_ENERGY_SHIELD,
            MagicianTier3Skills::Diastrophism => SKILL_DIASTROPHISM,
            MagicianTier3Skills::SpiritElemental => SKILL_SPIRIT_ELEMENTAL,
            MagicianTier3Skills::DancingSword => SKILL_DANCING_SWORD,
        }
    }
}

impl From<MagicianTier4Skills> for SkillCode {
    fn from(v: MagicianTier4Skills) -> SkillCode {
        match v {
            MagicianTier4Skills::FireElemental => SKILL_FIRE_ELEMENTAL,
            MagicianTier4Skills::FlameWave => SKILL_FLAME_WAVE,
            MagicianTier4Skills::Distortion => SKILL_DISTORTION,
            MagicianTier4Skills::MMeteo => SKILL_M_METEO,
        }
    }
}

impl From<MagicianTier5Skills> for SkillCode {
    fn from(v: MagicianTier5Skills) -> SkillCode {
        match v {
            MagicianTier5Skills::Silraphim => SKILL_SILRAPHIM,
            MagicianTier5Skills::Tenus => SKILL_TENUS,
            MagicianTier5Skills::Ignis => SKILL_IGNIS,
            MagicianTier5Skills::Anima => SKILL_ANIMA,
        }
    }
}

impl From<MagicianSkills> for SkillCode {
    fn from(v: MagicianSkills) -> SkillCode {
        match v {
            MagicianSkills::Tier1(s) => s.into(),
            MagicianSkills::Tier2(s) => s.into(),
            MagicianSkills::Tier3(s) => s.into(),
            MagicianSkills::Tier4(s) => s.into(),
            MagicianSkills::Tier5(s) => s.into(),
        }
    }
}

impl TryFrom<SkillCode> for MagicianSkills {
    type Error = ();

    fn try_from(code: SkillCode) -> Result<Self, Self::Error> {
        match code {
            c if c == SKILL_AGONY => Ok(Self::Tier1(MagicianTier1Skills::Agony)),
            c if c == SKILL_FIRE_BOLT => Ok(Self::Tier1(MagicianTier1Skills::FireBolt)),
            c if c == SKILL_ZENITH => Ok(Self::Tier1(MagicianTier1Skills::Zenith)),
            c if c == SKILL_FIRE_BALL => Ok(Self::Tier1(MagicianTier1Skills::FireBall)),
            c if c == SKILL_MENTAL_MASTERY => Ok(Self::Tier2(MagicianTier2Skills::MentalMastery)),
            c if c == SKILL_WATORNADO => Ok(Self::Tier2(MagicianTier2Skills::Watornado)),
            c if c == SKILL_ENCHANT_WEAPON => Ok(Self::Tier2(MagicianTier2Skills::EnchantWeapon)),
            c if c == SKILL_DEAD_RAY => Ok(Self::Tier2(MagicianTier2Skills::DeadRay)),
            c if c == SKILL_ENERGY_SHIELD => Ok(Self::Tier3(MagicianTier3Skills::EnergyShield)),
            c if c == SKILL_DIASTROPHISM => Ok(Self::Tier3(MagicianTier3Skills::Diastrophism)),
            c if c == SKILL_SPIRIT_ELEMENTAL => {
                Ok(Self::Tier3(MagicianTier3Skills::SpiritElemental))
            }
            c if c == SKILL_DANCING_SWORD => Ok(Self::Tier3(MagicianTier3Skills::DancingSword)),
            c if c == SKILL_FIRE_ELEMENTAL => Ok(Self::Tier4(MagicianTier4Skills::FireElemental)),
            c if c == SKILL_FLAME_WAVE => Ok(Self::Tier4(MagicianTier4Skills::FlameWave)),
            c if c == SKILL_DISTORTION => Ok(Self::Tier4(MagicianTier4Skills::Distortion)),
            c if c == SKILL_M_METEO => Ok(Self::Tier4(MagicianTier4Skills::MMeteo)),
            c if c == SKILL_SILRAPHIM => Ok(Self::Tier5(MagicianTier5Skills::Silraphim)),
            c if c == SKILL_TENUS => Ok(Self::Tier5(MagicianTier5Skills::Tenus)),
            c if c == SKILL_IGNIS => Ok(Self::Tier5(MagicianTier5Skills::Ignis)),
            c if c == SKILL_ANIMA => Ok(Self::Tier5(MagicianTier5Skills::Anima)),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tier1_maps_to_codes() {
        let code: SkillCode = MagicianTier1Skills::Agony.into();
        assert_eq!(code, SKILL_AGONY);
    }

    #[test]
    fn try_from_classifies_skill_and_tier() {
        let result = MagicianSkills::try_from(SKILL_DIASTROPHISM);
        assert!(result.is_ok());
        let skill = match result {
            Ok(skill) => skill,
            Err(error) => {
                assert!(false, "unexpected magician skill failure: {error:?}");
                return;
            }
        };
        assert_eq!(skill.tier(), ChangeJobTier::Tier3);
    }
}
