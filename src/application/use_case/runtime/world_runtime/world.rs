use std::collections::{HashMap, HashSet};
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::thread;
use std::time::{Duration, Instant};

use serde::Serialize;

use super::game_server::dto::{
    FieldSpawnConfigState, GameServerData, MonsterId, MonsterTemplateState, NpcId, NpcSpawnId,
    NpcSpawnLocation, NpcTemplateState, RuntimeMonsterId, RuntimeMonsterState, RuntimeNpcId,
    RuntimeNpcState,
};
use crate::adapters::fileread::save_debug_json;
use crate::debug_ui;
use crate::domain::field::dto::FieldId;
use crate::domain::skill::state::SkillState;
use crate::domain::user_player::calc::{
    build_calc_input_from_state, calculate_user_player, recalculate_user_player_state,
};
use crate::domain::user_player::char_motion_state::CharMotionState;
use crate::domain::user_player::dto::{UserContinueSkill, UserPlayerData};
use crate::domain::user_player::state::{PlayerStateSource, UserPlayerId, UserPlayerState};

use super::fields::FieldRuntimeState;
use super::handlers;
use super::scheduler::{Scheduler, SchedulerEvent};
use super::world_command::RecordDataBootstrapQueryResult;
use super::world_command::RecordDataResponseContext;
use super::world_command::RecordDataWireSnapshot;
use super::world_command::WorldCommand;
use crate::world_transport::WorldNetCommand;

pub const WORLD_TICK_RATE_HZ: u64 = 70;
pub const WORLD_TICK_INTERVAL: Duration = Duration::from_nanos(1_000_000_000 / WORLD_TICK_RATE_HZ);

#[derive(Debug)]
enum RecordDataBootstrapStatus {
    Pending,
    Ready,
    Failed {
        object_serial: u32,
        login_safe_key: i32,
    },
}

#[derive(Debug)]
struct RecordDataBootstrapSession {
    request_id: u64,
    player_name: String,
    mode: i32,
    object_serial: Option<u32>,
    player_data: Option<UserPlayerData>,
    wire_snapshot: Option<RecordDataWireSnapshot>,
    response_context: RecordDataResponseContext,
    status: RecordDataBootstrapStatus,
}

#[derive(Debug, Serialize)]
struct DebugPlayerStateSnapshot {
    trigger: String,
    player_id: String,
    player_state: UserPlayerState,
    active_continue_skills: Vec<UserContinueSkill>,
    calc_debug_breakdown: Vec<String>,
}

pub struct World {
    pub players: HashMap<UserPlayerId, UserPlayerState>,
    pub record_data_bootstrap_sessions: HashMap<usize, RecordDataBootstrapSession>,
    pub fields: FieldRuntimeState,
    pub monster_templates: HashMap<MonsterId, MonsterTemplateState>,
    pub npc_templates: HashMap<NpcId, NpcTemplateState>,
    pub runtime_monsters: HashMap<RuntimeMonsterId, RuntimeMonsterState>,
    pub runtime_npcs: HashMap<RuntimeNpcId, RuntimeNpcState>,
    pub skill_state: SkillState,
    pub scheduler: Scheduler,
    pub rx_world: Receiver<WorldCommand>,
    pub tx_net: Sender<WorldNetCommand>,
    pub next_instance_id: u64,
}

impl World {
    pub fn new(
        skill_state: SkillState,
        rx_world: Receiver<WorldCommand>,
        tx_net: Sender<WorldNetCommand>,
    ) -> Self {
        Self {
            players: HashMap::new(),
            record_data_bootstrap_sessions: HashMap::new(),
            fields: FieldRuntimeState::default(),
            monster_templates: HashMap::new(),
            npc_templates: HashMap::new(),
            runtime_monsters: HashMap::new(),
            runtime_npcs: HashMap::new(),
            skill_state,
            scheduler: Scheduler::default(),
            rx_world,
            tx_net,
            next_instance_id: 1,
        }
    }

    pub fn allocate_instance_id(&mut self) -> u64 {
        let value = self.next_instance_id;
        self.next_instance_id = self.next_instance_id.saturating_add(1);
        value
    }

    pub fn tick(&mut self, now: Instant) {
        self.drain_commands(now);

        for event in self.scheduler.pop_due(now) {
            handlers::scheduler_events::handle_event(self, now, event);
        }

        for (player_id, player) in &self.players {
            debug_ui::publish_player_info_panel(player_id.clone(), player, &self.skill_state);
        }

        for monster in self.runtime_monsters.values() {
            debug_ui::publish_runtime_monster_panel(monster);
        }

        for npc in self.runtime_npcs.values() {
            debug_ui::publish_runtime_npc_panel(npc);
        }
    }

    fn drain_commands(&mut self, now: Instant) {
        loop {
            match self.rx_world.try_recv() {
                Ok(command) => self.handle_command(command, now),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => break,
            }
        }
    }

