use socket::socket;

#[derive(Debug, Clone)]
#[socket]
pub struct CrackerName {
    #[socket(string_size = 12)]
    pub nam1: String,
    #[socket(string_size = 12)]
    pub name2: String,
    #[socket(string_size = 12)]
    pub class: String,
}

#[socket]
#[derive(Debug, Clone, Default)]
pub struct Cracker {
    pub find_counter: i32,
    #[socket(vec_size = 64)]
    pub crack_name: Vec<CrackerName>,
    #[socket(string_size = 128)]
    pub temp: String,
}

socket::protocol_module! {
    FindCracker => Cracker = 0x48470036,
}
