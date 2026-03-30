use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use socket::Responder;

use crate::protocol::game_server::{
    field::{
        FieldMemory, StageAreaBossRuleChunk, StageAreaMemory, StageAreaMonsterRuleChunk,
        StageAreaNpcChunk, StageAreaStartPointChunk, StageAreaWarmupSummary,
    },
    monster::MonsterMemory,
    npc::NpcMemory,
    open_item::DefItemInfo,
    GameServerWarmupMeta,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GameServerWarmupSnapshot {
    pub meta: Option<GameServerWarmupMeta>,
    pub fields: Vec<FieldMemory>,
    pub monsters: Vec<MonsterMemory>,
    pub npcs: Vec<NpcMemory>,
    pub open_items: Vec<DefItemInfo>,
    pub stage_areas: Vec<StageAreaMemory>,
}

#[derive(Debug, Default)]
pub struct GameServerWarmupCollector {
    snapshot: GameServerWarmupSnapshot,
    completed: bool,
    disconnected: bool,
}

impl GameServerWarmupCollector {
    pub fn begin(&mut self, meta: GameServerWarmupMeta) {
        self.snapshot = GameServerWarmupSnapshot {
            meta: Some(meta),
            fields: Vec::new(),
            monsters: Vec::new(),
            npcs: Vec::new(),
            open_items: Vec::new(),
            stage_areas: Vec::new(),
        };
        self.completed = false;
        self.disconnected = false;
    }

    pub fn push_field(&mut self, item: FieldMemory) {
        self.snapshot.fields.push(item);
    }

    pub fn push_monster(&mut self, item: MonsterMemory) {
        self.snapshot.monsters.push(item);
    }

    pub fn push_npc(&mut self, item: NpcMemory) {
        self.snapshot.npcs.push(item);
    }

    pub fn push_open_item(&mut self, item: DefItemInfo) {
        self.snapshot.open_items.push(item);
    }

    pub fn push_stage_area(&mut self, item: StageAreaMemory) {
        self.snapshot.stage_areas.push(item);
    }

    fn ensure_stage_area(&mut self, index: usize) -> &mut StageAreaMemory {
        if self.snapshot.stage_areas.len() <= index {
            self.snapshot
                .stage_areas
                .resize_with(index + 1, StageAreaMemory::default);
        }

        &mut self.snapshot.stage_areas[index]
    }

    pub fn apply_stage_area_summary(&mut self, index: usize, item: StageAreaWarmupSummary) {
        *self.ensure_stage_area(index) = item.into();
    }

    pub fn apply_stage_area_npcs(&mut self, chunk: StageAreaNpcChunk) {
        let stage_area = self.ensure_stage_area(chunk.stage_area_index.max(0) as usize);
        for (relative_index, npc) in chunk
            .entries
            .into_iter()
            .take(chunk.count.max(0) as usize)
            .enumerate()
        {
            let target_index = chunk.offset.max(0) as usize + relative_index;
            if target_index >= stage_area.trans_char_fixed.len() {
                break;
            }

            stage_area.trans_char_fixed[target_index] = npc;
        }
    }

    pub fn apply_stage_area_start_points(&mut self, chunk: StageAreaStartPointChunk) {
        let stage_area = self.ensure_stage_area(chunk.stage_area_index.max(0) as usize);
        for relative_index in 0..(chunk.count.max(0) as usize) {
            let target_index = chunk.offset.max(0) as usize + relative_index;
            if target_index >= stage_area.start_point.len() {
                break;
            }

            stage_area.start_point[target_index] = chunk.points[relative_index].clone();
            stage_area.start_point_near_play[target_index] = chunk.near_play[relative_index];
            stage_area.start_point_mon_count[target_index] = chunk.monster_count[relative_index];
            stage_area.dw_start_point_open_time[target_index] = chunk.open_time[relative_index];
        }
    }

    pub fn apply_stage_area_monster_rules(&mut self, chunk: StageAreaMonsterRuleChunk) {
        let stage_area = self.ensure_stage_area(chunk.stage_area_index.max(0) as usize);
        for (relative_index, rule) in chunk
            .entries
            .into_iter()
            .take(chunk.count.max(0) as usize)
            .enumerate()
        {
            let target_index = chunk.offset.max(0) as usize + relative_index;
            if target_index >= stage_area.rs_monster_list.rs_monster.len() {
                break;
            }

            stage_area.rs_monster_list.rs_monster[target_index] = rule;
        }
    }

    pub fn apply_stage_area_boss_rules(&mut self, chunk: StageAreaBossRuleChunk) {
        let stage_area = self.ensure_stage_area(chunk.stage_area_index.max(0) as usize);
        for (relative_index, rule) in chunk
            .entries
            .into_iter()
            .take(chunk.count.max(0) as usize)
            .enumerate()
        {
            let target_index = chunk.offset.max(0) as usize + relative_index;
            if target_index >= stage_area.rs_monster_list.s_boss_monsters.len() {
                break;
            }

            stage_area.rs_monster_list.s_boss_monsters[target_index] = rule;
        }
    }

    pub fn take(&mut self) -> GameServerWarmupSnapshot {
        self.completed = true;
        std::mem::take(&mut self.snapshot)
    }

    pub fn snapshot(&self) -> GameServerWarmupSnapshot {
        self.snapshot.clone()
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    pub fn mark_disconnected(&mut self) {
        self.disconnected = true;
    }

    pub fn is_disconnected(&self) -> bool {
        self.disconnected
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeSequenceSource {
    SnapshotBegin { request_id: u32 },
    SnapshotEnd { request_id: u32 },
    MonsterSync,
    MonsterDespawn,
    NpcSync,
    NpcDespawn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeResyncRequest {
    pub request_id: u32,
    pub field_id: i32,
}

#[derive(Default)]
pub struct GameServerRuntimeSyncTracker {
    snapshot_started: bool,
    snapshot_completed: bool,
    disconnected: bool,
    last_request_id: Option<u32>,
    last_sequence: Option<u64>,
    last_field_id: Option<i32>,
    current_snapshot_sequence: Option<u64>,
    pending_field_resyncs: HashSet<i32>,
    runtime_responder: Option<Responder>,
    next_request_id: u32,
}

impl std::fmt::Debug for GameServerRuntimeSyncTracker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GameServerRuntimeSyncTracker")
            .field("snapshot_started", &self.snapshot_started)
            .field("snapshot_completed", &self.snapshot_completed)
            .field("disconnected", &self.disconnected)
            .field("last_request_id", &self.last_request_id)
            .field("last_sequence", &self.last_sequence)
            .field("last_field_id", &self.last_field_id)
            .field("current_snapshot_sequence", &self.current_snapshot_sequence)
            .field("pending_field_resyncs", &self.pending_field_resyncs)
            .field(
                "has_runtime_responder",
                &self
                    .runtime_responder
                    .as_ref()
                    .map(|_| true)
                    .unwrap_or(false),
            )
            .field("next_request_id", &self.next_request_id)
            .finish()
    }
}

impl GameServerRuntimeSyncTracker {
    pub fn bind_runtime_responder(&mut self, responder: Responder) {
        self.runtime_responder = Some(responder);
        self.disconnected = false;
    }

    pub fn runtime_responder(&self) -> Option<Responder> {
        self.runtime_responder.clone()
    }

    pub fn reset_for_new_session(&mut self) {
        self.snapshot_started = false;
        self.snapshot_completed = false;
        self.disconnected = false;
        self.last_request_id = None;
        self.last_sequence = None;
        self.last_field_id = None;
        self.current_snapshot_sequence = None;
        self.pending_field_resyncs.clear();
        self.runtime_responder = None;
        self.next_request_id = self.next_request_id.max(10_000);
    }

    pub fn mark_snapshot_begin(&mut self, request_id: u32, sequence: u64, field_id: i32) {
        self.snapshot_started = true;
        self.snapshot_completed = false;
        self.disconnected = false;
        self.last_request_id = Some(request_id);
        self.last_sequence = Some(sequence);
        self.last_field_id = Some(field_id);
        self.current_snapshot_sequence = Some(sequence);
        self.pending_field_resyncs.remove(&field_id);
    }

    pub fn mark_snapshot_completed(&mut self, request_id: u32, sequence: u64, field_id: i32) {
        self.snapshot_started = true;
        self.snapshot_completed = true;
        self.last_request_id = Some(request_id);
        self.last_sequence = Some(sequence);
        self.last_field_id = Some(field_id);
        if self.current_snapshot_sequence == Some(sequence) {
            self.current_snapshot_sequence = None;
        }
        self.pending_field_resyncs.remove(&field_id);
    }

    pub fn mark_disconnected(&mut self) {
        self.disconnected = true;
        self.runtime_responder = None;
        self.current_snapshot_sequence = None;
        self.pending_field_resyncs.clear();
    }

    pub fn snapshot_completed(&self) -> bool {
        self.snapshot_completed
    }

    pub fn is_disconnected(&self) -> bool {
        self.disconnected
    }

    pub fn observe_sequence(
        &mut self,
        sequence: u64,
        field_id: i32,
        source: RuntimeSequenceSource,
    ) -> Option<RuntimeResyncRequest> {
        let is_same_snapshot_sequence = self.current_snapshot_sequence == Some(sequence);

        let needs_resync = if is_same_snapshot_sequence {
            false
        } else {
            self.last_sequence
                .is_some_and(|last_sequence| sequence > last_sequence.saturating_add(1))
        };

        self.last_sequence = Some(sequence);
        self.last_field_id = Some(field_id);
        if let RuntimeSequenceSource::SnapshotBegin { request_id }
        | RuntimeSequenceSource::SnapshotEnd { request_id } = source
        {
            self.last_request_id = Some(request_id);
        }

        if !needs_resync {
            return None;
        }

        let resync_field_id = if field_id < 0 { -1 } else { field_id };
        if !self.pending_field_resyncs.insert(resync_field_id) {
            return None;
        }

        let request_id = self.allocate_request_id();
        Some(RuntimeResyncRequest {
            request_id,
            field_id: resync_field_id,
        })
    }

    fn allocate_request_id(&mut self) -> u32 {
        let next = self.next_request_id.max(10_000);
        self.next_request_id = next.saturating_add(1);
        next
    }
}

#[cfg(test)]
mod tests {
    use super::{GameServerRuntimeSyncTracker, RuntimeSequenceSource};

    #[test]
    fn repeated_snapshot_packets_do_not_trigger_gap_resync() {
        let mut tracker = GameServerRuntimeSyncTracker::default();

        assert!(tracker
            .observe_sequence(
                10,
                3,
                RuntimeSequenceSource::SnapshotBegin { request_id: 1 }
            )
            .is_none());
        tracker.mark_snapshot_begin(1, 10, 3);

        assert!(tracker
            .observe_sequence(10, 3, RuntimeSequenceSource::MonsterSync)
            .is_none());
        assert!(tracker
            .observe_sequence(10, 3, RuntimeSequenceSource::NpcSync)
            .is_none());

        assert!(tracker
            .observe_sequence(10, 3, RuntimeSequenceSource::SnapshotEnd { request_id: 1 })
            .is_none());
    }

    #[test]
    fn gap_after_runtime_packet_requests_field_resync_once() {
        let mut tracker = GameServerRuntimeSyncTracker::default();

        assert!(tracker
            .observe_sequence(20, 7, RuntimeSequenceSource::MonsterSync)
            .is_none());

        let request = tracker.observe_sequence(23, 7, RuntimeSequenceSource::NpcSync);
        assert_eq!(request.map(|item| item.field_id), Some(7));

        assert!(tracker
            .observe_sequence(30, 7, RuntimeSequenceSource::MonsterDespawn)
            .is_none());
    }

    #[test]
    fn full_scope_gap_requests_global_resync() {
        let mut tracker = GameServerRuntimeSyncTracker::default();

        assert!(tracker
            .observe_sequence(5, -1, RuntimeSequenceSource::MonsterSync)
            .is_none());

        let request = tracker.observe_sequence(
            8,
            -1,
            RuntimeSequenceSource::SnapshotBegin { request_id: 2 },
        );
        assert_eq!(request.map(|item| item.field_id), Some(-1));
    }
}
