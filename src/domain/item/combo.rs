use crate::domain::item::item_code::ItemCode;
use crate::domain::user_player::dto::{ComboBonusStats, UserRecordItem};

#[derive(Debug, Clone)]
struct ComboRule {
    amount_required: u8,
    bonus: ComboBonusStats,
}

#[derive(Debug, Clone, Copy)]
struct ComboSet {
    codes: &'static [ItemCode],
    rules: &'static [ComboRule],
}

const COMBO_FORCA_DOS_DEUSES_CODES: &[ItemCode] = &[
    ItemCode::OR248,
    ItemCode::OR264,
    ItemCode::OA144,
    ItemCode::OA160,
    ItemCode::OS117,
];
const COMBO_SABEDORIA_DOS_ANCIOES_CODES: &[ItemCode] = &[
    ItemCode::OR129,
    ItemCode::OR145,
    ItemCode::OA129,
    ItemCode::OA145,
];
const COMBO_ESSENCIA_DOS_ANCIOES_CODES: &[ItemCode] = &[
    ItemCode::OA228,
    ItemCode::OA244,
    ItemCode::DB128,
    ItemCode::DB144,
    ItemCode::DG128,
    ItemCode::DG144,
];
const COMBO_PODER_DOS_ANCIOES_CODES: &[ItemCode] = &[
    ItemCode::DA302,
    ItemCode::DA402,
    ItemCode::WA130,
    ItemCode::WA146,
    ItemCode::WC129,
    ItemCode::WC145,
    ItemCode::WD129,
    ItemCode::WD145,
    ItemCode::WH131,
    ItemCode::WH147,
    ItemCode::WM131,
    ItemCode::WM147,
    ItemCode::WN129,
    ItemCode::WN145,
    ItemCode::WP130,
    ItemCode::WP146,
    ItemCode::WS131,
    ItemCode::WS147,
    ItemCode::WS233,
    ItemCode::WS249,
    ItemCode::WT130,
    ItemCode::WT146,
    ItemCode::WV129,
    ItemCode::WV145,
];
const COMBO_FORCA_NEFASTA_CODES: &[ItemCode] = &[
    ItemCode::OA252,
    ItemCode::OA268,
    ItemCode::DB150,
    ItemCode::DB166,
    ItemCode::DG130,
    ItemCode::DG146,
];

const COMBO_FORCA_DOS_DEUSES_RULES: &[ComboRule] = &[
    ComboRule {
        amount_required: 3,
        bonus: ComboBonusStats {
            add_hp: 150,
            add_mp: 150,
            add_res: 150,
            regen_hp: 2.0,
            regen_mp: 2.0,
            regen_res: 2.0,
            defense: 0,
            absorption: 0,
            critical: 0,
            attack_rating: 0,
            block: 0,
            weight_capacity: 0,
        },
    },
    ComboRule {
        amount_required: 4,
        bonus: ComboBonusStats {
            add_hp: 0,
            add_mp: 0,
            add_res: 0,
            regen_hp: 0.0,
            regen_mp: 0.0,
            regen_res: 0.0,
            defense: 100,
            absorption: 3,
            critical: 5,
            attack_rating: 120,
            block: 8,
            weight_capacity: 0,
        },
    },
];

const COMBO_SABEDORIA_DOS_ANCIOES_RULES: &[ComboRule] = &[ComboRule {
    amount_required: 3,
    bonus: ComboBonusStats {
        add_hp: 50,
        add_mp: 100,
        add_res: 0,
        regen_hp: 2.0,
        regen_mp: 2.0,
        regen_res: 2.0,
        defense: 100,
        absorption: 7,
        critical: 9,
        attack_rating: 120,
        block: 14,
        weight_capacity: 0,
    },
}];

const COMBO_ESSENCIA_DOS_ANCIOES_RULES: &[ComboRule] = &[ComboRule {
    amount_required: 3,
    bonus: ComboBonusStats {
        add_hp: 0,
        add_mp: 0,
        add_res: 0,
        regen_hp: 2.0,
        regen_mp: 2.0,
        regen_res: 2.0,
        defense: 0,
        absorption: 0,
        critical: 0,
        attack_rating: 0,
        block: 0,
        weight_capacity: 0,
    },
}];

const COMBO_PODER_DOS_ANCIOES_RULES: &[ComboRule] = &[ComboRule {
    amount_required: 2,
    bonus: ComboBonusStats {
        add_hp: 400,
        add_mp: 0,
        add_res: 0,
        regen_hp: 0.0,
        regen_mp: 0.0,
        regen_res: 0.0,
        defense: 0,
        absorption: 0,
        critical: 0,
        attack_rating: 0,
        block: 0,
        weight_capacity: 0,
    },
}];

const COMBO_FORCA_NEFASTA_RULES: &[ComboRule] = &[ComboRule {
    amount_required: 3,
    bonus: ComboBonusStats {
        add_hp: 0,
        add_mp: 0,
        add_res: 100,
        regen_hp: 0.0,
        regen_mp: 0.0,
        regen_res: 0.0,
        defense: 300,
        absorption: 0,
        critical: 0,
        attack_rating: 0,
        block: 0,
        weight_capacity: 300,
    },
}];

