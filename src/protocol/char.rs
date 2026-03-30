use serde::{Deserialize, Serialize};
use socket::socket;
use socket::WireString;

use crate::protocol::job_code::RuntimeJobCode;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[socket]
pub struct SelectionChar {
    #[socket(string_size = 32)]
    pub name: String,
    #[socket(string_size = 64)]
    pub model_name_head: String,
    #[socket(string_size = 64)]
    pub model_name_body: String,
    pub job_code: RuntimeJobCode,
    pub level: i32,
    pub brood: u32,
    pub dw_armor_code: u32,
    pub start_field: i32,
    pub pos_x: i32,
    pub pos_z: i32,
    pub temp: [u32; 13],
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[socket]
pub struct Char {
    #[socket(string_size = 32)]
    pub sz_name: String,
    #[socket(string_size = 64)]
    pub sz_model_name: String,
    pub sz_model_name2: WireString<60>,
    pub model_name_code2: u32,
    pub dw_object_serial: u32,
    pub class_clan: i32,
    pub state: i32,
    pub size_level: i32,
    pub dw_char_sound_code: u32,
    pub job_code: RuntimeJobCode,
    pub level: i32,
    pub strength: i32,
    pub spirit: i32,
    pub talent: i32,
    pub dexterity: i32,
    pub health: i32,
    pub accuracy: i32,
    pub attack_rating: i32,
    pub attack_damage: [i32; 2],
    pub attack_speed: i32,
    pub shooting_range: i32,
    pub critical_hit: i32,
    pub defence: i32,
    pub chance_block: i32,
    pub absorption: i32,
    pub move_speed: i32,
    pub sight: i32,
    pub weight: [i16; 2],
    pub resistance: [i16; 8],
    pub attack_resistance: [i16; 8],
    pub unused_life: [i16; 2],
    pub mana: [i16; 2],
    pub stamina: [i16; 2],
    pub life_regen: f32,
    pub mana_regen: f32,
    pub stamina_regen: f32,
    pub exp: i32,
    pub next_exp: i32,
    pub money: i32,
    pub lp_mon_info_addr: i32,
    pub brood: u32,
    pub state_point: i32,
    #[socket(string_size = 4)]
    pub b_update_info: String,
    pub arrow_posi: [i16; 2],
    pub potion_space: i32,
    pub life_function: i32,
    pub mana_function: i32,
    pub stamina_function: i32,
    pub damage_function: [i16; 2],
    pub reform_code: u32,
    pub change_job: u32,
    pub job_bit_mask: u32,
    pub player_killing: [u16; 2],
    pub play_class: [u16; 2],
    pub exp_high: i32,
    pub dw_event_time_t: u32,
    pub s_event_param: [i16; 2],
    pub s_present_item: [i16; 2],
    pub gravity_scroll_check: [i16; 2],
    pub life: [i32; 2],
    pub dw_temp: [u32; 9],
    pub dw_login_server_ip: u32,
    pub dw_login_server_safe_key: u32,
    pub w_version: [u16; 2],
}
