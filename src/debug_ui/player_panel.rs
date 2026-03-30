use chrono::Local;

use crate::domain::user_player::dto::PlayerJob;
use crate::debug_ui::publish_panel;
use crate::domain::skill::dtos::codes::SkillCode;
use crate::domain::skill::state::SkillState;
use crate::domain::user_player::state::{UserPlayerId, UserPlayerState};

use super::shared::{
    chip, field_label_pt, player_job_label_pt, pretty_value_pt, section_title_pt, stat_bar,
};
use super::store::DebugPanel;

pub const PLAYER_PANEL_PREFIX: &str = "player_info:";

pub fn publish_player_info_panel(
    player_id: UserPlayerId,
    player: &UserPlayerState,
    skills_state: &SkillState,
) {
    let char_info = &player.core.char_info;
    let player_name = format!("Player {}", player_id.0);

    let position = match player.runtime.map_position {
        Some((x, y, z)) => format!("{x}, {y}, {z}"),
        None => "Desconhecida".to_string(),
    };

    let target = player
        .runtime
        .target_object_serial
        .map(|v| v.to_string())
        .unwrap_or_else(|| "Nenhum".to_string());

    let active_continue_skills = if player.runtime.active_continue_skills.is_empty() {
        "Nenhuma".to_string()
    } else {
        player
            .runtime
            .active_continue_skills
            .iter()
            .map(|entry| {
                let remaining_secs = entry
                    .1
                    .expires_at
                    .checked_duration_since(std::time::Instant::now())
                    .unwrap_or_default()
                    .as_secs();

                format!("Skill ativa por mais {remaining_secs}s")
            })
            .collect::<Vec<_>>()
            .join(", ")
    };

    let active_buff_names =
        resolve_skill_names(skills_state, char_info.job_code, &player.runtime.active_buff_codes);
    let passive_skill_names =
        resolve_skill_names(skills_state, char_info.job_code, &player.runtime.passive_skill_codes);

    let cooldowns = if player.runtime.cooldowns.is_empty() {
        "Nenhum".to_string()
    } else {
        player
            .runtime
            .cooldowns
            .iter()
            .map(|(skill_code, until)| {
                let skill_label =
                    resolve_skill_name_single(skills_state, char_info.job_code, *skill_code);
                format!("{skill_label}: {until}")
            })
            .collect::<Vec<_>>()
            .join(", ")
    };

    let premium_timers = if player.runtime.premium_timers.is_empty() {
        "Nenhum".to_string()
    } else {
        player
            .runtime
            .premium_timers
            .iter()
            .map(|(kind, value)| format!("{kind}:{value}"))
            .collect::<Vec<_>>()
            .join(", ")
    };

    let passive_runtime = if player.runtime.passive_skills.is_empty() {
        "Nenhuma".to_string()
    } else {
        format!(
            "{} skill(s) passiva(s) em runtime",
            player.runtime.passive_skills.len()
        )
    };

    publish_panel(
        format!("{PLAYER_PANEL_PREFIX}{}", player_id.0),
        player_name,
        vec![
            ("section".to_string(), "overview".to_string()),
            ("job_code".to_string(), player_job_label_pt(char_info.job_code)),
            ("level".to_string(), char_info.level.to_string()),
            ("position".to_string(), position),
            ("target".to_string(), target),
            ("updated_at".to_string(), Local::now().format("%H:%M:%S%.3f").to_string()),
            ("life".to_string(), format!("{}/{}", char_info.life[0], char_info.life[1])),
            ("mana".to_string(), format!("{}/{}", char_info.mana[0], char_info.mana[1])),
            (
                "stamina".to_string(),
                format!("{}/{}", char_info.stamina[0], char_info.stamina[1]),
            ),
            ("section".to_string(), "identity".to_string()),
            ("player_id".to_string(), player_id.0.to_string()),
            ("job_code".to_string(), player_job_label_pt(char_info.job_code)),
            ("change_job".to_string(), format!("{:?}", char_info.change_job)),
            ("state".to_string(), format!("{:?}", char_info.state)),
            (
                "position".to_string(),
                match player.runtime.map_position {
                    Some((x, y, z)) => format!("{x}, {y}, {z}"),
                    None => "Desconhecida".to_string(),
                },
            ),
            (
                "target".to_string(),
                player
                    .runtime
                    .target_object_serial
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "Nenhum".to_string()),
            ),
            ("updated_at".to_string(), Local::now().format("%H:%M:%S%.3f").to_string()),
            ("section".to_string(), "vitals".to_string()),
            ("life".to_string(), format!("{}/{}", char_info.life[0], char_info.life[1])),
            ("mana".to_string(), format!("{}/{}", char_info.mana[0], char_info.mana[1])),
            (
                "stamina".to_string(),
                format!("{}/{}", char_info.stamina[0], char_info.stamina[1]),
            ),
            ("life_regen".to_string(), format!("{:.3}", char_info.life_regen)),
            ("mana_regen".to_string(), format!("{:.3}", char_info.mana_regen)),
            (
                "stamina_regen".to_string(),
                format!("{:.3}", char_info.stamina_regen),
            ),
            (
                "weight".to_string(),
                format!("{}/{}", char_info.weight[0], char_info.weight[1]),
            ),
            ("section".to_string(), "progression".to_string()),
            ("state_point".to_string(), char_info.state_point.to_string()),
            ("section".to_string(), "attributes".to_string()),
            ("strength".to_string(), char_info.strength.to_string()),
            ("spirit".to_string(), char_info.spirit.to_string()),
            ("talent".to_string(), char_info.talent.to_string()),
            ("dexterity".to_string(), char_info.dexterity.to_string()),
            ("health".to_string(), char_info.health.to_string()),
            ("section".to_string(), "combat".to_string()),
            ("attack_rating".to_string(), char_info.attack_rating.to_string()),
            (
                "attack_damage".to_string(),
                format!("{} ~ {}", char_info.attack_damage[0], char_info.attack_damage[1]),
            ),
            ("attack_speed".to_string(), char_info.attack_speed.to_string()),
            (
                "shooting_range".to_string(),
                char_info.shooting_range.to_string(),
            ),
            ("critical_hit".to_string(), char_info.critical_hit.to_string()),
            ("defence".to_string(), char_info.defence.to_string()),
            ("chance_block".to_string(), char_info.chance_block.to_string()),
            ("absorption".to_string(), char_info.absorption.to_string()),
            ("move_speed".to_string(), char_info.move_speed.to_string()),
            ("sight".to_string(), char_info.sight.to_string()),
            ("section".to_string(), "runtime".to_string()),
            ("active_buffs".to_string(), active_buff_names),
            ("passive_skill_codes".to_string(), passive_skill_names),
            ("active_continue_skills".to_string(), active_continue_skills),
            ("cooldowns".to_string(), cooldowns),
            ("premium_timers".to_string(), premium_timers),
            ("passive_runtime".to_string(), passive_runtime),
            (
                "regen_acc".to_string(),
                format!(
                    "Vida {:.3} | Mana {:.3} | Stamina {:.3}",
                    player.runtime.regen_acc_life,
                    player.runtime.regen_acc_mana,
                    player.runtime.regen_acc_stamina
                ),
            ),
            ("section".to_string(), "resistances".to_string()),
            (
                "elemental_resistance".to_string(),
                format_i16_array(&char_info.resistance),
            ),
            (
                "attack_resistance".to_string(),
                format_i16_array(&char_info.attack_resistance),
            ),
            ("section".to_string(), "misc".to_string()),
            ("potion_space".to_string(), char_info.potion_space.to_string()),
        ],
    );
}