const COMBO_SETS: &[ComboSet] = &[
    ComboSet {
        codes: COMBO_FORCA_DOS_DEUSES_CODES,
        rules: COMBO_FORCA_DOS_DEUSES_RULES,
    },
    ComboSet {
        codes: COMBO_SABEDORIA_DOS_ANCIOES_CODES,
        rules: COMBO_SABEDORIA_DOS_ANCIOES_RULES,
    },
    ComboSet {
        codes: COMBO_ESSENCIA_DOS_ANCIOES_CODES,
        rules: COMBO_ESSENCIA_DOS_ANCIOES_RULES,
    },
    ComboSet {
        codes: COMBO_PODER_DOS_ANCIOES_CODES,
        rules: COMBO_PODER_DOS_ANCIOES_RULES,
    },
    ComboSet {
        codes: COMBO_FORCA_NEFASTA_CODES,
        rules: COMBO_FORCA_NEFASTA_RULES,
    },
];

fn equipped_codes(records: &[UserRecordItem]) -> Vec<u32> {
    records
        .iter()
        .filter(|record| (1..=16).contains(&record.position))
        .filter(|record| record.item.not_use_flag == 0)
        .map(|record| record.item.code)
        .collect()
}

pub fn collect_combo_bonuses(records: &[UserRecordItem]) -> Vec<ComboBonusStats> {
    let equipped_codes = equipped_codes(records);
    if equipped_codes.is_empty() {
        return Vec::new();
    }

    let mut out = Vec::new();

    for combo in COMBO_SETS {
        let mut amount_found: usize = 0;

        for combo_code in combo.codes {
            let target_code = combo_code.raw();
            for equipped_code in &equipped_codes {
                if *equipped_code == target_code {
                    amount_found += 1;
                }
            }
        }

        if amount_found == 0 {
            continue;
        }

        for rule in combo.rules {
            if amount_found >= rule.amount_required as usize {
                out.push(rule.bonus.clone());
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::user_player::dto::{UserItem, UserRecordItem};

    fn equipped_record(position: i32, code: ItemCode) -> UserRecordItem {
        UserRecordItem {
            position,
            item: UserItem {
                code: code.raw(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    #[test]
    fn combo_forca_dos_deuses_triggers_on_three_items() {
        let records = vec![
            equipped_record(7, ItemCode::OR248),
            equipped_record(10, ItemCode::OA144),
            equipped_record(9, ItemCode::OS117),
        ];

        let bonuses = collect_combo_bonuses(&records);

        assert_eq!(bonuses.len(), 1);
        assert_eq!(bonuses[0].add_hp, 150);
        assert_eq!(bonuses[0].add_mp, 150);
        assert_eq!(bonuses[0].regen_hp, 2.0);
    }

    #[test]
    fn combo_forca_dos_deuses_triggers_second_rule_on_duplicate_match_count() {
        let records = vec![
            equipped_record(7, ItemCode::OR248),
            equipped_record(8, ItemCode::OR248),
            equipped_record(10, ItemCode::OA144),
            equipped_record(9, ItemCode::OS117),
        ];

        let bonuses = collect_combo_bonuses(&records);

        assert_eq!(bonuses.len(), 2);
        assert!(bonuses.iter().any(|b| b.add_hp == 150));
        assert!(bonuses
            .iter()
            .any(|b| b.defense == 100 && b.attack_rating == 120));
    }

    #[test]
    fn ignores_items_outside_equipped_positions() {
        let records = vec![
            equipped_record(0, ItemCode::OR248),
            equipped_record(10, ItemCode::OA144),
            equipped_record(9, ItemCode::OS117),
        ];

        let bonuses = collect_combo_bonuses(&records);
        assert!(bonuses.is_empty());
    }

    #[test]
    fn combo_forca_dos_deuses_triggers_for_current_item_variants() {
        let records = vec![
            equipped_record(5, ItemCode::OR264),
            equipped_record(4, ItemCode::OA160),
            equipped_record(7, ItemCode::OS117),
        ];

        let bonuses = collect_combo_bonuses(&records);

        assert_eq!(bonuses.len(), 1);
        assert_eq!(bonuses[0].add_hp, 150);
        assert_eq!(bonuses[0].add_mp, 150);
        assert_eq!(bonuses[0].add_res, 150);
    }

    #[test]
    fn combo_forca_nefasta_triggers_for_current_item_variants() {
        let records = vec![
            equipped_record(8, ItemCode::OA268),
            equipped_record(10, ItemCode::DB166),
            equipped_record(9, ItemCode::DG146),
        ];

        let bonuses = collect_combo_bonuses(&records);

        assert_eq!(bonuses.len(), 1);
        assert_eq!(bonuses[0].defense, 300);
        assert_eq!(bonuses[0].add_res, 100);
        assert_eq!(bonuses[0].weight_capacity, 300);
    }

    #[test]
    fn combo_poder_dos_ancioes_triggers_with_verus_weapon_variant() {
        let records = vec![
            equipped_record(3, ItemCode::DA402),
            equipped_record(1, ItemCode::WM147),
        ];

        let bonuses = collect_combo_bonuses(&records);

        assert_eq!(bonuses.len(), 1);
        assert_eq!(bonuses[0].add_hp, 400);
    }
}