    fn handle_command(&mut self, command: WorldCommand, now: Instant) {
        match command {
            WorldCommand::AddPlayer { player_id, data } => {
                handlers::add_player::handle(self, player_id, data, now);
                self.spawn_debug_player_dump(player_id, "add_player");
            }
            WorldCommand::RemovePlayer {
                player_id,
                reason: _,
            } => handlers::remove_player::handle(self, player_id),
            WorldCommand::ProcessSkill {
                player_id,
                skill_code,
                from_party,
            } => {
                log::debug!(
                    "received ProcessSkill command: player_id={:?} skill_code=0x{:X} from_party={}",
                    player_id,
                    skill_code.raw(),
                    from_party
                );
                let previous_version = self.players.get(&player_id).map(|p| p.meta.state_version);
                handlers::process_continue_skill::handle(
                    self, player_id, skill_code, from_party, now,
                );
                self.spawn_debug_player_dump_if_changed(
                    player_id,
                    previous_version,
                    "process_skill",
                );
            }
            WorldCommand::ProcessTargetBuff { cast } => {
                log::debug!(
                    "received ProcessTargetBuff command: caster_id={:?} skill_code=0x{:X} point={} target_id={:?}",
                    cast.caster_id,
                    cast.skill_code.raw(),
                    cast.point,
                    cast.target_id
                );
                let caster_id = cast.caster_id;
                let previous_version = self.players.get(&caster_id).map(|p| p.meta.state_version);
                handlers::process_target_buff_skill::handle(self, cast, now);
                self.spawn_debug_player_dump_if_changed(
                    caster_id,
                    previous_version,
                    "process_target_buff_skill",
                );
            }
            WorldCommand::ProcessPartySkill { cast } => {
                log::debug!(
                    "received ProcessPartySkill command: caster_id={:?} skill_code=0x{:X} point={} targets={}",
                    cast.caster_id,
                    cast.skill_code.raw(),
                    cast.point,
                    cast.targets.len()
                );
                let caster_id = cast.caster_id;
                let previous_version = self.players.get(&caster_id).map(|p| p.meta.state_version);
                handlers::process_party_skill::handle(self, cast, now);
                self.spawn_debug_player_dump_if_changed(
                    caster_id,
                    previous_version,
                    "process_party_skill",
                );
            }
            WorldCommand::CancelSkill {
                player_id,
                skill_code,
            } => {
                log::debug!(
                    "received CancelSkill command: player_id={:?} skill_code=0x{:X}",
                    player_id,
                    skill_code.raw()
                );
                let previous_version = self.players.get(&player_id).map(|p| p.meta.state_version);
                handlers::cancel_skill::handle(self, player_id, skill_code);
                self.spawn_debug_player_dump_if_changed(
                    player_id,
                    previous_version,
                    "cancel_skill",
                );
            }
            WorldCommand::UpsertPlayerField {
                player_id,
                field_id,
                observed_at,
            } => {
                let previous_version = self.players.get(&player_id).map(|p| p.meta.state_version);
                if let Some(previous_field_id) =
                    self.fields.player_field.insert(player_id, field_id)
                {
                    if previous_field_id != field_id {
                        if let Some(players) = self.fields.field_players.get_mut(&previous_field_id)
                        {
                            players.remove(&player_id);
                        }
                    }
                }
                let _ = observed_at;
                self.fields
                    .field_players
                    .entry(field_id)
                    .or_default()
                    .insert(player_id);
                self.remove_active_buffs_if_field_disallows_skills(player_id, field_id);
                self.spawn_debug_player_dump_if_changed(
                    player_id,
                    previous_version,
                    "upsert_player_field",
                );
            }
            WorldCommand::UpsertPlayerPosition {
                player_id,
                x,
                y,
                z,
                state,
                observed_at: _,
            } => {
                self.fields.player_position.insert(player_id, (x, y, z));
                if let Some(player) = self.players.get_mut(&player_id) {
                    player.runtime.map_position = Some((x, y, z));
                    if !matches!(state, CharMotionState::Unknown(_)) {
                        player.core.char_info.state = state;
                    }
                }
            }
            WorldCommand::SyncCharMotionState {
                player_id,
                state,
                observed_at,
            } => {
                let _ = observed_at;
                if let Some(player) = self.players.get_mut(&player_id) {
                    player.core.char_info.state = state;
                    player.touch_update(PlayerStateSource::HandlerUpdate);
                }
            }
            WorldCommand::LoadGameServerWarmup { data, observed_at } => {
                let _ = observed_at;
                let (monster_templates, npc_templates, field_bindings) =
                    Self::build_game_server_runtime_indexes(&data);
                self.monster_templates = monster_templates;
                self.npc_templates = npc_templates;
                self.fields.field_spawn_config = field_bindings.field_spawn_config;
                self.fields.monster_template_fields = field_bindings.monster_template_fields;
                self.fields.field_monster_templates = field_bindings.field_monster_templates;
                self.fields.npc_template_fields = field_bindings.npc_template_fields;
                self.fields.field_npc_templates = field_bindings.field_npc_templates;
                log::info!(
                    "loaded filtered game-server data: fields={} monsters={} npcs={} open_items={} monster_templates={} npc_templates={} field_spawn_configs={} monster_template_fields={} npc_template_fields={}",
                    data.fields.len(),
                    data.monsters.len(),
                    data.npcs.len(),
                    data.open_items.len(),
                    self.monster_templates.len(),
                    self.npc_templates.len(),
                    self.fields.field_spawn_config.len(),
                    self.fields.monster_template_fields.len(),
                    self.fields.npc_template_fields.len()
                );
            }
            WorldCommand::ResetRuntimeScope {
                field_id,
                include_monsters,
                include_npcs,
                observed_at,
            } => {
                let _ = observed_at;
                self.reset_runtime_scope(field_id, include_monsters, include_npcs);
            }
            WorldCommand::UpsertRuntimeMonster { state, observed_at } => {
                let _ = observed_at;
                self.upsert_runtime_monster(state);
            }
            WorldCommand::RemoveRuntimeMonster {
                runtime_monster_id,
                observed_at,
            } => {
                let _ = observed_at;
                self.remove_runtime_monster(runtime_monster_id);
            }
            WorldCommand::UpsertRuntimeNpc { state, observed_at } => {
                let _ = observed_at;
                self.upsert_runtime_npc(state);
            }
            WorldCommand::RemoveRuntimeNpc {
                runtime_npc_id,
                observed_at,
            } => {
                let _ = observed_at;
                self.remove_runtime_npc(runtime_npc_id);
            }
            WorldCommand::RuntimeSnapshotCompleted {
                request_id,
                sequence,
                field_id,
                observed_at,
            } => {
                let _ = observed_at;
                self.log_runtime_snapshot_summary(request_id, sequence, field_id);
            }
            WorldCommand::BeginRecordDataBootstrap {
                connection_id,
                request_id,
                player_name,
                mode,
                observed_at,
            } => {
                let _ = observed_at;
                self.record_data_bootstrap_sessions.insert(
                    connection_id,
                    RecordDataBootstrapSession {
                        request_id,
                        player_name,
                        mode,
                        object_serial: None,
                        player_data: None,
                        wire_snapshot: None,
                        response_context: RecordDataResponseContext::default(),
                        status: RecordDataBootstrapStatus::Pending,
                    },
                );
            }
            WorldCommand::StoreBootstrappedRecordData {
                connection_id,
                object_serial,
                player_data,
                wire_snapshot,
                response_context,
                observed_at,
            } => {
                let _ = observed_at;
                if let Some(session) = self.record_data_bootstrap_sessions.get_mut(&connection_id) {
                    session.object_serial = Some(object_serial);
                    session.player_data = Some(player_data);
                    session.wire_snapshot = Some(wire_snapshot);
                    session.response_context = response_context;
                    session.status = RecordDataBootstrapStatus::Ready;
                }
            }
            WorldCommand::StoreBootstrappedFailRecordData {
                connection_id,
                object_serial,
                login_safe_key,
                observed_at,
            } => {
                let _ = observed_at;
                if let Some(session) = self.record_data_bootstrap_sessions.get_mut(&connection_id) {
                    session.status = RecordDataBootstrapStatus::Failed {
                        object_serial,
                        login_safe_key,
                    };
                }
            }
            WorldCommand::ClearRecordDataBootstrap {
                connection_id,
                observed_at,
            } => {
                let _ = observed_at;
                self.record_data_bootstrap_sessions.remove(&connection_id);
            }
            WorldCommand::QueryRecordDataBootstrap {
                connection_id,
                request_id,
                reply,
            } => {
                let result = self.query_record_data_bootstrap(connection_id, request_id);
                let _ = reply.send(result);
            }
        }
    }

