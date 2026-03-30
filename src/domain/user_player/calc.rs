use crate::domain::item::combo::collect_combo_bonuses;
use crate::domain::item::dto::EquippableItemType;
use crate::domain::user_player::dto::{
    CalcContext, CalculatedCharStats, ChangeJobTierState, ComboBonusStats, PlayerJob,
    TotalItemStats, UserCalcInput, UserCalcOutput, UserChar, UserContinueSkill, UserItem,
    UserPassiveSkill, UserRecordItem,
};
use crate::domain::user_player::state::UserPlayerState;
use crate::domain::{skill::formula as skill_formula, skill::state::SkillState};

fn clamp_i32_to_i16(v: i32) -> i16 {
    if v > i16::MAX as i32 {
        i16::MAX
    } else if v < i16::MIN as i32 {
        i16::MIN
    } else {
        v as i16
    }
}

fn check_require_item_to_set(player: &UserChar, item: &UserItem) -> bool {
    if item.level > player.level {
        return false;
    }
    if item.dexterity > player.dexterity {
        return false;
    }
    if item.strength > player.strength {
        return false;
    }
    if item.talent > player.talent {
        return false;
    }
    if item.spirit > player.spirit {
        return false;
    }
    if item.health > player.health {
        return false;
    }
    true
}

fn normalize_equipped_items<'a>(
    player: &UserChar,
    equipped_items: &'a [UserRecordItem],
) -> Vec<&'a UserRecordItem> {
    let mut seen_position = [false; 17];
    let mut out = Vec::with_capacity(equipped_items.len());

    for record_item in equipped_items {
        let position = record_item.position;
        if position <= 0 || position as usize >= seen_position.len() {
            continue;
        }
        if seen_position[position as usize] {
            continue;
        }

        let item = &record_item.item;
        if item.not_use_flag != 0 {
            continue;
        }
        if !check_require_item_to_set(player, item) {
            continue;
        }

        seen_position[position as usize] = true;
        out.push(record_item);
    }

    out
}

fn apply_combo_bonus(total: &mut TotalItemStats, bonus: &ComboBonusStats) {
    total.f_increase_life += bonus.add_hp as f32;
    total.f_increase_mana += bonus.add_mp as f32;
    total.f_increase_stamina += bonus.add_res as f32;
    total.f_life_regen += bonus.regen_hp;
    total.f_mana_regen += bonus.regen_mp;
    total.f_stamina_regen += bonus.regen_res;
    total.defence += bonus.defense;
    total.f_absorb += bonus.absorption as f32;
    total.critical_hit += bonus.critical;
    total.attack_rating += bonus.attack_rating;
    total.f_block_rating += bonus.block as f32;
    total.weight += bonus.weight_capacity;
}

#[derive(Default)]
struct AggregatedItemStats {
    total: TotalItemStats,
    absorb_item_count: i32,
}

