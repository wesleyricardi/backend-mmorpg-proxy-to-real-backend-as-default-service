use super::char::Char;
use socket::socket;

// size of RecordItem repr(C) struct, used for decompression
const RECORD_ITEM_SIZE: usize = 556;
const RECORD_ITEM_MAX: usize = 200;

#[socket]
#[derive(Debug, Clone)]
pub struct FunctionMemory {
    pub param: [i32; 8],
    pub data: [u8; 6000],
}

#[derive(Debug, Clone)]
#[socket]
pub struct FuncValue {
    pub func: u32,
    pub len: u32,
    pub checksum: u32,
}

#[socket]
#[derive(Debug, Clone)]
pub struct ClientFuncPos {
    pub client_version: i32,
    pub check_mem_sum: u32,
    pub func_count: i32,
    pub func_value: [FuncValue; 64],
}

#[socket]
#[derive(Debug, Clone)]
pub struct ItemPremiumRemainingTime {
    pub third_eye: u32,
    pub exp_up: u32,
    pub vamp_cuspid: u32,
    pub mana_recharg: u32,
    pub mightof_awell: u32,
    pub mightof_awell2: u32,
    pub mana_reduce: u32,
    pub phenix_pet: u32,
    pub help_pet: u32,
    pub vamp_cuspid_ex: u32,
    pub stamina_reduce: u32,
    pub big_head: u32,
}

#[derive(Debug, Clone)]
#[socket]
pub struct RecordData {
    pub size: i32,
    pub code: i32,
    #[socket(string_size = 8)]
    pub header: String,
    pub char_info: Char,
    pub game_save: GameSave,
    pub throw_item: [ThrowItem; 64],
    pub throw_item_count: i32,
    pub item_count: i32,
    pub item_sub_start: i32,
    pub data_size: i32,
    pub record_item_compress: [u8; 31188],
}

#[socket]
#[derive(Debug, Clone)]
pub struct RecordDatas {
    pub count: i32,
    pub total: i32,
    pub data_size: i32,
    pub data: RecordData,
}

#[derive(Debug, Clone)]
#[socket]
pub struct SkillInfo {
    // TODO: Corrigir protocolo aqui é um array de u8 com 20 posicao,
    // representando level de cada skill, comecando da primeira skill ate a ultima.
    // 0..19 sao index da skill da classs.
    pub b_skill_point: [u8; 20],
    pub w_skill_mastery: [u16; 20],
    #[socket(string_size = 20)]
    pub b_short_key: String,
    #[socket(string_size = 20)]
    pub mouse_pos: String,
    pub w_select_skill: [u16; 2],
}

#[derive(Debug, Clone)]
#[socket]
pub struct QuestInfo {
    pub quest_code: [u16; 2],
    pub data: [u32; 7],
}

#[derive(Debug, Clone)]
#[socket]
pub struct LastQuestInfo {
    pub last_quest: [u16; 32],
    pub last_quest_count: i32,
}

#[derive(Debug, Clone)]
#[socket]
pub struct ThrowItem {
    pub code: u32,
    pub key: u32,
    pub sum: u32,
}

#[derive(Debug, Clone)]
#[socket]
pub struct GameSave {
    pub head: u32,
    pub play_stage_num: i32,
    pub camera_mode: i32,
    pub pos_x: i32,
    pub pos_z: i32,
    pub last_money: i32,
    pub check_sum_char_info: u32,
    pub skill_info: SkillInfo,
    pub save_time: i32,
    pub pcr_no: i32,
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
    #[socket(string_size = 32)]
    pub master_id: String,
    pub quest_info: QuestInfo,
    pub last_quest_info: LastQuestInfo,
    pub server_id: u32,
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
    pub checksum: i32,
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
}

socket::protocol_module! {
    DamageEncodeMem => FunctionMemory = 0x50322f00,
    ClientFuncPos => ClientFuncPos = 0x50320400,
    ItemPremiumRemainingTime => ItemPremiumRemainingTime = 0x50320130,
    ClientRecordData => RecordDatas = 0x48470081,
}
