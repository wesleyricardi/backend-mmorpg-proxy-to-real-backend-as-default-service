pub const LISTEN_PORT: u16 = 10009;

pub const SERVER_ADDR: &str = "127.0.0.1";
pub const SERVER_PORT: u16 = 10008;

pub fn debug_ui_enabled() -> bool {
    match std::env::var("SOURCE_BACKEND_DEBUG_UI") {
        Ok(value) => matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "1" | "true" | "yes" | "on"
        ),
        Err(_) => cfg!(debug_assertions),
    }
}
