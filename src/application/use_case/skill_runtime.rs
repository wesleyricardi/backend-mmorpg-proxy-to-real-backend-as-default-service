use socket::{Data, Responder};

use crate::error::AppError;
use crate::protocol::command::{CancelSkillRuntimeCommand, ProcessSkillRuntimeCommand};
use crate::protocol::transcode::TransPartySkill;
use crate::state::ServerState;

pub async fn upsert_party_skill_runtime(
    _data: Data<ServerState>,
    _responder: Responder,
    _payload: TransPartySkill,
) -> Result<(), AppError> {
    Ok(())
}

pub async fn upsert_process_skill_runtime(
    _data: Data<ServerState>,
    _responder: Responder,
    _payload: ProcessSkillRuntimeCommand,
) -> Result<(), AppError> {
    Ok(())
}

pub async fn remove_active_skill_runtime(
    _data: Data<ServerState>,
    _responder: Responder,
    _payload: CancelSkillRuntimeCommand,
) -> Result<(), AppError> {
    Ok(())
}
