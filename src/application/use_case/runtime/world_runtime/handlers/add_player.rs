use std::time::Instant;

use crate::domain::field::dto::FieldId;
use crate::domain::user_player::calc::recalculate_user_player_state;
use crate::domain::user_player::state::{UserPlayerId, UserPlayerState};

use super::super::world::World;

pub fn handle(world: &mut World, player_id: UserPlayerId, data: UserPlayerState, now: Instant) {
    if let Some(field_id) = FieldId::from_raw(data.progression.game_save.play_stage_num) {
        world.fields.player_field.insert(player_id, field_id);
        world
            .fields
            .field_players
            .entry(field_id)
            .or_default()
            .insert(player_id);
    }
    let mut data = data;
    recalculate_user_player_state(&mut data, Some(&world.skill_state));
    world.players.insert(player_id, data);
    world.schedule_regen_for_player(player_id, now);
}
