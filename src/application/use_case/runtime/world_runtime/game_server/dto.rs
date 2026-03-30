use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::domain::field::dto::FieldId;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GameServerData {
    pub fields: Vec<GameField>,
    pub monsters: Vec<GameMonster>,
    pub npcs: Vec<GameNpc>,
    pub open_items: Vec<GameOpenItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldPoint {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldPoint2D {
    pub x: i32,
    pub z: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MonsterId(pub u32);

impl MonsterId {
    pub fn from_legacy_auto_char_code(value: u32) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NpcId(pub String);

impl NpcId {
    pub fn from_file_name(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NpcSpawnId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RuntimeMonsterId(pub u32);

impl RuntimeMonsterId {
    pub fn from_object_serial(value: u32) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RuntimeNpcId(pub u32);

impl RuntimeNpcId {
    pub fn from_object_serial(value: u32) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameFieldWarpGate {
    pub position: WorldPoint,
    pub height: i32,
    pub size: i32,
    pub limit_level: i32,
    pub special_effect: i32,
    pub destinations: Vec<WorldPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldNpcSpawn {
    pub npc_id: Option<NpcId>,
    pub name: String,
    pub position: WorldPoint,
    pub direction: WorldPoint,
    pub dialog_title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldMonsterSpawnRule {
    pub monster_id: Option<MonsterId>,
    pub monster_name: String,
    pub open_percentage: i32,
    pub spawn_start_index: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldMonsterSpawnPoint {
    pub index: usize,
    pub position: WorldPoint2D,
    pub state: i32,
    pub near_play_count: i32,
    pub monster_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldBossSpawnRule {
    pub master_monster_id: Option<MonsterId>,
    pub master_monster_name: String,
    pub slave_monster_id: Option<MonsterId>,
    pub slave_monster_name: String,
    pub slave_count: i32,
    pub open_hours: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameField {
    pub index: usize,
    pub field_id: Option<FieldId>,
    pub stage_file: String,
    pub map_image_file: String,
    pub title_image_file: String,
    pub state: i32,
    pub event_code: i32,
    pub back_music_code: i32,
    pub limit_level: i32,
    pub field_code: i32,
    pub server_code: i32,
    pub center: WorldPoint2D,
    pub start_points: Vec<WorldPoint2D>,
    pub warp_gates: Vec<GameFieldWarpGate>,
    pub npc_spawns: Vec<FieldNpcSpawn>,
    pub monster_spawn_points: Vec<FieldMonsterSpawnPoint>,
    pub monster_spawn_rules: Vec<FieldMonsterSpawnRule>,
    pub boss_spawn_rules: Vec<FieldBossSpawnRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterDrop {
    pub item_code: u32,
    pub percentage: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameMonster {
    pub index: usize,
    pub id: MonsterId,
    pub auto_char_code: u32,
    pub name: String,
    pub model_file: String,
    pub level: i32,
    pub life: i32,
    pub attack_rating: i32,
    pub attack_damage_min: i32,
    pub attack_damage_max: i32,
    pub defense: i32,
    pub absorption: i32,
    pub move_speed: i32,
    pub sight: i32,
    pub move_range: i32,
    pub exp: i64,
    pub nature: i32,
    pub undead: i32,
    pub class_code: i32,
    pub is_boss: bool,
    pub drops: Vec<MonsterDrop>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NpcServices {
    pub teleport: bool,
    pub warehouse: bool,
    pub item_mix: bool,
    pub aging: bool,
    pub collect_money: bool,
    pub event_gift: bool,
    pub smelting: bool,
    pub manufacture: bool,
    pub mixture_reset: bool,
    pub remodel: bool,
    pub coin_shop: bool,
    pub time_shop: bool,
    pub clan_service: bool,
    pub gift_express: bool,
    pub wing_quest: bool,
    pub star_point: bool,
    pub give_money: bool,
    pub bless_castle: bool,
    pub polling: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameNpc {
    pub index: usize,
    pub id: NpcId,
    pub file_name: String,
    pub name: String,
    pub model_file: String,
    pub dialog_title: Option<String>,
    pub quest_code: i32,
    pub quest_param: i32,
    pub services: NpcServices,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameOpenItem {
    pub index: usize,
    pub code: u32,
    pub name: String,
    pub level_requirement: i32,
    pub price: i32,
    pub weight: i32,
    pub attack_speed: i32,
    pub attack_rating: i32,
    pub critical_hit: i32,
    pub defense: i32,
    pub block_rating: f32,
    pub absorb: f32,
    pub min_damage: i16,
    pub max_damage: i16,
    pub durability_min: i16,
    pub durability_max: i16,
    pub job_code_mask: u32,
    pub unique_item: u32,
    pub disp_effect: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcSpawnLocation {
    pub spawn_id: NpcSpawnId,
    pub npc_id: NpcId,
    pub field_id: FieldId,
    pub position: WorldPoint,
    pub direction: WorldPoint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterTemplateState {
    pub monster: GameMonster,
    pub field_ids: HashSet<FieldId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcTemplateState {
    pub npc: GameNpc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldSpawnConfigState {
    pub npc_spawns: Vec<NpcSpawnLocation>,
    pub monster_spawn_points: Vec<FieldMonsterSpawnPoint>,
    pub monster_spawn_rules: Vec<FieldMonsterSpawnRule>,
    pub boss_spawn_rules: Vec<FieldBossSpawnRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeMonsterState {
    pub id: RuntimeMonsterId,
    pub template_id: Option<MonsterId>,
    pub field_id: Option<FieldId>,
    pub name: String,
    pub position: WorldPoint,
    pub direction: WorldPoint,
    pub current_life: i32,
    pub max_life: i32,
    pub motion_state: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeNpcState {
    pub id: RuntimeNpcId,
    pub template_id: Option<NpcId>,
    pub field_id: Option<FieldId>,
    pub name: String,
    pub position: WorldPoint,
    pub direction: WorldPoint,
    pub motion_state: i32,
}

#[cfg(test)]
mod tests {
    use super::{MonsterId, NpcId, RuntimeMonsterId, RuntimeNpcId};

    #[test]
    fn monster_id_uses_legacy_auto_char_code() {
        assert_eq!(
            MonsterId::from_legacy_auto_char_code(0x1234_ABCD).0,
            0x1234_ABCD
        );
    }

    #[test]
    fn npc_id_uses_file_name() {
        assert_eq!(
            NpcId::from_file_name("teleporter.npc").0,
            "teleporter.npc".to_string()
        );
    }

    #[test]
    fn runtime_monster_id_uses_object_serial() {
        assert_eq!(
            RuntimeMonsterId::from_object_serial(0xDEAD_BEEF).0,
            0xDEAD_BEEF
        );
    }

    #[test]
    fn runtime_npc_id_uses_object_serial() {
        assert_eq!(RuntimeNpcId::from_object_serial(0x00AB_CDEF).0, 0x00AB_CDEF);
    }
}
