use super::monster::MonsterMemory;
use super::npc::TransPlayerInfo;
use super::open_item::DefItemInfo;
use crate::protocol::char::Char;
use crate::protocol::item::Item;
use serde::{Deserialize, Serialize};
use socket::socket;

pub const FILED_GATE_MAX: usize = 12;
pub const FIELD_AMBENT_MAX: usize = 80;
pub const FIELD_START_POINT_MAX: usize = 8;
pub const FIELD_STAGE_OBJ_MAX: usize = 50;
pub const STG_START_POINT_MAX: usize = 200;
pub const STG_ITEM_MAX: usize = 1024;
pub const FIX_CHAR_MAX: usize = 100;
pub const STAGE_MONSTER_LIST_MAX: usize = 50;
pub const STAGE_BOSS_MONSTER_MAX: usize = 16;
pub const STAGE_AREA_NPC_CHUNK_MAX: usize = 8;
pub const STAGE_AREA_START_POINT_CHUNK_MAX: usize = 32;
pub const STAGE_AREA_MONSTER_RULE_CHUNK_MAX: usize = 16;
pub const STAGE_AREA_BOSS_RULE_CHUNK_MAX: usize = 8;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[socket]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[socket]
pub struct Point3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[socket]
pub struct FieldGate {
    pub x: i32,
    pub z: i32,
    pub y: i32,
    pub lps_field_addr: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[socket]
pub struct WarpGate {
    pub x: i32,
    pub z: i32,
    pub y: i32,
    pub height: i32,
    pub size: i32,
    pub out_gate: [FieldGate; FILED_GATE_MAX],
    pub out_gate_count: i32,
    pub limit_level: i32,
    pub special_effect: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[socket]
pub struct AmbientPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub round: i32,
    pub ambent_num: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[socket]
pub struct ActionFieldCamera {
    pub fix_pos: Point3D,
    pub left_x: i32,
    pub right_x: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct FieldMemory {
    pub head: u32,
    #[socket(string_size = 64)]
    pub sz_name: String,
    #[socket(string_size = 64)]
    pub sz_name_map: String,
    #[socket(string_size = 64)]
    pub sz_name_title: String,
    pub state: i32,
    pub back_image_code: [i32; 3],
    pub back_music_code: i32,
    pub field_event: i32,
    pub gate_count: i32,
    pub field_gate: [FieldGate; FILED_GATE_MAX],
    pub warp_gate_count: i32,
    pub warp_gate_active_num: i32,
    pub warp_gate: [WarpGate; FILED_GATE_MAX],
    pub pos_warp_out: Point3D,
    #[serde(with = "serde_arrays")]
    pub ambient_pos: [AmbientPos; FIELD_AMBENT_MAX],
    pub ambent_count: i32,
    pub limit_level: i32,
    pub field_sight: i32,
    #[serde(with = "serde_arrays")]
    pub lp_stage_object_name_addr: [u32; FIELD_STAGE_OBJ_MAX],
    #[serde(with = "serde_arrays")]
    pub stg_obj_bip: [u32; FIELD_STAGE_OBJ_MAX],
    pub stg_obj_count: i32,
    pub c_x: i32,
    pub c_z: i32,
    pub field_code: i32,
    pub server_code: i32,
    pub start_point: [Point; FIELD_START_POINT_MAX],
    pub start_point_cnt: i32,
    pub action_camera: ActionFieldCamera,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct StageCharInfo {
    pub sm_char_info: Char,
    pub posi_state: i32,
    pub start_fixed: i32,
    pub start_posi: Point,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[socket]
pub struct StageStartPoint {
    pub state: i32,
    pub x: i32,
    pub z: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct PsItem {
    pub state: i32,
    pub serial: i32,
    pub item_info: Item,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct StageItem {
    pub state: i32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub item: PsItem,
    pub dw_create_time: u32,
    pub dw_lose_time: u32,
    pub level: u32,
    pub begin_mode: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[socket]
pub struct StageMonsterEntry {
    #[socket(string_size = 32)]
    pub sz_monster_name: String,
    pub open_percentage: i32,
    pub lp_char_info_addr: u32,
    pub num_open_start: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[socket]
pub struct StageBossMonsterEntry {
    #[socket(string_size = 32)]
    pub master_monster_name: String,
    pub master_monster_auto_code: u32,
    pub master_monster_addr: u32,
    #[socket(string_size = 32)]
    pub slave_monster_name: String,
    pub slave_monster_auto_code: u32,
    pub slave_monster_addr: u32,
    pub slave_count: i32,
    pub b_open_time: [u8; 32],
    pub open_time_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct StageMonsterList {
    #[serde(with = "serde_arrays")]
    pub rs_monster: [StageMonsterEntry; STAGE_MONSTER_LIST_MAX],
    pub pecetage_count: i32,
    pub counter: i32,
    pub limit_max: i32,
    pub open_interval: i32,
    pub open_limit: i32,
    pub dw_interval_time: u32,
    pub s_boss_monsters: [StageBossMonsterEntry; STAGE_BOSS_MONSTER_MAX],
    pub boss_monster_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct StageAreaWarmupSummary {
    pub head: u32,
    pub lp_stage_addr: u32,
    pub lp_field_addr: u32,
    #[socket(string_size = 64)]
    pub sz_start_point_file: String,
    #[socket(string_size = 64)]
    pub sz_char_fixed_file: String,
    #[socket(string_size = 64)]
    pub sz_char_monster_file: String,
    pub start_point_cnt: i32,
    pub start_point_last_open_count: i32,
    pub start_point_open_count: i32,
    pub start_point_active_count: i32,
    pub stg_char_info_cnt: i32,
    pub item_setting_count: i32,
    pub monster_setting_count: i32,
    pub counter: i32,
    pub field_night_day: u32,
    pub monster_count: i32,
    pub monster_last_point: i32,
    pub evn_monster: i32,
    pub event_dw_monster_time: u32,
    pub event_lp_chr_monster_addr: u32,
    pub event_monster_percentage: i32,
    pub dw_active_mode: u32,
    pub rs_monster_pecetage_count: i32,
    pub rs_monster_counter: i32,
    pub rs_monster_limit_max: i32,
    pub rs_monster_open_interval: i32,
    pub rs_monster_open_limit: i32,
    pub rs_monster_dw_interval_time: u32,
    pub rs_monster_boss_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct StageAreaMemory {
    pub head: u32,
    pub lp_stage_addr: u32,
    pub lp_field_addr: u32,
    #[socket(string_size = 64)]
    pub sz_start_point_file: String,
    #[socket(string_size = 64)]
    pub sz_char_fixed_file: String,
    #[socket(string_size = 64)]
    pub sz_char_monster_file: String,
    #[serde(with = "serde_arrays")]
    pub start_point: [StageStartPoint; STG_START_POINT_MAX],
    #[serde(with = "serde_arrays")]
    pub start_point_near_play: [i32; STG_START_POINT_MAX],
    #[serde(with = "serde_arrays")]
    pub start_point_mon_count: [i32; STG_START_POINT_MAX],
    pub start_point_cnt: i32,
    pub start_point_last_open_count: i32,
    #[serde(with = "serde_arrays")]
    pub dw_start_point_open_time: [u32; STG_START_POINT_MAX],
    pub start_point_open_count: i32,
    pub start_point_active_count: i32,
    #[serde(with = "serde_arrays")]
    pub trans_char_fixed: [TransPlayerInfo; FIX_CHAR_MAX],
    pub rs_monster_list: StageMonsterList,
    pub stg_char_info_cnt: i32,
    pub item_setting_count: i32,
    pub monster_setting_count: i32,
    pub counter: i32,
    pub field_night_day: u32,
    pub monster_count: i32,
    pub monster_last_point: i32,
    pub evn_monster: i32,
    pub event_dw_monster_time: u32,
    pub event_lp_chr_monster_addr: u32,
    pub event_monster_percentage: i32,
    pub dw_active_mode: u32,
}

impl From<StageAreaWarmupSummary> for StageAreaMemory {
    fn from(value: StageAreaWarmupSummary) -> Self {
        let mut stage_area = Self::default();
        stage_area.head = value.head;
        stage_area.lp_stage_addr = value.lp_stage_addr;
        stage_area.lp_field_addr = value.lp_field_addr;
        stage_area.sz_start_point_file = value.sz_start_point_file;
        stage_area.sz_char_fixed_file = value.sz_char_fixed_file;
        stage_area.sz_char_monster_file = value.sz_char_monster_file;
        stage_area.start_point_cnt = value.start_point_cnt;
        stage_area.start_point_last_open_count = value.start_point_last_open_count;
        stage_area.start_point_open_count = value.start_point_open_count;
        stage_area.start_point_active_count = value.start_point_active_count;
        stage_area.stg_char_info_cnt = value.stg_char_info_cnt;
        stage_area.item_setting_count = value.item_setting_count;
        stage_area.monster_setting_count = value.monster_setting_count;
        stage_area.counter = value.counter;
        stage_area.field_night_day = value.field_night_day;
        stage_area.monster_count = value.monster_count;
        stage_area.monster_last_point = value.monster_last_point;
        stage_area.evn_monster = value.evn_monster;
        stage_area.event_dw_monster_time = value.event_dw_monster_time;
        stage_area.event_lp_chr_monster_addr = value.event_lp_chr_monster_addr;
        stage_area.event_monster_percentage = value.event_monster_percentage;
        stage_area.dw_active_mode = value.dw_active_mode;
        stage_area.rs_monster_list.pecetage_count = value.rs_monster_pecetage_count;
        stage_area.rs_monster_list.counter = value.rs_monster_counter;
        stage_area.rs_monster_list.limit_max = value.rs_monster_limit_max;
        stage_area.rs_monster_list.open_interval = value.rs_monster_open_interval;
        stage_area.rs_monster_list.open_limit = value.rs_monster_open_limit;
        stage_area.rs_monster_list.dw_interval_time = value.rs_monster_dw_interval_time;
        stage_area.rs_monster_list.boss_monster_count = value.rs_monster_boss_count;
        stage_area
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct StageAreaNpcChunk {
    pub stage_area_index: i32,
    pub total_stage_areas: i32,
    pub offset: i32,
    pub count: i32,
    #[serde(with = "serde_arrays")]
    pub entries: [TransPlayerInfo; STAGE_AREA_NPC_CHUNK_MAX],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct StageAreaStartPointChunk {
    pub stage_area_index: i32,
    pub total_stage_areas: i32,
    pub offset: i32,
    pub count: i32,
    #[serde(with = "serde_arrays")]
    pub points: [StageStartPoint; STAGE_AREA_START_POINT_CHUNK_MAX],
    #[serde(with = "serde_arrays")]
    pub near_play: [i32; STAGE_AREA_START_POINT_CHUNK_MAX],
    #[serde(with = "serde_arrays")]
    pub monster_count: [i32; STAGE_AREA_START_POINT_CHUNK_MAX],
    #[serde(with = "serde_arrays")]
    pub open_time: [u32; STAGE_AREA_START_POINT_CHUNK_MAX],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct StageAreaMonsterRuleChunk {
    pub stage_area_index: i32,
    pub total_stage_areas: i32,
    pub offset: i32,
    pub count: i32,
    #[serde(with = "serde_arrays")]
    pub entries: [StageMonsterEntry; STAGE_AREA_MONSTER_RULE_CHUNK_MAX],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct StageAreaBossRuleChunk {
    pub stage_area_index: i32,
    pub total_stage_areas: i32,
    pub offset: i32,
    pub count: i32,
    #[serde(with = "serde_arrays")]
    pub entries: [StageBossMonsterEntry; STAGE_AREA_BOSS_RULE_CHUNK_MAX],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct GameServerLoadedState {
    pub field: FieldMemory,
    pub stage_area: StageAreaMemory,
    pub default_item_template: DefItemInfo,
    pub monster_template: MonsterMemory,
}

impl Default for StageMonsterList {
    fn default() -> Self {
        Self {
            rs_monster: std::array::from_fn(|_| StageMonsterEntry::default()),
            pecetage_count: 0,
            counter: 0,
            limit_max: 0,
            open_interval: 0,
            open_limit: 0,
            dw_interval_time: 0,
            s_boss_monsters: std::array::from_fn(|_| StageBossMonsterEntry::default()),
            boss_monster_count: 0,
        }
    }
}

impl Default for StageAreaMemory {
    fn default() -> Self {
        Self {
            head: 0,
            lp_stage_addr: 0,
            lp_field_addr: 0,
            sz_start_point_file: String::new(),
            sz_char_fixed_file: String::new(),
            sz_char_monster_file: String::new(),
            start_point: std::array::from_fn(|_| StageStartPoint::default()),
            start_point_near_play: [0; STG_START_POINT_MAX],
            start_point_mon_count: [0; STG_START_POINT_MAX],
            start_point_cnt: 0,
            start_point_last_open_count: 0,
            dw_start_point_open_time: [0; STG_START_POINT_MAX],
            start_point_open_count: 0,
            start_point_active_count: 0,
            trans_char_fixed: std::array::from_fn(|_| TransPlayerInfo::default()),
            rs_monster_list: StageMonsterList::default(),
            stg_char_info_cnt: 0,
            item_setting_count: 0,
            monster_setting_count: 0,
            counter: 0,
            field_night_day: 0,
            monster_count: 0,
            monster_last_point: 0,
            evn_monster: 0,
            event_dw_monster_time: 0,
            event_lp_chr_monster_addr: 0,
            event_monster_percentage: 0,
            dw_active_mode: 0,
        }
    }
}