fn aggregate_item_stats(
    input: &UserCalcInput,
    normalized: &[&UserRecordItem],
) -> AggregatedItemStats {
    let mut acc = AggregatedItemStats::default();
    let total = &mut acc.total;

    for record_item in normalized {
        let current = &record_item.item;
        total.attack_rating += current.attack_rating;
        total.critical_hit += current.critical_hit;
        total.f_absorb += current.f_absorb;
        total.defence += current.defense;
        total.f_block_rating += current.f_block_rating;
        total.f_speed += current.f_speed;
        total.sight += current.sight;
        total.shooting_range += current.shooting_range;
        total.f_mana_regen += current.f_mana_regen;
        total.f_life_regen += current.f_life_regen;
        total.f_stamina_regen += current.f_stamina_regen;
        total.f_increase_life += current.f_increase_life;
        total.f_increase_mana += current.f_increase_mana;
        total.f_increase_stamina += current.f_increase_stamina;
        total.f_magic_mastery += current.f_magic_mastery;

        if input.quest_flags.check_quest_item_down_flag && current.item_kind_code == 0 {
            total.attack_speed += current.attack_speed.saturating_sub(2);
        } else {
            total.attack_speed += current.attack_speed;
        }

        total.damage[0] += current.damage[0];
        total.damage[1] += current.damage[1];
        total.mana[0] += current.mana[0];
        total.mana[1] += current.mana[1];
        total.life[0] += current.life[0];
        total.life[1] += current.life[1];
        total.stamina[0] += current.stamina[0];
        total.stamina[1] += current.stamina[1];

        for i in 0..8 {
            total.resistance[i] += current.resistance[i];
        }

        if current.f_absorb != 0.0 {
            acc.absorb_item_count += 1;
        }

        if input
            .base_player
            .job_bit_mask
            .intersects(current.job_code_mask)
        {
            total.job_item.add_f_absorb += current.job_item.add_f_absorb;
            total.job_item.add_defense += current.job_item.add_defense;
            total.job_item.add_f_speed += current.job_item.add_f_speed;
            total.job_item.add_f_block_rating += current.job_item.add_f_block_rating;
            total.job_item.add_attack_speed += current.job_item.add_attack_speed;
            total.job_item.add_critical_hit += current.job_item.add_critical_hit;
            total.job_item.add_shooting_range += current.job_item.add_shooting_range;
            total.job_item.add_f_magic_mastery += current.job_item.add_f_magic_mastery;

            for i in 0..8 {
                total.job_item.add_resistance[i] += current.job_item.add_resistance[i];
                total.job_item.lev_attack_resistance[i] +=
                    current.job_item.lev_attack_resistance[i];
            }
            total.job_item.lev_mana += current.job_item.lev_mana;
            total.job_item.lev_life += current.job_item.lev_life;
            total.job_item.lev_attack_rating += current.job_item.lev_attack_rating;
            total.job_item.lev_damage[0] += current.job_item.lev_damage[0];
            total.job_item.lev_damage[1] += current.job_item.lev_damage[1];
            total.job_item.per_mana_regen += current.job_item.per_mana_regen;
            total.job_item.per_life_regen += current.job_item.per_life_regen;
            total.job_item.per_stamina_regen += current.job_item.per_stamina_regen;

            if current.job_item.add_f_absorb != 0.0 {
                acc.absorb_item_count += 1;
            }
        }

        if current.potion_space > 0 {
            total.potion_space = current.potion_space;
        }
    }

    for bonus in &input.combo_bonuses {
        apply_combo_bonus(total, bonus);
    }

    acc.absorb_item_count /= 2;
    if acc.absorb_item_count > 2 {
        acc.absorb_item_count = 2;
    }

    acc
}

fn resolve_life_preset(player: &UserChar, input: &UserCalcInput) -> PlayerJob {
    if let Some(preset) = input.life_preset {
        return preset;
    }
    player.job_code
}

fn life_coefficients(preset: PlayerJob) -> (f32, f32, f32, f32, f32, f32) {
    match preset {
        PlayerJob::Fighter => (2.0, 2.8, 0.5, 0.1, 0.1, 0.1),
        PlayerJob::Mechanician => (2.0, 2.8, 0.5, 0.1, 0.1, 0.1),
        PlayerJob::Archer => (1.2, 1.8, 0.2, 0.1, 0.2, 0.1),
        PlayerJob::Pikeman => (2.0, 2.6, 0.4, 0.1, 0.1, 0.2),
        PlayerJob::Atalanta => (1.2, 1.8, 0.3, 0.2, 0.1, 0.1),
        PlayerJob::Knight => (2.0, 2.4, 0.4, 0.2, 0.3, 0.1),
        PlayerJob::Magician => (1.5, 2.4, 0.1, 0.2, 0.2, 0.1),
        PlayerJob::Priestess => (1.5, 2.8, 0.1, 0.3, 0.2, 0.1),
        PlayerJob::Assassine => (2.0, 2.6, 0.4, 0.1, 0.1, 0.2),
        PlayerJob::Shaman => (1.5, 2.4, 0.1, 0.2, 0.2, 0.1),
        PlayerJob::Martial => (2.0, 2.8, 0.5, 0.1, 0.1, 0.1),
    }
}

fn get_life(player: &UserChar, input: &UserCalcInput) -> f32 {
    let (lvl, hp, strg, spr, tal, dex) = life_coefficients(resolve_life_preset(player, input));
    (player.level as f32 * lvl)
        + (player.health as f32 * hp)
        + (player.strength as f32 * strg)
        + (player.spirit as f32 * spr)
        + (player.talent as f32 * tal)
        + (player.dexterity as f32 * dex)
}

