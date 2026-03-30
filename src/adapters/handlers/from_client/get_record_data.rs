use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use socket::{Data, Request, RequestVecBody, Responder};

use crate::adapters::{record_data_response, world};
use crate::application::use_case::runtime::world_runtime::world_command::RecordDataBootstrapQueryResult;
use crate::error::AppError;
use crate::protocol::command::CharCommand;
use crate::proxy;
use crate::state::ServerState;

const RECORD_DATA_BOOTSTRAP_TIMEOUT: Duration = Duration::from_secs(5);
const RECORD_DATA_BOOTSTRAP_POLL_INTERVAL: Duration = Duration::from_millis(20);

static NEXT_RECORD_DATA_REQUEST_ID: AtomicU64 = AtomicU64::new(1);

pub async fn get_record_data(
    original_request: RequestVecBody,
    request: Request<CharCommand>,
    responder: Responder,
    data: Data<ServerState>,
) -> Result<(), AppError> {
    let request_id = NEXT_RECORD_DATA_REQUEST_ID.fetch_add(1, Ordering::Relaxed);
    let observed_at = Instant::now();

    data.pending_record_data.lock().await.remove(&responder.id);

    world::begin_record_data_bootstrap_for_connection(
        responder.id,
        request_id,
        request.body.name.clone(),
        request.body.l_param,
        observed_at,
    )
    .await;

    proxy::forward_to_server(original_request, responder.clone(), data).await?;

    let deadline = tokio::time::Instant::now() + RECORD_DATA_BOOTSTRAP_TIMEOUT;
    loop {
        if tokio::time::Instant::now() >= deadline {
            world::clear_record_data_bootstrap_for_connection(responder.id, Instant::now()).await;
            return Err(crate::protocol_error!(
                crate::error::ProtocolErrorKind::ForwardFailed,
                "record_data bootstrap timed out for connection {}",
                responder.id
            ));
        }

        match world::query_record_data_bootstrap_for_connection(responder.id, request_id).await {
            Some(RecordDataBootstrapQueryResult::Ready {
                player_data,
                wire_snapshot,
                response_context,
            }) => {
                let packets = record_data_response::build_record_data_packets(
                    &player_data,
                    &wire_snapshot,
                    &response_context,
                )?;

                for packet in packets {
                    responder.send_byte(packet.into()).await.map_err(|_| {
                        crate::protocol_error!(
                            crate::error::ProtocolErrorKind::ForwardFailed,
                            "failed to send rebuilt RecordData to client {}",
                            responder.id
                        )
                    })?;
                }
                world::clear_record_data_bootstrap_for_connection(responder.id, Instant::now())
                    .await;
                return Ok(());
            }
            Some(RecordDataBootstrapQueryResult::Failed {
                object_serial,
                login_safe_key,
            }) => {
                responder
                    .send(crate::protocol::command::Message::FailRecordData(
                        record_data_response::build_fail_record_data_command(
                            &request.body,
                            object_serial,
                            login_safe_key,
                        ),
                    ))
                    .await
                    .map_err(|_| {
                        crate::protocol_error!(
                            crate::error::ProtocolErrorKind::ForwardFailed,
                            "failed to send FailRecordData to client {}",
                            responder.id
                        )
                    })?;
                world::clear_record_data_bootstrap_for_connection(responder.id, Instant::now())
                    .await;
                return Ok(());
            }
            Some(RecordDataBootstrapQueryResult::Superseded) => {
                return Err(crate::protocol_error!(
                    crate::error::ProtocolErrorKind::ForwardFailed,
                    "record_data bootstrap superseded by newer request for connection {}",
                    responder.id
                ));
            }
            Some(RecordDataBootstrapQueryResult::Missing)
            | Some(RecordDataBootstrapQueryResult::Pending)
            | None => {
                tokio::time::sleep(RECORD_DATA_BOOTSTRAP_POLL_INTERVAL).await;
            }
        }
    }
}
