use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CharMotionState {
    Stand,
    Walk,
    Run,
    Dead,
    Unknown(i32),
}

impl Default for CharMotionState {
    fn default() -> Self {
        Self::Unknown(0)
    }
}

impl CharMotionState {
    pub const STAND_RAW: i32 = 0x40;
    pub const WALK_RAW: i32 = 0x50;
    pub const RUN_RAW: i32 = 0x60;
    pub const DEAD_RAW: i32 = 0x120;

    pub fn from_raw(raw: i32) -> Self {
        match raw {
            Self::STAND_RAW => Self::Stand,
            Self::WALK_RAW => Self::Walk,
            Self::RUN_RAW => Self::Run,
            Self::DEAD_RAW => Self::Dead,
            value => Self::Unknown(value),
        }
    }

    pub fn raw(self) -> i32 {
        match self {
            Self::Stand => Self::STAND_RAW,
            Self::Walk => Self::WALK_RAW,
            Self::Run => Self::RUN_RAW,
            Self::Dead => Self::DEAD_RAW,
            Self::Unknown(value) => value,
        }
    }

    pub fn is_dead(self) -> bool {
        matches!(self, Self::Dead)
    }
}

impl Serialize for CharMotionState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.raw())
    }
}

impl<'de> Deserialize<'de> for CharMotionState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = i32::deserialize(deserializer).map_err(D::Error::custom)?;
        Ok(Self::from_raw(raw))
    }
}
