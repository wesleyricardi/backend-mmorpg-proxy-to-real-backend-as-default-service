use socket::socket;

#[socket]
#[derive(Debug, Clone)]
pub struct BattleArenaInfo {
    pub flag: i32,
    pub end_time: i16,
}

#[socket]
#[derive(Debug, Clone)]
pub struct BattleArenaRanking {
    #[socket(string_size = 32)]
    pub name1: String,
    #[socket(string_size = 32)]
    pub name2: String,
    #[socket(string_size = 32)]
    pub name3: String,
    pub score: [i32; 3],
}

socket::protocol_module! {
    SetBattleArena => BattleArenaInfo = 0x49000029,
    SetBattleEvent => BattleArenaInfo = 0x49000032,
    HellGates => BattleArenaInfo = 0x49000039,
    SetBattleArenaRanking => BattleArenaRanking = 0x49000030,
    SetBattleEventRanking => BattleArenaRanking = 0x49000032,
}
