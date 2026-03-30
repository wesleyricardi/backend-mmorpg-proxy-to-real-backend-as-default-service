use super::char::SelectionChar;
use socket::socket;

#[socket]
#[derive(Debug, Clone)]
pub struct RequestUserInfo {
    pub l_param: i32,
    pub w_param: i32,
    pub s_param: i32,
    #[socket(string_size = 32)]
    pub id: String,
    #[socket(string_size = 32)]
    pub password: String,
    #[socket(string_size = 20)]
    pub mac: String,
    #[socket(string_size = 260)]
    pub path: String,
    #[socket(string_size = 16)]
    pub pc_name: String,
}

#[socket]
#[derive(Debug, Clone)]
pub struct UserPlayerInfo {
    #[socket(string_size = 32)]
    pub id: String,
    pub player_use_count: i32,
    pub char_info: [SelectionChar; 6],
}

socket::protocol_module! {
    GetUserInfo => RequestUserInfo = 0x48470085,
    SetUserInfo => UserPlayerInfo = 0x48470086,
}
