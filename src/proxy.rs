use std::sync::Arc;

use log::trace;
use socket::{Data, FromBody, Request, RequestVecBody, Responder, SocketPacket};

use crate::{
    config::global::LISTEN_PORT,
    error::{AppError, ProtocolErrorKind},
    protocol::{
        chat::route_code::ProcessInfo,
        command::route_code::{GetUserRecord, ProcessTimeMax},
        data::route_code::ClientRecordData,
        server_info::{self, route_code::SetServerList as SET_SERVER_LIST, ServerList},
        transcode::route_code::{
            Adminmode2, BlesscastleInfo, CheckExpmoney, CheckNetstate, Hacktrap, IdGetuserinfo,
            InvenPosition, InvenPosition2, LimitDamage, Playerinfo, Position, ProcessSkill2,
            Shoptitem, Transplaydata, UpdateServerParam,
        },
    },
    state::ServerState,
};

pub async fn forward_to_server(
    req: impl IntoForwardRequest,
    res: Responder,
    data: Data<ServerState>,
) -> Result<(), AppError> {
    let req = req.into_forward_request();
    let connection_id = res.id;

    translate_message_from_client(&req);

    let Some(server_responder) = data
        .servers_connected
        .lock()
        .await
        .get_server(connection_id)
    else {
        log::error!("obtain server stream writer failed");
        return Ok(());
    };

    if let Err(error) = server_responder.send_byte(req.into()).await {
        log::error!("forward message to server failed, error: {:?}", error);
    }

    Ok(())
}

pub trait IntoForwardRequest {
    fn into_forward_request(self) -> RequestVecBody;
}

impl IntoForwardRequest for RequestVecBody {
    fn into_forward_request(self) -> RequestVecBody {
        self
    }
}

impl<T> IntoForwardRequest for Request<T>
where
    T: FromBody + SocketPacket,
{
    fn into_forward_request(self) -> RequestVecBody {
        RequestVecBody {
            head: self.head,
            body: Arc::from(self.body.to_bytes()),
        }
    }
}

pub async fn forward_to_client(
    request: RequestVecBody,
    server_responder: Responder,
    data: Data<ServerState>,
) -> Result<(), AppError> {
    let connection_id = server_responder.id;
    let client_responder = data
        .servers_connected
        .lock()
        .await
        .get_client(connection_id);

    let Some(client_responder) = client_responder else {
        log::warn!(
            "client {} not found while forwarding message from server",
            connection_id
        );
        return Ok(());
    };

    translate_message_from_server(&request);

    if !are_not_to_forward(request.head.code) {
        if let Err(error) = write_message_to_client(request, client_responder).await {
            log::error!("fail to write message to client, err: {:?}", error);
            return Err(crate::protocol_error!(
                ProtocolErrorKind::ForwardFailed,
                "failed to write message to client for connection {}",
                connection_id
            ));
        }
    }

    Ok(())
}

fn are_not_to_forward(message_code: i32) -> bool {
    match message_code {
        _ => false,
    }
}

async fn write_message_to_client(
    request: RequestVecBody,
    responder: Responder,
) -> Result<(), Box<dyn std::error::Error>> {
    log::trace!(
        "write message to the client {}, req: {:?}",
        responder.id,
        request
    );
    match request.head.code {
        SET_SERVER_LIST => {
            let server_list = ServerList::from_bytes(&request.body)
                .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))?;
            rewrite_server_list(server_list, responder).await
        }
        _ => {
            responder.send_byte(request.into()).await?;
            Ok(())
        }
    }
}

async fn rewrite_server_list(
    mut server_list: ServerList,
    responder: Responder,
) -> Result<(), Box<dyn std::error::Error>> {
    // server_list.server_time = chrono::Utc::now().timestamp() as u32;
    let listen_port = LISTEN_PORT as u32;
    server_list.servers_info[0].port1 = listen_port;
    server_list.servers_info[0].port2 = listen_port;
    server_list.servers_info[0].port3 = listen_port;

    responder
        .send(server_info::Message::SetServerList(server_list))
        .await?;
    Ok(())
}

