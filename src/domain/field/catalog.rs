use super::dto::{FieldId, FieldPolicy, FieldStateKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldCatalogEntry {
    pub field_id: FieldId,
    pub name: &'static str,
    pub state_kind: FieldStateKind,
    pub server_code: Option<i32>,
    pub limit_level: Option<i32>,
    pub gate_count: i32,
    pub warp_gate_count: i32,
    pub start_point_count: i32,
    pub policy: FieldPolicy,
}

macro_rules! field_row {
    ($id:expr, $name:expr, $state:expr, $server:expr, $limit:expr, $gate:expr, $warp:expr, $start:expr) => {
        FieldCatalogEntry {
            field_id: $id,
            name: $name,
            state_kind: $state,
            server_code: $server,
            limit_level: $limit,
            gate_count: $gate,
            warp_gate_count: $warp,
            start_point_count: $start,
            policy: FieldPolicy {
                can_use_skill: true,
                is_safe_zone: false,
                pvp_allowed: false,
                can_trade: true,
                can_drop: true,
                can_open_shop: false,
            },
        }
    };
}

const FIELD_FORE3: FieldCatalogEntry = field_row!(
    FieldId::Fore3,
    "fore-3",
    FieldStateKind::Forest,
    Some(1),
    None,
    1,
    1,
    2
);
const FIELD_FORE2: FieldCatalogEntry = field_row!(
    FieldId::Fore2,
    "fore-2",
    FieldStateKind::Forest,
    Some(1),
    None,
    1,
    1,
    2
);
const FIELD_FORE1: FieldCatalogEntry = field_row!(
    FieldId::Fore1,
    "fore-1",
    FieldStateKind::Forest,
    Some(1),
    None,
    2,
    0,
    3
);
const FIELD_VILLAGE2: FieldCatalogEntry = FieldCatalogEntry {
    field_id: FieldId::Village2,
    name: "village-2",
    state_kind: FieldStateKind::Village,
    server_code: Some(1),
    limit_level: None,
    gate_count: 0,
    warp_gate_count: 1,
    start_point_count: 4,
    policy: FieldPolicy {
        can_use_skill: false,
        is_safe_zone: true,
        pvp_allowed: false,
        can_trade: true,
        can_drop: true,
        can_open_shop: true,
    },
};
const FIELD_RUIN4: FieldCatalogEntry = field_row!(
    FieldId::Ruin4,
    "ruin-4",
    FieldStateKind::Ruin,
    Some(1),
    None,
    1,
    0,
    2
);
const FIELD_RUIN3: FieldCatalogEntry = field_row!(
    FieldId::Ruin3,
    "ruin-3",
    FieldStateKind::Ruin,
    Some(1),
    None,
    1,
    0,
    2
);
const FIELD_RUIN2: FieldCatalogEntry = field_row!(
    FieldId::Ruin2,
    "ruin-2",
    FieldStateKind::Ruin,
    Some(1),
    None,
    3,
    1,
    3
);
const FIELD_RUIN1: FieldCatalogEntry = field_row!(
    FieldId::Ruin1,
    "ruin-1",
    FieldStateKind::Ruin,
    Some(1),
    None,
    1,
    1,
    2
);
const FIELD_DE1: FieldCatalogEntry = field_row!(
    FieldId::De1,
    "de-1",
    FieldStateKind::Desert,
    Some(1),
    None,
    2,
    0,
    2
);
const FIELD_VILLAGE1: FieldCatalogEntry = field_row!(
    FieldId::Village1,
    "village-1",
    FieldStateKind::Desert,
    Some(1),
    None,
    2,
    1,
    1
);
const FIELD_DE2: FieldCatalogEntry = field_row!(
    FieldId::De2,
    "de-2",
    FieldStateKind::Desert,
    Some(1),
    None,
    0,
    0,
    2
);
const FIELD_DE3: FieldCatalogEntry = field_row!(
    FieldId::De3,
    "de-3",
    FieldStateKind::Desert,
    Some(3),
    Some(55),
    1,
    1,
    2
);
const FIELD_DE4: FieldCatalogEntry = field_row!(
    FieldId::De4,
    "de-4",
    FieldStateKind::Desert,
    Some(3),
    Some(55),
    1,
    1,
    2
);
const FIELD_DUN1: FieldCatalogEntry = field_row!(
    FieldId::Dun1,
    "dun-1",
    FieldStateKind::Dungeon,
    Some(3),
    Some(40),
    0,
    2,
    0
);
const FIELD_DUN2: FieldCatalogEntry = field_row!(
    FieldId::Dun2,
    "dun-2",
    FieldStateKind::Dungeon,
    Some(3),
    Some(50),
    0,
    3,
    0
);
const FIELD_DUN3: FieldCatalogEntry = field_row!(
    FieldId::Dun3,
    "dun-3",
    FieldStateKind::Dungeon,
    Some(3),
    None,
    0,
    1,
    0
);
const FIELD_OFFICE: FieldCatalogEntry = field_row!(
    FieldId::Office,
    "office",
    FieldStateKind::Room,
    Some(1),
    Some(1000),
    0,
    0,
    0
);
const FIELD_FOREVER_FALL_04: FieldCatalogEntry = field_row!(
    FieldId::ForeverFall04,
    "forever-fall-04",
    FieldStateKind::Forest,
    Some(3),
    Some(0),
    1,
    0,
    2
);
const FIELD_FOREVER_FALL_03: FieldCatalogEntry = field_row!(
    FieldId::ForeverFall03,
    "forever-fall-03",
    FieldStateKind::Forest,
    Some(3),
    Some(0),
    1,
    2,
    2
);
const FIELD_FOREVER_FALL_02: FieldCatalogEntry = field_row!(
    FieldId::ForeverFall02,
    "forever-fall-02",
    FieldStateKind::Forest,
    Some(3),
    Some(0),
    1,
    0,
    3
);
const FIELD_FOREVER_FALL_01: FieldCatalogEntry = field_row!(
    FieldId::ForeverFall01,
    "forever-fall-01",
    FieldStateKind::Forest,
    Some(3),
    None,
    1,
    1,
    2
);
const FIELD_PILAI: FieldCatalogEntry = FieldCatalogEntry {
    field_id: FieldId::Pilai,
    name: "pilai",
    state_kind: FieldStateKind::Village,
    server_code: Some(1),
    limit_level: None,
    gate_count: 0,
    warp_gate_count: 2,
    start_point_count: 2,
    policy: FieldPolicy {
        can_use_skill: false,
        is_safe_zone: true,
        pvp_allowed: false,
        can_trade: true,
        can_drop: true,
        can_open_shop: true,
    },
};
const FIELD_DUN4: FieldCatalogEntry = field_row!(
    FieldId::Dun4,
    "dun-4",
    FieldStateKind::Dungeon,
    Some(3),
    None,
    0,
    2,
    0
);
const FIELD_DUN5: FieldCatalogEntry = field_row!(
    FieldId::Dun5,
    "dun-5",
    FieldStateKind::Dungeon,
    Some(3),
    None,
    0,
    5,
    0
);
const FIELD_TCAVE: FieldCatalogEntry = field_row!(
    FieldId::TCave,
    "tcave",
    FieldStateKind::Dungeon,
    Some(1),
    None,
    0,
    2,
    2
);
const FIELD_MCAVE: FieldCatalogEntry = field_row!(
    FieldId::MCave,
    "mcave",
    FieldStateKind::Dungeon,
    Some(1),
    None,
    0,
    2,
    2
);
const FIELD_DCAVE: FieldCatalogEntry = field_row!(
    FieldId::DCave,
    "dcave",
    FieldStateKind::Dungeon,
    Some(3),
    None,
    0,
    2,
    0
);
const FIELD_IRON1: FieldCatalogEntry = field_row!(
    FieldId::Iron1,
    "iron-1",
    FieldStateKind::Iron,
    Some(3),
    Some(80),
    1,
    0,
    3
);
const FIELD_IRON2: FieldCatalogEntry = field_row!(
    FieldId::Iron2,
    "iron-2",
    FieldStateKind::Iron,
    Some(3),
    Some(85),
    1,
    0,
    2
);
const FIELD_ICE_URA: FieldCatalogEntry = field_row!(
    FieldId::IceUra,
    "ice_ura",
    FieldStateKind::Ice,
    Some(3),
    Some(90),
    1,
    1,
    0
);
const FIELD_SOD1: FieldCatalogEntry = field_row!(
    FieldId::Sod1,
    "sod-1",
    FieldStateKind::Ruin,
    Some(3),
    None,
    0,
    0,
    0
);
const FIELD_ICE1: FieldCatalogEntry = field_row!(
    FieldId::Ice1,
    "ice1",
    FieldStateKind::Ice,
    Some(3),
    Some(100),
    1,
    0,
    0
);
const FIELD_QUEST_IV: FieldCatalogEntry = field_row!(
    FieldId::QuestIv,
    "quest_iv",
    FieldStateKind::QuestZone,
    Some(3),
    None,
    0,
    0,
    0
);
const FIELD_CASTLE: FieldCatalogEntry = FieldCatalogEntry {
    field_id: FieldId::Castle,
    name: "castle",
    state_kind: FieldStateKind::Castle,
    server_code: Some(3),
    limit_level: None,
    gate_count: 0,
    warp_gate_count: 1,
    start_point_count: 3,
    policy: FieldPolicy {
        can_use_skill: true,
        is_safe_zone: false,
        pvp_allowed: true,
        can_trade: true,
        can_drop: true,
        can_open_shop: false,
    },
};
const FIELD_GREEDY: FieldCatalogEntry = field_row!(
    FieldId::Greedy,
    "greedy",
    FieldStateKind::Ice,
    Some(1),
    Some(70),
    0,
    0,
    0
);
const FIELD_ICE2: FieldCatalogEntry = field_row!(
    FieldId::Ice2,
    "ice2",
    FieldStateKind::Ice,
    Some(3),
    Some(100),
    1,
    1,
    0
);
const FIELD_BOSS: FieldCatalogEntry = field_row!(
    FieldId::Boss,
    "boss",
    FieldStateKind::Dungeon,
    Some(3),
    None,
    0,
    1,
    0
);
const FIELD_LOST: FieldCatalogEntry = field_row!(
    FieldId::Lost,
    "lost",
    FieldStateKind::Ruin,
    Some(3),
    Some(102),
    1,
    1,
    0
);
const FIELD_LOST_TEMPLE: FieldCatalogEntry = field_row!(
    FieldId::LostTemple,
    "losttemple",
    FieldStateKind::Ruin,
    Some(3),
    Some(105),
    0,
    0,
    0
);
const FIELD_FALL_GAME: FieldCatalogEntry = field_row!(
    FieldId::FallGame,
    "fall_game",
    FieldStateKind::Action,
    Some(1),
    None,
    0,
    4,
    0
);
const FIELD_DUN7: FieldCatalogEntry = field_row!(
    FieldId::Dun7,
    "dun-7",
    FieldStateKind::Dungeon,
    Some(3),
    Some(105),
    0,
    2,
    0
);
const FIELD_DUN8: FieldCatalogEntry = field_row!(
    FieldId::Dun8,
    "dun-8",
    FieldStateKind::Dungeon,
    Some(3),
    Some(108),
    0,
    2,
    0
);
const FIELD_DUN6: FieldCatalogEntry = field_row!(
    FieldId::Dun6,
    "dun-6",
    FieldStateKind::Dungeon,
    Some(3),
    Some(80),
    0,
    2,
    0
);
const FIELD_DUN9: FieldCatalogEntry = field_row!(
    FieldId::Dun9,
    "dun-9",
    FieldStateKind::Dungeon,
    Some(3),
    Some(105),
    0,
    1,
    2
);
const FIELD_MINE1: FieldCatalogEntry = field_row!(
    FieldId::Mine1,
    "mine-1",
    FieldStateKind::Dungeon,
    Some(3),
    Some(108),
    0,
    1,
    0
);
const FIELD_SLAB: FieldCatalogEntry = field_row!(
    FieldId::Slab,
    "slab",
    FieldStateKind::Dungeon,
    Some(3),
    Some(120),
    0,
    2,
    0
);
const FIELD_ANCIENT_W: FieldCatalogEntry = field_row!(
    FieldId::AncientW,
    "ancientw",
    FieldStateKind::Dungeon,
    Some(3),
    None,
    0,
    0,
    1
);
const FIELD_CONDADO_PECADOR: FieldCatalogEntry = field_row!(
    FieldId::CondadoPecador,
    "condado-pecador",
    FieldStateKind::Dungeon,
    Some(3),
    None,
    0,
    0,
    1
);
const FIELD_DC1: FieldCatalogEntry = field_row!(
    FieldId::Dc1,
    "dc1",
    FieldStateKind::Dungeon,
    Some(3),
    Some(110),
    0,
    1,
    1
);
const FIELD_BA1: FieldCatalogEntry = field_row!(
    FieldId::Ba1,
    "ba1",
    FieldStateKind::Ruin,
    Some(1),
    None,
    0,
    0,
    0
);
const FIELD_HEART_OF_FIRE: FieldCatalogEntry = field_row!(
    FieldId::HeartOfFire,
    "heartoffire",
    FieldStateKind::Dungeon,
    Some(1),
    Some(115),
    0,
    0,
    0
);
const FIELD_TEMPLE2: FieldCatalogEntry = field_row!(
    FieldId::Temple2,
    "temple2",
    FieldStateKind::Ruin,
    Some(3),
    Some(130),
    0,
    0,
    1
);
const FIELD_AW_FL: FieldCatalogEntry = field_row!(
    FieldId::AwFl,
    "aw_fl",
    FieldStateKind::Ice,
    Some(1),
    Some(140),
    0,
    0,
    1
);
const FIELD_CRYSTAL_NEST: FieldCatalogEntry = field_row!(
    FieldId::CrystalNest,
    "crystalnest",
    FieldStateKind::Dungeon,
    Some(3),
    Some(120),
    0,
    0,
    1
);
const FIELD_LAND_OF_NURWN: FieldCatalogEntry = field_row!(
    FieldId::LandOfNurwn,
    "landofnurwn",
    FieldStateKind::Ice,
    Some(3),
    Some(125),
    0,
    0,
    2
);
const FIELD_ARENA: FieldCatalogEntry = field_row!(
    FieldId::Arena,
    "arena",
    FieldStateKind::Iron,
    Some(3),
    Some(100),
    0,
    0,
    0
);
const FIELD_COLISEU: FieldCatalogEntry = field_row!(
    FieldId::Coliseu,
    "coliseu",
    FieldStateKind::Desert,
    Some(3),
    Some(100),
    0,
    0,
    0
);
const FIELD_HAPPY_PT: FieldCatalogEntry = FieldCatalogEntry {
    field_id: FieldId::HappyPt,
    name: "happypt",
    state_kind: FieldStateKind::Castle,
    server_code: Some(3),
    limit_level: Some(130),
    gate_count: 0,
    warp_gate_count: 2,
    start_point_count: 1,
    policy: FieldPolicy {
        can_use_skill: true,
        is_safe_zone: false,
        pvp_allowed: true,
        can_trade: true,
        can_drop: true,
        can_open_shop: false,
    },
};
const FIELD_HAPPY_PT1: FieldCatalogEntry = FieldCatalogEntry {
    field_id: FieldId::HappyPt1,
    name: "happypt1",
    state_kind: FieldStateKind::Castle,
    server_code: Some(3),
    limit_level: Some(135),
    gate_count: 0,
    warp_gate_count: 2,
    start_point_count: 1,
    policy: FieldPolicy {
        can_use_skill: true,
        is_safe_zone: false,
        pvp_allowed: true,
        can_trade: true,
        can_drop: true,
        can_open_shop: false,
    },
};

impl FieldId {
    pub fn get_catalog_entry(&self) -> &'static FieldCatalogEntry {
        match self {
            FieldId::Fore3 => &FIELD_FORE3,
            FieldId::Fore2 => &FIELD_FORE2,
            FieldId::Fore1 => &FIELD_FORE1,
            FieldId::Village2 => &FIELD_VILLAGE2,
            FieldId::Ruin4 => &FIELD_RUIN4,
            FieldId::Ruin3 => &FIELD_RUIN3,
            FieldId::Ruin2 => &FIELD_RUIN2,
            FieldId::Ruin1 => &FIELD_RUIN1,
            FieldId::De1 => &FIELD_DE1,
            FieldId::Village1 => &FIELD_VILLAGE1,
            FieldId::De2 => &FIELD_DE2,
            FieldId::De3 => &FIELD_DE3,
            FieldId::De4 => &FIELD_DE4,
            FieldId::Dun1 => &FIELD_DUN1,
            FieldId::Dun2 => &FIELD_DUN2,
            FieldId::Dun3 => &FIELD_DUN3,
            FieldId::Office => &FIELD_OFFICE,
            FieldId::ForeverFall04 => &FIELD_FOREVER_FALL_04,
            FieldId::ForeverFall03 => &FIELD_FOREVER_FALL_03,
            FieldId::ForeverFall02 => &FIELD_FOREVER_FALL_02,
            FieldId::ForeverFall01 => &FIELD_FOREVER_FALL_01,
            FieldId::Pilai => &FIELD_PILAI,
            FieldId::Dun4 => &FIELD_DUN4,
            FieldId::Dun5 => &FIELD_DUN5,
            FieldId::TCave => &FIELD_TCAVE,
            FieldId::MCave => &FIELD_MCAVE,
            FieldId::DCave => &FIELD_DCAVE,
            FieldId::Iron1 => &FIELD_IRON1,
            FieldId::Iron2 => &FIELD_IRON2,
            FieldId::IceUra => &FIELD_ICE_URA,
            FieldId::Sod1 => &FIELD_SOD1,
            FieldId::Ice1 => &FIELD_ICE1,
            FieldId::QuestIv => &FIELD_QUEST_IV,
            FieldId::Castle => &FIELD_CASTLE,
            FieldId::Greedy => &FIELD_GREEDY,
            FieldId::Ice2 => &FIELD_ICE2,
            FieldId::Boss => &FIELD_BOSS,
            FieldId::Lost => &FIELD_LOST,
            FieldId::LostTemple => &FIELD_LOST_TEMPLE,
            FieldId::FallGame => &FIELD_FALL_GAME,
            FieldId::Dun7 => &FIELD_DUN7,
            FieldId::Dun8 => &FIELD_DUN8,
            FieldId::Dun6 => &FIELD_DUN6,
            FieldId::Dun9 => &FIELD_DUN9,
            FieldId::Mine1 => &FIELD_MINE1,
            FieldId::Slab => &FIELD_SLAB,
            FieldId::AncientW => &FIELD_ANCIENT_W,
            FieldId::CondadoPecador => &FIELD_CONDADO_PECADOR,
            FieldId::Dc1 => &FIELD_DC1,
            FieldId::Ba1 => &FIELD_BA1,
            FieldId::HeartOfFire => &FIELD_HEART_OF_FIRE,
            FieldId::Temple2 => &FIELD_TEMPLE2,
            FieldId::AwFl => &FIELD_AW_FL,
            FieldId::CrystalNest => &FIELD_CRYSTAL_NEST,
            FieldId::LandOfNurwn => &FIELD_LAND_OF_NURWN,
            FieldId::Arena => &FIELD_ARENA,
            FieldId::Coliseu => &FIELD_COLISEU,
            FieldId::HappyPt => &FIELD_HAPPY_PT,
            FieldId::HappyPt1 => &FIELD_HAPPY_PT1,
        }
    }
}