pub fn render_player_content(ui: &mut eframe::egui::Ui, panel: &DebugPanel) {
    let sections_order = [
        "identity",
        "vitals",
        "progression",
        "attributes",
        "combat",
        "runtime",
        "resistances",
        "misc",
    ];

    render_player_header(ui, panel);
    ui.add_space(10.0);
    render_overview(ui, panel);
    ui.add_space(10.0);

    for section_name in sections_order {
        let fields = section_fields(panel, section_name);
        if fields.is_empty() {
            continue;
        }

        render_info_grid(ui, &panel.panel_id, section_name, &fields);
        ui.add_space(10.0);
    }
}

fn resolve_skill_names(skills_state: &SkillState, job_code: PlayerJob, codes: &[SkillCode]) -> String {
    if codes.is_empty() {
        return "Nenhuma".to_string();
    }

    let Some(class_cfg) = skills_state.state.get(&job_code) else {
        return codes
            .iter()
            .map(|code| format!("0x{:X}", code.raw()))
            .collect::<Vec<_>>()
            .join(", ");
    };

    codes
        .iter()
        .map(|code| {
            class_cfg
                .by_code
                .get(code)
                .map(|entry| {
                    let (name, _description, require_level) = entry.get_info();
                    if name.trim().is_empty() {
                        format!("0x{:X}", code.raw())
                    } else {
                        format!("{name} (Nv. {require_level})")
                    }
                })
                .unwrap_or_else(|| format!("0x{:X}", code.raw()))
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn resolve_skill_name_single(skills_state: &SkillState, job_code: PlayerJob, code: SkillCode) -> String {
    let Some(class_cfg) = skills_state.state.get(&job_code) else {
        return format!("0x{:X}", code.raw());
    };

    class_cfg
        .by_code
        .get(&code)
        .map(|entry| {
            let (name, _description, require_level) = entry.get_info();
            if name.trim().is_empty() {
                format!("0x{:X}", code.raw())
            } else {
                format!("{name} (Nv. {require_level})")
            }
        })
        .unwrap_or_else(|| format!("0x{:X}", code.raw()))
}

fn format_i16_array(values: &[i16]) -> String {
    values
        .iter()
        .enumerate()
        .map(|(idx, value)| format!("{idx}: {value}"))
        .collect::<Vec<_>>()
        .join("  |  ")
}

fn section_fields(panel: &DebugPanel, section_name: &str) -> Vec<(String, String)> {
    let mut current_section_name = String::new();
    let mut current_fields: Vec<(String, String)> = Vec::new();

    for (key, value) in &panel.fields {
        if key == "section" {
            if current_section_name == section_name {
                return current_fields;
            }

            current_section_name = value.clone();
            current_fields = Vec::new();
        } else if current_section_name == section_name {
            current_fields.push((key.clone(), value.clone()));
        }
    }

    if current_section_name == section_name {
        current_fields
    } else {
        Vec::new()
    }
}

fn render_info_grid(
    ui: &mut eframe::egui::Ui,
    panel_id: &str,
    section_name: &str,
    fields: &[(String, String)],
) {
    eframe::egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.heading(section_title_pt(section_name));
        ui.add_space(8.0);

        eframe::egui::Grid::new(format!("grid_section:{panel_id}:{section_name}"))
            .num_columns(2)
            .spacing([24.0, 8.0])
            .striped(true)
            .show(ui, |ui| {
                for (key, value) in fields {
                    ui.label(field_label_pt(key));
                    ui.label(pretty_value_pt(key, value));
                    ui.end_row();
                }
            });
    });
}

fn find_field_value(fields: &[(String, String)], key: &str) -> String {
    fields
        .iter()
        .find(|(k, _)| k == key)
        .map(|(_, v)| pretty_value_pt(key, v))
        .unwrap_or_else(|| "-".to_string())
}

fn summary_chips_from_fields(fields: &[(String, String)]) -> Vec<(String, String)> {
    vec![
        ("Classe".to_string(), find_field_value(fields, "job_code")),
        ("Nivel".to_string(), find_field_value(fields, "level")),
        ("Vida".to_string(), find_field_value(fields, "life")),
        ("Mana".to_string(), find_field_value(fields, "mana")),
        ("Stamina".to_string(), find_field_value(fields, "stamina")),
        ("Posicao".to_string(), find_field_value(fields, "position")),
    ]
}

fn render_overview(ui: &mut eframe::egui::Ui, panel: &DebugPanel) {
    let overview_fields = section_fields(panel, "overview");
    let chips = summary_chips_from_fields(&overview_fields);

    eframe::egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.heading("Resumo");
        ui.add_space(8.0);

        eframe::egui::Grid::new(format!("overview_grid:{}", panel.panel_id))
            .num_columns(3)
            .spacing([12.0, 12.0])
            .show(ui, |ui| {
                for (idx, (label, value)) in chips.iter().enumerate() {
                    chip(ui, label, value);
                    if (idx + 1) % 3 == 0 {
                        ui.end_row();
                    }
                }
            });

        ui.add_space(12.0);

        let life_raw = overview_fields
            .iter()
            .find(|(k, _)| k == "life")
            .map(|(_, v)| v.as_str())
            .unwrap_or("-");
        let mana_raw = overview_fields
            .iter()
            .find(|(k, _)| k == "mana")
            .map(|(_, v)| v.as_str())
            .unwrap_or("-");
        let stamina_raw = overview_fields
            .iter()
            .find(|(k, _)| k == "stamina")
            .map(|(_, v)| v.as_str())
            .unwrap_or("-");

        stat_bar(ui, "life", "Vida", life_raw);
        ui.add_space(6.0);
        stat_bar(ui, "mana", "Mana", mana_raw);
        ui.add_space(6.0);
        stat_bar(ui, "stamina", "Stamina", stamina_raw);
    });
}

fn render_player_header(ui: &mut eframe::egui::Ui, panel: &DebugPanel) {
    eframe::egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.heading(&panel.title);
        ui.label(format!("Atualizado as {}", panel.updated_at));
        ui.label(format!("Panel ID: {}", panel.panel_id));
        ui.label(format!("Campos recebidos: {}", panel.fields.len()));
    });
}
