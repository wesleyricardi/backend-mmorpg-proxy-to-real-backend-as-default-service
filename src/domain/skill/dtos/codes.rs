#![allow(dead_code, non_upper_case_globals)]

use core::convert::TryFrom;

use serde::{Deserialize, Serialize};

pub const SIN_MAX_SKILL: usize = 260;
pub const SIN_MAX_USE_SKILL: usize = 21;
pub const SIN_MAX_SKILLBOX: usize = 28;

pub const SKILL_MASK1: u32 = 0xFF00_0000;
pub const SKILL_MASK2: u32 = 0xFFFF_0000;
pub const SKILL_MASK3: u32 = 0x0000_FFFF;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SkillCode(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SkillSlot(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SkillGroupBits(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChangeJobBits(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidSkillCode(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SkillDef {
    pub code: SkillCode,
    pub passive: bool,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SkillGroup {
    Mechanician = GROUP_MECHANICIAN,
    Fighter = GROUP_FIGHTER,
    Pikeman = GROUP_PIKEMAN,
    Archer = GROUP_ARCHER,
    Knight = GROUP_KNIGHT,
    Atalanta = GROUP_ATALANTA,
    Priestess = GROUP_PRIESTESS,
    Magician = GROUP_MAGICIAN,
    Other = GROUP_OTHERSKILL,
    Assassin = GROUP_ASSASSINE,
    Shaman = GROUP_SHAMAN,
    Martial = GROUP_MARTIAL,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChangeJobTier {
    Tier1 = CHANGE_JOB1,
    Tier2 = CHANGE_JOB2,
    Tier3 = CHANGE_JOB3,
    Tier4 = CHANGE_JOB4,
    Tier5 = CHANGE_JOB5,
}

pub const SKILL_ALL_WEAPON_USE: u32 = 0x11000000;
pub const GROUP_MECHANICIAN: u32 = 0x01000000;
pub const GROUP_FIGHTER: u32 = 0x02000000;
pub const GROUP_PIKEMAN: u32 = 0x03000000;
pub const GROUP_ARCHER: u32 = 0x04000000;
pub const GROUP_ASSASSINE: u32 = 0x0A000000;
pub const GROUP_MARTIAL: u32 = 0x0C000000;
pub const GROUP_KNIGHT: u32 = 0x05000000;
pub const GROUP_ATALANTA: u32 = 0x06000000;
pub const GROUP_PRIESTESS: u32 = 0x07000000;
pub const GROUP_MAGICIAN: u32 = 0x08000000;
pub const GROUP_SHAMAN: u32 = 0x0B000000;
pub const GROUP_OTHERSKILL: u32 = 0x09000000;
pub const CHANGE_JOB1: u32 = 0x00010000;
pub const CHANGE_JOB2: u32 = 0x00020000;
pub const CHANGE_JOB3: u32 = 0x00040000;
pub const CHANGE_JOB4: u32 = 0x00080000;
pub const CHANGE_JOB5: u32 = 0x00090000;
pub const SKILL_1: u32 = 0x00000001;
pub const SKILL_2: u32 = 0x00000002;
pub const SKILL_3: u32 = 0x00000003;
pub const SKILL_4: u32 = 0x00000004;
pub const SKILL_5: u32 = 0x00000005;
pub const SKILL_6: u32 = 0x00000006;
pub const SKILL_7: u32 = 0x00000007;
pub const SKILL_8: u32 = 0x00000008;
pub const SKILL_9: u32 = 0x00000009;
pub const SKILL_10: u32 = 0x00000010;
pub const SKILL_11: u32 = 0x00000011;
pub const SKILL_12: u32 = 0x00000012;
pub const SKILL_13: u32 = 0x00000013;
pub const SKILL_14: u32 = 0x00000014;
pub const SKILL_15: u32 = 0x00000015;
pub const SKILL_16: u32 = 0x00000016;
pub const CLANSKILL_ABSORB: u32 = GROUP_OTHERSKILL | CHANGE_JOB1 | SKILL_4;
pub const CLANSKILL_ATTACK: u32 = GROUP_OTHERSKILL | CHANGE_JOB1 | SKILL_5;
pub const CLANSKILL_EVASION: u32 = GROUP_OTHERSKILL | CHANGE_JOB1 | SKILL_6;
pub const BOOSTER_ITEM_LIFE: u32 = GROUP_OTHERSKILL | CHANGE_JOB1 | SKILL_7;
pub const BOOSTER_ITEM_MANA: u32 = GROUP_OTHERSKILL | CHANGE_JOB1 | SKILL_8;
pub const BOOSTER_ITEM_STAMINA: u32 = GROUP_OTHERSKILL | CHANGE_JOB1 | SKILL_9;
pub const SKILLDELAY_ITEM_LIFE: u32 = GROUP_OTHERSKILL | CHANGE_JOB2 | SKILL_1;
pub const SCROLL_INVULNERABILITY: u32 = GROUP_OTHERSKILL | CHANGE_JOB2 | SKILL_1;
pub const SCROLL_CRITICAL: u32 = GROUP_OTHERSKILL | CHANGE_JOB2 | SKILL_2;
pub const SCROLL_EVASION: u32 = GROUP_OTHERSKILL | CHANGE_JOB2 | SKILL_3;
pub const STONE_R_FIRECRYTAL: u32 = GROUP_OTHERSKILL | CHANGE_JOB2 | SKILL_4;
pub const STONE_R_ICECRYTAL: u32 = GROUP_OTHERSKILL | CHANGE_JOB2 | SKILL_5;
pub const STONE_R_LINGHTINGCRYTAL: u32 = GROUP_OTHERSKILL | CHANGE_JOB2 | SKILL_6;
pub const STONE_A_FIGHTER: u32 = GROUP_OTHERSKILL | CHANGE_JOB3 | SKILL_1;
pub const STONE_A_MECHANICIAN: u32 = GROUP_OTHERSKILL | CHANGE_JOB3 | SKILL_2;
pub const STONE_A_PIKEMAN: u32 = GROUP_OTHERSKILL | CHANGE_JOB3 | SKILL_3;
pub const STONE_A_ARCHER: u32 = GROUP_OTHERSKILL | CHANGE_JOB3 | SKILL_4;
pub const STONE_A_KNIGHT: u32 = GROUP_OTHERSKILL | CHANGE_JOB3 | SKILL_5;
pub const STONE_A_ATALANTA: u32 = GROUP_OTHERSKILL | CHANGE_JOB3 | SKILL_6;
pub const STONE_A_MAGICIAN: u32 = GROUP_OTHERSKILL | CHANGE_JOB3 | SKILL_7;
pub const STONE_A_PRIESTESS: u32 = GROUP_OTHERSKILL | CHANGE_JOB3 | SKILL_8;
pub const SCROLL_P_INVULNERABILITY: u32 = GROUP_OTHERSKILL | CHANGE_JOB4 | SKILL_1;
pub const SCROLL_P_CRITICAL: u32 = GROUP_OTHERSKILL | CHANGE_JOB4 | SKILL_2;
pub const SCROLL_P_EVASION: u32 = GROUP_OTHERSKILL | CHANGE_JOB4 | SKILL_3;
pub const CHAT_DELAY: u32 = GROUP_OTHERSKILL | CHANGE_JOB4 | SKILL_4;
pub const WING_EFFECT1: u32 = GROUP_OTHERSKILL | CHANGE_JOB4 | SKILL_5;
pub const WING_EFFECT2: u32 = GROUP_OTHERSKILL | CHANGE_JOB4 | SKILL_6;
pub const WING_EFFECT3: u32 = GROUP_OTHERSKILL | CHANGE_JOB4 | SKILL_7;
pub const QUEST_DIARY: u32 = GROUP_OTHERSKILL | CHANGE_JOB4 | SKILL_8;
pub const SODCROWN_GOLD: u32 = GROUP_OTHERSKILL | CHANGE_JOB4 | SKILL_9;
pub const SODCROWN_SILVER: u32 = GROUP_OTHERSKILL | CHANGE_JOB4 | SKILL_10;
pub const SODCROWN_BRONZE: u32 = GROUP_OTHERSKILL | CHANGE_JOB4 | SKILL_11;
pub const BLESSCROWN_SILVER: u32 = GROUP_OTHERSKILL | CHANGE_JOB4 | SKILL_15;
pub const BLESSCROWN_BRONZE: u32 = GROUP_OTHERSKILL | CHANGE_JOB4 | SKILL_16;

macro_rules! define_skill_registry {
    ($($name:ident => ($expr:expr, $passive:expr),)+) => {
        $(pub const $name: SkillCode = SkillCode($expr);)+

        pub const fn skill_def_by_raw(raw: u32) -> Option<SkillDef> {
            match SkillCode(raw) {
                $($name => Some(SkillDef { code: $name, passive: $passive }),)+
                _ => None,
            }
        }
    };
}

define_skill_registry! {
    SKILL_SWORD_BLAST => (GROUP_KNIGHT | CHANGE_JOB1 | SKILL_1, false),
    SKILL_HOLY_BODY => (GROUP_KNIGHT | CHANGE_JOB1 | SKILL_2, false),
    SKILL_PHYSICAL_TRANING => (GROUP_KNIGHT | CHANGE_JOB1 | SKILL_3, true),
    SKILL_DOUBLE_CRASH => (GROUP_KNIGHT | CHANGE_JOB1 | SKILL_4, false),
    SKILL_HOLY_VALOR => (GROUP_KNIGHT | CHANGE_JOB2 | SKILL_1, false),
    SKILL_BRANDISH => (GROUP_KNIGHT | CHANGE_JOB2 | SKILL_2, false),
    SKILL_PIERCING => (GROUP_KNIGHT | CHANGE_JOB2 | SKILL_3, false),
    SKILL_DRASTIC_SPIRIT => (GROUP_KNIGHT | CHANGE_JOB2 | SKILL_4, false),
    SKILL_SWORD_MASTERY => (GROUP_KNIGHT | CHANGE_JOB3 | SKILL_1, true),
    SKILL_DIVINE_INHALATION => (GROUP_KNIGHT | CHANGE_JOB3 | SKILL_2, false),
    SKILL_HOLY_INCANTATION => (GROUP_KNIGHT | CHANGE_JOB3 | SKILL_3, false),
    SKILL_GRAND_CROSS => (GROUP_KNIGHT | CHANGE_JOB3 | SKILL_4, false),
    SKILL_SWORD_OF_JUSTICE => (GROUP_KNIGHT | CHANGE_JOB4 | SKILL_1, false),
    SKILL_GODLY_SHIELD => (GROUP_KNIGHT | CHANGE_JOB4 | SKILL_2, false),
    SKILL_GOD_BLESS => (GROUP_KNIGHT | CHANGE_JOB4 | SKILL_3, false),
    SKILL_DIVINE_PIERCING => (GROUP_KNIGHT | CHANGE_JOB4 | SKILL_4, false),
    SKILL_S_BREAKER => (GROUP_KNIGHT | CHANGE_JOB5 | SKILL_1, false),
    SKILL_C_MOON => (GROUP_KNIGHT | CHANGE_JOB5 | SKILL_2, false),
    SKILL_S_BLADE => (GROUP_KNIGHT | CHANGE_JOB5 | SKILL_3, false),
    SKILL_H_BENEDIC => (GROUP_KNIGHT | CHANGE_JOB5 | SKILL_4, false),
    SKILL_SHIELD_STRIKE => (GROUP_ATALANTA | CHANGE_JOB1 | SKILL_1, false),
    SKILL_FARINA => (GROUP_ATALANTA | CHANGE_JOB1 | SKILL_2, false),
    SKILL_THROWING_MASTERY => (GROUP_ATALANTA | CHANGE_JOB1 | SKILL_3, true),
    SKILL_VIGOR_SPEAR => (GROUP_ATALANTA | CHANGE_JOB1 | SKILL_4, false),
    SKILL_WINDY => (GROUP_ATALANTA | CHANGE_JOB2 | SKILL_1, false),
    SKILL_TWIST_JAVELIN => (GROUP_ATALANTA | CHANGE_JOB2 | SKILL_2, false),
    SKILL_SOUL_SUCKER => (GROUP_ATALANTA | CHANGE_JOB2 | SKILL_3, false),
    SKILL_FIRE_JAVELIN => (GROUP_ATALANTA | CHANGE_JOB2 | SKILL_4, false),
    SKILL_SPLIT_JAVELIN => (GROUP_ATALANTA | CHANGE_JOB3 | SKILL_1, false),
    SKILL_TRIUMPH_OF_VALHALLA => (GROUP_ATALANTA | CHANGE_JOB3 | SKILL_2, false),
    SKILL_LIGHTNING_JAVELIN => (GROUP_ATALANTA | CHANGE_JOB3 | SKILL_3, false),
    SKILL_STORM_JAVELIN => (GROUP_ATALANTA | CHANGE_JOB3 | SKILL_4, false),
    SKILL_HALL_OF_VALHALLA => (GROUP_ATALANTA | CHANGE_JOB4 | SKILL_1, false),
    SKILL_X_RAGE => (GROUP_ATALANTA | CHANGE_JOB4 | SKILL_2, false),
    SKILL_FROST_JAVELIN => (GROUP_ATALANTA | CHANGE_JOB4 | SKILL_3, false),
    SKILL_VENGEANCE => (GROUP_ATALANTA | CHANGE_JOB4 | SKILL_4, false),
    SKILL_TALARIA => (GROUP_ATALANTA | CHANGE_JOB5 | SKILL_1, false),
    SKILL_G_COUP => (GROUP_ATALANTA | CHANGE_JOB5 | SKILL_2, false),
    SKILL_ARCUDA => (GROUP_ATALANTA | CHANGE_JOB5 | SKILL_3, false),
    SKILL_S_FEAR => (GROUP_ATALANTA | CHANGE_JOB5 | SKILL_4, false),
    SKILL_HEALING => (GROUP_PRIESTESS | CHANGE_JOB1 | SKILL_1, false),
    SKILL_HOLY_BOLT => (GROUP_PRIESTESS | CHANGE_JOB1 | SKILL_2, false),
    SKILL_MULTISPARK => (GROUP_PRIESTESS | CHANGE_JOB1 | SKILL_3, false),
    SKILL_HOLY_MIND => (GROUP_PRIESTESS | CHANGE_JOB1 | SKILL_4, false),
    SKILL_MEDITATION => (GROUP_PRIESTESS | CHANGE_JOB2 | SKILL_1, true),
    SKILL_DIVINE_LIGHTNING => (GROUP_PRIESTESS | CHANGE_JOB2 | SKILL_2, false),
    SKILL_HOLY_REFLECTION => (GROUP_PRIESTESS | CHANGE_JOB2 | SKILL_3, false),
    SKILL_GRAND_HEALING => (GROUP_PRIESTESS | CHANGE_JOB2 | SKILL_4, false),
    SKILL_VIGOR_BALL => (GROUP_PRIESTESS | CHANGE_JOB3 | SKILL_1, false),
    SKILL_RESURRECTION => (GROUP_PRIESTESS | CHANGE_JOB3 | SKILL_2, false),
    SKILL_EXTINCTION => (GROUP_PRIESTESS | CHANGE_JOB3 | SKILL_3, false),
    SKILL_VIRTUAL_LIFE => (GROUP_PRIESTESS | CHANGE_JOB3 | SKILL_4, false),
    SKILL_GLACIAL_SPIKE => (GROUP_PRIESTESS | CHANGE_JOB4 | SKILL_1, false),
    SKILL_REGENERATION_FIELD => (GROUP_PRIESTESS | CHANGE_JOB4 | SKILL_2, false),
    SKILL_CHAIN_LIGHTNING => (GROUP_PRIESTESS | CHANGE_JOB4 | SKILL_3, false),
    SKILL_SUMMON_MUSPELL => (GROUP_PRIESTESS | CHANGE_JOB4 | SKILL_4, false),
    SKILL_S_IMPACT => (GROUP_PRIESTESS | CHANGE_JOB5 | SKILL_1, false),
    SKILL_P_ICE => (GROUP_PRIESTESS | CHANGE_JOB5 | SKILL_2, false),
    SKILL_RAMIEL => (GROUP_PRIESTESS | CHANGE_JOB5 | SKILL_3, false),
    SKILL_KRISHNA => (GROUP_PRIESTESS | CHANGE_JOB5 | SKILL_4, false),
    SKILL_AGONY => (GROUP_MAGICIAN | CHANGE_JOB1 | SKILL_1, false),
    SKILL_FIRE_BOLT => (GROUP_MAGICIAN | CHANGE_JOB1 | SKILL_2, false),
    SKILL_ZENITH => (GROUP_MAGICIAN | CHANGE_JOB1 | SKILL_3, false),
    SKILL_FIRE_BALL => (GROUP_MAGICIAN | CHANGE_JOB1 | SKILL_4, false),
    SKILL_MENTAL_MASTERY => (GROUP_MAGICIAN | CHANGE_JOB2 | SKILL_1, true),
    SKILL_WATORNADO => (GROUP_MAGICIAN | CHANGE_JOB2 | SKILL_2, false),
    SKILL_ENCHANT_WEAPON => (GROUP_MAGICIAN | CHANGE_JOB2 | SKILL_3, false),
    SKILL_DEAD_RAY => (GROUP_MAGICIAN | CHANGE_JOB2 | SKILL_4, false),
    SKILL_ENERGY_SHIELD => (GROUP_MAGICIAN | CHANGE_JOB3 | SKILL_1, false),
    SKILL_DIASTROPHISM => (GROUP_MAGICIAN | CHANGE_JOB3 | SKILL_2, false),
    SKILL_SPIRIT_ELEMENTAL => (GROUP_MAGICIAN | CHANGE_JOB3 | SKILL_3, false),
    SKILL_DANCING_SWORD => (GROUP_MAGICIAN | CHANGE_JOB3 | SKILL_4, false),
    SKILL_FIRE_ELEMENTAL => (GROUP_MAGICIAN | CHANGE_JOB4 | SKILL_1, false),
    SKILL_FLAME_WAVE => (GROUP_MAGICIAN | CHANGE_JOB4 | SKILL_2, false),
    SKILL_DISTORTION => (GROUP_MAGICIAN | CHANGE_JOB4 | SKILL_3, false),
    SKILL_M_METEO => (GROUP_MAGICIAN | CHANGE_JOB4 | SKILL_4, false),
    SKILL_SILRAPHIM => (GROUP_MAGICIAN | CHANGE_JOB5 | SKILL_1, false),
    SKILL_TENUS => (GROUP_MAGICIAN | CHANGE_JOB5 | SKILL_2, false),
    SKILL_IGNIS => (GROUP_MAGICIAN | CHANGE_JOB5 | SKILL_3, false),
    SKILL_ANIMA => (GROUP_MAGICIAN | CHANGE_JOB5 | SKILL_4, false),
    SKILL_NORMAL_ATTACK => (0x11111111, false),
    SKILL_EXTREME_SHIELD => (GROUP_MECHANICIAN | CHANGE_JOB1 | SKILL_1, false),
    SKILL_MECHANIC_BOMB => (GROUP_MECHANICIAN | CHANGE_JOB1 | SKILL_2, false),
    SKILL_PHYSICAL_ABSORB => (GROUP_MECHANICIAN | CHANGE_JOB1 | SKILL_3, false),
    SKILL_POISON_ATTRIBUTE => (GROUP_MECHANICIAN | CHANGE_JOB1 | SKILL_4, true),
    SKILL_GREAT_SMASH => (GROUP_MECHANICIAN | CHANGE_JOB2 | SKILL_1, false),
    SKILL_MAXIMIZE => (GROUP_MECHANICIAN | CHANGE_JOB2 | SKILL_2, false),
    SKILL_AUTOMATION => (GROUP_MECHANICIAN | CHANGE_JOB2 | SKILL_3, false),
    SKILL_SPARK => (GROUP_MECHANICIAN | CHANGE_JOB2 | SKILL_4, false),
    SKILL_METAL_ARMOR => (GROUP_MECHANICIAN | CHANGE_JOB3 | SKILL_1, false),
    SKILL_GRAND_SMASH => (GROUP_MECHANICIAN | CHANGE_JOB3 | SKILL_2, false),
    SKILL_MECHANIC_WEAPON => (GROUP_MECHANICIAN | CHANGE_JOB3 | SKILL_3, true),
    SKILL_SPARK_SHIELD => (GROUP_MECHANICIAN | CHANGE_JOB3 | SKILL_4, false),
    SKILL_IMPULSION => (GROUP_MECHANICIAN | CHANGE_JOB4 | SKILL_1, false),
    SKILL_COMPULSION => (GROUP_MECHANICIAN | CHANGE_JOB4 | SKILL_2, false),
    SKILL_MAGNETIC_SPHERE => (GROUP_MECHANICIAN | CHANGE_JOB4 | SKILL_3, false),
    SKILL_METAL_GOLEM => (GROUP_MECHANICIAN | CHANGE_JOB4 | SKILL_4, false),
    SKILL_LAND_M => (GROUP_MECHANICIAN | CHANGE_JOB5 | SKILL_1, false),
    SKILL_H_SONIC => (GROUP_MECHANICIAN | CHANGE_JOB5 | SKILL_2, false),
    SKILL_R_SMASH => (GROUP_MECHANICIAN | CHANGE_JOB5 | SKILL_3, false),
    SKILL_P_ENHENCE => (GROUP_MECHANICIAN | CHANGE_JOB5 | SKILL_4, false),
    SKILL_MELEE_MASTERY => (GROUP_FIGHTER | CHANGE_JOB1 | SKILL_1, true),
    SKILL_FIRE_ATTRIBUTE => (GROUP_FIGHTER | CHANGE_JOB1 | SKILL_2, true),
    SKILL_RAVING => (GROUP_FIGHTER | CHANGE_JOB1 | SKILL_3, false),
    SKILL_IMPACT => (GROUP_FIGHTER | CHANGE_JOB1 | SKILL_4, false),
    SKILL_TRIPLE_IMPACT => (GROUP_FIGHTER | CHANGE_JOB2 | SKILL_1, false),
    SKILL_BRUTAL_SWING => (GROUP_FIGHTER | CHANGE_JOB2 | SKILL_2, false),
    SKILL_ROAR => (GROUP_FIGHTER | CHANGE_JOB2 | SKILL_3, false),
    SKILL_RAGE_OF_ZECRAM => (GROUP_FIGHTER | CHANGE_JOB2 | SKILL_4, false),
    SKILL_CONCENTRATION => (GROUP_FIGHTER | CHANGE_JOB3 | SKILL_1, false),
    SKILL_AVANGING_CRASH => (GROUP_FIGHTER | CHANGE_JOB3 | SKILL_2, false),
    SKILL_SWIFT_AXE => (GROUP_FIGHTER | CHANGE_JOB3 | SKILL_3, false),
    SKILL_BONE_CRASH => (GROUP_FIGHTER | CHANGE_JOB3 | SKILL_4, false),
    SKILL_DETORYER => (GROUP_FIGHTER | CHANGE_JOB4 | SKILL_1, false),
    SKILL_BERSERKER => (GROUP_FIGHTER | CHANGE_JOB4 | SKILL_2, false),
    SKILL_CYCLONE_STRIKE => (GROUP_FIGHTER | CHANGE_JOB4 | SKILL_3, false),
    SKILL_BOOST_HEALTH => (GROUP_FIGHTER | CHANGE_JOB4 | SKILL_4, true),
    SKILL_D_HIT => (GROUP_FIGHTER | CHANGE_JOB5 | SKILL_1, false),
    SKILL_P_DASH => (GROUP_FIGHTER | CHANGE_JOB5 | SKILL_2, false),
    SKILL_M_BLOW => (GROUP_FIGHTER | CHANGE_JOB5 | SKILL_3, false),
    SKILL_B_BERSERKER => (GROUP_FIGHTER | CHANGE_JOB5 | SKILL_4, false),
    SKILL_PIKE_WIND => (GROUP_PIKEMAN | CHANGE_JOB1 | SKILL_1, false),
    SKILL_ICE_ATTRIBUTE => (GROUP_PIKEMAN | CHANGE_JOB1 | SKILL_2, true),
    SKILL_CRITICAL_HIT => (GROUP_PIKEMAN | CHANGE_JOB1 | SKILL_3, false),
    SKILL_JUMPING_CRASH => (GROUP_PIKEMAN | CHANGE_JOB1 | SKILL_4, false),
    SKILL_GROUND_PIKE => (GROUP_PIKEMAN | CHANGE_JOB2 | SKILL_1, false),
    SKILL_TORNADO => (GROUP_PIKEMAN | CHANGE_JOB2 | SKILL_2, false),
    SKILL_WEAPONE_DEFENCE_MASTERY => (GROUP_PIKEMAN | CHANGE_JOB2 | SKILL_3, true),
    SKILL_EXPANSION => (GROUP_PIKEMAN | CHANGE_JOB2 | SKILL_4, false),
    SKILL_VENOM_SPEAR => (GROUP_PIKEMAN | CHANGE_JOB3 | SKILL_1, false),
    SKILL_VANISH => (GROUP_PIKEMAN | CHANGE_JOB3 | SKILL_2, false),
    SKILL_CRITICAL_MASTERY => (GROUP_PIKEMAN | CHANGE_JOB3 | SKILL_3, true),
    SKILL_CHAIN_LANCE => (GROUP_PIKEMAN | CHANGE_JOB3 | SKILL_4, false),
    SKILL_ASSASSIN_EYE => (GROUP_PIKEMAN | CHANGE_JOB4 | SKILL_1, false),
    SKILL_CHARGING_STRIKE => (GROUP_PIKEMAN | CHANGE_JOB4 | SKILL_2, false),
    SKILL_VAGUE => (GROUP_PIKEMAN | CHANGE_JOB4 | SKILL_3, false),
    SKILL_SHADOW_MASTER => (GROUP_PIKEMAN | CHANGE_JOB4 | SKILL_4, false),
    SKILL_D_REAPER => (GROUP_PIKEMAN | CHANGE_JOB5 | SKILL_1, false),
    SKILL_F_SPEAR => (GROUP_PIKEMAN | CHANGE_JOB5 | SKILL_2, false),
    SKILL_AMPLIFIED => (GROUP_PIKEMAN | CHANGE_JOB5 | SKILL_3, false),
    SKILL_SS_ATTACK => (GROUP_PIKEMAN | CHANGE_JOB5 | SKILL_4, false),
    SKILL_SCOUT_HAWK => (GROUP_ARCHER | CHANGE_JOB1 | SKILL_1, false),
    SKILL_SHOOTING_MASTERY => (GROUP_ARCHER | CHANGE_JOB1 | SKILL_2, true),
    SKILL_WIND_ARROW => (GROUP_ARCHER | CHANGE_JOB1 | SKILL_3, false),
    SKILL_PERFECT_AIM => (GROUP_ARCHER | CHANGE_JOB1 | SKILL_4, false),
    SKILL_DIONS_EYE => (GROUP_ARCHER | CHANGE_JOB2 | SKILL_1, true),
    SKILL_FALCON => (GROUP_ARCHER | CHANGE_JOB2 | SKILL_2, false),
    SKILL_ARROW_OF_RAGE => (GROUP_ARCHER | CHANGE_JOB2 | SKILL_3, false),
    SKILL_AVALANCHE => (GROUP_ARCHER | CHANGE_JOB2 | SKILL_4, false),
    SKILL_ELEMENTAL_SHOT => (GROUP_ARCHER | CHANGE_JOB3 | SKILL_1, false),
    SKILL_GOLDEN_FALCON => (GROUP_ARCHER | CHANGE_JOB3 | SKILL_2, false),
    SKILL_BOMB_SHOT => (GROUP_ARCHER | CHANGE_JOB3 | SKILL_3, false),
    SKILL_PERFORATION => (GROUP_ARCHER | CHANGE_JOB3 | SKILL_4, false),
    SKILL_RECALL_WOLVERIN => (GROUP_ARCHER | CHANGE_JOB4 | SKILL_1, false),
    SKILL_EVASION_MASTERY => (GROUP_ARCHER | CHANGE_JOB4 | SKILL_2, true),
    SKILL_PHOENIX_SHOT => (GROUP_ARCHER | CHANGE_JOB4 | SKILL_3, false),
    SKILL_FORCE_OF_NATURE => (GROUP_ARCHER | CHANGE_JOB4 | SKILL_4, false),
    SKILL_E_SHOT => (GROUP_ARCHER | CHANGE_JOB5 | SKILL_1, false),
    SKILL_S_ROPE => (GROUP_ARCHER | CHANGE_JOB5 | SKILL_2, false),
    SKILL_N_SPLASH => (GROUP_ARCHER | CHANGE_JOB5 | SKILL_3, false),
    SKILL_C_TRAP => (GROUP_ARCHER | CHANGE_JOB5 | SKILL_4, false),
    SKILL_STINGGER => (GROUP_ASSASSINE | CHANGE_JOB1 | SKILL_1, false),
    SKILL_R_HIT => (GROUP_ASSASSINE | CHANGE_JOB1 | SKILL_2, false),
    SKILL_D_MASTERY => (GROUP_ASSASSINE | CHANGE_JOB1 | SKILL_3, true),
    SKILL_WISP => (GROUP_ASSASSINE | CHANGE_JOB1 | SKILL_4, false),
    SKILL_V_THRONE => (GROUP_ASSASSINE | CHANGE_JOB2 | SKILL_1, false),
    SKILL_ALAS => (GROUP_ASSASSINE | CHANGE_JOB2 | SKILL_2, false),
    SKILL_S_SHOCK => (GROUP_ASSASSINE | CHANGE_JOB2 | SKILL_3, false),
    SKILL_A_MASTERY => (GROUP_ASSASSINE | CHANGE_JOB2 | SKILL_4, true),
    SKILL_S_SWORD => (GROUP_ASSASSINE | CHANGE_JOB3 | SKILL_1, false),
    SKILL_B_UP => (GROUP_ASSASSINE | CHANGE_JOB3 | SKILL_2, false),
    SKILL_INPES => (GROUP_ASSASSINE | CHANGE_JOB3 | SKILL_3, false),
    SKILL_BLIND => (GROUP_ASSASSINE | CHANGE_JOB3 | SKILL_4, false),
    SKILL_F_WIND => (GROUP_ASSASSINE | CHANGE_JOB4 | SKILL_1, false),
    SKILL_F_MASTERY => (GROUP_ASSASSINE | CHANGE_JOB4 | SKILL_2, true),
    SKILL_POLLUTED => (GROUP_ASSASSINE | CHANGE_JOB4 | SKILL_3, false),
    SKILL_P_SHADOW => (GROUP_ASSASSINE | CHANGE_JOB4 | SKILL_4, false),
    SKILL_J_BOMB => (GROUP_ASSASSINE | CHANGE_JOB5 | SKILL_1, false),
    SKILL_R_SLASH => (GROUP_ASSASSINE | CHANGE_JOB5 | SKILL_2, false),
    SKILL_V_STAB => (GROUP_ASSASSINE | CHANGE_JOB5 | SKILL_3, false),
    SKILL_STORM => (GROUP_ASSASSINE | CHANGE_JOB5 | SKILL_4, false),
    SKILL_DARKBOLT => (GROUP_SHAMAN | CHANGE_JOB1 | SKILL_1, false),
    SKILL_DARKWAVE => (GROUP_SHAMAN | CHANGE_JOB1 | SKILL_2, false),
    SKILL_CURSELAZY => (GROUP_SHAMAN | CHANGE_JOB1 | SKILL_3, false),
    SKILL_L_PEACE => (GROUP_SHAMAN | CHANGE_JOB1 | SKILL_4, true),
    SKILL_S_FLARE => (GROUP_SHAMAN | CHANGE_JOB2 | SKILL_1, false),
    SKILL_S_MANACLE => (GROUP_SHAMAN | CHANGE_JOB2 | SKILL_2, false),
    SKILL_C_HUNT => (GROUP_SHAMAN | CHANGE_JOB2 | SKILL_3, false),
    SKILL_A_MIGAL => (GROUP_SHAMAN | CHANGE_JOB2 | SKILL_4, false),
    SKILL_R_MAKER => (GROUP_SHAMAN | CHANGE_JOB3 | SKILL_1, false),
    SKILL_L_GHOST => (GROUP_SHAMAN | CHANGE_JOB3 | SKILL_2, false),
    SKILL_HAUNT => (GROUP_SHAMAN | CHANGE_JOB3 | SKILL_3, false),
    SKILL_SCRATCH => (GROUP_SHAMAN | CHANGE_JOB3 | SKILL_4, false),
    SKILL_R_KNIGHT => (GROUP_SHAMAN | CHANGE_JOB4 | SKILL_1, false),
    SKILL_JUDGE => (GROUP_SHAMAN | CHANGE_JOB4 | SKILL_2, false),
    SKILL_A_MIDRANDA => (GROUP_SHAMAN | CHANGE_JOB4 | SKILL_3, false),
    SKILL_M_PRAY => (GROUP_SHAMAN | CHANGE_JOB4 | SKILL_4, false),
    SKILL_CREED => (GROUP_SHAMAN | CHANGE_JOB5 | SKILL_1, false),
    SKILL_P_DEITY => (GROUP_SHAMAN | CHANGE_JOB5 | SKILL_2, false),
    SKILL_G_NAIL => (GROUP_SHAMAN | CHANGE_JOB5 | SKILL_3, false),
    SKILL_H_REGENE => (GROUP_SHAMAN | CHANGE_JOB5 | SKILL_4, true),
    SKILL_LOW_KICK => (GROUP_MARTIAL | CHANGE_JOB1 | SKILL_1, false),
    SKILL_S_MASTERY => (GROUP_MARTIAL | CHANGE_JOB1 | SKILL_2, true),
    SKILL_DBBLOW => (GROUP_MARTIAL | CHANGE_JOB1 | SKILL_3, false),
    SKILL_H_STRAIGHT => (GROUP_MARTIAL | CHANGE_JOB1 | SKILL_4, false),
    SKILL_RAGE_UP => (GROUP_MARTIAL | CHANGE_JOB2 | SKILL_1, false),
    SKILL_PATRIOT => (GROUP_MARTIAL | CHANGE_JOB2 | SKILL_2, false),
    SKILL_R_ELBOW => (GROUP_MARTIAL | CHANGE_JOB2 | SKILL_3, false),
    SKILL_S_MASTERY2 => (GROUP_MARTIAL | CHANGE_JOB2 | SKILL_4, true),
    SKILL_I_BULKUP => (GROUP_MARTIAL | CHANGE_JOB3 | SKILL_1, false),
    SKILL_T_CANNON => (GROUP_MARTIAL | CHANGE_JOB3 | SKILL_2, false),
    SKILL_WAR_CRY => (GROUP_MARTIAL | CHANGE_JOB3 | SKILL_3, false),
    SKILL_J_HEELKICK => (GROUP_MARTIAL | CHANGE_JOB3 | SKILL_4, false),
    SKILL_COMBINATION => (GROUP_MARTIAL | CHANGE_JOB4 | SKILL_1, false),
    SKILL_STEELERS => (GROUP_MARTIAL | CHANGE_JOB4 | SKILL_2, false),
    SKILL_B_CHECK => (GROUP_MARTIAL | CHANGE_JOB4 | SKILL_3, false),
    SKILL_TYPHOON => (GROUP_MARTIAL | CHANGE_JOB4 | SKILL_4, false),
    SKILL_D_MASTERY2 => (GROUP_MARTIAL | CHANGE_JOB5 | SKILL_1, true),
    SKILL_H_HAWK => (GROUP_MARTIAL | CHANGE_JOB5 | SKILL_2, false),
    SKILL_L_BREAKING => (GROUP_MARTIAL | CHANGE_JOB5 | SKILL_3, false),
    SKILL_H_TRAINING => (GROUP_MARTIAL | CHANGE_JOB5 | SKILL_4, false),
    SKILL_FORCE_ORB => (GROUP_OTHERSKILL | CHANGE_JOB1 | SKILL_1, false),
    SKILL_TOP_LVL => (GROUP_OTHERSKILL | CHANGE_JOB4 | SKILL_12, false),
    SKILL_TOP_PVP => (GROUP_OTHERSKILL | CHANGE_JOB4 | SKILL_13, false),
    SKILL_PVP_MODE => (GROUP_OTHERSKILL | CHANGE_JOB4 | SKILL_14, false),
}

pub const GROUP_ASSASSIN: u32 = GROUP_ASSASSINE;
pub const SKILL_PHYSICAL_TRAINING: SkillCode = SKILL_PHYSICAL_TRANING;
pub const SKILL_WEAPON_DEFENSE_MASTERY: SkillCode = SKILL_WEAPONE_DEFENCE_MASTERY;

pub const SKILL_SLOT_1: SkillSlot = SkillSlot(1);
pub const SKILL_SLOT_2: SkillSlot = SkillSlot(2);
pub const SKILL_SLOT_3: SkillSlot = SkillSlot(3);
pub const SKILL_SLOT_4: SkillSlot = SkillSlot(4);
pub const SKILL_SLOT_5: SkillSlot = SkillSlot(5);
pub const SKILL_SLOT_6: SkillSlot = SkillSlot(6);
pub const SKILL_SLOT_7: SkillSlot = SkillSlot(7);
pub const SKILL_SLOT_8: SkillSlot = SkillSlot(8);
pub const SKILL_SLOT_9: SkillSlot = SkillSlot(9);
pub const SKILL_SLOT_10: SkillSlot = SkillSlot(10);
pub const SKILL_SLOT_11: SkillSlot = SkillSlot(11);
pub const SKILL_SLOT_12: SkillSlot = SkillSlot(12);
pub const SKILL_SLOT_13: SkillSlot = SkillSlot(13);
pub const SKILL_SLOT_14: SkillSlot = SkillSlot(14);
pub const SKILL_SLOT_15: SkillSlot = SkillSlot(15);
pub const SKILL_SLOT_16: SkillSlot = SkillSlot(16);

impl SkillCode {
    pub const fn new(group: SkillGroup, tier: ChangeJobTier, slot: SkillSlot) -> Self {
        Self((group as u32) | (tier as u32) | (slot.0 as u32))
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    pub const fn group(self) -> SkillGroupBits {
        SkillGroupBits(self.0 & SKILL_MASK1)
    }

    pub const fn tier(self) -> ChangeJobBits {
        ChangeJobBits(self.0 & (SKILL_MASK2 ^ SKILL_MASK1))
    }

    pub const fn slot(self) -> SkillSlot {
        SkillSlot((self.0 & SKILL_MASK3) as u16)
    }

    pub const fn group_bits(self) -> u32 {
        self.group().0
    }

    pub const fn change_job_bits(self) -> u32 {
        self.tier().0
    }

    pub const fn slot_bits(self) -> u32 {
        self.slot().0 as u32
    }

    pub const fn group_enum(self) -> Option<SkillGroup> {
        match self.group().0 {
            GROUP_MECHANICIAN => Some(SkillGroup::Mechanician),
            GROUP_FIGHTER => Some(SkillGroup::Fighter),
            GROUP_PIKEMAN => Some(SkillGroup::Pikeman),
            GROUP_ARCHER => Some(SkillGroup::Archer),
            GROUP_KNIGHT => Some(SkillGroup::Knight),
            GROUP_ATALANTA => Some(SkillGroup::Atalanta),
            GROUP_PRIESTESS => Some(SkillGroup::Priestess),
            GROUP_MAGICIAN => Some(SkillGroup::Magician),
            GROUP_OTHERSKILL => Some(SkillGroup::Other),
            GROUP_ASSASSINE => Some(SkillGroup::Assassin),
            GROUP_SHAMAN => Some(SkillGroup::Shaman),
            GROUP_MARTIAL => Some(SkillGroup::Martial),
            _ => None,
        }
    }

    pub const fn tier_enum(self) -> Option<ChangeJobTier> {
        match self.tier().0 {
            CHANGE_JOB1 => Some(ChangeJobTier::Tier1),
            CHANGE_JOB2 => Some(ChangeJobTier::Tier2),
            CHANGE_JOB3 => Some(ChangeJobTier::Tier3),
            CHANGE_JOB4 => Some(ChangeJobTier::Tier4),
            CHANGE_JOB5 => Some(ChangeJobTier::Tier5),
            _ => None,
        }
    }

    pub fn is_passive(self) -> bool {
        skill_def_opt(self).is_some_and(|d| d.passive)
    }
}

impl Default for SkillCode {
    fn default() -> Self {
        SKILL_NORMAL_ATTACK
    }
}

pub fn is_skill_group(code: SkillCode, group: SkillGroup) -> bool {
    code.group_enum() == Some(group)
}

pub fn is_change_job_tier(code: SkillCode, tier: ChangeJobTier) -> bool {
    code.tier_enum() == Some(tier)
}

pub fn skill_slot(code: SkillCode) -> SkillSlot {
    code.slot()
}

pub const fn skill_def_opt(code: SkillCode) -> Option<SkillDef> {
    skill_def_by_raw(code.raw())
}

pub fn is_passive_skill(code: SkillCode) -> bool {
    skill_def_opt(code).is_some_and(|d| d.passive)
}

impl TryFrom<u32> for SkillCode {
    type Error = InvalidSkillCode;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match skill_def_by_raw(value) {
            Some(def) => Ok(def.code),
            None => Err(InvalidSkillCode(value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_accepts_valid_skill_codes() {
        let extreme_shield = SkillCode::try_from(SKILL_EXTREME_SHIELD.raw());
        assert!(extreme_shield.is_ok());
        assert_eq!(
            match extreme_shield {
                Ok(code) => code,
                Err(error) => {
                    assert!(false, "unexpected skill code failure: {error:?}");
                    return;
                }
            },
            SKILL_EXTREME_SHIELD
        );
        let force_orb = SkillCode::try_from(SKILL_FORCE_ORB.raw());
        assert!(force_orb.is_ok());
        assert_eq!(
            match force_orb {
                Ok(code) => code,
                Err(error) => {
                    assert!(false, "unexpected skill code failure: {error:?}");
                    return;
                }
            },
            SKILL_FORCE_ORB
        );
    }

    #[test]
    fn try_from_rejects_invalid_skill_code() {
        assert!(SkillCode::try_from(0xDEAD_BEEF).is_err());
    }

    #[test]
    fn decompose_bits_matches_expected() {
        let code = SKILL_EXTREME_SHIELD;
        assert_eq!(code.group_bits(), GROUP_MECHANICIAN);
        assert_eq!(code.change_job_bits(), CHANGE_JOB1);
        assert_eq!(code.slot(), SKILL_SLOT_1);
        assert_eq!(code.group_enum(), Some(SkillGroup::Mechanician));
        assert_eq!(code.tier_enum(), Some(ChangeJobTier::Tier1));
    }

    #[test]
    fn aliases_share_same_raw_value() {
        assert_eq!(SKILLDELAY_ITEM_LIFE, SCROLL_INVULNERABILITY);
        assert_eq!(SKILL_PHYSICAL_TRANING, SKILL_PHYSICAL_TRAINING);
        assert_eq!(SKILL_WEAPONE_DEFENCE_MASTERY, SKILL_WEAPON_DEFENSE_MASTERY);
    }

    #[test]
    fn passive_classification_matches_rules() {
        assert!(is_passive_skill(SKILL_MELEE_MASTERY));
        assert!(!is_passive_skill(SKILL_SPARK));
    }
}
