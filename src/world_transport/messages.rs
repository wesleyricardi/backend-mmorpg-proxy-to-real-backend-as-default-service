use socket::{Bytes, IntoMessageBuffer, Message as SocketMessage};

macro_rules! impl_world_message {
    ($name:ident { $($variant:ident => $message_ty:path),+ $(,)? }) => {
        pub enum $name {
            $($variant($message_ty),)+
        }

        impl IntoMessageBuffer for $name {
            fn into_message_buffer(self) -> SocketMessage<Bytes> {
                match self {
                    $(Self::$variant(message) => message.into_message_buffer(),)+
                }
            }
        }

        $(
            impl From<$message_ty> for $name {
                fn from(message: $message_ty) -> Self {
                    Self::$variant(message)
                }
            }
        )+
    };
}

impl_world_message!(ClientWorldMessage {
    Arena => crate::protocol::arena::Message,
    Chat => crate::protocol::chat::Message,
    Command => crate::protocol::command::Message,
    Cracker => crate::protocol::cracker::Message,
    Data => crate::protocol::data::Message,
    Item => crate::protocol::item::Message,
    ServerInfo => crate::protocol::server_info::Message,
    Shop => crate::protocol::shop::Message,
    Skill => crate::protocol::skill::Message,
    Transcode => crate::protocol::transcode::Message,
    User => crate::protocol::user::Message,
});

impl_world_message!(ServerWorldMessage {
    Arena => crate::protocol::arena::Message,
    Chat => crate::protocol::chat::Message,
    Command => crate::protocol::command::Message,
    Cracker => crate::protocol::cracker::Message,
    Data => crate::protocol::data::Message,
    GameServer => crate::protocol::game_server::Message,
    Item => crate::protocol::item::Message,
    ServerInfo => crate::protocol::server_info::Message,
    Shop => crate::protocol::shop::Message,
    Skill => crate::protocol::skill::Message,
    Transcode => crate::protocol::transcode::Message,
    User => crate::protocol::user::Message,
});
