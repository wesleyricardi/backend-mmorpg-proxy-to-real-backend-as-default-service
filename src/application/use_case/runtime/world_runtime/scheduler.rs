use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::Instant;

use crate::domain::skill::dtos::codes::SkillCode;
use crate::domain::user_player::state::{ContinueSkillCode, UserPlayerId};

#[derive(Debug, Clone)]
pub enum SchedulerEvent {
    BuffExpire {
        player_id: UserPlayerId,
        continue_skill_code: ContinueSkillCode,
        instance_id: u64,
    },
    CooldownReady {
        player_id: UserPlayerId,
        skill_code: SkillCode,
    },
    RegenTick {
        player_id: UserPlayerId,
    },
}

#[derive(Debug, Clone)]
pub struct ScheduledItem {
    pub deadline: Instant,
    pub event: SchedulerEvent,
}

impl PartialEq for ScheduledItem {
    fn eq(&self, other: &Self) -> bool {
        self.deadline.eq(&other.deadline)
    }
}

impl Eq for ScheduledItem {}

impl PartialOrd for ScheduledItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScheduledItem {
    fn cmp(&self, other: &Self) -> Ordering {
        // reverse for min-heap behavior
        other.deadline.cmp(&self.deadline)
    }
}

#[derive(Default)]
pub struct Scheduler {
    heap: BinaryHeap<ScheduledItem>,
}

impl Scheduler {
    pub fn push(&mut self, deadline: Instant, event: SchedulerEvent) {
        self.heap.push(ScheduledItem { deadline, event });
    }

    pub fn pop_due(&mut self, now: Instant) -> Vec<SchedulerEvent> {
        let mut out = Vec::new();
        while let Some(top) = self.heap.peek() {
            if top.deadline > now {
                break;
            }
            if let Some(item) = self.heap.pop() {
                out.push(item.event);
            }
        }
        out
    }
}