    fn query_record_data_bootstrap(
        &self,
        connection_id: usize,
        request_id: u64,
    ) -> RecordDataBootstrapQueryResult {
        let Some(session) = self.record_data_bootstrap_sessions.get(&connection_id) else {
            return RecordDataBootstrapQueryResult::Missing;
        };

        if session.request_id != request_id {
            return RecordDataBootstrapQueryResult::Superseded;
        }

        let _ = (&session.player_name, session.mode);

        match session.status {
            RecordDataBootstrapStatus::Pending => RecordDataBootstrapQueryResult::Pending,
            RecordDataBootstrapStatus::Ready => {
                let Some(player_data) = session.player_data.clone() else {
                    return RecordDataBootstrapQueryResult::Pending;
                };
                let Some(wire_snapshot) = session.wire_snapshot.clone() else {
                    return RecordDataBootstrapQueryResult::Pending;
                };

                RecordDataBootstrapQueryResult::Ready {
                    player_data,
                    wire_snapshot,
                    response_context: session.response_context.clone(),
                }
            }
            RecordDataBootstrapStatus::Failed {
                object_serial,
                login_safe_key,
            } => RecordDataBootstrapQueryResult::Failed {
                object_serial,
                login_safe_key,
            },
        }
    }

    fn reset_runtime_scope(
        &mut self,
        field_id: Option<FieldId>,
        include_monsters: bool,
        include_npcs: bool,
    ) {
        if include_monsters {
            let runtime_monster_ids = match field_id {
                Some(field_id) => self
                    .fields
                    .field_runtime_monsters
                    .get(&field_id)
                    .cloned()
                    .unwrap_or_default()
                    .into_iter()
                    .collect::<Vec<_>>(),
                None => self.runtime_monsters.keys().copied().collect::<Vec<_>>(),
            };

            for runtime_monster_id in runtime_monster_ids {
                self.remove_runtime_monster(runtime_monster_id);
            }
        }

        if include_npcs {
            let runtime_npc_ids = match field_id {
                Some(field_id) => self
                    .fields
                    .field_runtime_npcs
                    .get(&field_id)
                    .cloned()
                    .unwrap_or_default()
                    .into_iter()
                    .collect::<Vec<_>>(),
                None => self.runtime_npcs.keys().copied().collect::<Vec<_>>(),
            };

            for runtime_npc_id in runtime_npc_ids {
                self.remove_runtime_npc(runtime_npc_id);
            }
        }
    }

    fn upsert_runtime_monster(&mut self, state: RuntimeMonsterState) {
        let runtime_monster_id = state.id;
        let next_field_id = state.field_id;
        let next_position = (state.position.x, state.position.y, state.position.z);

        let previous_field_id = self
            .runtime_monsters
            .get(&runtime_monster_id)
            .and_then(|current| current.field_id);

        if let Some(previous_field_id) = previous_field_id {
            if Some(previous_field_id) != next_field_id {
                if let Some(field_runtime_monsters) = self
                    .fields
                    .field_runtime_monsters
                    .get_mut(&previous_field_id)
                {
                    field_runtime_monsters.remove(&runtime_monster_id);
                }
            }
        }

        match next_field_id {
            Some(field_id) => {
                self.fields
                    .runtime_monster_field
                    .insert(runtime_monster_id, field_id);
                self.fields
                    .field_runtime_monsters
                    .entry(field_id)
                    .or_default()
                    .insert(runtime_monster_id);
            }
            None => {
                self.fields
                    .runtime_monster_field
                    .remove(&runtime_monster_id);
            }
        }

        self.fields
            .runtime_monster_position
            .insert(runtime_monster_id, next_position);
        self.runtime_monsters.insert(runtime_monster_id, state);
    }

    fn remove_runtime_monster(&mut self, runtime_monster_id: RuntimeMonsterId) {
        if let Some(previous_field_id) = self
            .runtime_monsters
            .remove(&runtime_monster_id)
            .and_then(|state| state.field_id)
        {
            if let Some(field_runtime_monsters) = self
                .fields
                .field_runtime_monsters
                .get_mut(&previous_field_id)
            {
                field_runtime_monsters.remove(&runtime_monster_id);
            }
        }

        self.fields
            .runtime_monster_field
            .remove(&runtime_monster_id);
        self.fields
            .runtime_monster_position
            .remove(&runtime_monster_id);
        debug_ui::remove_runtime_monster_panel(runtime_monster_id);
    }

    fn upsert_runtime_npc(&mut self, state: RuntimeNpcState) {
        let runtime_npc_id = state.id;
        let next_field_id = state.field_id;
        let next_position = (state.position.x, state.position.y, state.position.z);

        let previous_field_id = self
            .runtime_npcs
            .get(&runtime_npc_id)
            .and_then(|current| current.field_id);

        if let Some(previous_field_id) = previous_field_id {
            if Some(previous_field_id) != next_field_id {
                if let Some(field_runtime_npcs) =
                    self.fields.field_runtime_npcs.get_mut(&previous_field_id)
                {
                    field_runtime_npcs.remove(&runtime_npc_id);
                }
            }
        }

        match next_field_id {
            Some(field_id) => {
                self.fields
                    .runtime_npc_field
                    .insert(runtime_npc_id, field_id);
                self.fields
                    .field_runtime_npcs
                    .entry(field_id)
                    .or_default()
                    .insert(runtime_npc_id);
            }
            None => {
                self.fields.runtime_npc_field.remove(&runtime_npc_id);
            }
        }

        self.fields
            .runtime_npc_position
            .insert(runtime_npc_id, next_position);
        self.runtime_npcs.insert(runtime_npc_id, state);
    }

