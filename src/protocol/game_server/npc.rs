use super::monster::MonsterInfo;
use crate::protocol::char::Char;
use serde::{Deserialize, Serialize};
use socket::socket;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[socket]
pub struct TransPlayerInfo {
    pub size: i32,
    pub code: i32,
    pub sm_char_info: Char,
    pub dw_object_serial: u32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub ax: i32,
    pub ay: i32,
    pub az: i32,
    pub state: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct NpcMemory {
    #[socket(string_size = 64)]
    pub file_name: String,
    pub char_info: Char,
    pub monster_info: MonsterInfo,
}
