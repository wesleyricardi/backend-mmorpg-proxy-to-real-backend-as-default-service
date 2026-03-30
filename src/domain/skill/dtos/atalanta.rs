use core::convert::TryFrom;

use crate::domain::skill::dtos::codes::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AtalantaSkills {
    Tier1(AtalantaTier1Skills),
    Tier2(AtalantaTier2Skills),
    Tier3(AtalantaTier3Skills),
    Tier4(AtalantaTier4Skills),
    Tier5(AtalantaTier5Skills),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AtalantaTier1Skills {
    ShieldStrike,
    Farina,
    ThrowingMastery,
    VigorSpear,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AtalantaTier2Skills {
    Windy,
    TwistJavelin,
    SoulSucker,
    FireJavelin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AtalantaTier3Skills {
    SplitJavelin,
    TriumphOfValhalla,
    LightningJavelin,
    StormJavelin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AtalantaTier4Skills {
    HallOfValhalla,
    XRage,
    FrostJavelin,
    Vengeance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AtalantaTier5Skills {
    Talaria,
    GCoup,
    Arcuda,
    SFear,
}

impl AtalantaSkills {
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

impl From<AtalantaTier1Skills> for SkillCode {
    fn from(v: AtalantaTier1Skills) -> SkillCode {
        match v {
            AtalantaTier1Skills::ShieldStrike => SKILL_SHIELD_STRIKE,
            AtalantaTier1Skills::Farina => SKILL_FARINA,
            AtalantaTier1Skills::ThrowingMastery => SKILL_THROWING_MASTERY,
            AtalantaTier1Skills::VigorSpear => SKILL_VIGOR_SPEAR,
        }
    }
}

impl From<AtalantaTier2Skills> for SkillCode {
    fn from(v: AtalantaTier2Skills) -> SkillCode {
        match v {
            AtalantaTier2Skills::Windy => SKILL_WINDY,
            AtalantaTier2Skills::TwistJavelin => SKILL_TWIST_JAVELIN,
            AtalantaTier2Skills::SoulSucker => SKILL_SOUL_SUCKER,
            AtalantaTier2Skills::FireJavelin => SKILL_FIRE_JAVELIN,
        }
    }
}

impl From<AtalantaTier3Skills> for SkillCode {
    fn from(v: AtalantaTier3Skills) -> SkillCode {
        match v {
            AtalantaTier3Skills::SplitJavelin => SKILL_SPLIT_JAVELIN,
            AtalantaTier3Skills::TriumphOfValhalla => SKILL_TRIUMPH_OF_VALHALLA,
            AtalantaTier3Skills::LightningJavelin => SKILL_LIGHTNING_JAVELIN,
            AtalantaTier3Skills::StormJavelin => SKILL_STORM_JAVELIN,
        }
    }
}

impl From<AtalantaTier4Skills> for SkillCode {
    fn from(v: AtalantaTier4Skills) -> SkillCode {
        match v {
            AtalantaTier4Skills::HallOfValhalla => SKILL_HALL_OF_VALHALLA,
            AtalantaTier4Skills::XRage => SKILL_X_RAGE,
            AtalantaTier4Skills::FrostJavelin => SKILL_FROST_JAVELIN,
            AtalantaTier4Skills::Vengeance => SKILL_VENGEANCE,
        }
    }
}

impl From<AtalantaTier5Skills> for SkillCode {
    fn from(v: AtalantaTier5Skills) -> SkillCode {
        match v {
            AtalantaTier5Skills::Talaria => SKILL_TALARIA,
            AtalantaTier5Skills::GCoup => SKILL_G_COUP,
            AtalantaTier5Skills::Arcuda => SKILL_ARCUDA,
            AtalantaTier5Skills::SFear => SKILL_S_FEAR,
        }
    }
}

impl From<AtalantaSkills> for SkillCode {
    fn from(v: AtalantaSkills) -> SkillCode {
        match v {
            AtalantaSkills::Tier1(s) => s.into(),
            AtalantaSkills::Tier2(s) => s.into(),
            AtalantaSkills::Tier3(s) => s.into(),
            AtalantaSkills::Tier4(s) => s.into(),
            AtalantaSkills::Tier5(s) => s.into(),
        }
    }
}

impl TryFrom<SkillCode> for AtalantaSkills {
    type Error = ();

    fn try_from(code: SkillCode) -> Result<Self, Self::Error> {
        match code {
            c if c == SKILL_SHIELD_STRIKE => Ok(Self::Tier1(AtalantaTier1Skills::ShieldStrike)),
            c if c == SKILL_FARINA => Ok(Self::Tier1(AtalantaTier1Skills::Farina)),
            c if c == SKILL_THROWING_MASTERY => {
                Ok(Self::Tier1(AtalantaTier1Skills::ThrowingMastery))
            }
            c if c == SKILL_VIGOR_SPEAR => Ok(Self::Tier1(AtalantaTier1Skills::VigorSpear)),
            c if c == SKILL_WINDY => Ok(Self::Tier2(AtalantaTier2Skills::Windy)),
            c if c == SKILL_TWIST_JAVELIN => Ok(Self::Tier2(AtalantaTier2Skills::TwistJavelin)),
            c if c == SKILL_SOUL_SUCKER => Ok(Self::Tier2(AtalantaTier2Skills::SoulSucker)),
            c if c == SKILL_FIRE_JAVELIN => Ok(Self::Tier2(AtalantaTier2Skills::FireJavelin)),
            c if c == SKILL_SPLIT_JAVELIN => Ok(Self::Tier3(AtalantaTier3Skills::SplitJavelin)),
            c if c == SKILL_TRIUMPH_OF_VALHALLA => {
                Ok(Self::Tier3(AtalantaTier3Skills::TriumphOfValhalla))
            }
            c if c == SKILL_LIGHTNING_JAVELIN => {
                Ok(Self::Tier3(AtalantaTier3Skills::LightningJavelin))
            }
            c if c == SKILL_STORM_JAVELIN => Ok(Self::Tier3(AtalantaTier3Skills::StormJavelin)),
            c if c == SKILL_HALL_OF_VALHALLA => {
                Ok(Self::Tier4(AtalantaTier4Skills::HallOfValhalla))
            }
            c if c == SKILL_X_RAGE => Ok(Self::Tier4(AtalantaTier4Skills::XRage)),
            c if c == SKILL_FROST_JAVELIN => Ok(Self::Tier4(AtalantaTier4Skills::FrostJavelin)),
            c if c == SKILL_VENGEANCE => Ok(Self::Tier4(AtalantaTier4Skills::Vengeance)),
            c if c == SKILL_TALARIA => Ok(Self::Tier5(AtalantaTier5Skills::Talaria)),
            c if c == SKILL_G_COUP => Ok(Self::Tier5(AtalantaTier5Skills::GCoup)),
            c if c == SKILL_ARCUDA => Ok(Self::Tier5(AtalantaTier5Skills::Arcuda)),
            c if c == SKILL_S_FEAR => Ok(Self::Tier5(AtalantaTier5Skills::SFear)),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tier1_maps_to_codes() {
        let code: SkillCode = AtalantaTier1Skills::ShieldStrike.into();
        assert_eq!(code, SKILL_SHIELD_STRIKE);
    }

    #[test]
    fn try_from_classifies_skill_and_tier() {
        let result = AtalantaSkills::try_from(SKILL_TRIUMPH_OF_VALHALLA);
        assert!(result.is_ok());
        let skill = match result {
            Ok(skill) => skill,
            Err(error) => {
                assert!(false, "unexpected atalanta skill failure: {error:?}");
                return;
            }
        };
        assert_eq!(skill.tier(), ChangeJobTier::Tier3);
    }
}
