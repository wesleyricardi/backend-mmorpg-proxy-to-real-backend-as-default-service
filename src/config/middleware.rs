use std::mem::size_of;

use socket::{
    BeforeExtractContext, BeforeExtractError, ReqHead, RequestVecBody, Responder, SocketPacket,
};

use crate::protocol::{
    chat::ChatMessage,
    data::{route_code::ClientRecordData as CLIENT_RECORD_DATA, RecordDatas},
    transcode::{
        route_code::{Processinfo as PROCESS_INFO, RangeDamage as RANGE_DAMAGE},
        TransSkillAttackData2,
    },
};

const REQUEST_HEAD_SIZE: usize = 8;
const RECORD_DATAS_CONTAINER_FIELDS_SIZE: usize = 12;
const RANGE_DAMAGE_BASE_BODY_SIZE: usize = 104;
const RANGE_DAMAGE_TARGET_COUNT_OFFSET: usize = 84;
const RANGE_DAMAGE_TARGET_SERIAL_SIZE: usize = 4;
const PROCESS_INFO_BASE_BODY_SIZE: usize = 24;
const PROCESS_INFO_MAX_MESSAGE_SIZE: usize = 256;

pub async fn normalize_request(
    req: RequestVecBody,
    ctx: BeforeExtractContext,
) -> Result<RequestVecBody, BeforeExtractError> {
    let responder = &ctx.responder;

    if req.head.code == CLIENT_RECORD_DATA {
        return Ok(normalize_client_record_data(req, responder));
    }

    if req.head.code == RANGE_DAMAGE {
        return Ok(normalize_range_damage(req, responder));
    }

    if req.head.code == PROCESS_INFO {
        return Ok(normalize_process_info(req, responder));
    }

    Ok(req)
}

fn normalize_client_record_data(req: RequestVecBody, responder: &Responder) -> RequestVecBody {
    let expected_body_size = size_of::<<RecordDatas as SocketPacket>::CRepr>();
    let original_body_size = req.body.len();

    if original_body_size == expected_body_size {
        return req;
    }

    if original_body_size < RECORD_DATAS_CONTAINER_FIELDS_SIZE {
        log::warn!(
            "client_record_data normalize skipped for client {}: body too small ({})",
            responder.id,
            original_body_size
        );
        return req;
    }

    let container_data_size =
        i32::from_le_bytes([req.body[8], req.body[9], req.body[10], req.body[11]]);
    if container_data_size <= 0 {
        log::warn!(
            "client_record_data normalize skipped for client {}: invalid container_data_size ({})",
            responder.id,
            container_data_size
        );
        return req;
    }

    let container_data_size = container_data_size as usize;
    let available_inner_size = original_body_size - RECORD_DATAS_CONTAINER_FIELDS_SIZE;
    if container_data_size > available_inner_size {
        log::warn!(
            "client_record_data normalize skipped for client {}: container_data_size ({}) > available_inner_size ({})",
            responder.id,
            container_data_size,
            available_inner_size
        );
        return req;
    }

    let mut normalized_body = req.body.to_vec();
    normalized_body.resize(expected_body_size, 0);
    let normalized_size = REQUEST_HEAD_SIZE + normalized_body.len();

    RequestVecBody {
        head: ReqHead {
            size: normalized_size,
            code: req.head.code,
        },
        body: normalized_body.into(),
    }
}

fn normalize_range_damage(req: RequestVecBody, responder: &Responder) -> RequestVecBody {
    let expected_body_size = size_of::<<TransSkillAttackData2 as SocketPacket>::CRepr>();
    let original_body_size = req.body.len();

    if original_body_size == expected_body_size {
        return req;
    }

    if original_body_size < RANGE_DAMAGE_BASE_BODY_SIZE {
        log::warn!(
            "range_damage normalize skipped for client {}: body too small ({})",
            responder.id,
            original_body_size
        );
        return req;
    }

    let target_count = i32::from_le_bytes([
        req.body[RANGE_DAMAGE_TARGET_COUNT_OFFSET],
        req.body[RANGE_DAMAGE_TARGET_COUNT_OFFSET + 1],
        req.body[RANGE_DAMAGE_TARGET_COUNT_OFFSET + 2],
        req.body[RANGE_DAMAGE_TARGET_COUNT_OFFSET + 3],
    ]);

    if target_count < 0 {
        log::warn!(
            "range_damage normalize skipped for client {}: invalid target_count ({})",
            responder.id,
            target_count
        );
        return req;
    }

    let target_count = target_count as usize;
    let variable_expected_body_size =
        RANGE_DAMAGE_BASE_BODY_SIZE + (target_count * RANGE_DAMAGE_TARGET_SERIAL_SIZE);

    if variable_expected_body_size != original_body_size {
        log::warn!(
            "range_damage normalize skipped for client {}: body_size ({}) does not match target_count-derived size ({})",
            responder.id,
            original_body_size,
            variable_expected_body_size
        );
        return req;
    }

    if variable_expected_body_size > expected_body_size {
        log::warn!(
            "range_damage normalize skipped for client {}: target_count-derived size ({}) exceeds fixed packet size ({})",
            responder.id,
            variable_expected_body_size,
            expected_body_size
        );
        return req;
    }

    let mut normalized_body = req.body.to_vec();
    normalized_body.resize(expected_body_size, 0);
    let normalized_size = REQUEST_HEAD_SIZE + normalized_body.len();

    RequestVecBody {
        head: ReqHead {
            size: normalized_size,
            code: req.head.code,
        },
        body: normalized_body.into(),
    }
}

fn normalize_process_info(req: RequestVecBody, responder: &Responder) -> RequestVecBody {
    let expected_body_size = size_of::<<ChatMessage as SocketPacket>::CRepr>();
    let original_body_size = req.body.len();

    if original_body_size == expected_body_size {
        return req;
    }

    if original_body_size < PROCESS_INFO_BASE_BODY_SIZE {
        log::warn!(
            "processinfo normalize skipped for client {}: body too small ({})",
            responder.id,
            original_body_size
        );
        return req;
    }

    let message_size = original_body_size - PROCESS_INFO_BASE_BODY_SIZE;
    if message_size > PROCESS_INFO_MAX_MESSAGE_SIZE {
        log::warn!(
            "processinfo normalize skipped for client {}: message too large ({})",
            responder.id,
            message_size
        );
        return req;
    }

    let mut normalized_body = req.body.to_vec();
    normalized_body.resize(expected_body_size, 0);
    let normalized_size = REQUEST_HEAD_SIZE + normalized_body.len();

    RequestVecBody {
        head: ReqHead {
            size: normalized_size,
            code: req.head.code,
        },
        body: normalized_body.into(),
    }
}