fn get_sin_char_item_damage(player: &UserChar, weapon: Option<&UserItem>) -> f32 {
    let Some(weapon) = weapon else {
        return 0.0;
    };
    if player.job_bit_mask.intersects(weapon.job_code_mask) || weapon.unique_item == 2 {
        player.level as f32 / 6.0
    } else {
        0.0
    }
}

fn get_attack_damage(
    player: &UserChar,
    weapon: Option<&UserItem>,
    input: &UserCalcInput,
    total: &TotalItemStats,
) -> [f32; 2] {
    let mut attack_damage = [0.0_f32, 0.0_f32];
    let sin_char_item_damage = get_sin_char_item_damage(player, weapon);
    let melee_fn = input.damage_function[0];
    let shoot_fn = input.damage_function[1];
    let cast_fn = input.damage_function[2];

    if weapon.is_none() {
        if melee_fn == 1 {
            attack_damage[0] = 1.0
                + (player.strength as f32 + 130.0) / 130.0
                + (player.talent as f32 + player.dexterity as f32) / 40.0
                + total.job_item.lev_damage[0] as f32;
            attack_damage[1] = 2.0
                + (player.strength as f32 + 130.0) / 130.0
                + (player.talent as f32 + player.dexterity as f32) / 35.0
                + total.job_item.lev_damage[1] as f32;
            return attack_damage;
        }

        if (2..=4).contains(&melee_fn) {
            attack_damage[0] = 1.0
                + (player.strength as f32 + 200.0) / 200.0
                + (player.talent as f32 + player.dexterity as f32) / 50.0
                + total.job_item.lev_damage[0] as f32;
            attack_damage[1] = 2.0
                + (player.strength as f32 + 200.0) / 200.0
                + (player.talent as f32 + player.dexterity as f32) / 45.0
                + total.job_item.lev_damage[1] as f32;
            return attack_damage;
        }
    }

    let Some(weapon) = weapon else {
        return attack_damage;
    };
    let total_damage_avg = (total.damage[0] as f32 + total.damage[1] as f32) / 16.0;
    let strg = player.strength as f32;
    let dex = player.dexterity as f32;
    let tal = player.talent as f32;
    let spr = player.spirit as f32;
    let is_shooting = weapon.swap_item.flag == 2
        || weapon.shooting_range > 0 && weapon.job_item.add_shooting_range > 0;
    let is_casting = weapon.f_magic_mastery > 0.0;

    if is_shooting {
        if shoot_fn == 1 {
            attack_damage[0] = 1.0
                + (total.damage[0] as f32 * (dex + 130.0) / 130.0)
                + ((tal + strg) / 40.0)
                + total.job_item.lev_damage[0] as f32
                + sin_char_item_damage
                + total_damage_avg;
            attack_damage[1] = 3.0
                + (total.damage[1] as f32 * (dex + 130.0) / 130.0)
                + ((tal + strg) / 40.0)
                + total.job_item.lev_damage[1] as f32
                + sin_char_item_damage;
            return attack_damage;
        }
        if shoot_fn == 2 {
            attack_damage[0] = 1.0
                + (total.damage[0] as f32 * (dex + 190.0) / 190.0)
                + ((tal + strg) / 50.0)
                + total.job_item.lev_damage[0] as f32
                + sin_char_item_damage
                + total_damage_avg;
            attack_damage[1] = 3.0
                + (total.damage[1] as f32 * (dex + 190.0) / 190.0)
                + ((tal + strg) / 50.0)
                + total.job_item.lev_damage[1] as f32
                + sin_char_item_damage;
            return attack_damage;
        }
    } else if is_casting {
        if cast_fn == 1 {
            attack_damage[0] = 1.0
                + (total.damage[0] as f32 * (spr + 150.0) / 150.0)
                + (tal / 30.0)
                + total.job_item.lev_damage[0] as f32
                + sin_char_item_damage
                + total_damage_avg;
            attack_damage[1] = 3.0
                + (total.damage[1] as f32 * (spr + 150.0) / 150.0)
                + (tal / 30.0)
                + total.job_item.lev_damage[1] as f32
                + sin_char_item_damage;
            return attack_damage;
        }
        if cast_fn == 2 {
            attack_damage[0] = 1.0
                + (total.damage[0] as f32 * (spr + 170.0) / 170.0)
                + (tal / 30.0)
                + total.job_item.lev_damage[0] as f32
                + sin_char_item_damage
                + total_damage_avg;
            attack_damage[1] = 3.0
                + (total.damage[1] as f32 * (spr + 170.0) / 170.0)
                + (tal / 30.0)
                + total.job_item.lev_damage[1] as f32
                + sin_char_item_damage;
            return attack_damage;
        }
    }

    let (strength_div, talent_div) = match melee_fn {
        1 => (130.0, 40.0),
        2 => (150.0, 45.0),
        3 => (190.0, 50.0),
        _ => (130.0, 40.0),
    };
    attack_damage[0] = 1.0
        + (total.damage[0] as f32 * (strg + strength_div) / strength_div)
        + ((tal + dex) / talent_div)
        + total.job_item.lev_damage[0] as f32
        + sin_char_item_damage
        + total_damage_avg;
    attack_damage[1] = 3.0
        + (total.damage[1] as f32 * (strg + strength_div) / strength_div)
        + ((tal + dex) / talent_div)
        + total.job_item.lev_damage[1] as f32
        + sin_char_item_damage;
    attack_damage
}

