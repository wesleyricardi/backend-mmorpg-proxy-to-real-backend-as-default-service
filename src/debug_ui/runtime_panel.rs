use crate::application::use_case::runtime::world_runtime::game_server::dto::{
    RuntimeMonsterId, RuntimeMonsterState, RuntimeNpcId, RuntimeNpcState,
};
use crate::debug_ui::{publish_panel, remove_panel};

use super::shared::{field_label_pt, pretty_value_pt};
use super::store::DebugPanel;

pub const NPC_PANEL_PREFIX: &str = "runtime_npc:";
pub const MONSTER_PANEL_PREFIX: &str = "runtime_monster:";

pub fn publish_runtime_npc_panel(state: &RuntimeNpcState) {
    publish_panel(
        format!("{NPC_PANEL_PREFIX}{}", state.id.0),
        format!("NPC {}", state.id.0),
        vec![
            ("object_serial".to_string(), state.id.0.to_string()),
            (
                "template_id".to_string(),
                state
                    .template_id
                    .as_ref()
                    .map(|value| value.0.clone())
                    .unwrap_or_else(|| "-".to_string()),
            ),
            (
                "field_id".to_string(),
                state
                    .field_id
                    .map(|value| value.raw().to_string())
                    .unwrap_or_else(|| "-".to_string()),
            ),
            ("name".to_string(), state.name.clone()),
            (
                "position".to_string(),
                format!("{}, {}, {}", state.position.x, state.position.y, state.position.z),
            ),
            (
                "direction".to_string(),
                format!(
                    "{}, {}, {}",
                    state.direction.x, state.direction.y, state.direction.z
                ),
            ),
            ("motion_state".to_string(), state.motion_state.to_string()),
        ],
    );
}

pub fn publish_runtime_monster_panel(state: &RuntimeMonsterState) {
    publish_panel(
        format!("{MONSTER_PANEL_PREFIX}{}", state.id.0),
        format!("Monster {}", state.id.0),
        vec![
            ("object_serial".to_string(), state.id.0.to_string()),
            (
                "template_id".to_string(),
                state
                    .template_id
                    .map(|value| value.0.to_string())
                    .unwrap_or_else(|| "-".to_string()),
            ),
            (
                "field_id".to_string(),
                state
                    .field_id
                    .map(|value| value.raw().to_string())
                    .unwrap_or_else(|| "-".to_string()),
            ),
            ("name".to_string(), state.name.clone()),
            (
                "position".to_string(),
                format!("{}, {}, {}", state.position.x, state.position.y, state.position.z),
            ),
            (
                "direction".to_string(),
                format!(
                    "{}, {}, {}",
                    state.direction.x, state.direction.y, state.direction.z
                ),
            ),
            ("current_life".to_string(), state.current_life.to_string()),
            ("max_life".to_string(), state.max_life.to_string()),
            ("motion_state".to_string(), state.motion_state.to_string()),
        ],
    );
}

pub fn remove_runtime_npc_panel(runtime_npc_id: RuntimeNpcId) {
    remove_panel(format!("{NPC_PANEL_PREFIX}{}", runtime_npc_id.0));
}

pub fn remove_runtime_monster_panel(runtime_monster_id: RuntimeMonsterId) {
    remove_panel(format!("{MONSTER_PANEL_PREFIX}{}", runtime_monster_id.0));
}

pub fn render_placeholder(
    ui: &mut eframe::egui::Ui,
    title: &str,
    subtitle: &str,
    panel: Option<&DebugPanel>,
) {
    ui.heading(title);
    ui.label(subtitle);
    ui.add_space(12.0);

    if let Some(panel) = panel {
        eframe::egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.heading(&panel.title);
            ui.label(format!("Atualizado as {}", panel.updated_at));
            ui.label(format!("Panel ID: {}", panel.panel_id));
            ui.label(format!("Campos recebidos: {}", panel.fields.len()));
        });
        ui.add_space(12.0);

        eframe::egui::Frame::group(ui.style()).show(ui, |ui| {
            eframe::egui::Grid::new(format!("runtime_panel_grid:{}", panel.panel_id))
                .num_columns(2)
                .spacing([24.0, 8.0])
                .striped(true)
                .show(ui, |ui| {
                    for (key, value) in &panel.fields {
                        ui.label(field_label_pt(key));
                        ui.label(pretty_value_pt(key, value));
                        ui.end_row();
                    }
                });
        });
    } else {
        ui.label("Nenhum painel publicado ainda.");
    }
}