    fn remove_runtime_npc(&mut self, runtime_npc_id: RuntimeNpcId) {
        if let Some(previous_field_id) = self
            .runtime_npcs
            .remove(&runtime_npc_id)
            .and_then(|state| state.field_id)
        {
            if let Some(field_runtime_npcs) =
                self.fields.field_runtime_npcs.get_mut(&previous_field_id)
            {
                field_runtime_npcs.remove(&runtime_npc_id);
            }
        }

        self.fields.runtime_npc_field.remove(&runtime_npc_id);
        self.fields.runtime_npc_position.remove(&runtime_npc_id);
        debug_ui::remove_runtime_npc_panel(runtime_npc_id);
    }

    fn log_runtime_snapshot_summary(
        &self,
        request_id: u32,
        sequence: u64,
        field_id: Option<FieldId>,
    ) {
        let monster_count = match field_id {
            Some(field_id) => self
                .fields
                .field_runtime_monsters
                .get(&field_id)
                .map(|ids| ids.len())
                .unwrap_or_default(),
            None => self.runtime_monsters.len(),
        };
        let npc_count = match field_id {
            Some(field_id) => self
                .fields
                .field_runtime_npcs
                .get(&field_id)
                .map(|ids| ids.len())
                .unwrap_or_default(),
            None => self.runtime_npcs.len(),
        };
        let monster_field_count = self
            .fields
            .field_runtime_monsters
            .values()
            .filter(|runtime_ids| !runtime_ids.is_empty())
            .count();
        let npc_field_count = self
            .fields
            .field_runtime_npcs
            .values()
            .filter(|runtime_ids| !runtime_ids.is_empty())
            .count();

        log::info!(
            "world runtime snapshot applied: request_id={} sequence={} scope_field_id={:?} runtime_monsters={} runtime_npcs={} runtime_monster_fields={} runtime_npc_fields={}",
            request_id,
            sequence,
            field_id,
            monster_count,
            npc_count,
            monster_field_count,
            npc_field_count
        );
    }

    fn build_game_server_runtime_indexes(
        data: &GameServerData,
    ) -> (
        HashMap<MonsterId, MonsterTemplateState>,
        HashMap<NpcId, NpcTemplateState>,
        RuntimeFieldBindings,
    ) {
        let mut monster_templates = data
            .monsters
            .iter()
            .cloned()
            .map(|monster| {
                (
                    monster.id,
                    MonsterTemplateState {
                        monster,
                        field_ids: HashSet::new(),
                    },
                )
            })
            .collect::<HashMap<_, _>>();
        let npc_templates = data
            .npcs
            .iter()
            .cloned()
            .map(|npc| (npc.id.clone(), NpcTemplateState { npc }))
            .collect::<HashMap<_, _>>();
        let mut field_bindings = RuntimeFieldBindings::default();
        let mut next_npc_spawn_id = 1_u64;

        for field in &data.fields {
            let Some(field_id) = field.field_id else {
                continue;
            };

            let mut npc_spawns = Vec::new();

            for spawn in &field.npc_spawns {
                let Some(npc_id) = spawn.npc_id.clone() else {
                    continue;
                };
                let spawn_id = NpcSpawnId(next_npc_spawn_id);
                next_npc_spawn_id = next_npc_spawn_id.saturating_add(1);

                field_bindings
                    .field_npc_templates
                    .entry(field_id)
                    .or_default()
                    .insert(npc_id.clone());
                field_bindings
                    .npc_template_fields
                    .entry(npc_id.clone())
                    .or_default()
                    .insert(field_id);
                npc_spawns.push(NpcSpawnLocation {
                    spawn_id,
                    npc_id,
                    field_id,
                    position: spawn.position.clone(),
                    direction: spawn.direction.clone(),
                });
            }

            for rule in &field.monster_spawn_rules {
                let Some(monster_id) = rule.monster_id else {
                    continue;
                };

                field_bindings
                    .field_monster_templates
                    .entry(field_id)
                    .or_default()
                    .insert(monster_id);
                field_bindings
                    .monster_template_fields
                    .entry(monster_id)
                    .or_default()
                    .insert(field_id);

                if let Some(monster_state) = monster_templates.get_mut(&monster_id) {
                    monster_state.field_ids.insert(field_id);
                }
            }

            field_bindings.field_spawn_config.insert(
                field_id,
                FieldSpawnConfigState {
                    npc_spawns,
                    monster_spawn_points: field.monster_spawn_points.clone(),
                    monster_spawn_rules: field.monster_spawn_rules.clone(),
                    boss_spawn_rules: field.boss_spawn_rules.clone(),
                },
            );
        }

        (monster_templates, npc_templates, field_bindings)
    }

    pub fn schedule_regen_for_player(&mut self, player_id: UserPlayerId, now: Instant) {
        self.scheduler.push(
            now + WORLD_TICK_INTERVAL,
            SchedulerEvent::RegenTick { player_id },
        );
    }

    pub fn player_state_version(&self, player_id: UserPlayerId) -> Option<u64> {
        self.players.get(&player_id).map(|p| p.meta.state_version)
    }

    pub fn spawn_debug_player_dump_if_changed(
        &self,
        player_id: UserPlayerId,
        previous_version: Option<u64>,
        trigger: &'static str,
    ) {
        let Some(current_version) = self.player_state_version(player_id) else {
            return;
        };

        if previous_version == Some(current_version) {
            return;
        }

        self.spawn_debug_player_dump(player_id, trigger);
    }

    pub fn spawn_debug_player_dump(&self, player_id: UserPlayerId, trigger: &'static str) {
        let Some(player_state) = self.players.get(&player_id).cloned() else {
            return;
        };

        let snapshot = DebugPlayerStateSnapshot {
            trigger: trigger.to_string(),
            player_id: player_id.0.to_string(),
            active_continue_skills: player_state.active_continue_skills_vec(),
            calc_debug_breakdown: calculate_user_player(
                &build_calc_input_from_state(&player_state),
                Some(&self.skill_state),
            )
            .debug_breakdown,
            player_state,
        };

        thread::spawn(move || {
            if let Err(err) = save_debug_json(&snapshot) {
                log::warn!("failed to write debug player state JSON: {:?}", err);
            }
        });
    }