fn apply_final_modifiers(input: &UserCalcInput, stats: &mut CalculatedCharStats) {
    // battle ranking bonus
    if let Some(tier) = input.battle_modifiers.ranking_tier {
        let pct = match tier {
            0 => 1,
            1 => 3,
            2 => 5,
            3 => 7,
            4 => 10,
            5 => 15,
            6 => 20,
            7 => 25,
            _ => 0,
        } as f32;
        if pct > 0.0 {
            stats.attack_damage[0] += stats.attack_damage[0] * (pct / 100.0);
            stats.attack_damage[1] += stats.attack_damage[1] * (pct / 100.0);
        }
    }

    // relic bonuses
    if input.battle_modifiers.relics_active[0] {
        stats.attack_damage[0] += stats.attack_damage[0] * 0.10;
        stats.attack_damage[1] += stats.attack_damage[1] * 0.10;
    }
    if input.battle_modifiers.relics_active[2] {
        stats.defence += stats.defence * 0.20;
        stats.chance_block += 5.0;
    }
    if input.battle_modifiers.relics_active[3] {
        stats.life[1] += stats.life[1] * 0.25;
    }
    if input.battle_modifiers.relics_active[7] {
        stats.critical_hit += 10.0;
        stats.attack_damage[0] += stats.attack_damage[0] * 0.15;
        stats.attack_damage[1] += stats.attack_damage[1] * 0.15;
    }

    if input.quest_flags.level_90_damage_penalty {
        stats.attack_damage[0] *= 0.70;
        stats.attack_damage[1] *= 0.70;
    }
}

