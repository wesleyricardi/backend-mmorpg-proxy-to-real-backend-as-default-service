use self::field::{
    FieldMemory, StageAreaBossRuleChunk, StageAreaMemory, StageAreaMonsterRuleChunk,
    StageAreaNpcChunk, StageAreaStartPointChunk, StageAreaWarmupSummary,
};
use self::monster::MonsterMemory;
use self::npc::NpcMemory;
use self::open_item::DefItemInfo;
use self::runtime::{RuntimeMonsterDespawn, RuntimeMonsterSync, RuntimeNpcDespawn, RuntimeNpcSync};
use serde::{Deserialize, Serialize};
use socket::socket;

pub mod field;
pub mod monster;
pub mod npc;
pub mod open_item;
pub mod runtime;
pub use self::runtime::{
    RequestWorldRuntimeSnapshotRequest, SubscribeWorldRuntimeRequest, WorldRuntimeSnapshotBegin,
    WorldRuntimeSnapshotEnd,
};

pub const GET_GAME_SERVER_ALL_DATA_CODE: i32 = 0x4A00_0100;
pub const GAME_SERVER_WARMUP_META_CODE: i32 = 0x4A00_0101;
pub const GAME_SERVER_WARMUP_FIELD_CODE: i32 = 0x4A00_0102;
pub const GAME_SERVER_WARMUP_MONSTER_CODE: i32 = 0x4A00_0103;
pub const GAME_SERVER_WARMUP_NPC_CODE: i32 = 0x4A00_0104;
pub const GAME_SERVER_WARMUP_OPEN_ITEM_CODE: i32 = 0x4A00_0105;
pub const GAME_SERVER_WARMUP_STAGE_AREA_CODE: i32 = 0x4A00_0106;
pub const GAME_SERVER_WARMUP_DONE_CODE: i32 = 0x4A00_0107;
pub const GAME_SERVER_WARMUP_STAGE_AREA_NPC_CODE: i32 = 0x4A00_0108;
pub const GAME_SERVER_WARMUP_STAGE_AREA_START_POINT_CODE: i32 = 0x4A00_0109;
pub const GAME_SERVER_WARMUP_STAGE_AREA_MONSTER_RULE_CODE: i32 = 0x4A00_010A;
pub const GAME_SERVER_WARMUP_STAGE_AREA_BOSS_RULE_CODE: i32 = 0x4A00_010B;
pub const SUBSCRIBE_WORLD_RUNTIME_CODE: i32 = 0x4A00_0200;
pub const REQUEST_WORLD_RUNTIME_SNAPSHOT_CODE: i32 = 0x4A00_0201;
pub const WORLD_RUNTIME_SNAPSHOT_BEGIN_CODE: i32 = 0x4A00_0210;
pub const WORLD_RUNTIME_MONSTER_SYNC_CODE: i32 = 0x4A00_0211;
pub const WORLD_RUNTIME_MONSTER_DESPAWN_CODE: i32 = 0x4A00_0212;
pub const WORLD_RUNTIME_NPC_SYNC_CODE: i32 = 0x4A00_0220;
pub const WORLD_RUNTIME_NPC_DESPAWN_CODE: i32 = 0x4A00_0221;
pub const WORLD_RUNTIME_SNAPSHOT_END_CODE: i32 = 0x4A00_0230;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct GetGameServerAllDataRequest {
    pub request_id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct GameServerWarmupMeta {
    pub request_id: u32,
    pub field_count: i32,
    pub monster_count: i32,
    pub npc_count: i32,
    pub open_item_count: i32,
    pub stage_area_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct GameServerFieldChunk {
    pub index: i32,
    pub total: i32,
    pub data: FieldMemory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct GameServerMonsterChunk {
    pub index: i32,
    pub total: i32,
    pub data: MonsterMemory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct GameServerNpcChunk {
    pub index: i32,
    pub total: i32,
    pub data: NpcMemory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct GameServerOpenItemChunk {
    pub index: i32,
    pub total: i32,
    pub data: DefItemInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct GameServerStageAreaChunk {
    pub index: i32,
    pub total: i32,
    pub data: StageAreaWarmupSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct GameServerStageAreaNpcChunk {
    pub data: StageAreaNpcChunk,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct GameServerStageAreaStartPointChunk {
    pub data: StageAreaStartPointChunk,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct GameServerStageAreaMonsterRuleChunk {
    pub data: StageAreaMonsterRuleChunk,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct GameServerStageAreaBossRuleChunk {
    pub data: StageAreaBossRuleChunk,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct GameServerWarmupDone {
    pub request_id: u32,
}

socket::protocol_module! {
    GetGameServerAllData => GetGameServerAllDataRequest = 0x4A00_0100,
    GameServerWarmupMeta => GameServerWarmupMeta = 0x4A00_0101,
    GameServerWarmupField => GameServerFieldChunk = 0x4A00_0102,
    GameServerWarmupMonster => GameServerMonsterChunk = 0x4A00_0103,
    GameServerWarmupNpc => GameServerNpcChunk = 0x4A00_0104,
    GameServerWarmupOpenItem => GameServerOpenItemChunk = 0x4A00_0105,
    GameServerWarmupStageArea => GameServerStageAreaChunk = 0x4A00_0106,
    GameServerWarmupDone => GameServerWarmupDone = 0x4A00_0107,
    GameServerWarmupStageAreaNpc => GameServerStageAreaNpcChunk = 0x4A00_0108,
    GameServerWarmupStageAreaStartPoint => GameServerStageAreaStartPointChunk = 0x4A00_0109,
    GameServerWarmupStageAreaMonsterRule => GameServerStageAreaMonsterRuleChunk = 0x4A00_010A,
    GameServerWarmupStageAreaBossRule => GameServerStageAreaBossRuleChunk = 0x4A00_010B,
    SubscribeWorldRuntime => SubscribeWorldRuntimeRequest = 0x4A00_0200,
    RequestWorldRuntimeSnapshot => RequestWorldRuntimeSnapshotRequest = 0x4A00_0201,
    WorldRuntimeSnapshotBegin => WorldRuntimeSnapshotBegin = 0x4A00_0210,
    WorldRuntimeMonsterSync => RuntimeMonsterSync = 0x4A00_0211,
    WorldRuntimeMonsterDespawn => RuntimeMonsterDespawn = 0x4A00_0212,
    WorldRuntimeNpcSync => RuntimeNpcSync = 0x4A00_0220,
    WorldRuntimeNpcDespawn => RuntimeNpcDespawn = 0x4A00_0221,
    WorldRuntimeSnapshotEnd => WorldRuntimeSnapshotEnd = 0x4A00_0230
}
