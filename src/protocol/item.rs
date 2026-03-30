use serde::{Deserialize, Serialize};
use socket::socket;
use socket::WireString;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct Header {
    pub head: u32,
    pub version: u32,
    pub time: u32,
    pub checksum: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct JobItem {
    pub add_f_absorb: f32,
    pub add_defense: i32,
    pub add_f_speed: f32,
    pub add_f_block_rating: f32,
    pub add_attack_speed: i32,
    pub add_critical_hit: i32,
    pub add_shooting_range: i32,
    pub add_f_magic_mastery: f32,
    pub add_resistance: [i16; 8],
    pub lev_attack_resistance: [i16; 8],
    pub lev_mana: i32,
    pub lev_life: i32,
    pub lev_attack_rating: i32,
    pub lev_damage: [i16; 2],
    pub per_mana_regen: f32,
    pub per_life_regen: f32,
    pub per_stamina_regen: f32,
    pub temp: [u32; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct RecordItem {
    pub count: i32,
    pub x: i32,
    pub y: i32,
    pub position: i32,
    pub item: Item,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct SwapItem {
    pub flag: u32,
    pub code: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct Item {
    pub size: u32,
    pub item_header: Header,
    pub durability: [i16; 2],
    pub code: u32,
    pub item_name: WireString<32>,
    pub weight: i32,
    pub price: i32,
    pub index: i32,
    pub potion_count: i32,
    pub resistance: [i16; 8],
    pub sight: i32,
    pub temp0: u32,
    pub damage: [i16; 2],
    pub shooting_range: i32,
    pub attack_speed: i32,
    pub attack_rating: i32,
    pub critical_hit: i32,
    pub f_absorb: f32,
    pub defense: i32,
    pub f_block_rating: f32,
    pub f_speed: f32,
    pub potion_space: i32,
    pub f_magic_mastery: f32,
    pub f_mana_regen: f32,
    pub f_life_regen: f32,
    pub f_stamina_regen: f32,
    pub f_increase_life: f32,
    pub f_increase_mana: f32,
    pub f_increase_stamina: f32,
    pub level: i32,
    pub strength: i32,
    pub spirit: i32,
    pub talent: i32,
    pub dexterity: i32,
    pub health: i32,
    pub mana: [i16; 2],
    pub life: [i16; 2],
    pub stamina: [i16; 2],
    pub money: i32,
    pub not_use_flag: i32,
    pub back_up_key: u32,
    pub back_up_chk_sum: u32,
    pub scale_blink: [i16; 2],
    pub unique_item: u32,
    pub effect_blink: [i16; 2],
    pub effect_color: [i16; 4],
    pub disp_effect: u32,
    pub job_code_mask: u32,
    pub job_item: JobItem,
    pub item_kind_code: u32,
    pub item_kind_mask: u32,
    pub item_aging_num: [i16; 2],
    pub item_aging_count: [i16; 2],
    pub item_aging_protect: [i16; 2],
    pub special_item_flag: [i16; 2],
    pub swap_item: SwapItem,
    pub dw_create_time: u32,
    pub linked_item: i32,
    pub lock_item: i32,
    pub coin: i32,
    pub dw_temp: [u32; 6],
}

#[socket]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemInfo {
    pub item: Item,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub code: [u32; 4],
}

socket::protocol_module! {
    DelItem => ItemInfo = 0x48470051,
    PullItem => ItemInfo = 0x8C000C8Bu32,
    ThrowItem => ItemInfo = 0x8A000B4Au32,
}