pub fn calculate_user_player(
    input: &UserCalcInput,
    skill_state: Option<&SkillState>,
) -> UserCalcOutput {
    let player = &input.base_player;
    let normalized = normalize_equipped_items(player, &input.equipped_items);
    let weapon = normalized
        .iter()
        .find(|item| item.position == 1)
        .map(|record| &record.item);
    let AggregatedItemStats {
        total,
        absorb_item_count,
    } = aggregate_item_stats(input, &normalized);

    let mut out = CalculatedCharStats {
        level: player.level as f32,
        strength: player.strength as f32,
        spirit: player.spirit as f32,
        talent: player.talent as f32,
        dexterity: player.dexterity as f32,
        health: player.health as f32,
        ..Default::default()
    };

    out.attack_rating = (out.dexterity * 3.1)
        + (out.level * 1.9)
        + (out.talent * 1.5)
        + total.attack_rating as f32
        + total.job_item.lev_attack_rating as f32;
    out.attack_damage = get_attack_damage(player, weapon, input, &total);
    out.attack_damage[0] += 1.0;
    out.attack_damage[1] += 1.0;
    out.attack_speed = total.attack_speed as f32 + total.job_item.add_attack_speed as f32;
    out.critical_hit = total.critical_hit as f32 + total.job_item.add_critical_hit as f32;
    out.shooting_range = total.shooting_range as f32 + total.job_item.add_shooting_range as f32;

    let dex_term = if matches!(resolve_life_preset(player, input), PlayerJob::Atalanta) {
        out.dexterity * 0.2
    } else {
        out.dexterity
    };
    out.defence = (out.talent * 1.5)
        + (out.level * 2.0)
        + dex_term
        + (out.health * 3.0)
        + total.defence as f32
        + total.job_item.add_defense as f32;

    out.absorption = (out.defence / 100.0)
        + (out.level / 10.0)
        + (out.strength / 40.0)
        + (out.talent / 40.0)
        + (out.spirit / 60.0)
        + (out.dexterity / 60.0)
        + total.f_absorb
        + total.job_item.add_f_absorb
        + 1.0
        + absorb_item_count as f32;

    out.chance_block = total.f_block_rating + total.job_item.add_f_block_rating;
    out.evade = 0.0;
    for i in 0..8 {
        out.resistance[i] = total.resistance[i] as f32 + total.job_item.add_resistance[i] as f32;
    }

    out.sight = total.sight as f32 + 180.0;
    out.weight[0] = player.weight[0] as f32;
    out.weight[1] = (out.strength * 2.0) + (out.health * 1.5) + (out.level * 3.0) + 60.0;
    if input.premium_state.might_of_awell_active {
        out.weight[1] += input.premium_state.might_of_awell_weight_bonus as f32;
    }
    if input.premium_state.gravity_scroll_weight_bonus > 0 {
        out.weight[1] += (input.premium_state.gravity_scroll_weight_bonus * 50) as f32;
    }
    out.weight[1] += ((player.dexterity / 10) * 20) as f32;
    out.weight[1] += ((player.health / 10) * 6) as f32;
    out.weight[1] += ((player.spirit / 10) * 40) as f32;
    out.weight[1] += total.weight as f32;

    let now_weight = out.weight[0];
    let max_weight = out.weight[1].max(1.0);
    out.move_speed = (((out.talent + out.health + out.level + 60.0) / 150.0)
        - (now_weight / max_weight))
        + total.f_speed
        + total.job_item.add_f_speed
        + 1.0;

    out.life[0] = player.life[0] as f32;
    out.life[1] = get_life(player, input) + total.f_increase_life + total.job_item.lev_life as f32;
    if !input.quest_flags.is_change_job_3_completed
        && !input.quest_flags.has_quest_weapon_equipped
        && player.change_job.as_i32() >= 3
        && player.level >= 40
    {
        out.life[1] += 15.0;
    }
    if input.quest_flags.has_level_90_quest_bonus && player.level >= 90 {
        out.life[1] += 40.0;
    }
    if input.premium_state.life_booster_percent > 0 {
        out.life[1] += out.life[1] * (input.premium_state.life_booster_percent as f32 / 100.0);
    }

    let mana_fn = if input.mana_function == 0 {
        player.mana_function
    } else {
        input.mana_function
    };
    out.mana[0] = player.mana[0] as f32;
    out.mana[1] = match mana_fn {
        1 => out.level * 1.5 + out.spirit * 3.8,
        2 => out.level * 0.9 + out.spirit * 2.7,
        3 => out.level * 0.6 + out.spirit * 2.2,
        _ => out.level * 0.9 + out.spirit * 2.7,
    } + total.f_increase_mana
        + total.job_item.lev_mana as f32;
    if input.premium_state.mana_booster_percent > 0 {
        out.mana[1] += out.mana[1] * (input.premium_state.mana_booster_percent as f32 / 100.0);
    }

    out.stamina[0] = player.stamina[0] as f32;
    out.stamina[1] = (out.health * 1.4)
        + ((out.strength + out.talent) / 2.0)
        + (out.level * 2.3)
        + 80.0
        + out.spirit
        + total.f_increase_stamina;
    if input.premium_state.stamina_booster_percent > 0 {
        out.stamina[1] +=
            out.stamina[1] * (input.premium_state.stamina_booster_percent as f32 / 100.0);
    }

    out.life_regen = total.f_life_regen + total.job_item.per_life_regen;
    out.mana_regen = total.f_mana_regen + total.job_item.per_mana_regen;
    out.stamina_regen = total.f_stamina_regen + total.job_item.per_stamina_regen;

    let player_job = player.job_code;
    let mut debug_breakdown = vec![
        format!("normalized_equipped_items={}", normalized.len()),
        format!("absorb_item_count={}", absorb_item_count),
        format!("combo_bonuses={}", input.combo_bonuses.len()),
    ];
    for record in &normalized {
        debug_breakdown.push(format!(
            "equipped_item[pos:{}]=code:0x{:08X} not_use:{} name:{}",
            record.position, record.item.code, record.item.not_use_flag, record.item.item_name
        ));
    }
    for (idx, bonus) in input.combo_bonuses.iter().enumerate() {
        debug_breakdown.push(format!(
            "combo_bonus[{idx}]=hp:{} mp:{} res:{} hp_regen:{} mp_regen:{} res_regen:{} def:{} abs:{} crit:{} ar:{} block:{} weight:{}",
            bonus.add_hp,
            bonus.add_mp,
            bonus.add_res,
            bonus.regen_hp,
            bonus.regen_mp,
            bonus.regen_res,
            bonus.defense,
            bonus.absorption,
            bonus.critical,
            bonus.attack_rating,
            bonus.block,
            bonus.weight_capacity
        ));
    }
    skill_formula::apply_skill_stat_formulas(
        skill_state,
        player_job,
        player.change_job,
        weapon.and_then(|v| EquippableItemType::try_from(v.code & 0xFFFF_0000).ok()),
        &input.active_continue_skills,
        &input.passive_skills,
        &mut out,
        &mut debug_breakdown,
    );

    if input.env_modifiers.view_damage_percent > 0 {
        let pct = input.env_modifiers.view_damage_percent as f32 / 100.0;
        out.attack_damage[0] += out.attack_damage[0] * pct;
        out.attack_damage[1] += out.attack_damage[1] * pct;
    } else if input.env_modifiers.view_damage_flat > 0 {
        out.attack_damage[0] += input.env_modifiers.view_damage_flat as f32;
        out.attack_damage[1] += input.env_modifiers.view_damage_flat as f32;
    }

    if input.premium_state.help_pet_active && input.premium_state.help_pet_damage_percent > 0 {
        let pct = input.premium_state.help_pet_damage_percent as f32 / 100.0;
        out.attack_damage[0] += out.attack_damage[0] * pct;
        out.attack_damage[1] += out.attack_damage[1] * pct;
    }

    apply_final_modifiers(input, &mut out);

    let mut updated_char = input.base_player.clone();
    updated_char.attack_rating = out.attack_rating.round() as i32;
    updated_char.attack_damage = [
        out.attack_damage[0].round() as i32,
        out.attack_damage[1].round() as i32,
    ];
    updated_char.attack_speed = out.attack_speed.round() as i32;
    updated_char.shooting_range = out.shooting_range.round() as i32;
    updated_char.critical_hit = out.critical_hit.round() as i32;
    updated_char.defence = out.defence.round() as i32;
    updated_char.absorption = out.absorption.round() as i32;
    updated_char.chance_block = out.chance_block.round() as i32;
    updated_char.move_speed = out.move_speed.round() as i32;
    updated_char.sight = out.sight.round() as i32;
    updated_char.potion_space = total.potion_space.max(2);
    updated_char.weight = [
        clamp_i32_to_i16(out.weight[0].round() as i32),
        clamp_i32_to_i16(out.weight[1].round() as i32),
    ];
    for i in 0..8 {
        updated_char.resistance[i] = clamp_i32_to_i16(out.resistance[i].round() as i32);
    }
    updated_char.life[1] = clamp_i32_to_i16(out.life[1].round() as i32);
    updated_char.mana[1] = clamp_i32_to_i16(out.mana[1].round() as i32);
    updated_char.stamina[1] = clamp_i32_to_i16(out.stamina[1].round() as i32);
    updated_char.life_regen = out.life_regen;
    updated_char.mana_regen = out.mana_regen;
    updated_char.stamina_regen = out.stamina_regen;

    UserCalcOutput {
        updated_char_info: updated_char,
        derived_stats: out,
        debug_breakdown,
    }
}

