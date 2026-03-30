use crate::protocol::item::Item;
use serde::{Deserialize, Serialize};
use socket::socket;

pub const SPECIAL_JOB_RANDOM_MAX: usize = 12;
pub const ITEM_INFO_RUNTIME_SIZE: usize =
    core::mem::size_of::<<Item as socket::SocketPacket>::CRepr>();

#[derive(Debug, Clone, Serialize, Deserialize)]
#[socket]
pub struct DefItemInfo {
    pub item: Item,
    pub s_durability: [i16; 2],
    pub s_resistance: [[i16; 2]; 8],
    pub s_damage: [i16; 4],
    pub s_attack_rating: [i16; 2],
    pub s_defence: [i16; 2],
    pub f_block_rating: [f32; 2],
    pub f_absorb: [f32; 2],
    pub f_speed: [f32; 2],
    pub increase_life: [i32; 2],
    pub increase_mana: [i32; 2],
    pub increase_stamina: [i32; 2],
    pub f_mana_regen: [f32; 2],
    pub f_life_regen: [f32; 2],
    pub f_stamina_regen: [f32; 2],
    pub s_special_defence: [i16; 2],
    pub f_special_absorb: [f32; 2],
    pub f_special_f_speed: [f32; 2],
    pub f_special_magic_mastery: [f32; 2],
    pub f_special_mana_regen: [f32; 2],
    pub lev_attack_rating: [i32; 2],
    pub dw_job_bit_code_random: [u32; SPECIAL_JOB_RANDOM_MAX],
    pub job_bit_code_random_count: i32,
    pub s_gen_day: [i16; 2],
    #[serde(with = "serde_arrays")]
    pub def_compress_data: [u32; ITEM_INFO_RUNTIME_SIZE],
    pub def_compress_data_len: i32,
}
