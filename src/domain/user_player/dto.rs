use serde::{Deserialize, Serialize};

use crate::domain::item::dto::{EquippableItemType, PlayerJobMask};
use crate::domain::skill::dtos::codes::SkillCode;
use crate::domain::user_player::char_motion_state::CharMotionState;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserPlayerData {
    pub char_info: UserChar,
    pub game_save: UserGameSave,
    #[serde(with = "serde_arrays")]
    pub throw_item: [UserThrowItem; 64],
    pub throw_item_count: i32,
    pub item_count: i32,
    pub item_sub_start: i32,
    pub record_items: Vec<UserRecordItem>,
}

impl Default for UserPlayerData {
    fn default() -> Self {
        Self {
            char_info: UserChar::default(),
            game_save: UserGameSave::default(),
            throw_item: std::array::from_fn(|_| UserThrowItem::default()),
            throw_item_count: 0,
            item_count: 0,
            item_sub_start: 0,
            record_items: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UserChar {
    pub state: CharMotionState,
    pub job_code: PlayerJob,
    pub level: i32,
    pub strength: i32,
    pub spirit: i32,
    pub talent: i32,
    pub dexterity: i32,
    pub health: i32,
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
    pub life: [i16; 2],
    pub mana: [i16; 2],
    pub stamina: [i16; 2],
    pub life_regen: f32,
    pub mana_regen: f32,
    pub stamina_regen: f32,
    pub state_point: i32,
    pub potion_space: i32,
    pub life_function: i32,
    pub mana_function: i32,
    pub stamina_function: i32,
    pub damage_function: [i16; 2],
    pub change_job: ChangeJobTierState,
    pub job_bit_mask: PlayerJobMask,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UserSkillInfo {
    //Aqui é um array de u8 com 20 posicao,
    //representando ponto (up) em cada skill, comecando da 1ª até a 20ª skills
    pub b_skill_point: [u8; 20],
    //maestria em cada skill, seguindo a mesma ordem do array de cima, onde cada u16 representa um bitmask de maestria
    pub w_skill_mastery: [u16; 20],
    pub b_short_key: String,
    pub mouse_pos: String,
    //representa o código da skill selecionada em cada mão, onde 0 é nenhuma skill selecionada, seguindo a ordem das skills do jogo
    pub w_select_skill: [u16; 2],
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UserQuestInfo {
    pub quest_code: [u16; 2],
    pub data: [u32; 7],
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UserLastQuestInfo {
    pub last_quest: [u16; 32],
    pub last_quest_count: i32,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UserThrowItem {
    pub code: u32,
    pub key: u32,
    pub sum: u32,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UserGameSave {
    pub play_stage_num: i32,
    pub skill_info: UserSkillInfo,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UserRecordItem {
    pub count: i32,
    pub x: i32,
    pub y: i32,
    pub position: i32,
    pub item: UserItem,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UserItemHeader {
    pub head: u32,
    pub version: u32,
    pub time: u32,
    pub checksum: u32,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UserJobItem {
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

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UserSwapItem {
    pub flag: u32,
    pub code: u32,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UserItem {
    pub checksum: u32,
    pub size: u32,
    pub item_header: UserItemHeader,
    pub durability: [i16; 2],
    pub code: u32,
    pub item_name: String,
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
    pub job_code_mask: PlayerJobMask,
    pub job_item: UserJobItem,
    pub item_kind_code: u32,
    pub item_kind_mask: u32,
    pub item_aging_num: [i16; 2],
    pub item_aging_count: [i16; 2],
    pub item_aging_protect: [i16; 2],
    pub special_item_flag: [i16; 2],
    pub swap_item: UserSwapItem,
    pub dw_create_time: u32,
    pub linked_item: i32,
    pub lock_item: i32,
    pub coin: i32,
    pub dw_temp: [u32; 6],
}

pub type DamageRange = [f32; 2];

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CalculatedCharStats {
    pub level: f32,
    pub strength: f32,
    pub spirit: f32,
    pub talent: f32,
    pub dexterity: f32,
    pub health: f32,
    pub attack_rating: f32,
    pub attack_damage: DamageRange,
    pub attack_speed: f32,
    pub shooting_range: f32,
    pub critical_hit: f32,
    pub evade: f32,
    pub defence: f32,
    pub chance_block: f32,
    pub absorption: f32,
    pub move_speed: f32,
    pub sight: f32,
    pub weight: [f32; 2],
    pub resistance: [f32; 8],
    pub life: [f32; 2],
    pub mana: [f32; 2],
    pub stamina: [f32; 2],
    pub life_regen: f32,
    pub mana_regen: f32,
    pub stamina_regen: f32,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TotalItemJobStats {
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
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TotalItemStats {
    pub attack_rating: i32,
    pub critical_hit: i32,
    pub f_absorb: f32,
    pub defence: i32,
    pub f_block_rating: f32,
    pub f_speed: f32,
    pub sight: i32,
    pub shooting_range: i32,
    pub f_mana_regen: f32,
    pub f_life_regen: f32,
    pub f_stamina_regen: f32,
    pub f_increase_life: f32,
    pub f_increase_mana: f32,
    pub f_increase_stamina: f32,
    pub f_magic_mastery: f32,
    pub attack_speed: i32,
    pub damage: [i16; 2],
    pub mana: [i16; 2],
    pub life: [i16; 2],
    pub stamina: [i16; 2],
    pub resistance: [i16; 8],
    pub weight: i32,
    pub potion_space: i32,
    pub job_item: TotalItemJobStats,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ComboBonusStats {
    pub add_hp: i32,
    pub add_mp: i32,
    pub add_res: i32,
    pub regen_hp: f32,
    pub regen_mp: f32,
    pub regen_res: f32,
    pub defense: i32,
    pub absorption: i32,
    pub critical: i32,
    pub attack_rating: i32,
    pub block: i32,
    pub weight_capacity: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum PlayerJob {
    #[default]
    Fighter,
    Mechanician,
    Archer,
    Pikeman,
    Atalanta,
    Knight,
    Magician,
    Priestess,
    Assassine,
    Shaman,
    Martial,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidPlayerJob(pub u32);

impl PlayerJob {
    pub const fn job_mask(self) -> PlayerJobMask {
        match self {
            Self::Mechanician => PlayerJobMask::MECHANICIAN,
            Self::Fighter => PlayerJobMask::FIGHTER,
            Self::Pikeman => PlayerJobMask::PIKEMAN,
            Self::Archer => PlayerJobMask::ARCHER,
            Self::Knight => PlayerJobMask::KNIGHT,
            Self::Atalanta => PlayerJobMask::ATALANTA,
            Self::Priestess => PlayerJobMask::PRIESTESS,
            Self::Magician => PlayerJobMask::MAGICIAN,
            Self::Assassine => PlayerJobMask::ASSASSIN,
            Self::Shaman => PlayerJobMask::SHAMAN,
            Self::Martial => PlayerJobMask::MARTIAL,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum ChangeJobTierState {
    #[default]
    Tier1,
    Tier2,
    Tier3,
    Tier4,
    Tier5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidChangeJobTier(pub i32);

impl ChangeJobTierState {
    pub const fn as_i32(self) -> i32 {
        match self {
            Self::Tier1 => 1,
            Self::Tier2 => 2,
            Self::Tier3 => 3,
            Self::Tier4 => 4,
            Self::Tier5 => 5,
        }
    }
}

impl TryFrom<i32> for ChangeJobTierState {
    type Error = InvalidChangeJobTier;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Tier1),
            2 => Ok(Self::Tier2),
            3 => Ok(Self::Tier3),
            4 => Ok(Self::Tier4),
            5 => Ok(Self::Tier5),
            other => Err(InvalidChangeJobTier(other)),
        }
    }
}

impl TryFrom<u32> for ChangeJobTierState {
    type Error = InvalidChangeJobTier;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        ChangeJobTierState::try_from(value as i32)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CalcContext {
    pub damage_function: [i32; 3],
    pub mana_function: i32,
    pub check_quest_item_down_flag: bool,
    pub has_level_90_quest_bonus: bool,
    pub change_job_tier: ChangeJobTierState,
    pub is_weapon_shooting: bool,
    pub is_weapon_casting: bool,
    pub life_preset: Option<PlayerJob>,
    pub combo_bonuses: Vec<ComboBonusStats>,
    pub active_buff_codes: Vec<SkillCode>,
    pub passive_skill_codes: Vec<SkillCode>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserContinueSkill {
    pub code: SkillCode,
    pub point: i32,
    pub flag: i32,
    pub party_flag: bool,
    pub element_index: i32,
    pub plus_state: [i32; 2],
    pub use_weapon_codes: [Option<EquippableItemType>; 8],
}

impl Default for UserContinueSkill {
    fn default() -> Self {
        Self {
            code: SkillCode::default(),
            point: 0,
            flag: 1,
            party_flag: false,
            element_index: 0,
            plus_state: [0, 0],
            use_weapon_codes: [None; 8],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UserPassiveSkill {
    pub code: SkillCode,
    pub point: i32,
    pub use_weapon_codes: [Option<EquippableItemType>; 8],
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UserPremiumState {
    pub help_pet_active: bool,
    pub help_pet_damage_percent: i32,
    pub gravity_scroll_weight_bonus: i32,
    pub might_of_awell_active: bool,
    pub might_of_awell_weight_bonus: i32,
    pub life_booster_percent: i32,
    pub mana_booster_percent: i32,
    pub stamina_booster_percent: i32,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UserQuestFlags {
    pub check_quest_item_down_flag: bool,
    pub has_level_90_quest_bonus: bool,
    pub is_change_job_3_completed: bool,
    pub has_quest_weapon_equipped: bool,
    pub level_90_damage_penalty: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UserBattleModifiers {
    pub ranking_tier: Option<u8>,
    pub relics_active: [bool; 12],
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UserEnvModifiers {
    pub view_damage_percent: i32,
    pub view_damage_flat: i32,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UserCalcInput {
    pub base_player: UserChar,
    pub equipped_items: Vec<UserRecordItem>,
    pub active_continue_skills: Vec<UserContinueSkill>,
    pub passive_skills: Vec<UserPassiveSkill>,
    pub premium_state: UserPremiumState,
    pub quest_flags: UserQuestFlags,
    pub battle_modifiers: UserBattleModifiers,
    pub env_modifiers: UserEnvModifiers,
    pub combo_bonuses: Vec<ComboBonusStats>,
    pub damage_function: [i32; 3],
    pub mana_function: i32,
    pub life_preset: Option<PlayerJob>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UserCalcOutput {
    pub updated_char_info: UserChar,
    pub derived_stats: CalculatedCharStats,
    pub debug_breakdown: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::{PlayerJob, PlayerJobMask};

    #[test]
    fn job_mask_mapping_is_stable() {
        assert_eq!(
            PlayerJob::Mechanician.job_mask(),
            PlayerJobMask::MECHANICIAN
        );
        assert_eq!(PlayerJob::Fighter.job_mask(), PlayerJobMask::FIGHTER);
        assert_eq!(PlayerJob::Pikeman.job_mask(), PlayerJobMask::PIKEMAN);
        assert_eq!(PlayerJob::Archer.job_mask(), PlayerJobMask::ARCHER);
        assert_eq!(PlayerJob::Knight.job_mask(), PlayerJobMask::KNIGHT);
        assert_eq!(PlayerJob::Atalanta.job_mask(), PlayerJobMask::ATALANTA);
        assert_eq!(PlayerJob::Priestess.job_mask(), PlayerJobMask::PRIESTESS);
        assert_eq!(PlayerJob::Magician.job_mask(), PlayerJobMask::MAGICIAN);
        assert_eq!(PlayerJob::Assassine.job_mask(), PlayerJobMask::ASSASSIN);
        assert_eq!(PlayerJob::Shaman.job_mask(), PlayerJobMask::SHAMAN);
        assert_eq!(PlayerJob::Martial.job_mask(), PlayerJobMask::MARTIAL);
    }
}