pub fn apply_calc_output(state: &mut UserPlayerState, output: &UserCalcOutput) {
    state.core.char_info = output.updated_char_info.clone();
    state.inventory.potion_space = state.core.char_info.potion_space.max(2);
}

pub fn build_calc_input_from_state(state: &UserPlayerState) -> UserCalcInput {
    UserCalcInput {
        base_player: state.core.char_info.clone(),
        equipped_items: state.inventory.record_items.clone(),
        combo_bonuses: collect_combo_bonuses(&state.inventory.record_items),
        active_continue_skills: state.active_continue_skills_vec(),
        passive_skills: state.runtime.passive_skills.clone(),
        damage_function: [
            state.core.char_info.damage_function[0] as i32,
            state.core.char_info.damage_function[1] as i32,
            0,
        ],
        mana_function: state.core.char_info.mana_function,
        ..Default::default()
    }
}

pub fn recalculate_user_player_state(
    state: &mut UserPlayerState,
    skill_state: Option<&SkillState>,
) -> UserCalcOutput {
    let input = build_calc_input_from_state(state);
    let output = calculate_user_player(&input, skill_state);
    apply_calc_output(state, &output);
    output
}

// Compatibility wrappers for existing call sites/tests.
pub fn calculate_char_stats(
    player: &UserChar,
    equipped_items: &[UserRecordItem],
    ctx: &CalcContext,
) -> CalculatedCharStats {
    let active_continue_skills = ctx
        .active_buff_codes
        .iter()
        .copied()
        .map(|code| UserContinueSkill {
            code,
            ..Default::default()
        })
        .collect();
    let passive_skills = ctx
        .passive_skill_codes
        .iter()
        .copied()
        .map(|code| UserPassiveSkill {
            code,
            ..Default::default()
        })
        .collect();

    let input = UserCalcInput {
        base_player: player.clone(),
        equipped_items: equipped_items.to_vec(),
        combo_bonuses: ctx.combo_bonuses.clone(),
        active_continue_skills,
        passive_skills,
        damage_function: ctx.damage_function,
        mana_function: ctx.mana_function,
        quest_flags: crate::domain::user_player::dto::UserQuestFlags {
            check_quest_item_down_flag: ctx.check_quest_item_down_flag,
            has_level_90_quest_bonus: ctx.has_level_90_quest_bonus,
            ..Default::default()
        },
        life_preset: ctx.life_preset,
        ..Default::default()
    };
    calculate_user_player(&input, None).derived_stats
}

