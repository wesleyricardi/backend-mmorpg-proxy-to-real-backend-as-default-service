pub const NPC_MESSAGE_MAX: usize = 20;
pub const FALL_ITEM_MAX: usize = 200;
pub const FALL_ITEM2_MAX: usize = 3;
pub const _SIN_MAX_USE_SKILL: usize = 20;
pub const RECORD_ITEM_MAX: usize = 200;
pub const THROW_ITEM_INFO_MAX: usize = 64;
pub const LAST_QUEST_MAX: usize = 32;
/// Legacy base size used for `RecordDatas` wire compatibility when the
/// container is forwarded without `pub data: RecordData` payload bytes.
pub const RECORD_DATAS_BASE_C_SIZE: usize = 40;
