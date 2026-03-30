use crate::domain::user_player::dto::PlayerJob;

pub fn section_title_pt(section: &str) -> &'static str {
    match section {
        "overview" => "Resumo",
        "identity" => "Identidade",
        "vitals" => "Status vitais",
        "progression" => "Progressao",
        "attributes" => "Atributos",
        "combat" => "Combate",
        "runtime" => "Estado atual",
        "resistances" => "Resistencias",
        "misc" => "Outras informacoes",
        _ => "Informacoes",
    }
}

pub fn field_label_pt(key: &str) -> &str {
    match key {
        "player_id" => "ID do player",
        "object_serial" => "Object serial",
        "name" => "Nome",
        "template_id" => "Template",
        "field_id" => "Field",
        "job_code" => "Classe",
        "change_job" => "Evolucao",
        "level" => "Nivel",
        "state" => "Estado",
        "position" => "Posicao",
        "direction" => "Direcao",
        "target" => "Alvo",
        "updated_at" => "Atualizado em",
        "life" => "Vida",
        "mana" => "Mana",
        "stamina" => "Stamina",
        "current_life" => "Vida atual",
        "max_life" => "Vida maxima",
        "life_regen" => "Regeneracao de vida",
        "mana_regen" => "Regeneracao de mana",
        "stamina_regen" => "Regeneracao de stamina",
        "weight" => "Peso",
        "state_point" => "Pontos disponiveis",
        "strength" => "Forca",
        "spirit" => "Espirito",
        "talent" => "Talento",
        "dexterity" => "Agilidade",
        "health" => "Vitalidade",
        "attack_rating" => "Taxa de ataque",
        "attack_damage" => "Dano",
        "attack_speed" => "Velocidade de ataque",
        "shooting_range" => "Alcance",
        "critical_hit" => "Critico",
        "defence" => "Defesa",
        "chance_block" => "Bloqueio",
        "absorption" => "Absorcao",
        "move_speed" => "Velocidade de movimento",
        "sight" => "Visao",
        "active_buffs" => "Buffs ativos",
        "passive_skill_codes" => "Skills passivas",
        "active_continue_skills" => "Skills continuas",
        "cooldowns" => "Recargas",
        "premium_timers" => "Timers premium",
        "passive_runtime" => "Passivas em runtime",
        "regen_acc" => "Acumulo de regeneracao",
        "motion_state" => "Motion state",
        "elemental_resistance" => "Resistencias elementais",
        "attack_resistance" => "Resistencias de ataque",
        "potion_space" => "Espaco de pocoes",
        _ => key,
    }
}

pub fn pretty_value_pt(key: &str, value: &str) -> String {
    let value = value.trim();

    if value.is_empty() {
        return "-".to_string();
    }

    match key {
        "target" if value.eq_ignore_ascii_case("none") => "Nenhum".to_string(),
        "target" if value.eq_ignore_ascii_case("nenhum") => "Nenhum".to_string(),
        "position" if value.eq_ignore_ascii_case("unknown") => "Desconhecida".to_string(),
        "active_buffs" | "passive_skill_codes" | "active_continue_skills" | "cooldowns"
            if value.eq_ignore_ascii_case("none") =>
        {
            "Nenhuma".to_string()
        }
        "premium_timers" if value.eq_ignore_ascii_case("none") => "Nenhum".to_string(),
        "passive_runtime" if value.eq_ignore_ascii_case("none") => "Nenhuma".to_string(),
        _ => value.to_string(),
    }
}

pub fn player_job_label_pt(job: PlayerJob) -> String {
    match format!("{job:?}").as_str() {
        "Mechanician" => "Mecanico".to_string(),
        "Fighter" => "Lutador".to_string(),
        "Pikeman" => "Pikeman".to_string(),
        "Archer" => "Arqueira".to_string(),
        "Knight" => "Cavaleiro".to_string(),
        "Atalanta" => "Atalanta".to_string(),
        "Priestess" => "Sacerdotisa".to_string(),
        "Magician" => "Mago".to_string(),
        "Assassin" => "Assassina".to_string(),
        "Shaman" => "Xama".to_string(),
        other => other.to_string(),
    }
}

pub fn chip(ui: &mut eframe::egui::Ui, label: &str, value: &str) {
    eframe::egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.vertical(|ui| {
            ui.small(label);
            ui.strong(value);
        });
    });
}

pub fn parse_current_max(text: &str) -> Option<(f32, f32)> {
    let (left, right) = text.split_once('/')?;
    let current = left.trim().replace(',', ".").parse::<f32>().ok()?;
    let max = right.trim().replace(',', ".").parse::<f32>().ok()?;
    Some((current, max))
}

pub fn finite_f32(value: f32, fallback: f32) -> f32 {
    if value.is_finite() {
        value
    } else {
        fallback
    }
}

pub fn safe_progress(current: f32, max: f32) -> f32 {
    if !current.is_finite() || !max.is_finite() || max <= 0.0 {
        0.0
    } else {
        (current / max).clamp(0.0, 1.0)
    }
}

pub fn stat_bar(ui: &mut eframe::egui::Ui, key: &str, title: &str, raw_value: &str) {
    let pretty = pretty_value_pt(key, raw_value);

    ui.label(format!("{title}: {pretty}"));

    if let Some((current, max)) = parse_current_max(raw_value) {
        let progress = safe_progress(current, max);
        let width = finite_f32(ui.available_width(), 200.0).max(16.0);

        ui.add(
            eframe::egui::ProgressBar::new(progress)
                .show_percentage()
                .desired_width(width),
        );
    }
}
