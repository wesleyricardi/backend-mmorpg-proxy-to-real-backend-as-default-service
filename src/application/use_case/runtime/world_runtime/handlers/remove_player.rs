use crate::debug_ui;
use crate::domain::user_player::state::UserPlayerId;

use super::super::world::World;

pub fn handle(world: &mut World, player_id: UserPlayerId) {
    world.players.remove(&player_id);
    if let Some(field_id) = world.fields.player_field.remove(&player_id) {
        if let Some(players) = world.fields.field_players.get_mut(&field_id) {
            players.remove(&player_id);
        }
    }
    world.fields.player_position.remove(&player_id);
    debug_ui::remove_player_info_panel(player_id);
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;

    use crate::application::use_case::runtime::world_runtime::world::World;
    use crate::domain::field::dto::FieldId;
    use crate::domain::skill::dtos::codes::SKILL_CONCENTRATION;
    use crate::domain::skill::state::SkillState;
    use crate::domain::user_player::dto::{UserCalcOutput, UserPlayerData};
    use crate::domain::user_player::state::{UserPlayerId, UserPlayerState};

    use super::handle;

    #[test]
    fn remove_player_clears_player_field_and_runtime_state() {
        let (tx_net, _rx_net) = mpsc::channel();
        let (_tx_world, rx_world) = mpsc::channel();
        let mut world = World::new(SkillState::default(), rx_world, tx_net);
        let player_id = UserPlayerId(777);
        let mut player = UserPlayerState::from_record_data(
            1,
            UserPlayerData::default(),
            UserCalcOutput::default(),
        );
        player.runtime.cooldowns.insert(SKILL_CONCENTRATION, 123);
        world.players.insert(player_id, player);
        world
            .fields
            .player_field
            .insert(player_id, FieldId::QuestIv);

        handle(&mut world, player_id);

        assert!(!world.players.contains_key(&player_id));
        assert!(!world.fields.player_field.contains_key(&player_id));
        assert!(!world.fields.player_position.contains_key(&player_id));
    }
}
