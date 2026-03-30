use socket::socket;

#[socket]
#[derive(Debug, Clone)]
pub struct PacketItemClass {
    pub weapon: i32,
    pub armor: i32,
    pub shield: i32,
    pub bracelets: i32,
    pub gauntlets: i32,
    pub boots: i32,
    pub ring: i32,
    pub amy: i32,
    pub shel: i32,
    pub time: u32,
}

socket::protocol_module! {
    ItemClass => PacketItemClass = 0x49000023,
}
