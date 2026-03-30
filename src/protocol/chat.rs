use socket::socket;

#[socket]
#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub ip: u32,
    pub object_serial: u32,
    #[socket(string_size = 256)]
    pub message: String,
}

socket::protocol_module! {
    ProcessInfo => ChatMessage = 0x484700E4,
    WhisperMessage => ChatMessage = 0x48471005,
    ChatMessage => ChatMessage = 0x48471001,
}
