use std::time::Instant;

use tokio::sync::oneshot;

use super::game_server::dto::{
    GameServerData, RuntimeMonsterId, RuntimeMonsterState, RuntimeNpcId, RuntimeNpcState,
};
use crate::domain::field::dto::FieldId;
use crate::domain::skill::dtos::codes::SkillCode;
use crate::domain::user_player::char_motion_state::CharMotionState;
use crate::domain::user_player::dto::UserPlayerData;
use crate::domain::user_player::state::{UserPlayerId, UserPlayerState};

#[derive(Debug, Clone, Copy)]
pub enum DisconnectReason {
    Unknown,
    ClientDisconnected,
}

#[derive(Debug)]
pub enum RecordDataBootstrapQueryResult {
    Missing,
    Pending,
    Superseded,
    Ready {
        player_data: UserPlayerData,
        wire_snapshot: RecordDataWireSnapshot,
        response_context: RecordDataResponseContext,
    },
    Failed {
        object_serial: u32,
        login_safe_key: i32,
    },
}

#[derive(Debug, Clone, Default)]
pub struct RecordDataWireSnapshot {
    pub char_info: RecordDataWireCharSnapshot,
    pub items: Vec<RecordDataWireItemSnapshot>,
}

#[derive(Debug, Clone, Default)]
pub struct RecordDataWireCharSnapshot {
    pub unused_life: [i16; 2],
}

#[derive(Debug, Clone, Default)]
pub struct RecordDataWireItemSnapshot {
    pub item_name_bytes: [u8; 32],
}

#[derive(Debug, Clone, Default)]
pub struct RecordDataResponseContext {
    pub packet_header: String,
    pub char_name: String,
    pub char_model_name: String,
    pub char_model_name2: String,
    pub char_model_name_code2: u32,
    pub game_save_head: u32,
    pub object_serial: u32,
    pub login_safe_key: i32,
    pub accuracy: i32,
    pub char_checksum: u32,
    pub exp: i32,
    pub next_exp: i32,
    pub money: i32,
    pub lp_mon_info_addr: i32,
    pub brood: u32,
    pub update_info: String,
    pub arrow_posi: [i16; 2],
    pub class_clan: i32,
    pub size_level: i32,
    pub char_sound_code: u32,
    pub exp_high: i32,
    pub event_time_t: u32,
    pub event_param: [i16; 2],
    pub present_item: [i16; 2],
    pub gravity_scroll_check: [i16; 2],
    pub player_killing: [u16; 2],
    pub play_class: [u16; 2],
    pub char_temp: [u32; 11],
    pub login_server_ip: u32,
    pub char_version: [u16; 2],
    pub pcr_no: i32,
    pub camera_mode: i32,
    pub pos_x: i32,
    pub pos_z: i32,
    pub last_money: i32,
    pub event_play: [i16; 2],
    pub pet_info: [i16; 2],
    pub potion_update: [i16; 2],
    pub potion_count: [[i16; 3]; 4],
    pub force_orb_using: [u16; 2],
    pub level_quest_log: u32,
    pub short_key_normal_attack: i32,
    pub bless_castle_tax: i32,
    pub bless_castle_master: u32,
    pub elementary_quest_log: u32,
    pub server_id: u32,
    pub save_time: i32,
    pub time_prime_item_x2: u32,
    pub time_prime_item_exp_up: u32,
    pub time_prime_item_vamp_cuspid: u32,
    pub time_prime_item_mana_recharg: u32,
    pub prime_item_package_code: u32,
    pub time_prime_item_mightof_awell: u32,
    pub time_prime_item_mightof_awell2: u32,
    pub time_prime_item_mana_reduce: u32,
    pub time_prime_item_phenix_pet: u32,
    pub time_prime_item_help_pet: u32,
    pub param_prime_item_help_pet: i32,
    pub time_prime_item_vamp_cuspid_ex: u32,
    pub time_prime_item_stamina_reduce: u32,
    pub total_sub_exp: i32,
    pub total_money: i32,
    pub total_exp: i32,
    pub master_id: String,
    pub quest_info: crate::domain::user_player::dto::UserQuestInfo,
    pub last_quest_info: crate::domain::user_player::dto::UserLastQuestInfo,
    pub life_booster_using: [u16; 2],
    pub mana_booster_using: [u16; 2],
    pub stamina_booster_using: [u16; 2],
    pub skill_delay_using: [u16; 2],
    pub time_prime_item_big_head: u32,
    pub param_prime_item_big_head: u32,
    pub time_prime_item_vip: u32,
    pub param_prime_item_vip: u32,
    pub quest_start_state: u32,
    pub quest_start_param1: u32,
    pub quest_start_param2: u32,
    pub quest_lv150a_state: u32,
    pub quest_lv150a_param: u32,
    pub quest_lv150a_time: u32,
    pub quest_lv150b_state: u32,
    pub quest_lv150b_param: u32,
    pub quest_lv150b_time: u32,
    pub pv_p_time: u32,
    pub pv_p_param: i32,
    pub weapon_class: i32,
    pub armor_class: i32,
    pub shield_class: i32,
    pub bracelets_class: i32,
    pub gauntlets_class: i32,
    pub boots_class: i32,
    pub ring_class: i32,
    pub amy_class: i32,
    pub shel_class: i32,
    pub time_class: u32,
    pub warehouse_page_time: u32,
    pub quest_lv155a_state: u32,
    pub quest_lv155a_param: u32,
    pub quest_lv155a_time: u32,
    pub random_monster_param: u32,
    pub random_monster_time: u32,
    pub temp2: [u32; 2],
    pub game_save_checksum: i32,
}

