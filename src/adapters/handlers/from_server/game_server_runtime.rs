use std::time::Instant;

use socket::{Data, Request, RequestVecBody, Responder};

use crate::adapters::legacy_sync::runtime_sync::request_runtime_snapshot;
use crate::application::use_case::runtime::world_runtime::game_server::dto::{
    MonsterId, NpcId, RuntimeMonsterId, RuntimeMonsterState, RuntimeNpcId, RuntimeNpcState,
    WorldPoint,
};
use crate::application::use_case::runtime::world_runtime::world_command::WorldCommand;
use crate::domain::field::dto::FieldId;
use crate::error::AppError;
use crate::protocol::command::Command;
use crate::protocol::game_server::runtime::{
    RuntimeMonsterDespawn, RuntimeMonsterSync, RuntimeNpcDespawn, RuntimeNpcSync,
    WorldRuntimeSnapshotBegin, WorldRuntimeSnapshotEnd,
};
use crate::state::{legacy_sync::RuntimeSequenceSource, ServerState};

fn world_sender() -> Option<&'static std::sync::mpsc::Sender<WorldCommand>> {
    crate::adapters::world::world_sender()
}

fn runtime_field_id(raw_field_id: i32) -> Option<FieldId> {
    if raw_field_id < 0 {
        None
    } else {
        FieldId::from_raw(raw_field_id)
    }
}

async fn schedule_resync_if_sequence_gapped(
    data: Data<ServerState>,
    sequence: u64,
    field_id: i32,
    source: RuntimeSequenceSource,
) {
    let resync_request = {
        data.game_server_runtime_sync
            .lock()
            .await
            .observe_sequence(sequence, field_id, source)
    };

    let Some(resync_request) = resync_request else {
        return;
    };

    match request_runtime_snapshot(
        data.clone(),
        resync_request.request_id,
        resync_request.field_id,
        true,
        true,
    )
    .await
    {
        Ok(request_id) => {
            log::warn!(
                "runtime sequence gap detected on {:?}: requesting re-sync request_id={} field_id={} missing_after_sequence={}",
                source,
                request_id,
                resync_request.field_id,
                sequence
            );
        }
        Err(error) => {
            log::error!(
                "runtime sequence gap detected on {:?}, but failed to request re-sync for field_id={}: {}",
                source,
                resync_request.field_id,
                error
            );
        }
    }
}

pub async fn runtime_snapshot_begin(
    request: Request<WorldRuntimeSnapshotBegin>,
    responder: Responder,
    data: Data<ServerState>,
) -> Result<(), AppError> {
    log::trace!(
        "runtime snapshot begin packet: responder_id={} request_id={} sequence={} field_id={} monsters={} npcs={}",
        responder.id,
        request.body.request_id,
        request.body.sequence,
        request.body.field_id,
        request.body.monster_count,
        request.body.npc_count
    );

    let Some(tx) = world_sender() else {
        log::error!("world sender not initialized for runtime snapshot begin");
        return Ok(());
    };

    schedule_resync_if_sequence_gapped(
        data.clone(),
        request.body.sequence,
        request.body.field_id,
        RuntimeSequenceSource::SnapshotBegin {
            request_id: request.body.request_id,
        },
    )
    .await;

    data.game_server_runtime_sync
        .lock()
        .await
        .mark_snapshot_begin(
            request.body.request_id,
            request.body.sequence,
            request.body.field_id,
        );

    let _ = tx.send(WorldCommand::ResetRuntimeScope {
        field_id: runtime_field_id(request.body.field_id),
        include_monsters: request.body.include_monsters != 0,
        include_npcs: request.body.include_npcs != 0,
        observed_at: Instant::now(),
    });

    Ok(())
}

pub async fn runtime_server_version(
    request: Request<Command>,
    responder: Responder,
    _data: Data<ServerState>,
) -> Result<(), AppError> {
    log::trace!(
        "runtime server version packet: responder_id={} version={} player_count_hint={}",
        responder.id,
        request.body.w_param,
        request.body.l_param
    );
    Ok(())
}

pub async fn runtime_monster_sync(
    request: Request<RuntimeMonsterSync>,
    responder: Responder,
    data: Data<ServerState>,
) -> Result<(), AppError> {
    log::trace!(
        "runtime monster sync packet: responder_id={} object_serial={} field_id={} template_auto_char_code={}",
        responder.id,
        request.body.object_serial,
        request.body.field_id,
        request.body.template_auto_char_code
    );

    let Some(tx) = world_sender() else {
        log::error!("world sender not initialized for runtime monster sync");
        return Ok(());
    };

    schedule_resync_if_sequence_gapped(
        data.clone(),
        request.body.sequence,
        request.body.field_id,
        RuntimeSequenceSource::MonsterSync,
    )
    .await;

    let state = RuntimeMonsterState {
        id: RuntimeMonsterId::from_object_serial(request.body.object_serial),
        template_id: if request.body.template_auto_char_code != 0 {
            Some(MonsterId::from_legacy_auto_char_code(
                request.body.template_auto_char_code,
            ))
        } else {
            None
        },
        field_id: runtime_field_id(request.body.field_id),
        name: request.body.name.clone(),
        position: WorldPoint {
            x: request.body.x,
            y: request.body.y,
            z: request.body.z,
        },
        direction: WorldPoint {
            x: request.body.ax,
            y: request.body.ay,
            z: request.body.az,
        },
        current_life: request.body.current_life,
        max_life: request.body.max_life,
        motion_state: request.body.motion_state,
    };

    let _ = tx.send(WorldCommand::UpsertRuntimeMonster {
        state,
        observed_at: Instant::now(),
    });

    Ok(())
}

