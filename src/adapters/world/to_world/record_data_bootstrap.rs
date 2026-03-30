use std::time::Instant;

use tokio::sync::oneshot;

use crate::application::use_case::runtime::world_runtime::world_command::{
    RecordDataBootstrapQueryResult, RecordDataResponseContext, RecordDataWireSnapshot, WorldCommand,
};
use crate::domain::user_player::dto::UserPlayerData;

use super::sender::world_sender;

pub async fn begin_record_data_bootstrap_for_connection(
    connection_id: usize,
    request_id: u64,
    player_name: String,
    mode: i32,
    observed_at: Instant,
) {
    let Some(tx) = world_sender() else {
        return;
    };

    let _ = tx.send(WorldCommand::BeginRecordDataBootstrap {
        connection_id,
        request_id,
        player_name,
        mode,
        observed_at,
    });
}

pub async fn mark_record_data_bootstrap_ready(
    connection_id: usize,
    object_serial: u32,
    player_data: UserPlayerData,
    wire_snapshot: RecordDataWireSnapshot,
    response_context: RecordDataResponseContext,
    observed_at: Instant,
) {
    let Some(tx) = world_sender() else {
        return;
    };

    let _ = tx.send(WorldCommand::StoreBootstrappedRecordData {
        connection_id,
        object_serial,
        player_data,
        wire_snapshot,
        response_context,
        observed_at,
    });
}

pub async fn mark_record_data_bootstrap_failed(
    connection_id: usize,
    object_serial: u32,
    login_safe_key: i32,
    observed_at: Instant,
) {
    let Some(tx) = world_sender() else {
        return;
    };

    let _ = tx.send(WorldCommand::StoreBootstrappedFailRecordData {
        connection_id,
        object_serial,
        login_safe_key,
        observed_at,
    });
}

pub async fn clear_record_data_bootstrap_for_connection(
    connection_id: usize,
    observed_at: Instant,
) {
    let Some(tx) = world_sender() else {
        return;
    };

    let _ = tx.send(WorldCommand::ClearRecordDataBootstrap {
        connection_id,
        observed_at,
    });
}

pub async fn query_record_data_bootstrap_for_connection(
    connection_id: usize,
    request_id: u64,
) -> Option<RecordDataBootstrapQueryResult> {
    let Some(tx) = world_sender() else {
        return None;
    };

    let (reply_tx, reply_rx) = oneshot::channel();
    let _ = tx.send(WorldCommand::QueryRecordDataBootstrap {
        connection_id,
        request_id,
        reply: reply_tx,
    });

    reply_rx.await.ok()
}
