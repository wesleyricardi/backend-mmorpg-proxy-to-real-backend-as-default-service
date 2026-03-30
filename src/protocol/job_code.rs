use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct RuntimeJobCode(pub u32);

impl RuntimeJobCode {
    pub const fn raw(self) -> u32 {
        self.0
    }
}

impl From<u32> for RuntimeJobCode {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<i32> for RuntimeJobCode {
    fn from(value: i32) -> Self {
        Self(value.max(0) as u32)
    }
}

impl From<RuntimeJobCode> for u32 {
    fn from(value: RuntimeJobCode) -> Self {
        value.0
    }
}

impl socket::SocketPacket for RuntimeJobCode {
    type CRepr = u32;

    fn to_c_repr(&self) -> Self::CRepr {
        self.0
    }

    fn from_c_repr(value: Self::CRepr) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct ClassJobCode(pub u32);

impl ClassJobCode {
    pub const fn raw(self) -> u32 {
        self.0
    }
}

impl From<u32> for ClassJobCode {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<ClassJobCode> for u32 {
    fn from(value: ClassJobCode) -> Self {
        value.0
    }
}
