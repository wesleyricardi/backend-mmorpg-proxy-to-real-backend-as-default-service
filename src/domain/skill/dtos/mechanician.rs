use core::convert::TryFrom;

use crate::domain::skill::dtos::codes::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MechanicianSkills {
    Tier1(MechanicianTier1Skills),
    Tier2(MechanicianTier2Skills),
    Tier3(MechanicianTier3Skills),
    Tier4(MechanicianTier4Skills),
    Tier5(MechanicianTier5Skills),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MechanicianTier1Skills {
    ExtremeShield,
    MechanicBomb,
    PhysicalAbsorb,
    PoisonAttribute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MechanicianTier2Skills {
    GreatSmash,
    Maximize,
    Automation,
    Spark,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MechanicianTier3Skills {
    MetalArmor,
    GrandSmash,
    MechanicWeapon,
    SparkShield,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MechanicianTier4Skills {
    Impulsion,
    Compulsion,
    MagneticSphere,
    MetalGolem,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MechanicianTier5Skills {
    LandM,
    HSonic,
    RSmash,
    PEnhence,
}

impl MechanicianSkills {
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

impl From<MechanicianTier1Skills> for SkillCode {
    fn from(v: MechanicianTier1Skills) -> SkillCode {
        match v {
            MechanicianTier1Skills::ExtremeShield => SKILL_EXTREME_SHIELD,
            MechanicianTier1Skills::MechanicBomb => SKILL_MECHANIC_BOMB,
            MechanicianTier1Skills::PhysicalAbsorb => SKILL_PHYSICAL_ABSORB,
            MechanicianTier1Skills::PoisonAttribute => SKILL_POISON_ATTRIBUTE,
        }
    }
}

impl From<MechanicianTier2Skills> for SkillCode {
    fn from(v: MechanicianTier2Skills) -> SkillCode {
        match v {
            MechanicianTier2Skills::GreatSmash => SKILL_GREAT_SMASH,
            MechanicianTier2Skills::Maximize => SKILL_MAXIMIZE,
            MechanicianTier2Skills::Automation => SKILL_AUTOMATION,
            MechanicianTier2Skills::Spark => SKILL_SPARK,
        }
    }
}

impl From<MechanicianTier3Skills> for SkillCode {
    fn from(v: MechanicianTier3Skills) -> SkillCode {
        match v {
            MechanicianTier3Skills::MetalArmor => SKILL_METAL_ARMOR,
            MechanicianTier3Skills::GrandSmash => SKILL_GRAND_SMASH,
            MechanicianTier3Skills::MechanicWeapon => SKILL_MECHANIC_WEAPON,
            MechanicianTier3Skills::SparkShield => SKILL_SPARK_SHIELD,
        }
    }
}

impl From<MechanicianTier4Skills> for SkillCode {
    fn from(v: MechanicianTier4Skills) -> SkillCode {
        match v {
            MechanicianTier4Skills::Impulsion => SKILL_IMPULSION,
            MechanicianTier4Skills::Compulsion => SKILL_COMPULSION,
            MechanicianTier4Skills::MagneticSphere => SKILL_MAGNETIC_SPHERE,
            MechanicianTier4Skills::MetalGolem => SKILL_METAL_GOLEM,
        }
    }
}

impl From<MechanicianTier5Skills> for SkillCode {
    fn from(v: MechanicianTier5Skills) -> SkillCode {
        match v {
            MechanicianTier5Skills::LandM => SKILL_LAND_M,
            MechanicianTier5Skills::HSonic => SKILL_H_SONIC,
            MechanicianTier5Skills::RSmash => SKILL_R_SMASH,
            MechanicianTier5Skills::PEnhence => SKILL_P_ENHENCE,
        }
    }
}

impl From<MechanicianSkills> for SkillCode {
    fn from(v: MechanicianSkills) -> SkillCode {
        match v {
            MechanicianSkills::Tier1(s) => s.into(),
            MechanicianSkills::Tier2(s) => s.into(),
            MechanicianSkills::Tier3(s) => s.into(),
            MechanicianSkills::Tier4(s) => s.into(),
            MechanicianSkills::Tier5(s) => s.into(),
        }
    }
}

impl TryFrom<SkillCode> for MechanicianSkills {
    type Error = ();

    fn try_from(code: SkillCode) -> Result<Self, Self::Error> {
        match code {
            c if c == SKILL_EXTREME_SHIELD => {
                Ok(Self::Tier1(MechanicianTier1Skills::ExtremeShield))
            }
            c if c == SKILL_MECHANIC_BOMB => Ok(Self::Tier1(MechanicianTier1Skills::MechanicBomb)),
            c if c == SKILL_PHYSICAL_ABSORB => {
                Ok(Self::Tier1(MechanicianTier1Skills::PhysicalAbsorb))
            }
            c if c == SKILL_POISON_ATTRIBUTE => {
                Ok(Self::Tier1(MechanicianTier1Skills::PoisonAttribute))
            }
            c if c == SKILL_GREAT_SMASH => Ok(Self::Tier2(MechanicianTier2Skills::GreatSmash)),
            c if c == SKILL_MAXIMIZE => Ok(Self::Tier2(MechanicianTier2Skills::Maximize)),
            c if c == SKILL_AUTOMATION => Ok(Self::Tier2(MechanicianTier2Skills::Automation)),
            c if c == SKILL_SPARK => Ok(Self::Tier2(MechanicianTier2Skills::Spark)),
            c if c == SKILL_METAL_ARMOR => Ok(Self::Tier3(MechanicianTier3Skills::MetalArmor)),
            c if c == SKILL_GRAND_SMASH => Ok(Self::Tier3(MechanicianTier3Skills::GrandSmash)),
            c if c == SKILL_MECHANIC_WEAPON => {
                Ok(Self::Tier3(MechanicianTier3Skills::MechanicWeapon))
            }
            c if c == SKILL_SPARK_SHIELD => Ok(Self::Tier3(MechanicianTier3Skills::SparkShield)),
            c if c == SKILL_IMPULSION => Ok(Self::Tier4(MechanicianTier4Skills::Impulsion)),
            c if c == SKILL_COMPULSION => Ok(Self::Tier4(MechanicianTier4Skills::Compulsion)),
            c if c == SKILL_MAGNETIC_SPHERE => {
                Ok(Self::Tier4(MechanicianTier4Skills::MagneticSphere))
            }
            c if c == SKILL_METAL_GOLEM => Ok(Self::Tier4(MechanicianTier4Skills::MetalGolem)),
            c if c == SKILL_LAND_M => Ok(Self::Tier5(MechanicianTier5Skills::LandM)),
            c if c == SKILL_H_SONIC => Ok(Self::Tier5(MechanicianTier5Skills::HSonic)),
            c if c == SKILL_R_SMASH => Ok(Self::Tier5(MechanicianTier5Skills::RSmash)),
            c if c == SKILL_P_ENHENCE => Ok(Self::Tier5(MechanicianTier5Skills::PEnhence)),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tier1_maps_to_codes() {
        let code: SkillCode = MechanicianTier1Skills::ExtremeShield.into();
        assert_eq!(code, SKILL_EXTREME_SHIELD);
    }

    #[test]
    fn try_from_classifies_skill_and_tier() {
        let result = MechanicianSkills::try_from(SKILL_GRAND_SMASH);
        assert!(result.is_ok());
        let skill = match result {
            Ok(skill) => skill,
            Err(error) => {
                assert!(false, "unexpected mechanician skill failure: {error:?}");
                return;
            }
        };
        assert_eq!(skill.tier(), ChangeJobTier::Tier3);
    }
}