fn translate_message_from_client(request: &RequestVecBody) {
    //-1610612732 = 0xA0000004
    match request.head.code {
        ProcessInfo | 0x4847008C | -1610612732 | GetUserRecord | ProcessTimeMax | IdGetuserinfo
        | UpdateServerParam | Playerinfo | 0x484700E0 | Transplaydata | 0x48470021
        | CheckNetstate | CheckExpmoney | LimitDamage | InvenPosition | InvenPosition2
        | Adminmode2 | BlesscastleInfo | Hacktrap | ClientRecordData | Position => {
            log::trace!(
                "Client> 0x{:X}, action: ao clicar no botão de entrar no jogo e durante runtime",
                request.head.code
            );
        }
        ProcessSkill2 => {
            log::info!(
                "Client> 0x{:X}, action: ao clicar no botão de usar skill (talves nao todas)",
                request.head.code
            );
        }
        0x709F00C0 => {
            log::info!(
                "Client> 0x{:X}, action: ao iniciar um ataque call by int dm_SendTransDamage in `client-doc\\legacy_code_base_cpp\\Damage.cpp`",
                request.head.code
            );
        }
        Shoptitem => {
            // server esta ignorando essa mensagem
            log::info!(
                "Client> 0x{:X}, action: ao clicar em comprar item no npc",
                request.head.code
            );
        }
        _ => {}
    }
}

fn translate_message_from_server(request: &RequestVecBody) {
    match request.head.code as u32 {
        0x18000000 | 0xE80B0000 | 0xCC050000 | 0x28000000 | 0x90000000 | 0xC000000 | 0x10000000
        | 0x49000012 | 0x49000013 | 0x49000019 | 0x74000000 | 0xB0000000 | 0x14000000
        | 0x58480001 | 0x58470008 | 0x58480000 | 0x58480002 | 0x58480003 | 0x58480004
        | 0x58480005 | 0x58470009 | 0x58480006 | 0x5847000A | 0x58480007 | 0x58480008
        | 0x58480009 | 0x30000000 | 0x4847008A | 0xE0020000 | 0xF8010000 | 0x50320110
        | 0x48470081 | 0x50322F00 | 0x38000000 | 0xB8020000 | 0x70040000 | 0xF8030000
        | 0x50060000 | 0x8080000 | 0x49000030 | 0x49000032 | 0x8190000A | 0x78060000
        | 0x18070000 | 0xC8060000 | 0x68070000 | 0xB8070000 | 0x60050000 | 0x48470020
        | 0x49470007 | 0x49000033 | 0xF0060000 | 0x38050000 | 0xD8050000 | 0x40070000
        | 0x49000020 | 0x48470086 | 0x484700C0 | 0x48470014 | 0x4847008B | 0x49000023
        | 0x70000000 | 0x49000011 | 0x48471005 | 0x49470004 | 0x49000014 | 0x49000021
        | 0x49000029 | 0x49000039 | 0x48470F11 | 0x48470036 | 0x50322EC0 | 0x48470080
        | 0xA0000005 | 0x4848000B | 0x50320130 | 0x48470084 => {
            log::trace!(
                "Server> 0x{:X}, action: ao entrar no jogo, repetem muito",
                request.head.code,
            );
        }
        0x50322020 => {
            log::info!(
                "Server> 0x{:X}, action: ao clicar num npc",
                request.head.code,
            );
        }
        0x6000014A => {
            // npc ferreiro
            log::info!(
                "Server> 0x{:X}, action: ao clicar no ferreiro",
                request.head.code,
            );
        }
        0x48471305 => {
            // npc que restaura aging
            log::info!(
                "Server> 0x{:X}, action: ao clicar num npc que restaura aging",
                request.head.code,
            );
        }
        0x4847005D => {
            log::info!(
                "Server> 0x{:X}, action: ao clicar num npc que abre janela de ir ao castelo bless",
                request.head.code,
            );
        }
        _ => {}
    }
}
