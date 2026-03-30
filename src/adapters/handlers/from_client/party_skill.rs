use socket::{Data, Request, Responder};

use super::skill_code::map_legacy_play_skill_code;
use crate::application::use_case::runtime::world_runtime::world_command::PartySkillContext;
use crate::domain::skill::dtos::codes::SKILL_HALL_OF_VALHALLA;
use crate::domain::user_player::state::UserPlayerId;
use crate::protocol::transcode::TransPartySkill;
use crate::{adapters::world, error::AppError, proxy, state::ServerState};

pub async fn party_skill(
    req: Request<TransPartySkill>,
    res: Responder,
    data: Data<ServerState>,
) -> Result<(), AppError> {
    let raw_skill_code = req.body.skill_code;
    log::debug!(
        "received PartySkill command: raw_skill_code=0x{:X} point={} party_count={}",
        raw_skill_code,
        req.body.point,
        req.body.party_count
    );
    let Some(skill_code) = map_legacy_play_skill_code(raw_skill_code) else {
        log::info!("invalid legacy play party skill code: {}", raw_skill_code);
        return proxy::forward_to_server(req, res, data).await;
    };

    let wire_params = req.body.wire_params();
    let context = match skill_code {
        SKILL_HALL_OF_VALHALLA => PartySkillContext::HallOfValhalla {
            triumph_of_valhalla_point: wire_params.w_param,
        },
        _ => PartySkillContext::StandardBuff,
    };
    let payload = world::PartySkillCommandPayload {
        skill_code,
        point: req.body.point,
        context,
        targets: req
            .body
            .active_targets()
            .object_serials
            .into_iter()
            .map(UserPlayerId)
            .collect(),
    };
    world::send_party_skill_from_connection(data.clone(), res.id, payload).await;
    proxy::forward_to_server(req, res, data).await
}