    fn remove_active_buffs_if_field_disallows_skills(
        &mut self,
        player_id: UserPlayerId,
        field_id: FieldId,
    ) {
        if field_id
            .get_catalog_entry()
            .policy
            .allows_skill_restrictive()
        {
            return;
        }

        let Some(player) = self.players.get_mut(&player_id) else {
            return;
        };

        if player.clear_active_continue_skills() {
            recalculate_user_player_state(player, Some(&self.skill_state));
            log::debug!(
                "field_policy_removed_active_buffs player_id={:?} field_id={}",
                player_id.0,
                field_id.raw()
            );
        }
    }
}

#[derive(Default)]
struct RuntimeFieldBindings {
    field_spawn_config: HashMap<FieldId, FieldSpawnConfigState>,
    monster_template_fields: HashMap<MonsterId, HashSet<FieldId>>,
    field_monster_templates: HashMap<FieldId, HashSet<MonsterId>>,
    npc_template_fields: HashMap<NpcId, HashSet<FieldId>>,
    field_npc_templates: HashMap<FieldId, HashSet<NpcId>>,
}

#[cfg(test)]
mod buff_field_tests {
    use std::collections::HashMap;
    use std::sync::mpsc;
    use std::time::Instant;

    use crate::domain::field::dto::FieldId;
    use crate::domain::skill::dtos::codes::SKILL_VIRTUAL_LIFE;
    use crate::domain::skill::dtos::skills::{
        Levels, SkillClassConfig, SkillEntry, SkillValueType, VirtualLife,
    };
    use crate::domain::skill::state::SkillState;
    use crate::domain::user_player::dto::{
        ChangeJobTierState, PlayerJob, UserCalcOutput, UserPlayerData,
    };
    use crate::domain::user_player::state::UserPlayerId;

    use super::*;

    fn mk_levels_i32(v: i32) -> Levels<i32> {
        Levels::new(SkillValueType::Fixed2, [v; 10])
    }

    fn mk_virtual_life_state() -> SkillState {
        let virtual_life = SkillEntry::VirtualLife(VirtualLife {
            name: "VirtualLife".to_string(),
            description: "test".to_string(),
            require_level: 1,
            use_stamina: None,
            require_mastery: None,
            element: None,
            use_weapon_code: vec![],
            skill_code: SKILL_VIRTUAL_LIFE,
            percent: mk_levels_i32(42),
            time: mk_levels_i32(300),
            use_mana: mk_levels_i32(0),
        });
        let mut by_code = HashMap::new();
        by_code.insert(SKILL_VIRTUAL_LIFE, virtual_life);
        SkillState {
            state: HashMap::from([(PlayerJob::Priestess, SkillClassConfig { by_code })]),
        }
    }

