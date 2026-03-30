use socket::{Data, Request, RequestVecBody, Responder};

use crate::{
    adapters::{record_data_codec, record_data_response},
    application::use_case,
    domain::user_player::state::UserPlayerId,
    error::AppError,
    protocol::data::RecordDatas,
    state::ServerState,
};

pub async fn server_client_record_data(
    request: Request<RecordDatas>,
    responder: Responder,
    data: Data<ServerState>,
) -> Result<(), AppError> {
    if request.body.total > 1 {
        log::info!(
            "ClientRecordData multipart from server {}: count={} total={} trans_size={}",
            responder.id,
            request.body.count,
            request.body.total,
            request.body.data_size
        );
    }

    let Some(record_datas) =
        assemble_record_datas_fragment(request.body.clone(), responder.id, data.clone()).await?
    else {
        return Ok(());
    };
    let response_context = record_data_codec::build_record_data_response_context(&record_datas);
    let wire_snapshot =
        record_data_codec::build_record_data_wire_snapshot(&record_datas).map_err(|err| {
            log::error!("Failed to build RecordDataWireSnapshot: {:?}", err);
            err
        })?;

    let user_player_data =
        record_data_codec::build_user_player_data(&record_datas).map_err(|err| {
            log::error!("Failed to build UserPlayerData: {:?}", err);
            err
        })?;
    let object_serial = response_context.object_serial;
    let player_id = UserPlayerId(object_serial);

    use_case::client_record_data::save_client_record_data_from_server(
        data,
        responder.clone(),
        player_id,
        user_player_data.clone(),
    )
    .await?;

    crate::adapters::world::mark_record_data_bootstrap_ready(
        responder.id,
        object_serial,
        user_player_data,
        wire_snapshot,
        response_context,
        std::time::Instant::now(),
    )
    .await;

    Ok(())
}

async fn assemble_record_datas_fragment(
    request: RecordDatas,
    connection_id: usize,
    data: Data<ServerState>,
) -> Result<Option<RecordDatas>, AppError> {
    let now = std::time::Instant::now();
    let trans_size = request.data_size.max(0) as usize;
    let serialized_record_data = request.data.to_bytes();
    let chunk_end = trans_size.min(serialized_record_data.len());
    let chunk = &serialized_record_data[..chunk_end];

    let mut pending = data.pending_record_data.lock().await;
    pending.retain(|_, state| !state.is_expired(now));
    let state = pending.entry(connection_id).or_default();
    let Some(assembled) =
        state.push_fragment(request.count, request.total, request.data_size, chunk, now)
    else {
        return Ok(None);
    };
    pending.remove(&connection_id);

    drop(pending);

    let mut normalized = assembled;
    normalized.resize(
        std::mem::size_of::<<crate::protocol::data::RecordData as socket::SocketPacket>::CRepr>(),
        0,
    );
    let record_data =
        crate::protocol::data::RecordData::from_bytes(&normalized).map_err(|err| {
            log::error!("Failed to parse assembled RecordData: {:?}", err);
            crate::parse_error!(
                crate::error::ParseErrorKind::RecordItem,
                "failed to parse assembled RecordData"
            )
        })?;

    Ok(Some(RecordDatas {
        count: request.count,
        total: request.total,
        data_size: request.data_size,
        data: record_data,
    }))
}
