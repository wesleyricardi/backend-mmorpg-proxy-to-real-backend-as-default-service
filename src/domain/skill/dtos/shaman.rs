use core::convert::TryFrom;

use crate::domain::skill::dtos::codes::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShamanSkills {
    Tier1(ShamanTier1Skills),
    Tier2(ShamanTier2Skills),
    Tier3(ShamanTier3Skills),
    Tier4(ShamanTier4Skills),
    Tier5(ShamanTier5Skills),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShamanTier1Skills {
    Darkbolt,
    Darkwave,
    Curselazy,
    LPeace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShamanTier2Skills {
    SFlare,
    SManacle,
    CHunt,
    AMigal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShamanTier3Skills {
    RMaker,
    LGhost,
    Haunt,
    Scratch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShamanTier4Skills {
    RKnight,
    Judge,
    AMidranda,
    MPray,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShamanTier5Skills {
    Creed,
    PDeity,
    GNail,
    HRegene,
}

impl ShamanSkills {
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

impl From<ShamanTier1Skills> for SkillCode {
    fn from(v: ShamanTier1Skills) -> SkillCode {
        match v {
            ShamanTier1Skills::Darkbolt => SKILL_DARKBOLT,
            ShamanTier1Skills::Darkwave => SKILL_DARKWAVE,
            ShamanTier1Skills::Curselazy => SKILL_CURSELAZY,
            ShamanTier1Skills::LPeace => SKILL_L_PEACE,
        }
    }
}

impl From<ShamanTier2Skills> for SkillCode {
    fn from(v: ShamanTier2Skills) -> SkillCode {
        match v {
            ShamanTier2Skills::SFlare => SKILL_S_FLARE,
            ShamanTier2Skills::SManacle => SKILL_S_MANACLE,
            ShamanTier2Skills::CHunt => SKILL_C_HUNT,
            ShamanTier2Skills::AMigal => SKILL_A_MIGAL,
        }
    }
}

impl From<ShamanTier3Skills> for SkillCode {
    fn from(v: ShamanTier3Skills) -> SkillCode {
        match v {
            ShamanTier3Skills::RMaker => SKILL_R_MAKER,
            ShamanTier3Skills::LGhost => SKILL_L_GHOST,
            ShamanTier3Skills::Haunt => SKILL_HAUNT,
            ShamanTier3Skills::Scratch => SKILL_SCRATCH,
        }
    }
}

impl From<ShamanTier4Skills> for SkillCode {
    fn from(v: ShamanTier4Skills) -> SkillCode {
        match v {
            ShamanTier4Skills::RKnight => SKILL_R_KNIGHT,
            ShamanTier4Skills::Judge => SKILL_JUDGE,
            ShamanTier4Skills::AMidranda => SKILL_A_MIDRANDA,
            ShamanTier4Skills::MPray => SKILL_M_PRAY,
        }
    }
}

impl From<ShamanTier5Skills> for SkillCode {
    fn from(v: ShamanTier5Skills) -> SkillCode {
        match v {
            ShamanTier5Skills::Creed => SKILL_CREED,
            ShamanTier5Skills::PDeity => SKILL_P_DEITY,
            ShamanTier5Skills::GNail => SKILL_G_NAIL,
            ShamanTier5Skills::HRegene => SKILL_H_REGENE,
        }
    }
}

impl From<ShamanSkills> for SkillCode {
    fn from(v: ShamanSkills) -> SkillCode {
        match v {
            ShamanSkills::Tier1(s) => s.into(),
            ShamanSkills::Tier2(s) => s.into(),
            ShamanSkills::Tier3(s) => s.into(),
            ShamanSkills::Tier4(s) => s.into(),
            ShamanSkills::Tier5(s) => s.into(),
        }
    }
}

impl TryFrom<SkillCode> for ShamanSkills {
    type Error = ();

    fn try_from(code: SkillCode) -> Result<Self, Self::Error> {
        match code {
            c if c == SKILL_DARKBOLT => Ok(Self::Tier1(ShamanTier1Skills::Darkbolt)),
            c if c == SKILL_DARKWAVE => Ok(Self::Tier1(ShamanTier1Skills::Darkwave)),
            c if c == SKILL_CURSELAZY => Ok(Self::Tier1(ShamanTier1Skills::Curselazy)),
            c if c == SKILL_L_PEACE => Ok(Self::Tier1(ShamanTier1Skills::LPeace)),
            c if c == SKILL_S_FLARE => Ok(Self::Tier2(ShamanTier2Skills::SFlare)),
            c if c == SKILL_S_MANACLE => Ok(Self::Tier2(ShamanTier2Skills::SManacle)),
            c if c == SKILL_C_HUNT => Ok(Self::Tier2(ShamanTier2Skills::CHunt)),
            c if c == SKILL_A_MIGAL => Ok(Self::Tier2(ShamanTier2Skills::AMigal)),
            c if c == SKILL_R_MAKER => Ok(Self::Tier3(ShamanTier3Skills::RMaker)),
            c if c == SKILL_L_GHOST => Ok(Self::Tier3(ShamanTier3Skills::LGhost)),
            c if c == SKILL_HAUNT => Ok(Self::Tier3(ShamanTier3Skills::Haunt)),
            c if c == SKILL_SCRATCH => Ok(Self::Tier3(ShamanTier3Skills::Scratch)),
            c if c == SKILL_R_KNIGHT => Ok(Self::Tier4(ShamanTier4Skills::RKnight)),
            c if c == SKILL_JUDGE => Ok(Self::Tier4(ShamanTier4Skills::Judge)),
            c if c == SKILL_A_MIDRANDA => Ok(Self::Tier4(ShamanTier4Skills::AMidranda)),
            c if c == SKILL_M_PRAY => Ok(Self::Tier4(ShamanTier4Skills::MPray)),
            c if c == SKILL_CREED => Ok(Self::Tier5(ShamanTier5Skills::Creed)),
            c if c == SKILL_P_DEITY => Ok(Self::Tier5(ShamanTier5Skills::PDeity)),
            c if c == SKILL_G_NAIL => Ok(Self::Tier5(ShamanTier5Skills::GNail)),
            c if c == SKILL_H_REGENE => Ok(Self::Tier5(ShamanTier5Skills::HRegene)),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tier1_maps_to_codes() {
        let code: SkillCode = ShamanTier1Skills::Darkbolt.into();
        assert_eq!(code, SKILL_DARKBOLT);
    }

    #[test]
    fn try_from_classifies_skill_and_tier() {
        let result = ShamanSkills::try_from(SKILL_L_GHOST);
        assert!(result.is_ok());
        let skill = match result {
            Ok(skill) => skill,
            Err(error) => {
                assert!(false, "unexpected shaman skill failure: {error:?}");
                return;
            }
        };
        assert_eq!(skill.tier(), ChangeJobTier::Tier3);
    }
}