    #[test]
    fn upsert_player_field_removes_active_buffs_when_target_field_blocks_skills() {
        let (tx_net, _rx_net) = mpsc::channel();
        let (_tx_world, rx_world) = mpsc::channel();
        let mut world = World::new(mk_virtual_life_state(), rx_world, tx_net);
        let player_id = UserPlayerId(654);

        let mut player_state = UserPlayerState::from_record_data(
            1,
            UserPlayerData::default(),
            UserCalcOutput::default(),
        );
        player_state.core.char_info.job_code = PlayerJob::Priestess;
        player_state.core.char_info.change_job = ChangeJobTierState::Tier4;
        player_state.core.char_info.level = 140;
        player_state.core.char_info.spirit = 820;
        player_state.core.char_info.health = 20;
        player_state.core.char_info.strength = 78;
        player_state.core.char_info.talent = 86;
        player_state.core.char_info.dexterity = 80;
        player_state.core.char_info.life = [1000, 0];
        player_state.progression.game_save.skill_info.b_skill_point[11] = 10;
        let baseline = calculate_user_player(
            &build_calc_input_from_state(&player_state),
            Some(&world.skill_state),
        )
        .updated_char_info
        .life[1];

        world.players.insert(player_id, player_state);
        world.fields.player_field.insert(player_id, FieldId::Castle);

        crate::application::use_case::runtime::world_runtime::handlers::process_continue_skill::handle(
            &mut world,
            player_id,
            SKILL_VIRTUAL_LIFE,
            false,
            Instant::now(),
        );

        world.handle_command(
            WorldCommand::UpsertPlayerField {
                player_id,
                field_id: FieldId::Pilai,
                observed_at: Instant::now(),
            },
            Instant::now(),
        );

        assert_eq!(
            world
                .players
                .get(&player_id)
                .map(|player| player.core.char_info.life[0]),
            Some(1000)
        );
        assert_eq!(
            world
                .players
                .get(&player_id)
                .map(|player| player.core.char_info.life[1]),
            Some(baseline)
        );
        assert_eq!(
            world
                .players
                .get(&player_id)
                .map(|player| player.runtime.active_continue_skills.is_empty()),
            Some(true)
        );
        assert_eq!(
            world
                .players
                .get(&player_id)
                .map(|player| player.runtime.active_buff_codes.is_empty()),
            Some(true)
        );
        assert_eq!(
            world.fields.player_field.get(&player_id),
            Some(&FieldId::Pilai)
        );
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::sync::mpsc;
    use std::time::Instant;

    use crate::application::use_case::runtime::world_runtime::game_server::dto::{
        FieldBossSpawnRule, FieldMonsterSpawnPoint, FieldMonsterSpawnRule, FieldNpcSpawn,
        GameField, GameMonster, GameNpc, GameOpenItem, GameServerData, MonsterId, NpcId,
        NpcServices, RuntimeMonsterId, RuntimeMonsterState, RuntimeNpcId, RuntimeNpcState,
        WorldPoint, WorldPoint2D,
    };
    use crate::application::use_case::runtime::world_runtime::world_command::WorldCommand;
    use crate::domain::field::dto::FieldId;
    use crate::domain::skill::state::SkillState;
    use crate::domain::user_player::char_motion_state::CharMotionState;
    use crate::domain::user_player::dto::{UserCalcOutput, UserPlayerData};
    use crate::domain::user_player::state::UserPlayerId;

    use super::World;

    #[test]
    fn upsert_player_field_updates_association() {
        let (_tx_net, rx_net) = mpsc::channel();
        drop(rx_net);
        let (tx_world, rx_world) = mpsc::channel();
        let mut world = World::new(SkillState::default(), rx_world, _tx_net);
        let now = Instant::now();
        let player_id = UserPlayerId(123);

        assert!(tx_world
            .send(WorldCommand::UpsertPlayerField {
                player_id,
                field_id: FieldId::Village1,
                observed_at: now,
            })
            .is_ok());

        world.tick(now);
        assert_eq!(
            world.fields.player_field.get(&player_id).copied(),
            Some(FieldId::Village1)
        );
    }

    #[test]
    fn field_membership_is_reused_for_same_field() {
        let (_tx_net, rx_net) = mpsc::channel();
        drop(rx_net);
        let (tx_world, rx_world) = mpsc::channel();
        let mut world = World::new(SkillState::default(), rx_world, _tx_net);
        let now = Instant::now();
        let player1 = UserPlayerId(101);
        let player2 = UserPlayerId(102);
        let player3 = UserPlayerId(201);
        let player4 = UserPlayerId(202);

        for (player, field_id) in [
            (player1, FieldId::De2),
            (player2, FieldId::De2),
            (player3, FieldId::ForeverFall01),
            (player4, FieldId::ForeverFall01),
        ] {
            assert!(tx_world
                .send(WorldCommand::UpsertPlayerField {
                    player_id: player,
                    field_id,
                    observed_at: now,
                })
                .is_ok());
        }
        world.tick(now);

        assert_eq!(world.fields.field_players.len(), 2);
        assert_eq!(
            world
                .fields
                .field_players
                .get(&FieldId::De2)
                .map(|v| v.len()),
            Some(2)
        );
        assert_eq!(
            world
                .fields
                .field_players
                .get(&FieldId::ForeverFall01)
                .map(|v| v.len()),
            Some(2)
        );
    }

    #[test]
    fn sync_char_motion_state_updates_only_motion_state() {
        let (tx_net, rx_net) = mpsc::channel();
        drop(rx_net);
        let (_tx_world, rx_world) = mpsc::channel();
        let mut world = World::new(SkillState::default(), rx_world, tx_net);
        let now = Instant::now();

        let mut data = UserPlayerData::default();
        let mut output = UserCalcOutput::default();
        output.updated_char_info = data.char_info.clone();

        let player_id = UserPlayerId(777);
        world.players.insert(
            player_id,
            crate::domain::user_player::state::UserPlayerState::from_record_data(3, data, output),
        );

        world.handle_command(
            WorldCommand::SyncCharMotionState {
                player_id,
                state: CharMotionState::Run,
                observed_at: now,
            },
            now,
        );

        assert_eq!(
            world
                .players
                .get(&player_id)
                .map(|player| player.core.char_info.state),
            Some(CharMotionState::Run)
        );
    }

    #[test]
    fn upsert_player_position_keeps_existing_motion_state_when_new_state_is_unknown() {
        let (tx_net, rx_net) = mpsc::channel();
        drop(rx_net);
        let (_tx_world, rx_world) = mpsc::channel();
        let mut world = World::new(SkillState::default(), rx_world, tx_net);
        let now = Instant::now();

        let mut data = UserPlayerData::default();
        let mut output = UserCalcOutput::default();
        output.updated_char_info = data.char_info.clone();

        let player_id = UserPlayerId(888);
        let mut player =
            crate::domain::user_player::state::UserPlayerState::from_record_data(4, data, output);
        player.core.char_info.state = CharMotionState::Walk;
        world.players.insert(player_id, player);

        world.handle_command(
            WorldCommand::UpsertPlayerPosition {
                player_id,
                x: 10,
                y: 20,
                z: 30,
                state: CharMotionState::Unknown(0),
                observed_at: now,
            },
            now,
        );

        assert_eq!(
            world
                .players
                .get(&player_id)
                .and_then(|player| player.runtime.map_position),
            Some((10, 20, 30))
        );
        assert_eq!(
            world
                .players
                .get(&player_id)
                .map(|player| player.core.char_info.state),
            Some(CharMotionState::Walk)
        );
    }

    #[test]
    fn load_game_server_warmup_populates_world_catalogs_and_field_indexes() {
        let (tx_net, rx_net) = mpsc::channel();
        drop(rx_net);
        let (_tx_world, rx_world) = mpsc::channel();
        let mut world = World::new(SkillState::default(), rx_world, tx_net);
        let now = Instant::now();

        let monster_id = MonsterId::from_legacy_auto_char_code(0xAA11);
        let npc_id = NpcId::from_file_name("teleporter.npc");
        let field_id = FieldId::Village1;

        let data = GameServerData {
            fields: vec![GameField {
                index: 0,
                field_id: Some(field_id),
                stage_file: "village1".to_string(),
                map_image_file: "village1_map".to_string(),
                title_image_file: "village1_title".to_string(),
                state: 0,
                event_code: 0,
                back_music_code: 1,
                limit_level: 0,
                field_code: field_id.raw(),
                server_code: 1,
                center: WorldPoint2D { x: 10, z: 20 },
                start_points: vec![],
                warp_gates: vec![],
                npc_spawns: vec![FieldNpcSpawn {
                    npc_id: Some(npc_id.clone()),
                    name: "Teleporter".to_string(),
                    position: WorldPoint {
                        x: 100,
                        y: 200,
                        z: 300,
                    },
                    direction: WorldPoint { x: 0, y: 90, z: 0 },
                    dialog_title: Some("Teleporter".to_string()),
                }],
                monster_spawn_points: vec![FieldMonsterSpawnPoint {
                    index: 0,
                    position: WorldPoint2D { x: 1000, z: 1200 },
                    state: 1,
                    near_play_count: 2,
                    monster_count: 3,
                }],
                monster_spawn_rules: vec![FieldMonsterSpawnRule {
                    monster_id: Some(monster_id),
                    monster_name: "Goblin".to_string(),
                    open_percentage: 100,
                    spawn_start_index: 0,
                }],
                boss_spawn_rules: vec![FieldBossSpawnRule {
                    master_monster_id: Some(monster_id),
                    master_monster_name: "Goblin".to_string(),
                    slave_monster_id: Some(monster_id),
                    slave_monster_name: "Goblin".to_string(),
                    slave_count: 2,
                    open_hours: vec![3, 15, 21],
                }],
            }],
            monsters: vec![GameMonster {
                index: 0,
                id: monster_id,
                auto_char_code: monster_id.0,
                name: "Goblin".to_string(),
                model_file: "goblin.ini".to_string(),
                level: 10,
                life: 50,
                attack_rating: 5,
                attack_damage_min: 3,
                attack_damage_max: 7,
                defense: 1,
                absorption: 0,
                move_speed: 4,
                sight: 5,
                move_range: 6,
                exp: 100,
                nature: 0,
                undead: 0,
                class_code: 1,
                is_boss: false,
                drops: vec![],
            }],
            npcs: vec![GameNpc {
                index: 0,
                id: npc_id.clone(),
                file_name: npc_id.0.clone(),
                name: "Teleporter".to_string(),
                model_file: "teleporter.ini".to_string(),
                dialog_title: Some("Teleport".to_string()),
                quest_code: 0,
                quest_param: 0,
                services: NpcServices {
                    teleport: true,
                    ..NpcServices::default()
                },
            }],
            open_items: vec![GameOpenItem {
                index: 0,
                code: 1,
                name: "Potion".to_string(),
                level_requirement: 1,
                price: 100,
                weight: 1,
                attack_speed: 0,
                attack_rating: 0,
                critical_hit: 0,
                defense: 0,
                block_rating: 0.0,
                absorb: 0.0,
                min_damage: 0,
                max_damage: 0,
                durability_min: 0,
                durability_max: 0,
                job_code_mask: 0,
                unique_item: 0,
                disp_effect: 0,
            }],
        };

        world.handle_command(
            WorldCommand::LoadGameServerWarmup {
                data: data.clone(),
                observed_at: now,
            },
            now,
        );

        assert_eq!(world.monster_templates.len(), 1);
        assert_eq!(world.npc_templates.len(), 1);
        assert_eq!(
            world
                .fields
                .field_spawn_config
                .get(&field_id)
                .map(|config| config.npc_spawns.len()),
            Some(1)
        );
        assert_eq!(
            world
                .fields
                .field_spawn_config
                .get(&field_id)
                .and_then(|config| config.npc_spawns.first())
                .map(|npc_spawn| (
                    npc_spawn.position.x,
                    npc_spawn.position.y,
                    npc_spawn.position.z
                )),
            Some((100, 200, 300))
        );
        assert_eq!(
            world
                .fields
                .field_npc_templates
                .get(&field_id)
                .map(|items| items.contains(&npc_id)),
            Some(true)
        );
        assert_eq!(
            world
                .fields
                .monster_template_fields
                .get(&monster_id)
                .cloned()
                .unwrap_or_default(),
            HashSet::from([field_id])
        );
        assert_eq!(
            world
                .fields
                .field_monster_templates
                .get(&field_id)
                .cloned()
                .unwrap_or_default(),
            HashSet::from([monster_id])
        );
        assert_eq!(
            world
                .monster_templates
                .get(&monster_id)
                .map(|state| state.field_ids.clone())
                .unwrap_or_default(),
            HashSet::from([field_id])
        );
        assert!(world.npc_templates.contains_key(&npc_id));
        assert_eq!(
            world
                .fields
                .field_spawn_config
                .get(&field_id)
                .and_then(|config| config.npc_spawns.first())
                .map(|npc_spawn| npc_spawn.field_id),
            Some(field_id)
        );
        assert_eq!(
            world
                .fields
                .field_spawn_config
                .get(&field_id)
                .and_then(|config| config.npc_spawns.first())
                .map(|npc_spawn| npc_spawn.npc_id.clone()),
            Some(npc_id)
        );
        assert_eq!(
            world
                .fields
                .field_spawn_config
                .get(&field_id)
                .map(|config| config.monster_spawn_points.len()),
            Some(1)
        );
        assert_eq!(
            world
                .fields
                .field_spawn_config
                .get(&field_id)
                .map(|config| config.monster_spawn_rules.len()),
            Some(1)
        );
        assert_eq!(
            world
                .fields
                .field_spawn_config
                .get(&field_id)
                .map(|config| config.boss_spawn_rules.len()),
            Some(1)
        );
    }

    #[test]
    fn upsert_runtime_monster_updates_runtime_indexes() {
        let (tx_net, rx_net) = mpsc::channel();
        drop(rx_net);
        let (_tx_world, rx_world) = mpsc::channel();
        let mut world = World::new(SkillState::default(), rx_world, tx_net);
        let now = Instant::now();
        let runtime_monster_id = RuntimeMonsterId::from_object_serial(0x1001);

        world.handle_command(
            WorldCommand::UpsertRuntimeMonster {
                state: RuntimeMonsterState {
                    id: runtime_monster_id,
                    template_id: Some(MonsterId::from_legacy_auto_char_code(0xAA11)),
                    field_id: Some(FieldId::Village1),
                    name: "Goblin".to_string(),
                    position: WorldPoint {
                        x: 100,
                        y: 200,
                        z: 300,
                    },
                    direction: WorldPoint { x: 0, y: 90, z: 0 },
                    current_life: 40,
                    max_life: 50,
                    motion_state: 2,
                },
                observed_at: now,
            },
            now,
        );

        assert!(world.runtime_monsters.contains_key(&runtime_monster_id));
        assert_eq!(
            world.fields.runtime_monster_field.get(&runtime_monster_id),
            Some(&FieldId::Village1)
        );
        assert_eq!(
            world
                .fields
                .runtime_monster_position
                .get(&runtime_monster_id),
            Some(&(100, 200, 300))
        );
        assert_eq!(
            world
                .fields
                .field_runtime_monsters
                .get(&FieldId::Village1)
                .map(|items| items.contains(&runtime_monster_id)),
            Some(true)
        );
    }

    #[test]
    fn runtime_monster_move_between_fields_reuses_indexes() {
        let (tx_net, rx_net) = mpsc::channel();
        drop(rx_net);
        let (_tx_world, rx_world) = mpsc::channel();
        let mut world = World::new(SkillState::default(), rx_world, tx_net);
        let now = Instant::now();
        let runtime_monster_id = RuntimeMonsterId::from_object_serial(0x1002);

        for (field_id, x) in [(FieldId::Village1, 100), (FieldId::Pilai, 400)] {
            world.handle_command(
                WorldCommand::UpsertRuntimeMonster {
                    state: RuntimeMonsterState {
                        id: runtime_monster_id,
                        template_id: None,
                        field_id: Some(field_id),
                        name: "Summon".to_string(),
                        position: WorldPoint { x, y: 50, z: 60 },
                        direction: WorldPoint { x: 0, y: 0, z: 0 },
                        current_life: 10,
                        max_life: 10,
                        motion_state: 1,
                    },
                    observed_at: now,
                },
                now,
            );
        }

        assert_eq!(
            world.fields.runtime_monster_field.get(&runtime_monster_id),
            Some(&FieldId::Pilai)
        );
        assert_eq!(
            world
                .fields
                .field_runtime_monsters
                .get(&FieldId::Village1)
                .map(|items| items.contains(&runtime_monster_id)),
            Some(false)
        );
        assert_eq!(
            world
                .fields
                .field_runtime_monsters
                .get(&FieldId::Pilai)
                .map(|items| items.contains(&runtime_monster_id)),
            Some(true)
        );
    }

    #[test]
    fn remove_runtime_monster_clears_runtime_indexes() {
        let (tx_net, rx_net) = mpsc::channel();
        drop(rx_net);
        let (_tx_world, rx_world) = mpsc::channel();
        let mut world = World::new(SkillState::default(), rx_world, tx_net);
        let now = Instant::now();
        let runtime_monster_id = RuntimeMonsterId::from_object_serial(0x1003);

        world.handle_command(
            WorldCommand::UpsertRuntimeMonster {
                state: RuntimeMonsterState {
                    id: runtime_monster_id,
                    template_id: None,
                    field_id: Some(FieldId::Village1),
                    name: "Ghost".to_string(),
                    position: WorldPoint { x: 1, y: 2, z: 3 },
                    direction: WorldPoint { x: 0, y: 0, z: 0 },
                    current_life: 1,
                    max_life: 1,
                    motion_state: 0,
                },
                observed_at: now,
            },
            now,
        );
        world.handle_command(
            WorldCommand::RemoveRuntimeMonster {
                runtime_monster_id,
                observed_at: now,
            },
            now,
        );

        assert!(!world.runtime_monsters.contains_key(&runtime_monster_id));
        assert!(!world
            .fields
            .runtime_monster_field
            .contains_key(&runtime_monster_id));
        assert!(!world
            .fields
            .runtime_monster_position
            .contains_key(&runtime_monster_id));
    }

    #[test]
    fn upsert_runtime_npc_updates_runtime_indexes() {
        let (tx_net, rx_net) = mpsc::channel();
        drop(rx_net);
        let (_tx_world, rx_world) = mpsc::channel();
        let mut world = World::new(SkillState::default(), rx_world, tx_net);
        let now = Instant::now();
        let runtime_npc_id = RuntimeNpcId::from_object_serial(0x2001);
        let npc_id = NpcId::from_file_name("teleporter.npc");

        world.handle_command(
            WorldCommand::UpsertRuntimeNpc {
                state: RuntimeNpcState {
                    id: runtime_npc_id,
                    template_id: Some(npc_id),
                    field_id: Some(FieldId::Village1),
                    name: "Teleporter".to_string(),
                    position: WorldPoint {
                        x: 10,
                        y: 20,
                        z: 30,
                    },
                    direction: WorldPoint { x: 0, y: 180, z: 0 },
                    motion_state: 0,
                },
                observed_at: now,
            },
            now,
        );

        assert!(world.runtime_npcs.contains_key(&runtime_npc_id));
        assert_eq!(
            world.fields.runtime_npc_field.get(&runtime_npc_id),
            Some(&FieldId::Village1)
        );
        assert_eq!(
            world.fields.runtime_npc_position.get(&runtime_npc_id),
            Some(&(10, 20, 30))
        );
        assert_eq!(
            world
                .fields
                .field_runtime_npcs
                .get(&FieldId::Village1)
                .map(|items| items.contains(&runtime_npc_id)),
            Some(true)
        );
    }

    #[test]
    fn remove_runtime_npc_clears_runtime_indexes() {
        let (tx_net, rx_net) = mpsc::channel();
        drop(rx_net);
        let (_tx_world, rx_world) = mpsc::channel();
        let mut world = World::new(SkillState::default(), rx_world, tx_net);
        let now = Instant::now();
        let runtime_npc_id = RuntimeNpcId::from_object_serial(0x2002);

        world.handle_command(
            WorldCommand::UpsertRuntimeNpc {
                state: RuntimeNpcState {
                    id: runtime_npc_id,
                    template_id: None,
                    field_id: Some(FieldId::Pilai),
                    name: "Guard".to_string(),
                    position: WorldPoint { x: 5, y: 6, z: 7 },
                    direction: WorldPoint { x: 0, y: 0, z: 0 },
                    motion_state: 0,
                },
                observed_at: now,
            },
            now,
        );
        world.handle_command(
            WorldCommand::RemoveRuntimeNpc {
                runtime_npc_id,
                observed_at: now,
            },
            now,
        );

        assert!(!world.runtime_npcs.contains_key(&runtime_npc_id));
        assert!(!world.fields.runtime_npc_field.contains_key(&runtime_npc_id));
        assert!(!world
            .fields
            .runtime_npc_position
            .contains_key(&runtime_npc_id));
    }

    #[test]
    fn reset_runtime_scope_removes_only_requested_field_entities() {
        let (tx_net, rx_net) = mpsc::channel();
        drop(rx_net);
        let (_tx_world, rx_world) = mpsc::channel();
        let mut world = World::new(SkillState::default(), rx_world, tx_net);
        let now = Instant::now();

        world.handle_command(
            WorldCommand::UpsertRuntimeMonster {
                state: RuntimeMonsterState {
                    id: RuntimeMonsterId::from_object_serial(0x3001),
                    template_id: None,
                    field_id: Some(FieldId::Village1),
                    name: "FieldA".to_string(),
                    position: WorldPoint { x: 1, y: 1, z: 1 },
                    direction: WorldPoint { x: 0, y: 0, z: 0 },
                    current_life: 1,
                    max_life: 1,
                    motion_state: 0,
                },
                observed_at: now,
            },
            now,
        );
        world.handle_command(
            WorldCommand::UpsertRuntimeMonster {
                state: RuntimeMonsterState {
                    id: RuntimeMonsterId::from_object_serial(0x3002),
                    template_id: None,
                    field_id: Some(FieldId::Pilai),
                    name: "FieldB".to_string(),
                    position: WorldPoint { x: 2, y: 2, z: 2 },
                    direction: WorldPoint { x: 0, y: 0, z: 0 },
                    current_life: 1,
                    max_life: 1,
                    motion_state: 0,
                },
                observed_at: now,
            },
            now,
        );

        world.handle_command(
            WorldCommand::ResetRuntimeScope {
                field_id: Some(FieldId::Village1),
                include_monsters: true,
                include_npcs: false,
                observed_at: now,
            },
            now,
        );

        assert!(!world
            .runtime_monsters
            .contains_key(&RuntimeMonsterId::from_object_serial(0x3001)));
        assert!(world
            .runtime_monsters
            .contains_key(&RuntimeMonsterId::from_object_serial(0x3002)));
    }
}
