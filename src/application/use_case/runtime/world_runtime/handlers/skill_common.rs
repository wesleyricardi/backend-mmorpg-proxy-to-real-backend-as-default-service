use crate::domain::user_player::state::UserPlayerId;

use super::super::world::World;

pub fn is_field_policy_allow_skill(world: &World, player_id: UserPlayerId) -> bool {
    let Some(field_id) = world.fields.player_field.get(&player_id).copied() else {
        log::debug!("field_missing_player_context player_id={:?}", player_id.0);
        return false;
    };
    let field_entry = field_id.get_catalog_entry();
    if !field_entry.policy.allows_skill_restrictive() {
        log::debug!(
            "field_policy_blocked player_id={:?} field_id={} reason=can_use_skill_not_explicitly_allowed",
            player_id.0,
            field_id.raw()
        );
        return false;
    }
    true
}
