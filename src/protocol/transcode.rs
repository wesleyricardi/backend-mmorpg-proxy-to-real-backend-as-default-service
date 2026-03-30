use socket::socket;

use super::{
    char::Char,
    chat::ChatMessage,
    command::{
        CancelSkillRuntimeCommand, CharCommand, Command, CommandEx, Ping, PlayerCommand,
        ProcessSkillRuntimeCommand,
    },
    data::ClientFuncPos,
    item::Item,
    item::ItemInfo,
    job_code::RuntimeJobCode,
};

const SKIL_ATTACK_CHAR_MAX: usize = 42;
const PARTY_PLAYER_MAX: usize = 6;
const TRADE_RECV_ITEMS_MAX: usize = 32;
const TRANS_TRADE_BUFF_SIZE: usize = 4000;
const WAREHOUSE_DATA_SIZE: usize = 8192;

#[socket]
#[derive(Debug, Clone)]
pub struct RsPlayPos {
    pub area: u32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[socket]
#[derive(Debug, Clone)]
pub struct TransPlayPos {
    pub play_pos: RsPlayPos,
    pub hp: [i32; 2],
}

#[socket]
#[derive(Debug, Clone)]
pub struct MotionState {
    pub motion_state: i32,
}

#[socket]
#[derive(Debug, Clone)]
pub struct TransPlayData {
    pub object_serial: u32,
    pub target: u32,
    pub play_buff_count: i32,
    pub start_posi: i32,
    pub hp: [i32; 2],
    pub auto_char_code: u32,
    pub area: [i16; 2],
    pub update_info: [u8; 4],
    pub event_info: [u8; 4],
}

#[socket]
#[derive(Debug, Clone)]
pub struct TransAttackData {
    pub dest_object_serial: u32,
    pub target_object_serial: u32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub attack_state: i32,
    pub attack_size: i32,
    pub power: i32,
    pub checksum: u32,
    pub rating: [i16; 2],
    pub damage_checksum: u32,
    pub critical: bool,
}

#[socket]
#[derive(Debug, Clone)]
pub struct TransAttackData2 {
    pub checksum: u32,
    pub dest_object_serial: u32,
    pub target_object_serial: u32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub attack_state: i32,
    pub attack_size: i32,
    pub power: [i16; 2],
    pub power2: [i16; 2],
    pub critical: [i16; 2],
    pub skill_code: i32,
    pub time: u32,
    pub attack_count: i32,
    pub motion_count: [i16; 2],
    pub weapon_code: u32,
    pub area: [i16; 2],
    pub temp: [i32; 4],
}

#[socket]
#[derive(Debug, Clone)]
pub struct TransSkillAttackData {
    pub dest_object_serial: u32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub attack_state: i32,
    pub attack_size: i32,
    pub power: i32,
    pub checksum: u32,
    pub target_count: i32,
    pub target_object_serial: [u32; SKIL_ATTACK_CHAR_MAX],
}

#[socket]
#[derive(Debug, Clone)]
pub struct TransSkillAttackData2 {
    pub dest_object_serial: u32,
    pub checksum: u32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub attack_state: i32,
    pub attack_size: i32,
    pub power: [i16; 2],
    pub power2: [i16; 2],
    pub critical: [i16; 2],
    pub skill_code: i32,
    pub time: u32,
    pub attack_count: i32,
    pub motion_count: [i16; 2],
    pub weapon_code: u32,
    pub area: [i16; 2],
    pub temp: [i32; 4],
    pub main_target_object: u32,
    pub target_count: i32,
    pub target_object_serial: [u32; SKIL_ATTACK_CHAR_MAX],
}

#[socket]
#[derive(Debug, Clone)]
pub struct TransPlayerInfo {
    pub char_info: Char,
    pub object_serial: u32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub ax: i32,
    pub ay: i32,
    pub az: i32,
    pub state: i32,
}

#[socket]
#[derive(Debug, Clone)]
pub struct CommandDword {
    pub l_param: u32,
    pub w_param: u32,
    pub s_param: u32,
    pub e_param: u32,
}

#[socket]
#[derive(Debug, Clone)]
pub struct UsingItem {
    pub code: u32,
    pub performance: [i16; 8],
}

#[socket]
#[derive(Debug, Clone)]
pub struct TransUsingItem {
    pub item_list_count: i32,
    pub item_list: [UsingItem; 16],
}

#[socket]
#[derive(Debug, Clone)]
pub struct PartyPlayInfo {
    pub object_serial: u32,
    pub level: u32,
    pub job_code: RuntimeJobCode,
    pub life: [i32; 2],
    pub x: i32,
    pub z: i32,
}

#[socket]
#[derive(Debug, Clone)]
pub struct TransPartyPlayInfo {
    pub object_serial: u32,
    pub temp: u32,
    pub party_user_count: i32,
    pub play_info: [PartyPlayInfo; PARTY_PLAYER_MAX],
}

#[socket]
#[derive(Debug, Clone)]
pub struct TransPartySkill {
    pub skill_code: u32,
    pub point: i32,
    pub w_param: i32,
    pub l_param: i32,
    pub s_param: i32,
    pub e_param: i32,
    pub party_count: i32,
    pub party_user: [u32; 8],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PartySkillWireParams {
    pub w_param: i32,
    pub l_param: i32,
    pub s_param: i32,
    pub e_param: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartySkillWireTargets {
    pub object_serials: Vec<u32>,
}

impl TransPartySkill {
    pub fn wire_params(&self) -> PartySkillWireParams {
        PartySkillWireParams {
            w_param: self.w_param,
            l_param: self.l_param,
            s_param: self.s_param,
            e_param: self.e_param,
        }
    }

    pub fn active_targets(&self) -> PartySkillWireTargets {
        let count = self.party_count.clamp(0, self.party_user.len() as i32) as usize;
        PartySkillWireTargets {
            object_serials: self.party_user[..count]
                .iter()
                .copied()
                .filter(|object_serial| *object_serial != 0)
                .collect(),
        }
    }
}

#[socket]
#[derive(Debug, Clone)]
pub struct TransResistance {
    pub param: i32,
    pub resistance: [i16; 8],
    pub absorb: i32,
}

#[socket]
#[derive(Debug, Clone)]
pub struct TransActionItem {
    pub state: i32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub stg_area: u32,
}

#[socket]
#[derive(Debug, Clone)]
pub struct TransTradeItems {
    pub sender: u32,
    pub receiver: u32,
    pub sum: u32,
    pub temp: [u32; 4],
    pub trade_buffer: [u8; TRANS_TRADE_BUFF_SIZE],
}

#[socket]
#[derive(Debug, Clone)]
pub struct TransWarehouse {
    pub checksum: u32,
    pub version: [u16; 2],
    pub warehouse_money: i32,
    pub user_money: i32,
    pub temp: [u32; 5],
    pub data_size: i32,
    pub data: [u8; WAREHOUSE_DATA_SIZE],
}

#[socket]
#[derive(Debug, Clone)]
pub struct RecordTradeItem {
    pub code: u32,
    pub head: u32,
    pub checksum: u32,
    pub state: i32,
}

#[socket]
#[derive(Debug, Clone)]
pub struct TransRecordTradeItems {
    pub money: i32,
    pub throw_item_count: i32,
    pub trade_user_name: [u8; 32],
    pub temp: [i32; 4],
    pub items: [RecordTradeItem; TRADE_RECV_ITEMS_MAX],
}

#[socket]
#[derive(Debug, Clone)]
pub struct ServerMoney {
    pub input_money: i32,
    pub counter: i32,
    pub total_x: i32,
    pub total_y: i32,
    pub total_z: i32,
}

#[socket]
#[derive(Debug, Clone)]
pub struct ServerExp {
    pub counter: i32,
    pub input_exp: i32,
    pub total1: i32,
    pub total2: i32,
}

#[socket]
#[derive(Debug, Clone)]
pub struct TransTotalExpMoney {
    pub server_money: ServerMoney,
    pub server_exp: ServerExp,
}

#[socket]
#[derive(Debug, Clone)]
pub struct ItemServer {
    pub code: u32,
    pub item_name_index: [u8; 32],
    pub last_category: [u8; 16],
    pub w: i32,
    pub h: i32,
    pub item_file_path: [u8; 64],
    pub class: u32,
    pub drop_item: [u8; 64],
    pub set_model_posi: u32,
    pub sound_index: i32,
    pub weapon_class: i32,
    pub flag: i32,
    pub x: i32,
    pub y: i32,
    pub set_x: i32,
    pub set_y: i32,
    pub item_ptr: u32,
    pub item_position: i32,
    pub potion_count: i32,
    pub not_use_flag: i32,
    pub sell_price: i32,
    pub old_x: i32,
    pub old_y: i32,
    pub temp_item_ptr: u32,
    pub item_info: Item,
}

#[socket]
#[derive(Debug, Clone)]
pub struct PacketCoinShopBuy {
    pub code: [u8; 32],
    pub spec: i32,
    pub count: i32,
}

#[socket]
#[derive(Debug, Clone)]
pub struct PacketSendItem {
    pub spec: i32,
    pub weapon: i32,
    pub result: i32,
    pub item: ItemServer,
}

#[socket]
#[derive(Debug, Clone)]
pub struct PacketUpdateItem {
    pub item: [ItemServer; 2],
}

#[socket]
#[derive(Debug, Clone)]
pub struct CraftItemServer {
    pub doc_index: i32,
    pub result: i32,
    pub item: ItemServer,
    pub sheltom_code: [u32; 12],
    pub head: [u32; 12],
    pub checksum: [u32; 12],
    pub index: i32,
    pub money: i32,
    pub stone_item_code: u32,
    pub stone_head: u32,
    pub stone_checksum: u32,
}

#[socket]
#[derive(Debug, Clone)]
pub struct MixtureResetItemServer {
    pub doc_index: i32,
    pub result: i32,
    pub index: i32,
    pub item: ItemServer,
    pub stone_item_code: u32,
    pub stone_head: u32,
    pub stone_checksum: u32,
}

#[socket]
#[derive(Debug, Clone)]
pub struct RemodelItemServer {
    pub doc_index: i32,
    pub result: i32,
    pub index: i32,
    pub item: ItemServer,
    pub stone_item_code: u32,
    pub stone_head: u32,
    pub stone_checksum: u32,
}

#[socket]
#[derive(Debug, Clone)]
pub struct SmeltingItemServer {
    pub doc_index: i32,
    pub result: i32,
    pub item: ItemServer,
    pub smelting_code: [u32; 5],
    pub head: [u32; 5],
    pub checksum: [u32; 5],
    pub index: i32,
    pub money: i32,
    pub stone_item_code: u32,
    pub stone_head: u32,
    pub stone_checksum: u32,
}

#[socket]
#[derive(Debug, Clone)]
pub struct ManufactureItemServer {
    pub doc_index: i32,
    pub result: i32,
    pub item: ItemServer,
    pub rune_code: [u32; 4],
    pub head: [u32; 4],
    pub checksum: [u32; 4],
    pub index: i32,
    pub money: i32,
    pub stone_item_code: u32,
    pub stone_head: u32,
    pub stone_checksum: u32,
}

#[socket]
#[derive(Debug, Clone)]
pub struct TransBuyShopItem {
    pub item: ItemServer,
    pub item_count: i32,
}

#[socket]
#[derive(Debug, Clone)]
pub struct TransClanCommandUser {
    pub param: [i32; 4],
    pub user_count: i32,
    pub user_buff: [u8; 2048],
}

socket::protocol_module! {
    Ping => Ping = 0x49000000,
    MotionState => MotionState = 0x4E425301u32,
    CombatInfo => Command = 0xa0000003u32,
    RemodelItem => RemodelItemServer = 0x60000001,
    ProgressQuest => Command = 0x70000000,
    CancelQuest => Command = 0x70000001,
    RecvBuyCoinShop => PacketCoinShopBuy = 0x49470005,
    RecvBuyTimeShop => PacketCoinShopBuy = 0x49470008,
    PremiumCancel => Command = 0x49000009,
    ShowiteminfoPerf => Command = 0x4900000c,
    PvpMode => Command = 0x49000015,
    SendChangeItem => PacketSendItem = 0x49000022,
    ChecksumSkill => Command = 0x49000034,
    AwardItem => Command = 0x49000035,
    RouletteItem => Command = 0x49000037,
    UpdateItem => PacketUpdateItem = 0x49000041,
    Getrecorddata => CharCommand = 0x48470082,
    IdGetuserinfo => PlayerCommand = 0x48470085,
    Insrecorddata => PlayerCommand = 0x48470088,
    Delrecorddata => PlayerCommand = 0x48470087,
    Isrecorddata => PlayerCommand = 0x48470089,
    CheckNetstate => Command = 0x4847008b,
    SetObjserial => Command = 0x4847008d,
    CheckNetstate2 => Command = 0x4847008e,
    CheckNetstate5 => Command = 0x48470091,
    Position => TransPlayPos = 0x48471002,
    Transplaydata => TransPlayData = 0x48470013,
    Attackdata => TransAttackData = 0x48470030,
    Playerinfo2 => TransPlayerInfo = 0x48470032,
    SetItemcopylist => Command = 0x48470034,
    SkilAttackdata => TransSkillAttackData = 0x48470035,
    UpdateCinfo => Command = 0x48470038,
    Openmonster => Command = 0x48470039,
    Tradeitems => TransTradeItems = 0x48470041,
    TradeRequst => Command = 0x48470042,
    TradeItemkey => TransTradeItems = 0x48470043,
    TradeReadyitem => TransTradeItems = 0x60000002,
    Warehouse => TransWarehouse = 0x48470047,
    Collectmoney => Command = 0x4847004b,
    SkilAttackuser => TransSkillAttackData = 0x4847004e,
    Playerinfo => TransPlayerInfo = 0x48470020,
    Putitem => TransActionItem = 0x8c000c8bu32,
    Throwitem => ItemInfo = 0x8a000b4au32,
    Savethrowmoney => Command = 0x48470056,
    UseitemCode => CommandEx = 0x4847005b,
    CommandUser => CommandEx = 0x4847005c,
    PaymentMoney => Command = 0x4847005e,
    AddStartposi => Command = 0x48470060,
    DelStartposi => Command = 0x48470061,
    AddNpc => TransPlayerInfo = 0x48470070,
    DelNpc => TransPlayerInfo = 0x48470071,
    PartyJoin => Command = 0x484700a2,
    PartyPlayupdate => TransPartyPlayInfo = 0x484700a3,
    PartyCommand => Command = 0x484700a5,
    PcbangPet => Command = 0x484700b0,
    PhenixPet => Command = 0x484700b1,
    HelpPet => Command = 0x484700b2,
    Healing => Command = 0x484700d2,
    Holymind => Command = 0x484700d3,
    Wisp => Command = 0x484700d9,
    GrandHealing => Command = 0x484700d4,
    RecordTrade => TransRecordTradeItems = 0x48480010,
    Processinfo => ChatMessage = 0x484700e4,
    Checkip => CharCommand = 0x484700e7,
    ContObjserial => Command = 0x48478010,
    FunctionMem2 => Command = 0x48478300,
    SetBlacklist3 => Command = 0x48478500,
    BadPlayer => Command = 0x48478600,
    FixAttack => Command = 0x48478700,
    ClientError => Command = 0x48478900,
    InvenErrItem => Command = 0x48478910,
    InvenErrMoney => Command = 0x48478920,
    InvenPosition => TransUsingItem = 0x48478930,
    InvenPosition2 => TransUsingItem = 0x48478931,
    Starpoint => Command = 0x48478a12,
    Givemoney => Command = 0x48478a16,
    Clanmoney => Command = 0x48478a18,
    ItemExpress => Command = 0x48478a80,
    OpenMyshop => ChatMessage = 0x48478a90,
    CallMyshop => Command = 0x48478aa0,
    MyshopItem => TransTradeItems = 0x48478ab0,
    MyshopMessage => Command = 0x48478ac0,
    MyshopTrade => Command = 0x48478ad0,
    Adminmode2 => Command = 0x5047108c,
    CheckExpmoney => TransTotalExpMoney = 0x50320010,
    CheckExpdata => Command = 0x50320020,
    ForceorbData => Command = 0x50320030,
    BoosterData => Command = 0x50320034,
    SkilldelayData => Command = 0x40a0500c,
    Craftitem => CraftItemServer = 0x6000800b,
    Agingitem => CraftItemServer = 0x600900a0,
    AgingUpgrade => ItemInfo = 0x600400c0,
    MakeLinkcore => ItemInfo = 0x50320205,
    UseLinkcore => ItemInfo = 0x50320206,
    Wingitem => CraftItemServer = 0x50320208,
    Shoptitem => TransBuyShopItem = 0x600810c0,
    ShopSellitem => ItemInfo = 0x609a10c0,
    QuestCommand => Command = 0x50320220,
    Checkitem => ItemInfo = 0x50320300,
    CheckItemmesh => Command = 0x50320330,
    ClientFuncpos => ClientFuncPos = 0x50320400,
    ProcessTimemax => Command = 0x50320500,
    Nsprite => CommandDword = 0x50320600,
    RecDamagedata => Command = 0x50320800,
    SodResult => CommandEx = 0x50320900,
    PartySkill => TransPartySkill = 0x50320a00,
    CancelSkill => CancelSkillRuntimeCommand = 0x50320a10,
    ProcessSkill => ProcessSkillRuntimeCommand = 0x50320a20,
    ProcessSkill2 => ProcessSkillRuntimeCommand = 0x50320a30,
    ProcessClanSkill => Command = 0x50320a35,
    UpdatelSkill => Command = 0x50320a40,
    Resistance => TransResistance = 0x50320a60,
    PublicPolling => Command = 0x50320a70,
    Hacktrap => Command = 0x50320a90,
    ClanService => PlayerCommand = 0x50320c00,
    ClanUpdate => TransClanCommandUser = 0x50320c10,
    DeadUser => Command = 0x50320e00,
    YahooMotion => Command = 0x50320e10,
    Clientinfo => Command = 0x50321000,
    WarningBlinkAtt => Command = 0x50321010,
    AttackDamage => TransAttackData2 = 0x709f00c0,
    RangeDamage => TransSkillAttackData2 = 0x70010a01,
    LimitDamage => Command = 0x50322050,
    WarningClient => Command = 0x50322060,
    RecAdminCommand => ChatMessage = 0x50322090,
    BlesscastleInfo => Command = 0x503220c0,
    BlesscastleTax => CommandEx = 0x503220d0,
    UpdateServerParam => Command = 0x503220f0,
    UserId => PlayerCommand = 0x50322100,
    Smeltingitem => SmeltingItemServer = 0x2b010005,
    Manufacture => ManufactureItemServer = 0x20c1000a,
    PremiumitemInit => Command = 0x50326004,
    MixtureResetItem => MixtureResetItemServer = 0x30a1000c,
    EventGame => Command = 0x50326007,
    Itemdoc => ChatMessage = 0x50326008,
}