pub async fn runtime_monster_despawn(
    request: Request<RuntimeMonsterDespawn>,
    _responder: Responder,
    data: Data<ServerState>,
) -> Result<(), AppError> {
    let Some(tx) = world_sender() else {
        log::error!("world sender not initialized for runtime monster despawn");
        return Ok(());
    };

    schedule_resync_if_sequence_gapped(
        data.clone(),
        request.body.sequence,
        request.body.field_id,
        RuntimeSequenceSource::MonsterDespawn,
    )
    .await;

    let _ = tx.send(WorldCommand::RemoveRuntimeMonster {
        runtime_monster_id: RuntimeMonsterId::from_object_serial(request.body.object_serial),
        observed_at: Instant::now(),
    });

    Ok(())
}

pub async fn runtime_npc_sync(
    request: Request<RuntimeNpcSync>,
    responder: Responder,
    data: Data<ServerState>,
) -> Result<(), AppError> {
    log::trace!(
        "runtime npc sync packet: responder_id={} object_serial={} field_id={} template_file='{}'",
        responder.id,
        request.body.object_serial,
        request.body.field_id,
        request.body.template_file
    );

    let Some(tx) = world_sender() else {
        log::error!("world sender not initialized for runtime npc sync");
        return Ok(());
    };

    schedule_resync_if_sequence_gapped(
        data.clone(),
        request.body.sequence,
        request.body.field_id,
        RuntimeSequenceSource::NpcSync,
    )
    .await;

    let template_file = request.body.template_file.trim();
    let state = RuntimeNpcState {
        id: RuntimeNpcId::from_object_serial(request.body.object_serial),
        template_id: if template_file.is_empty() {
            None
        } else {
            Some(NpcId::from_file_name(template_file))
        },
        field_id: runtime_field_id(request.body.field_id),
        name: request.body.name.clone(),
        position: WorldPoint {
            x: request.body.x,
            y: request.body.y,
            z: request.body.z,
        },
        direction: WorldPoint {
            x: request.body.ax,
            y: request.body.ay,
            z: request.body.az,
        },
        motion_state: request.body.motion_state,
    };

    let _ = tx.send(WorldCommand::UpsertRuntimeNpc {
        state,
        observed_at: Instant::now(),
    });

    Ok(())
}

pub async fn runtime_npc_despawn(
    request: Request<RuntimeNpcDespawn>,
    _responder: Responder,
    data: Data<ServerState>,
) -> Result<(), AppError> {
    let Some(tx) = world_sender() else {
        log::error!("world sender not initialized for runtime npc despawn");
        return Ok(());
    };

    schedule_resync_if_sequence_gapped(
        data.clone(),
        request.body.sequence,
        request.body.field_id,
        RuntimeSequenceSource::NpcDespawn,
    )
    .await;

    let _ = tx.send(WorldCommand::RemoveRuntimeNpc {
        runtime_npc_id: RuntimeNpcId::from_object_serial(request.body.object_serial),
        observed_at: Instant::now(),
    });

    Ok(())
}

pub async fn runtime_snapshot_end(
    request: Request<WorldRuntimeSnapshotEnd>,
    responder: Responder,
    data: Data<ServerState>,
) -> Result<(), AppError> {
    let Some(tx) = world_sender() else {
        log::error!("world sender not initialized for runtime snapshot end");
        return Ok(());
    };

    schedule_resync_if_sequence_gapped(
        data.clone(),
        request.body.sequence,
        request.body.field_id,
        RuntimeSequenceSource::SnapshotEnd {
            request_id: request.body.request_id,
        },
    )
    .await;

    let field_id = runtime_field_id(request.body.field_id);
    let _ = tx.send(WorldCommand::RuntimeSnapshotCompleted {
        request_id: request.body.request_id,
        sequence: request.body.sequence,
        field_id,
        observed_at: Instant::now(),
    });

    data.game_server_runtime_sync
        .lock()
        .await
        .mark_snapshot_completed(
            request.body.request_id,
            request.body.sequence,
            request.body.field_id,
        );

    log::info!(
        "runtime snapshot completed: responder_id={} request_id={} sequence={} field_id={}",
        responder.id,
        request.body.request_id,
        request.body.sequence,
        request.body.field_id
    );
    Ok(())
}

pub async fn ignore_unexpected_runtime_packet(
    request: RequestVecBody,
    _responder: Responder,
    _data: Data<ServerState>,
) -> Result<(), AppError> {
    log::warn!(
        "unexpected packet received on dedicated runtime connection: code=0x{:X}, size={}",
        request.head.code,
        request.head.size
    );
    Ok(())
}

pub async fn runtime_disconnect(responder: Responder, data: Data<ServerState>) {
    data.game_server_runtime_sync
        .lock()
        .await
        .mark_disconnected();
    log::warn!(
        "runtime sync connection {} closed on legacy side",
        responder.id
    );
}