pub fn items_from_record_data(records: &[UserRecordItem]) -> Vec<UserItem> {
    records.iter().map(|r| r.item.clone()).collect()
}

pub fn item_damage_pair_as_f32(item: &UserItem) -> [f32; 2] {
    [item.damage[0] as f32, item.damage[1] as f32]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::item::item_code::ItemCode;
    use crate::domain::skill::dtos::codes::SKILL_CONCENTRATION;
    use crate::domain::skill::dtos::codes::{SkillCode, SKILL_MELEE_MASTERY, SKILL_SPARK};
    use crate::domain::skill::dtos::skills::{
        Concentration, Levels, SkillClassConfig, SkillEntry, SkillValueType,
    };
    use crate::domain::skill::state::SkillState;
    use crate::domain::user_player::char_motion_state::CharMotionState;
    use crate::domain::user_player::dto::{UserItem, UserPlayerData, UserRecordItem};
    use std::collections::HashMap;

    fn base_player() -> UserChar {
        UserChar {
            state: CharMotionState::Unknown(0),
            level: 100,
            strength: 200,
            spirit: 100,
            talent: 120,
            dexterity: 150,
            health: 180,
            life: [100, 100],
            mana: [80, 80],
            stamina: [70, 70],
            damage_function: [1, 2],
            mana_function: 2,
            ..Default::default()
        }
    }

    #[test]
    fn calculate_user_player_updates_combat_and_char() {
        let player = base_player();
        let output = calculate_user_player(
            &UserCalcInput {
                base_player: player.clone(),
                damage_function: [1, 0, 0],
                mana_function: 2,
                ..Default::default()
            },
            None,
        );
        assert!(output.updated_char_info.attack_rating > 0);
        assert!(
            output.updated_char_info.attack_damage[1] >= output.updated_char_info.attack_damage[0]
        );
        assert!(output.updated_char_info.life[1] > 0);
    }

    #[test]
    fn duplicate_slot_keeps_first_item_only() {
        let player = base_player();
        let item1 = UserItem {
            attack_rating: 10,
            ..Default::default()
        };
        let item2 = UserItem {
            attack_rating: 50,
            ..Default::default()
        };
        let record1 = UserRecordItem {
            position: 1,
            item: item1,
            ..Default::default()
        };
        let record2 = UserRecordItem {
            position: 1,
            item: item2,
            ..Default::default()
        };

        let output = calculate_user_player(
            &UserCalcInput {
                base_player: player,
                equipped_items: vec![record1, record2],
                damage_function: [1, 0, 0],
                mana_function: 2,
                ..Default::default()
            },
            None,
        );
        assert!(output
            .debug_breakdown
            .iter()
            .any(|v| v == "normalized_equipped_items=1"));
    }

    #[test]
    fn apply_calc_output_mutates_user_player_state() {
        let base = base_player();
        let output = calculate_user_player(
            &UserCalcInput {
                base_player: base.clone(),
                damage_function: [1, 0, 0],
                mana_function: 2,
                ..Default::default()
            },
            None,
        );
        let mut state = crate::domain::user_player::state::UserPlayerState::from_record_data(
            1,
            crate::domain::user_player::dto::UserPlayerData {
                char_info: base,
                ..Default::default()
            },
            output.clone(),
        );

        apply_calc_output(&mut state, &output);
        assert_eq!(
            state.core.char_info.attack_rating,
            output.updated_char_info.attack_rating
        );
    }

    #[test]
    fn calculate_char_stats_accepts_typed_skill_codes_in_context() {
        let player = base_player();
        let ctx = CalcContext {
            active_buff_codes: vec![SKILL_SPARK],
            passive_skill_codes: vec![SKILL_MELEE_MASTERY],
            ..Default::default()
        };

        let output = calculate_char_stats(&player, &[], &ctx);
        assert!(output.attack_rating >= 0.0);
        assert!(SkillCode::try_from(SKILL_SPARK.raw()).is_ok());
    }

    #[test]
    fn calculate_user_player_applies_skill_state_rules() {
        let player = UserChar {
            job_code: PlayerJob::Fighter,
            change_job: ChangeJobTierState::Tier3,
            ..base_player()
        };
        let mut by_code = HashMap::new();
        by_code.insert(
            SKILL_CONCENTRATION,
            SkillEntry::Concentration(Concentration {
                name: "Concentration".to_string(),
                description: "test".to_string(),
                require_level: 1,
                use_stamina: Some((0, 0)),
                require_mastery: Some((0, 0)),
                element: Some((0, 0, 0)),
                use_weapon_code: vec![],
                skill_code: SKILL_CONCENTRATION,
                attack_rate: Levels::new(
                    SkillValueType::Fixed2,
                    [100, 100, 100, 100, 100, 100, 100, 100, 100, 100],
                ),
                time: Levels::new(SkillValueType::Fixed2, [1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
                use_mana: Levels::new(SkillValueType::Fixed2, [1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
            }),
        );
        let skill_state = SkillState {
            state: HashMap::from([(PlayerJob::Fighter, SkillClassConfig { by_code })]),
        };

        let output = calculate_user_player(
            &UserCalcInput {
                base_player: player,
                active_continue_skills: vec![UserContinueSkill {
                    code: SKILL_CONCENTRATION,
                    point: 1,
                    flag: 1,
                    ..Default::default()
                }],
                ..Default::default()
            },
            Some(&skill_state),
        );

        assert!(output
            .debug_breakdown
            .iter()
            .any(|v| v == "skill_formula_configured_applied=1"));
    }

    #[test]
    fn build_calc_input_from_state_collects_combo_item_bonuses() {
        let mut data = UserPlayerData::default();
        data.record_items = vec![
            UserRecordItem {
                position: 7,
                item: UserItem {
                    code: ItemCode::OR248.raw(),
                    ..Default::default()
                },
                ..Default::default()
            },
            UserRecordItem {
                position: 9,
                item: UserItem {
                    code: ItemCode::OS117.raw(),
                    ..Default::default()
                },
                ..Default::default()
            },
            UserRecordItem {
                position: 10,
                item: UserItem {
                    code: ItemCode::OA144.raw(),
                    ..Default::default()
                },
                ..Default::default()
            },
        ];
        let state = crate::domain::user_player::state::UserPlayerState::from_record_data(
            1,
            data,
            UserCalcOutput::default(),
        );

        let input = build_calc_input_from_state(&state);

        assert_eq!(input.combo_bonuses.len(), 1);
        assert_eq!(input.combo_bonuses[0].add_hp, 150);
        assert_eq!(input.combo_bonuses[0].add_mp, 150);
        assert_eq!(input.combo_bonuses[0].regen_hp, 2.0);
    }
}
