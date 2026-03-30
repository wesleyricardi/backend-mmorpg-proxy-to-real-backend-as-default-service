mod app;
mod player_panel;
mod runtime_panel;
mod shared;
mod store;

use std::sync::mpsc::{self, Sender};
use std::sync::{Mutex, OnceLock};
use std::thread;
use std::time::{Duration, Instant};

use chrono::Local;

pub use store::{DebugPanel, DebugPanelStore};

use crate::domain::skill::state::SkillState;
use crate::domain::user_player::state::{UserPlayerId, UserPlayerState};

#[derive(Debug, Clone, PartialEq, Eq)]
enum DebugUiMessage {
    Upsert(DebugPanel),
    Remove(String),
}

#[derive(Debug, Clone)]
struct DebugPanelPublishState {
    title: String,
    fields: Vec<(String, String)>,
    published_at: Instant,
}

static DEBUG_UI_SENDER: OnceLock<Mutex<Option<Sender<DebugUiMessage>>>> = OnceLock::new();
static DEBUG_UI_PUBLISH_STATE: OnceLock<Mutex<std::collections::HashMap<String, DebugPanelPublishState>>> =
    OnceLock::new();

fn sender_slot() -> &'static Mutex<Option<Sender<DebugUiMessage>>> {
    DEBUG_UI_SENDER.get_or_init(|| Mutex::new(None))
}

fn publish_state_slot(
) -> &'static Mutex<std::collections::HashMap<String, DebugPanelPublishState>> {
    DEBUG_UI_PUBLISH_STATE.get_or_init(|| Mutex::new(std::collections::HashMap::new()))
}

pub(crate) fn clear_sender() {
    if let Ok(mut slot) = sender_slot().lock() {
        *slot = None;
    }

    if let Ok(mut slot) = publish_state_slot().lock() {
        slot.clear();
    }
}

pub fn init(enabled: bool) {
    if !enabled {
        log::info!("debug UI disabled");
        return;
    }

    let Ok(mut slot) = sender_slot().lock() else {
        log::warn!("failed to acquire debug UI sender lock");
        return;
    };

    if slot.is_some() {
        return;
    }

    let (tx, rx) = mpsc::channel::<DebugUiMessage>();
    *slot = Some(tx);

    if let Err(err) = thread::Builder::new()
        .name("debug-ui".to_string())
        .spawn(move || {
            let result =
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| app::run_window(rx)));
            if result.is_err() {
                clear_sender();
                log::warn!("debug UI thread panicked during initialization or render loop");
            }
        })
    {
        log::warn!("failed to spawn debug UI thread: {:?}", err);
        *slot = None;
    }
}

pub fn publish_panel(
    panel_id: impl Into<String>,
    title: impl Into<String>,
    fields: Vec<(String, String)>,
) {
    let panel_id = panel_id.into();
    let title = title.into();
    let now = Instant::now();

    if should_skip_publish(&panel_id, &title, &fields, now) {
        return;
    }

    let Ok(slot) = sender_slot().lock() else {
        return;
    };
    let Some(sender) = slot.as_ref().cloned() else {
        return;
    };
    drop(slot);

    let panel = DebugPanel {
        panel_id: panel_id.clone(),
        title: title.clone(),
        updated_at: Local::now().format("%H:%M:%S%.3f").to_string(),
        fields,
    };

    if let Err(err) = sender.send(DebugUiMessage::Upsert(panel)) {
        log::debug!("debug UI channel unavailable: {:?}", err);
        if let Ok(mut state) = publish_state_slot().lock() {
            state.remove(&panel_id);
        }
    }
}

pub fn remove_panel(panel_id: impl Into<String>) {
    let panel_id = panel_id.into();

    if let Ok(mut state) = publish_state_slot().lock() {
        state.remove(&panel_id);
    }

    let Ok(slot) = sender_slot().lock() else {
        return;
    };
    let Some(sender) = slot.as_ref().cloned() else {
        return;
    };
    drop(slot);

    if let Err(err) = sender.send(DebugUiMessage::Remove(panel_id)) {
        log::debug!("debug UI channel unavailable: {:?}", err);
    }
}

fn should_skip_publish(
    panel_id: &str,
    title: &str,
    fields: &[(String, String)],
    now: Instant,
) -> bool {
    let Ok(mut state) = publish_state_slot().lock() else {
        return false;
    };

    let min_interval = panel_min_publish_interval(panel_id);

    if let Some(previous) = state.get(panel_id) {
        if previous.title == title && previous.fields == fields {
            return true;
        }

        if now.duration_since(previous.published_at) < min_interval {
            return true;
        }
    }

    state.insert(
        panel_id.to_string(),
        DebugPanelPublishState {
            title: title.to_string(),
            fields: fields.to_vec(),
            published_at: now,
        },
    );

    false
}

fn panel_min_publish_interval(panel_id: &str) -> Duration {
    if panel_id.starts_with("runtime_npc:") || panel_id.starts_with("runtime_monster:") {
        Duration::from_millis(150)
    } else {
        Duration::from_millis(0)
    }
}

pub fn publish_player_info_panel(
    player_id: UserPlayerId,
    player: &UserPlayerState,
    skills_state: &SkillState,
) {
    player_panel::publish_player_info_panel(player_id, player, skills_state);
}

pub fn remove_player_info_panel(player_id: UserPlayerId) {
    remove_panel(format!("player_info:{}", player_id.0));
}

pub fn publish_runtime_npc_panel(
    state: &crate::application::use_case::runtime::world_runtime::game_server::dto::RuntimeNpcState,
) {
    runtime_panel::publish_runtime_npc_panel(state);
}

pub fn publish_runtime_monster_panel(
    state: &crate::application::use_case::runtime::world_runtime::game_server::dto::RuntimeMonsterState,
) {
    runtime_panel::publish_runtime_monster_panel(state);
}

pub fn remove_runtime_npc_panel(
    runtime_npc_id: crate::application::use_case::runtime::world_runtime::game_server::dto::RuntimeNpcId,
) {
    runtime_panel::remove_runtime_npc_panel(runtime_npc_id);
}

pub fn remove_runtime_monster_panel(
    runtime_monster_id: crate::application::use_case::runtime::world_runtime::game_server::dto::RuntimeMonsterId,
) {
    runtime_panel::remove_runtime_monster_panel(runtime_monster_id);
}
