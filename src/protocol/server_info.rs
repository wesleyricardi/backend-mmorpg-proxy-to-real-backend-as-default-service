use socket::socket;

#[socket]
#[derive(Debug, Clone)]
pub struct ServerList {
    #[socket(string_size = 16)]
    pub server_name: String,
    pub server_time: u32,
    pub clan_ticket: i32,
    pub temp: [i32; 1],
    pub clan_count: i32,
    pub server_count: i32,
    pub servers_info: [ServerInfo; 2],
}

#[derive(Debug, Clone)]
#[socket]
pub struct ServerInfo {
    #[socket(string_size = 32)]
    pub name: String,
    #[socket(string_size = 20)]
    pub ip1: String,
    #[socket(string_size = 20)]
    pub ip2: String,
    #[socket(string_size = 20)]
    pub ip3: String,
    pub port1: u32,
    pub port2: u32,
    pub port3: u32,
    pub state: u32,
}

#[socket]
#[derive(Debug, Clone)]
pub struct AgingPercents {
    pub aging_ok_percent: [i32; 20],
    pub aging_stone_aging_ok_percent: [i32; 20],
    pub super_aging_stone_aging_fail_percent: [i32; 20],
    pub super_aging_stone1_5_aging_fail_percent: [i32; 20],
}

socket::protocol_module! {
    SetServerList => ServerList = 0x484700C0,
    SetAgingPercents => AgingPercents = 0x48470F11,
}
