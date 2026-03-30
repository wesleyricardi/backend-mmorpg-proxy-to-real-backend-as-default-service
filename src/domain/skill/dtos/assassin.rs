use core::convert::TryFrom;

use crate::domain::skill::dtos::codes::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssassinSkills {
    Tier1(AssassinTier1Skills),
    Tier2(AssassinTier2Skills),
    Tier3(AssassinTier3Skills),
    Tier4(AssassinTier4Skills),
    Tier5(AssassinTier5Skills),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssassinTier1Skills {
    Stingger,
    RHit,
    DMastery,
    Wisp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssassinTier2Skills {
    VThrone,
    Alas,
    SShock,
    AMastery,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssassinTier3Skills {
    SSword,
    BUp,
    Inpes,
    Blind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssassinTier4Skills {
    FWind,
    FMastery,
    Polluted,
    PShadow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssassinTier5Skills {
    JBomb,
    RSlash,
    VStab,
    Storm,
}

impl AssassinSkills {
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

impl From<AssassinTier1Skills> for SkillCode {
    fn from(v: AssassinTier1Skills) -> SkillCode {
        match v {
            AssassinTier1Skills::Stingger => SKILL_STINGGER,
            AssassinTier1Skills::RHit => SKILL_R_HIT,
            AssassinTier1Skills::DMastery => SKILL_D_MASTERY,
            AssassinTier1Skills::Wisp => SKILL_WISP,
        }
    }
}

impl From<AssassinTier2Skills> for SkillCode {
    fn from(v: AssassinTier2Skills) -> SkillCode {
        match v {
            AssassinTier2Skills::VThrone => SKILL_V_THRONE,
            AssassinTier2Skills::Alas => SKILL_ALAS,
            AssassinTier2Skills::SShock => SKILL_S_SHOCK,
            AssassinTier2Skills::AMastery => SKILL_A_MASTERY,
        }
    }
}

impl From<AssassinTier3Skills> for SkillCode {
    fn from(v: AssassinTier3Skills) -> SkillCode {
        match v {
            AssassinTier3Skills::SSword => SKILL_S_SWORD,
            AssassinTier3Skills::BUp => SKILL_B_UP,
            AssassinTier3Skills::Inpes => SKILL_INPES,
            AssassinTier3Skills::Blind => SKILL_BLIND,
        }
    }
}

impl From<AssassinTier4Skills> for SkillCode {
    fn from(v: AssassinTier4Skills) -> SkillCode {
        match v {
            AssassinTier4Skills::FWind => SKILL_F_WIND,
            AssassinTier4Skills::FMastery => SKILL_F_MASTERY,
            AssassinTier4Skills::Polluted => SKILL_POLLUTED,
            AssassinTier4Skills::PShadow => SKILL_P_SHADOW,
        }
    }
}

impl From<AssassinTier5Skills> for SkillCode {
    fn from(v: AssassinTier5Skills) -> SkillCode {
        match v {
            AssassinTier5Skills::JBomb => SKILL_J_BOMB,
            AssassinTier5Skills::RSlash => SKILL_R_SLASH,
            AssassinTier5Skills::VStab => SKILL_V_STAB,
            AssassinTier5Skills::Storm => SKILL_STORM,
        }
    }
}

impl From<AssassinSkills> for SkillCode {
    fn from(v: AssassinSkills) -> SkillCode {
        match v {
            AssassinSkills::Tier1(s) => s.into(),
            AssassinSkills::Tier2(s) => s.into(),
            AssassinSkills::Tier3(s) => s.into(),
            AssassinSkills::Tier4(s) => s.into(),
            AssassinSkills::Tier5(s) => s.into(),
        }
    }
}

impl TryFrom<SkillCode> for AssassinSkills {
    type Error = ();

    fn try_from(code: SkillCode) -> Result<Self, Self::Error> {
        match code {
            c if c == SKILL_STINGGER => Ok(Self::Tier1(AssassinTier1Skills::Stingger)),
            c if c == SKILL_R_HIT => Ok(Self::Tier1(AssassinTier1Skills::RHit)),
            c if c == SKILL_D_MASTERY => Ok(Self::Tier1(AssassinTier1Skills::DMastery)),
            c if c == SKILL_WISP => Ok(Self::Tier1(AssassinTier1Skills::Wisp)),
            c if c == SKILL_V_THRONE => Ok(Self::Tier2(AssassinTier2Skills::VThrone)),
            c if c == SKILL_ALAS => Ok(Self::Tier2(AssassinTier2Skills::Alas)),
            c if c == SKILL_S_SHOCK => Ok(Self::Tier2(AssassinTier2Skills::SShock)),
            c if c == SKILL_A_MASTERY => Ok(Self::Tier2(AssassinTier2Skills::AMastery)),
            c if c == SKILL_S_SWORD => Ok(Self::Tier3(AssassinTier3Skills::SSword)),
            c if c == SKILL_B_UP => Ok(Self::Tier3(AssassinTier3Skills::BUp)),
            c if c == SKILL_INPES => Ok(Self::Tier3(AssassinTier3Skills::Inpes)),
            c if c == SKILL_BLIND => Ok(Self::Tier3(AssassinTier3Skills::Blind)),
            c if c == SKILL_F_WIND => Ok(Self::Tier4(AssassinTier4Skills::FWind)),
            c if c == SKILL_F_MASTERY => Ok(Self::Tier4(AssassinTier4Skills::FMastery)),
            c if c == SKILL_POLLUTED => Ok(Self::Tier4(AssassinTier4Skills::Polluted)),
            c if c == SKILL_P_SHADOW => Ok(Self::Tier4(AssassinTier4Skills::PShadow)),
            c if c == SKILL_J_BOMB => Ok(Self::Tier5(AssassinTier5Skills::JBomb)),
            c if c == SKILL_R_SLASH => Ok(Self::Tier5(AssassinTier5Skills::RSlash)),
            c if c == SKILL_V_STAB => Ok(Self::Tier5(AssassinTier5Skills::VStab)),
            c if c == SKILL_STORM => Ok(Self::Tier5(AssassinTier5Skills::Storm)),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tier1_maps_to_codes() {
        let code: SkillCode = AssassinTier1Skills::Stingger.into();
        assert_eq!(code, SKILL_STINGGER);
    }

    #[test]
    fn try_from_classifies_skill_and_tier() {
        let result = AssassinSkills::try_from(SKILL_B_UP);
        assert!(result.is_ok());
        let skill = match result {
            Ok(skill) => skill,
            Err(error) => {
                assert!(false, "unexpected assassin skill failure: {error:?}");
                return;
            }
        };
        assert_eq!(skill.tier(), ChangeJobTier::Tier3);
    }
}
