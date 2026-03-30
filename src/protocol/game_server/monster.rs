use crate::protocol::char::Char;
use serde::{Deserialize, Serialize};
use socket::socket;

pub const FALLITEM_MAX: usize = 200;
pub const FALLITEM2_MAX: usize = 3;
pub const NPC_MESSAGE_MAX: usize = 20;
pub const NPC_SELL_ITEM_MAX: usize = 32;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct FallItem {
    pub dw_item_code: u32,
    pub percentage: i32,
    pub s_price: [i16; 2],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct MonsterInfo {
    #[socket(string_size = 32)]
    pub sz_name: String,
    pub skill_damage: [i16; 2],
    pub skill_range: i32,
    pub skill_rating: i32,
    pub skill_distance: i32,
    pub skill_range_rect: Rect,
    pub skill_curse: i32,
    pub attack_pattern: i32,
    pub active_hour: i32,
    pub iq: i32,
    pub nature: i32,
    pub undead: i32,
    pub generate_group: [i32; 2],
    pub get_exp: i64,
    pub sp_attack_percentage: i32,
    pub move_range: i32,
    pub damage_stun_pers: i32,
    pub potion_count: i32,
    pub potion_percent: i32,
    pub all_see_item: i32,
    pub fall_item_count: i32,
    pub fall_item_per_max: i32,
    pub fall_item_max: i32,
    pub fall_item_plus_count: i32,
    pub class_code: i32,
    pub use_event_model: i32,
    pub real_sight: i32,
    #[serde(with = "serde_arrays")]
    pub fall_items: [FallItem; FALLITEM_MAX],
    #[serde(with = "serde_arrays")]
    pub fall_items_plus: [FallItem; FALLITEM2_MAX],
    pub sell_attack_item_count: i32,
    pub sell_attack_item: [u32; NPC_SELL_ITEM_MAX],
    pub sell_defence_item_count: i32,
    pub sell_defence_item: [u32; NPC_SELL_ITEM_MAX],
    pub sell_etc_item_count: i32,
    pub sell_etc_item: [u32; NPC_SELL_ITEM_MAX],
    pub skill_master: i32,
    pub skill_change_job: i32,
    pub ware_house_master: i32,
    pub item_mix: i32,
    pub item_aging: i32,
    pub collect_money: i32,
    pub event_gift: i32,
    pub smelting: i32,
    pub manufacture: i32,
    pub mixture_reset: i32,
    pub remodel: i32,
    pub coin_shop: i32,
    pub time_shop: i32,
    pub award_item: i32,
    pub roulette: i32,
    pub use_roll_dice: i32,
    pub boss: i32,
    pub event_code: i32,
    pub event_info: i32,
    pub dw_event_item: u32,
    pub event_npc: i32,
    pub clan_npc: i32,
    pub gift_express: i32,
    pub wing_quest_npc: i32,
    pub star_point_npc: i32,
    pub give_money_npc: i32,
    pub tele_port_npc: i32,
    pub bless_castle_npc: i32,
    pub polling_npc: i32,
    pub sz_media_play_npc_title_addr: u32,
    pub sz_media_play_npc_path_addr: u32,
    pub quest_code: i32,
    pub quest_param: i32,
    pub open_count: [i16; 2],
    pub dw_auto_char_code: u32,
    pub lp_npc_message: [u32; NPC_MESSAGE_MAX],
    pub npc_msg_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct MonsterMemory {
    pub char_info: Char,
    pub monster_info: MonsterInfo,
}