#[derive(Debug)]
pub enum WorldCommand {
    AddPlayer {
        player_id: UserPlayerId,
        data: UserPlayerState,
    },
    RemovePlayer {
        player_id: UserPlayerId,
        reason: DisconnectReason,
    },
    ProcessSkill {
        player_id: UserPlayerId,
        skill_code: SkillCode,
        from_party: bool,
    },
    ProcessTargetBuff {
        cast: TargetBuffCast,
    },
    ProcessPartySkill {
        cast: PartySkillCast,
    },
    CancelSkill {
        player_id: UserPlayerId,
        skill_code: SkillCode,
    },
    UpsertPlayerField {
        player_id: UserPlayerId,
        field_id: FieldId,
        observed_at: Instant,
    },
    UpsertPlayerPosition {
        player_id: UserPlayerId,
        x: i32,
        y: i32,
        z: i32,
        state: CharMotionState,
        observed_at: Instant,
    },
    SyncCharMotionState {
        player_id: UserPlayerId,
        state: CharMotionState,
        observed_at: Instant,
    },
    LoadGameServerWarmup {
        data: GameServerData,
        observed_at: Instant,
    },
    ResetRuntimeScope {
        field_id: Option<FieldId>,
        include_monsters: bool,
        include_npcs: bool,
        observed_at: Instant,
    },
    UpsertRuntimeMonster {
        state: RuntimeMonsterState,
        observed_at: Instant,
    },
    RemoveRuntimeMonster {
        runtime_monster_id: RuntimeMonsterId,
        observed_at: Instant,
    },
    UpsertRuntimeNpc {
        state: RuntimeNpcState,
        observed_at: Instant,
    },
    RemoveRuntimeNpc {
        runtime_npc_id: RuntimeNpcId,
        observed_at: Instant,
    },
    RuntimeSnapshotCompleted {
        request_id: u32,
        sequence: u64,
        field_id: Option<FieldId>,
        observed_at: Instant,
    },
    BeginRecordDataBootstrap {
        connection_id: usize,
        request_id: u64,
        player_name: String,
        mode: i32,
        observed_at: Instant,
    },
    StoreBootstrappedRecordData {
        connection_id: usize,
        object_serial: u32,
        player_data: UserPlayerData,
        wire_snapshot: RecordDataWireSnapshot,
        response_context: RecordDataResponseContext,
        observed_at: Instant,
    },
    StoreBootstrappedFailRecordData {
        connection_id: usize,
        object_serial: u32,
        login_safe_key: i32,
        observed_at: Instant,
    },
    ClearRecordDataBootstrap {
        connection_id: usize,
        observed_at: Instant,
    },
    QueryRecordDataBootstrap {
        connection_id: usize,
        request_id: u64,
        reply: oneshot::Sender<RecordDataBootstrapQueryResult>,
    },
}

#[derive(Debug, Clone)]
pub struct PartySkillCast {
    pub caster_id: UserPlayerId,
    pub skill_code: SkillCode,
    pub point: i32,
    pub context: PartySkillContext,
    pub targets: Vec<UserPlayerId>,
}

#[derive(Debug, Clone)]
pub struct TargetBuffCast {
    pub caster_id: UserPlayerId,
    pub skill_code: SkillCode,
    pub point: i32,
    pub context: TargetBuffContext,
    pub target_id: UserPlayerId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartySkillContext {
    StandardBuff,
    HallOfValhalla { triumph_of_valhalla_point: i32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetBuffContext {
    StandardBuff,
}
