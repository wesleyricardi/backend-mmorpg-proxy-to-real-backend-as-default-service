#![allow(dead_code, non_upper_case_globals)]

use bitflags::bitflags;
use core::convert::TryFrom;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::domain::item::item_code::ItemCode;

pub const MAX_ITEM: usize = 2000;
pub const ITEMSIZE: i32 = 22;
pub const CHECK_COPY_ITEM: u32 = 1;
pub const CHECK_GIVE_ITEM: u32 = 2;
pub const MAX_ATTR_ITEM_LINE: usize = 20;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct InventoryPosition: u32 {
        const BOX     = 0x0000_0001;
        const LEFT_HAND   = 0x0000_0002;
        const RIGHT_HAND   = 0x0000_0004;
        const ARMOR   = 0x0000_0008;
        const BOOTS   = 0x0000_0010;
        const GLOVES  = 0x0000_0020;
        const LEFT_RING   = 0x0000_0040;
        const RIGHT_RING   = 0x0000_0080;
        const SHELTOM = 0x0000_0100;
        const AMULET  = 0x0000_0200;
        const ARMLET  = 0x0000_0800;
        const TWO_HAND = 0x0000_1000;
        const POTION  = 0x0000_2000;
        const COSTUME = 0x0000_4000;
        const LEFT_EARRING = 0x0001_0000;
        const RIGHT_EARRING = 0x0002_0000;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum EquipSlot {
    Box,
    LeftHand,
    RightHand,
    Armor,
    Boots,
    Gloves,
    LeftRing,
    RightRing,
    Sheltom,
    Amulet,
    Armlet,
    TwoHand,
    Potion,
    Costume,
    LeftEarring,
    RightEarring,
}

impl EquipSlot {
    pub const fn flag(self) -> InventoryPosition {
        match self {
            EquipSlot::Box => InventoryPosition::BOX,
            EquipSlot::LeftRing => InventoryPosition::LEFT_RING,
            EquipSlot::RightRing => InventoryPosition::RIGHT_RING,
            EquipSlot::LeftHand => InventoryPosition::LEFT_HAND,
            EquipSlot::RightHand => InventoryPosition::RIGHT_HAND,
            EquipSlot::Armor => InventoryPosition::ARMOR,
            EquipSlot::Boots => InventoryPosition::BOOTS,
            EquipSlot::Gloves => InventoryPosition::GLOVES,
            EquipSlot::Sheltom => InventoryPosition::SHELTOM,
            EquipSlot::Amulet => InventoryPosition::AMULET,
            EquipSlot::Armlet => InventoryPosition::ARMLET,
            EquipSlot::TwoHand => InventoryPosition::TWO_HAND,
            EquipSlot::Potion => InventoryPosition::POTION,
            EquipSlot::Costume => InventoryPosition::COSTUME,
            EquipSlot::LeftEarring => InventoryPosition::LEFT_EARRING,
            EquipSlot::RightEarring => InventoryPosition::RIGHT_EARRING,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemClass {
    WeaponOne,
    WeaponTwo,
    Armor,
    Boots,
    Gloves,
    Shield,
    Ring,
    Sheltom,
    Amulet,
    Armlet,
    MagicalStuff,
    Potion,
    Ecore,
    Quest,
    ForceOrb,
    Seel,
    Costume,
}

impl ItemClass {
    pub const fn allowed_slots(self) -> InventoryPosition {
        match self {
            ItemClass::WeaponOne => InventoryPosition::RIGHT_HAND,
            ItemClass::WeaponTwo => {
                InventoryPosition::RIGHT_HAND.union(InventoryPosition::LEFT_HAND)
            }

            ItemClass::Armor => InventoryPosition::ARMOR,
            ItemClass::Boots => InventoryPosition::BOOTS,
            ItemClass::Gloves => InventoryPosition::GLOVES,

            ItemClass::Shield => InventoryPosition::LEFT_HAND,
            ItemClass::Ring => InventoryPosition::LEFT_RING.union(InventoryPosition::RIGHT_RING),

            ItemClass::Sheltom => InventoryPosition::SHELTOM,
            ItemClass::Amulet => InventoryPosition::AMULET,
            ItemClass::Armlet => InventoryPosition::ARMLET,

            ItemClass::MagicalStuff => InventoryPosition::LEFT_HAND,

            ItemClass::Potion => InventoryPosition::POTION,

            ItemClass::Ecore => InventoryPosition::BOX,
            ItemClass::Quest => InventoryPosition::BOX,
            ItemClass::ForceOrb => InventoryPosition::BOX,
            ItemClass::Seel => InventoryPosition::BOX,

            ItemClass::Costume => InventoryPosition::COSTUME,
        }
    }

    pub const fn check_slot(self, slot: EquipSlot) -> bool {
        self.allowed_slots().contains(slot.flag())
    }

    pub const fn allow_on_lring(self) -> bool {
        return self.check_slot(EquipSlot::LeftRing);
    }
}

// ===========================
// 3) ItemType = parte do RAW
// ===========================

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemType {
    WA1 = 0x0101_0000,
    WC1 = 0x0102_0000,
    WH1 = 0x0103_0000,
    WM1 = 0x0104_0000,
    WP1 = 0x0105_0000,
    WS1 = 0x0106_0000,
    WS2 = 0x0107_0000,
    WT1 = 0x0108_0000,
    WN1 = 0x0109_0000,
    WD1 = 0x010A_0000,
    WV1 = 0x010B_0000,

    DA1 = 0x0201_0000,
    DB1 = 0x0202_0000,
    DG1 = 0x0203_0000,
    DS1 = 0x0204_0000,
    DA2 = 0x0205_0000,

    CA1 = 0x0207_0000,
    CA2 = 0x0208_0000,
    CA5 = 0x0210_0000,
    CA6 = 0x0211_0000,

    DA3 = 0x0212_0000,
    DA4 = 0x0213_0000,

    OA1 = 0x0301_0000,
    OA2 = 0x0302_0000,
    OM1 = 0x0303_0000,
    OR1 = 0x0304_0000,
    OR2 = 0x0305_0000,

    OS1 = 0x0235_0000,

    FO1 = 0x0306_0000,
    SE1 = 0x0307_0000,

    PR1 = 0x0308_0000,
    PR2 = 0x0309_0000,
    PR3 = 0x0310_0000,
    PR4 = 0x0311_0000,

    OE1 = 0x030A_0000,

    PM1 = 0x0401_0000,
    PL1 = 0x0402_0000,
    PS1 = 0x0403_0000,

    GG1 = 0x0501_0000,
    BS1 = 0x0502_0000,

    EC1 = 0x0601_0000,
    QT1 = 0x0701_0000,

    SP1 = 0x0801_0000,
    GP1 = 0x0802_0000,
    QW1 = 0x0803_0000,
    GF1 = 0x0804_0000,

    PZ1 = 0x0806_0000,
    PZ2 = 0x0807_0000,

    CH1 = 0x0808_0000,
    SD2 = 0x0809_0000,

    BC1 = 0x080A_0000,

    BI1 = 0x080B_0000,
    BI2 = 0x080C_0000,
    GP2 = 0x080D_0000,

    MA1 = 0x0901_0000,
    MA2 = 0x0902_0000,
    BI3 = 0x0903_0000,

    EV1 = 0x0904_0000,
    EV2 = 0x0906_0000,

    WR1 = 0x0A01_0000,
    DR1 = 0x0A02_0000,
    RR1 = 0x0A04_0000,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EquippableItemType {
    WA1 = ItemType::WA1 as u32,
    WC1 = ItemType::WC1 as u32,
    WH1 = ItemType::WH1 as u32,
    WM1 = ItemType::WM1 as u32,
    WP1 = ItemType::WP1 as u32,
    WS1 = ItemType::WS1 as u32,
    WS2 = ItemType::WS2 as u32,
    WT1 = ItemType::WT1 as u32,
    WN1 = ItemType::WN1 as u32,
    WD1 = ItemType::WD1 as u32,
    WV1 = ItemType::WV1 as u32,
    DA1 = ItemType::DA1 as u32,
    DB1 = ItemType::DB1 as u32,
    DG1 = ItemType::DG1 as u32,
    DS1 = ItemType::DS1 as u32,
    DA2 = ItemType::DA2 as u32,
    CA1 = ItemType::CA1 as u32,
    CA2 = ItemType::CA2 as u32,
    CA5 = ItemType::CA5 as u32,
    CA6 = ItemType::CA6 as u32,
    DA3 = ItemType::DA3 as u32,
    DA4 = ItemType::DA4 as u32,
    OA1 = ItemType::OA1 as u32,
    OA2 = ItemType::OA2 as u32,
    OM1 = ItemType::OM1 as u32,
    OR1 = ItemType::OR1 as u32,
    OR2 = ItemType::OR2 as u32,
    OS1 = ItemType::OS1 as u32,
    OE1 = ItemType::OE1 as u32,
    MA1 = ItemType::MA1 as u32,
    MA2 = ItemType::MA2 as u32,
    EV1 = ItemType::EV1 as u32,
    EV2 = ItemType::EV2 as u32,
    WR1 = ItemType::WR1 as u32,
    DR1 = ItemType::DR1 as u32,
    RR1 = ItemType::RR1 as u32,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct PlayerJobMask: u32 {
        const MECHANICIAN = 0x0000_0001;
        const FIGHTER = 0x0000_0002;
        const PIKEMAN = 0x0000_0004;
        const ARCHER = 0x0000_0008;
        const ASSASSIN = 0x0000_000A;
        const MARTIAL = 0x0000_000B;
        const KNIGHT = 0x0001_0000;
        const ATALANTA = 0x0002_0000;
        const PRIESTESS = 0x0004_0000;
        const MAGICIAN = 0x0008_0000;
        const SHAMAN = 0x000A_0000;
        // Wire values may contain combinations not explicitly named here.
        const _ = !0;
    }
}

impl Default for PlayerJobMask {
    fn default() -> Self {
        Self::empty()
    }
}

impl From<u32> for PlayerJobMask {
    fn from(value: u32) -> Self {
        Self::from_bits_retain(value)
    }
}

impl From<PlayerJobMask> for u32 {
    fn from(value: PlayerJobMask) -> Self {
        value.bits()
    }
}

impl Serialize for PlayerJobMask {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.bits())
    }
}

impl<'de> Deserialize<'de> for PlayerJobMask {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bits = u32::deserialize(deserializer)?;
        Ok(Self::from_bits_retain(bits))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidItemTypeCode(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidItemTypeRaw(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidEquippableItemTypeCode(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidEquippableItemTypeRaw(pub u32);

impl TryFrom<&str> for ItemType {
    type Error = InvalidItemTypeCode;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "WA1" => Ok(Self::WA1),
            "WC1" => Ok(Self::WC1),
            "WH1" => Ok(Self::WH1),
            "WM1" => Ok(Self::WM1),
            "WP1" => Ok(Self::WP1),
            "WS1" => Ok(Self::WS1),
            "WS2" => Ok(Self::WS2),
            "WT1" => Ok(Self::WT1),
            "WN1" => Ok(Self::WN1),
            "WD1" => Ok(Self::WD1),
            "WV1" => Ok(Self::WV1),
            "DA1" => Ok(Self::DA1),
            "DB1" => Ok(Self::DB1),
            "DG1" => Ok(Self::DG1),
            "DS1" => Ok(Self::DS1),
            "DA2" => Ok(Self::DA2),
            "CA1" => Ok(Self::CA1),
            "CA2" => Ok(Self::CA2),
            "CA5" => Ok(Self::CA5),
            "CA6" => Ok(Self::CA6),
            "DA3" => Ok(Self::DA3),
            "DA4" => Ok(Self::DA4),
            "OA1" => Ok(Self::OA1),
            "OA2" => Ok(Self::OA2),
            "OM1" => Ok(Self::OM1),
            "OR1" => Ok(Self::OR1),
            "OR2" => Ok(Self::OR2),
            "OS1" => Ok(Self::OS1),
            "FO1" => Ok(Self::FO1),
            "SE1" => Ok(Self::SE1),
            "PR1" => Ok(Self::PR1),
            "PR2" => Ok(Self::PR2),
            "PR3" => Ok(Self::PR3),
            "PR4" => Ok(Self::PR4),
            "OE1" => Ok(Self::OE1),
            "PM1" => Ok(Self::PM1),
            "PL1" => Ok(Self::PL1),
            "PS1" => Ok(Self::PS1),
            "GG1" => Ok(Self::GG1),
            "BS1" => Ok(Self::BS1),
            "EC1" => Ok(Self::EC1),
            "QT1" => Ok(Self::QT1),
            "SP1" => Ok(Self::SP1),
            "GP1" => Ok(Self::GP1),
            "QW1" => Ok(Self::QW1),
            "GF1" => Ok(Self::GF1),
            "PZ1" => Ok(Self::PZ1),
            "PZ2" => Ok(Self::PZ2),
            "CH1" => Ok(Self::CH1),
            "SD2" => Ok(Self::SD2),
            "BC1" => Ok(Self::BC1),
            "BI1" => Ok(Self::BI1),
            "BI2" => Ok(Self::BI2),
            "GP2" => Ok(Self::GP2),
            "MA1" => Ok(Self::MA1),
            "MA2" => Ok(Self::MA2),
            "BI3" => Ok(Self::BI3),
            "EV1" => Ok(Self::EV1),
            "EV2" => Ok(Self::EV2),
            "WR1" => Ok(Self::WR1),
            "DR1" => Ok(Self::DR1),
            "RR1" => Ok(Self::RR1),
            other => Err(InvalidItemTypeCode(other.to_string())),
        }
    }
}

impl TryFrom<&str> for EquippableItemType {
    type Error = InvalidEquippableItemTypeCode;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let item_type = ItemType::try_from(value)
            .map_err(|_| InvalidEquippableItemTypeCode(value.to_string()))?;
        Self::try_from(item_type).map_err(|_| InvalidEquippableItemTypeCode(value.to_string()))
    }
}

impl TryFrom<u32> for ItemType {
    type Error = InvalidItemTypeRaw;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x0101_0000 => Ok(Self::WA1),
            0x0102_0000 => Ok(Self::WC1),
            0x0103_0000 => Ok(Self::WH1),
            0x0104_0000 => Ok(Self::WM1),
            0x0105_0000 => Ok(Self::WP1),
            0x0106_0000 => Ok(Self::WS1),
            0x0107_0000 => Ok(Self::WS2),
            0x0108_0000 => Ok(Self::WT1),
            0x0109_0000 => Ok(Self::WN1),
            0x010A_0000 => Ok(Self::WD1),
            0x010B_0000 => Ok(Self::WV1),
            0x0201_0000 => Ok(Self::DA1),
            0x0202_0000 => Ok(Self::DB1),
            0x0203_0000 => Ok(Self::DG1),
            0x0204_0000 => Ok(Self::DS1),
            0x0205_0000 => Ok(Self::DA2),
            0x0207_0000 => Ok(Self::CA1),
            0x0208_0000 => Ok(Self::CA2),
            0x0210_0000 => Ok(Self::CA5),
            0x0211_0000 => Ok(Self::CA6),
            0x0212_0000 => Ok(Self::DA3),
            0x0213_0000 => Ok(Self::DA4),
            0x0301_0000 => Ok(Self::OA1),
            0x0302_0000 => Ok(Self::OA2),
            0x0303_0000 => Ok(Self::OM1),
            0x0304_0000 => Ok(Self::OR1),
            0x0305_0000 => Ok(Self::OR2),
            0x0235_0000 => Ok(Self::OS1),
            0x0306_0000 => Ok(Self::FO1),
            0x0307_0000 => Ok(Self::SE1),
            0x0308_0000 => Ok(Self::PR1),
            0x0309_0000 => Ok(Self::PR2),
            0x0310_0000 => Ok(Self::PR3),
            0x0311_0000 => Ok(Self::PR4),
            0x030A_0000 => Ok(Self::OE1),
            0x0401_0000 => Ok(Self::PM1),
            0x0402_0000 => Ok(Self::PL1),
            0x0403_0000 => Ok(Self::PS1),
            0x0501_0000 => Ok(Self::GG1),
            0x0502_0000 => Ok(Self::BS1),
            0x0601_0000 => Ok(Self::EC1),
            0x0701_0000 => Ok(Self::QT1),
            0x0801_0000 => Ok(Self::SP1),
            0x0802_0000 => Ok(Self::GP1),
            0x0803_0000 => Ok(Self::QW1),
            0x0804_0000 => Ok(Self::GF1),
            0x0806_0000 => Ok(Self::PZ1),
            0x0807_0000 => Ok(Self::PZ2),
            0x0808_0000 => Ok(Self::CH1),
            0x0809_0000 => Ok(Self::SD2),
            0x080A_0000 => Ok(Self::BC1),
            0x080B_0000 => Ok(Self::BI1),
            0x080C_0000 => Ok(Self::BI2),
            0x080D_0000 => Ok(Self::GP2),
            0x0901_0000 => Ok(Self::MA1),
            0x0902_0000 => Ok(Self::MA2),
            0x0903_0000 => Ok(Self::BI3),
            0x0904_0000 => Ok(Self::EV1),
            0x0906_0000 => Ok(Self::EV2),
            0x0A01_0000 => Ok(Self::WR1),
            0x0A02_0000 => Ok(Self::DR1),
            0x0A04_0000 => Ok(Self::RR1),
            other => Err(InvalidItemTypeRaw(other)),
        }
    }
}

impl TryFrom<u32> for EquippableItemType {
    type Error = InvalidEquippableItemTypeRaw;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let item_type =
            ItemType::try_from(value).map_err(|_| InvalidEquippableItemTypeRaw(value))?;
        Self::try_from(item_type).map_err(|_| InvalidEquippableItemTypeRaw(value))
    }
}

impl TryFrom<ItemType> for EquippableItemType {
    type Error = InvalidItemTypeRaw;

    fn try_from(value: ItemType) -> Result<Self, Self::Error> {
        match value {
            ItemType::WA1 => Ok(Self::WA1),
            ItemType::WC1 => Ok(Self::WC1),
            ItemType::WH1 => Ok(Self::WH1),
            ItemType::WM1 => Ok(Self::WM1),
            ItemType::WP1 => Ok(Self::WP1),
            ItemType::WS1 => Ok(Self::WS1),
            ItemType::WS2 => Ok(Self::WS2),
            ItemType::WT1 => Ok(Self::WT1),
            ItemType::WN1 => Ok(Self::WN1),
            ItemType::WD1 => Ok(Self::WD1),
            ItemType::WV1 => Ok(Self::WV1),
            ItemType::DA1 => Ok(Self::DA1),
            ItemType::DB1 => Ok(Self::DB1),
            ItemType::DG1 => Ok(Self::DG1),
            ItemType::DS1 => Ok(Self::DS1),
            ItemType::DA2 => Ok(Self::DA2),
            ItemType::CA1 => Ok(Self::CA1),
            ItemType::CA2 => Ok(Self::CA2),
            ItemType::CA5 => Ok(Self::CA5),
            ItemType::CA6 => Ok(Self::CA6),
            ItemType::DA3 => Ok(Self::DA3),
            ItemType::DA4 => Ok(Self::DA4),
            ItemType::OA1 => Ok(Self::OA1),
            ItemType::OA2 => Ok(Self::OA2),
            ItemType::OM1 => Ok(Self::OM1),
            ItemType::OR1 => Ok(Self::OR1),
            ItemType::OR2 => Ok(Self::OR2),
            ItemType::OS1 => Ok(Self::OS1),
            ItemType::OE1 => Ok(Self::OE1),
            ItemType::MA1 => Ok(Self::MA1),
            ItemType::MA2 => Ok(Self::MA2),
            ItemType::EV1 => Ok(Self::EV1),
            ItemType::EV2 => Ok(Self::EV2),
            ItemType::WR1 => Ok(Self::WR1),
            ItemType::DR1 => Ok(Self::DR1),
            ItemType::RR1 => Ok(Self::RR1),
            other => Err(InvalidItemTypeRaw(other as u32)),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemInfoElement {
    Bionic = 0,
    Earth = 1,
    Fire = 2,
    Ice = 3,
    Lighting = 4,
    Poison = 5,
    Water = 6,
    Wind = 7,
    None = 15,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemSound {
    Axes = 1,
    Claws = 2,
    Hammer = 3,
    Magicial = 4,
    Poles = 5,
    Shooters = 6,
    Swords = 7,
    Throwing = 8,
    Armor = 9,
    Boots = 10,
    Gloves = 11,
    Shields = 12,
    Amulet = 13,
    Armlet = 14,
    Ring = 15,
    Sheltom = 16,
    Potion = 17,
    Coin = 18,
    Staff = 19,
    EatPotion = 20,
    ShowInter = 21,
    RepairItem = 22,
    FaildMixItem = 23,
    Armor2 = 24,
    EatPotion2 = 25,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemKind {
    Normal = 0,
    Craft = 1,
    Aging = 2,
    Quest = 3,
    Event = 4,
    MakeMain = 5,
    MakeElement = 6,
    QuestWeapon = 7,
    Special = 8,
    Perfect = 9,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct SinAdd: u32 {
        const FIRE          = 0x0000_0001;
        const ICE           = 0x0000_0002;
        const LIGHTNING     = 0x0000_0004;
        const POISON        = 0x0000_0008;
        const BIO           = 0x0000_0010;
        const CRITICAL      = 0x0000_0020;
        const ATTACK_RATE   = 0x0000_0040;
        const DAMAGE_MIN    = 0x0000_0080;
        const DAMAGE_MAX    = 0x0000_0100;
        const ATTACK_SPEED  = 0x0000_0200;
        const ABSORB        = 0x0000_0400;
        const DEFENCE       = 0x0000_0800;
        const BLOCK_RATE    = 0x0000_1000;
        const MOVE_SPEED    = 0x0000_2000;
        const LIFE          = 0x0000_4000;
        const MANA          = 0x0000_8000;
        const STAMINA       = 0x0001_0000;
        const LIFE_REGEN    = 0x0002_0000;
        const MANA_REGEN    = 0x0004_0000;
        const STAMINA_REGEN = 0x0008_0000;
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ItemWeaponClass {
    #[default]
    NotShooting = 1,
    Shooting = 2,
    Casting = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidItemWeaponClass(pub i32);

impl TryFrom<i32> for ItemWeaponClass {
    type Error = InvalidItemWeaponClass;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::NotShooting),
            2 => Ok(Self::Shooting),
            3 => Ok(Self::Casting),
            other => Err(InvalidItemWeaponClass(other)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ItemCreateHeader {
    pub head: u32,
    pub version: u32,
    pub time: u32,
    pub checksum: u32,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ItemSpecial {
    pub add_absorb: f32,
    pub add_defense: i32,
    pub add_speed: f32,
    pub add_block_rating: f32,
    pub add_attack_speed: i32,
    pub add_critical_hit: i32,
    pub add_shooting_range: i32,
    pub add_magic_mastery: f32,
    pub add_resistance: [i16; 8],
    pub level_attack_resistance: [i16; 8],
    pub level_mana: i32,
    pub level_life: i32,
    pub level_attack_rating: i32,
    pub level_damage: [i16; 2],
    pub percent_mana_regen: f32,
    pub percent_life_regen: f32,
    pub percent_stamina_regen: f32,
    pub temp: [u32; 32],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SwapItem {
    pub flag: u32,
    pub code: ItemCode,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ItemInfo {
    pub size: u32,
    pub header: ItemCreateHeader,
    pub durability: [i16; 2],
    pub code: ItemCode,
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
    pub absorb: f32,
    pub defense: i32,
    pub block_rating: f32,
    pub speed: f32,
    pub potion_space: i32,
    pub magic_mastery: f32,
    pub mana_regen: f32,
    pub life_regen: f32,
    pub stamina_regen: f32,
    pub increase_life: f32,
    pub increase_mana: f32,
    pub increase_stamina: f32,
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
    pub backup_key: u32,
    pub backup_checksum: u32,
    pub scale_blink: [i16; 2],
    pub unique_item: u32,
    pub effect_blink: [i16; 2],
    pub effect_color: [i16; 4],
    pub display_effect: u32,
    pub job_code_mask: PlayerJobMask,
    pub job_item: ItemSpecial,
    pub item_kind_code: u32,
    pub item_kind_mask: u32,
    pub item_aging_num: [i16; 2],
    pub item_aging_count: [i16; 2],
    pub item_aging_protect: [i16; 2],
    pub special_item_flag: [i16; 2],
    pub swap_item: SwapItem,
    pub create_time: u32,
    pub linked_item: i32,
    pub lock_item: i32,
    pub coin: i32,
    pub temp: [u32; 6],
}

#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub code: ItemCode,
    pub name: &'static str,

    pub category: &'static str,
    pub w: i32,
    pub h: i32,
    pub item_file_path: &'static str,
    pub class: ItemClass,
    pub drop_item: &'static str,
    pub set_model_posi: Option<InventoryPosition>,
    pub sound: ItemSound,
    pub weapon_class: Option<ItemWeaponClass>,

    pub flag: i32,
    pub x: i32,
    pub y: i32,
    pub set_x: i32,
    pub set_y: i32,

    pub item_position: i32,

    pub potion_count: i32,
    pub not_use_flag: i32,
    pub sell_price: i32,
}

impl Default for Item {
    fn default() -> Self {
        Self {
            code: ItemCode::WA101,
            name: "",

            category: "",
            w: ITEMSIZE,
            h: ITEMSIZE,
            item_file_path: "",
            class: ItemClass::WeaponOne,
            drop_item: "",
            set_model_posi: None,
            sound: ItemSound::Axes,
            weapon_class: None,

            flag: 0,
            x: 0,
            y: 0,
            set_x: 0,
            set_y: 0,

            item_position: 0,

            potion_count: 0,
            not_use_flag: 0,
            sell_price: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ItemPrice {
    pub pure_sell_price: i32,
    pub sell_price: i32,
    pub repair_cost: i32,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ItemRequire {
    pub level: i32,
    pub strength: i32,
    pub spirit: i32,
    pub talent: i32,
    pub dexterity: i32,
    pub health: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ItemRow {
    pub code: ItemCode,
    pub w: i32,
    pub h: i32,
    pub class: ItemClass,
    pub model_posi: Option<InventoryPosition>,
    pub weapon_class: Option<ItemWeaponClass>,
}
