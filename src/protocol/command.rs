use socket::socket;

#[socket]
#[derive(Debug, Clone)]
pub struct Command {
    pub l_param: i32,
    pub w_param: i32,
    pub s_param: i32,
    pub e_param: i32,
}

#[socket]
#[derive(Debug, Clone)]
pub struct ProcessSkillRuntimeCommand {
    pub skill_code: i32,
    pub skill_point: i32,
    pub skill_flag: i32,
    pub element_index: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProcessSkillWireParams {
    pub l_param: i32,
    pub w_param: i32,
    pub s_param: i32,
    pub e_param: i32,
}

impl ProcessSkillRuntimeCommand {
    pub fn wire_params(&self) -> ProcessSkillWireParams {
        ProcessSkillWireParams {
            l_param: self.skill_code,
            w_param: self.skill_point,
            s_param: self.skill_flag,
            e_param: self.element_index,
        }
    }

    pub fn active_target_object_serial(&self) -> Option<u32> {
        let wire = self.wire_params();
        (wire.s_param > 0).then_some(wire.s_param as u32)
    }

    pub fn packed_point(&self) -> i32 {
        let wire = self.wire_params();
        let point = (wire.w_param >> 8) & 0xF;
        if point > 0 {
            point
        } else {
            wire.l_param
        }
    }
}

#[socket]
#[derive(Debug, Clone)]
pub struct CancelSkillRuntimeCommand {
    pub cancel_skill_code: i32,
    pub cancel_l_param: i32,
    pub cancel_s_param: i32,
    pub cancel_e_param: i32,
}

#[socket]
#[derive(Debug, Clone)]
pub struct Ping {
    pub time: u32,
}

#[socket]
#[derive(Debug, Clone)]
pub struct CommandEx {
    pub l_param: i32,
    pub w_param: i32,
    pub s_param: i32,
    pub e_param: i32,
    pub lx_param: i32,
    pub wx_param: i32,
    pub sx_param: i32,
    pub ex_param: i32,
}

#[socket]
#[derive(Debug, Clone)]
pub struct CharCommand {
    pub l_param: i32,
    pub w_param: i32,
    pub s_param: i32,
    #[socket(string_size = 32)]
    pub name: String,
}

#[socket]
#[derive(Debug, Clone)]
pub struct PlayerCommand {
    pub l_param: i32,
    pub w_param: i32,
    pub s_param: i32,
    #[socket(string_size = 32)]
    pub id: String,
    #[socket(string_size = 32)]
    pub char_name: String,
    #[socket(string_size = 20)]
    pub mac: String,
    #[socket(string_size = 260)]
    pub path: String,
    #[socket(string_size = 16)]
    pub pc_name: String,
}

#[socket]
#[derive(Debug, Clone)]
pub struct TopCommand {
    pub flag: i32,
}

socket::protocol_module! {
    ServerVersion => Command = 0x4847008A,
    FailConnect => Command = 0x48470023,
    SkillDisabled => Command = 0x49000020,
    SkillDelay => Command = 0x49000021,
    ProcessTimeMax => Command = 0x50320500,
    BlessCrown => Command = 0x49000019,
    SaveClient => Command = 0x484700e8,
    AgingEvent => Command = 0x8190000Au32,
    SendCoinShop => Command = 0x49470004,
    SendTimeShop => Command = 0x49470007,
    CodeType => Command = 0x49000014,
    CodeTime => Command = 0x49000013,
    GetClientFuncPos => Command = 0x50320410,
    RecvInitData => Command = 0x4900000f,
    RecordResult => Command = 0x48470084,
    SetItem => Command = 0x4847005A,
    NetState => Command = 0x4847008E,
    Ping => Ping = 0x49000000,
    GetItem => CommandEx = 0x4847005A,
    PlayerAction => CharCommand = 0x49000027,
    FailRecordData => CharCommand = 0x48470083,
    GetUserRecord => CharCommand = 0x48470082,
    InsertRecordData => PlayerCommand = 0x48470088,
    TopPVP => TopCommand = 0x49000011,
    TopLevel => TopCommand = 0x49000012,
}
