use serde::{Deserialize, Serialize};
use socket::socket;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct SubscribeWorldRuntimeRequest {
    pub request_id: u32,
    pub include_monsters: i32,
    pub include_npcs: i32,
    pub full_snapshot: i32,
    pub incremental_updates: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct RequestWorldRuntimeSnapshotRequest {
    pub request_id: u32,
    pub field_id: i32,
    pub include_monsters: i32,
    pub include_npcs: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct WorldRuntimeSnapshotBegin {
    pub request_id: u32,
    pub sequence: u64,
    pub field_id: i32,
    pub include_monsters: i32,
    pub include_npcs: i32,
    pub monster_count: i32,
    pub npc_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct RuntimeMonsterSync {
    pub sequence: u64,
    pub object_serial: u32,
    pub template_auto_char_code: u32,
    pub field_id: i32,
    pub change_mask: u32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub ax: i32,
    pub ay: i32,
    pub az: i32,
    pub current_life: i32,
    pub max_life: i32,
    pub motion_state: i32,
    #[socket(string_size = 64)]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct RuntimeMonsterDespawn {
    pub sequence: u64,
    pub object_serial: u32,
    pub field_id: i32,
    pub reason: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct RuntimeNpcSync {
    pub sequence: u64,
    pub object_serial: u32,
    pub field_id: i32,
    pub change_mask: u32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub ax: i32,
    pub ay: i32,
    pub az: i32,
    pub motion_state: i32,
    #[socket(string_size = 128)]
    pub template_file: String,
    #[socket(string_size = 64)]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct RuntimeNpcDespawn {
    pub sequence: u64,
    pub object_serial: u32,
    pub field_id: i32,
    pub reason: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct WorldRuntimeSnapshotEnd {
    pub request_id: u32,
    pub sequence: u64,
    pub field_id: i32,
}
