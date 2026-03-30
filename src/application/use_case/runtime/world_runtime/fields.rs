use std::collections::{HashMap, HashSet};

use super::game_server::dto::{
    FieldSpawnConfigState, MonsterId, NpcId, RuntimeMonsterId, RuntimeNpcId,
};
use crate::domain::field::dto::FieldId;
use crate::domain::user_player::state::UserPlayerId;

#[derive(Debug, Default)]
pub struct FieldRuntimeState {
    pub player_field: HashMap<UserPlayerId, FieldId>,
    pub field_players: HashMap<FieldId, HashSet<UserPlayerId>>,
    pub player_position: HashMap<UserPlayerId, (i32, i32, i32)>,
    pub field_spawn_config: HashMap<FieldId, FieldSpawnConfigState>,
    pub monster_template_fields: HashMap<MonsterId, HashSet<FieldId>>,
    pub field_monster_templates: HashMap<FieldId, HashSet<MonsterId>>,
    pub npc_template_fields: HashMap<NpcId, HashSet<FieldId>>,
    pub field_npc_templates: HashMap<FieldId, HashSet<NpcId>>,
    pub runtime_monster_field: HashMap<RuntimeMonsterId, FieldId>,
    pub field_runtime_monsters: HashMap<FieldId, HashSet<RuntimeMonsterId>>,
    pub runtime_monster_position: HashMap<RuntimeMonsterId, (i32, i32, i32)>,
    pub runtime_npc_field: HashMap<RuntimeNpcId, FieldId>,
    pub field_runtime_npcs: HashMap<FieldId, HashSet<RuntimeNpcId>>,
    pub runtime_npc_position: HashMap<RuntimeNpcId, (i32, i32, i32)>,
}
