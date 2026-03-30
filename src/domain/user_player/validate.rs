use std::fmt;

use crate::domain::user_player::dto::{
    UserChar, UserGameSave, UserItem, UserJobItem, UserPlayerData, UserRecordItem, UserSkillInfo,
    UserSwapItem, UserThrowItem,
};

#[derive(Debug, Clone, PartialEq)]
pub enum UserPlayerDataFieldChange {
    ThrowItemCount {
        old: i32,
        new: i32,
    },
    ItemCount {
        old: i32,
        new: i32,
    },
    ItemSubStart {
        old: i32,
        new: i32,
    },
    Char(UserCharFieldChange),
    GameSave(UserGameSaveFieldChange),
    ThrowItem {
        index: usize,
        change: UserThrowItemFieldChange,
    },
    RecordItem {
        index: usize,
        change: UserRecordItemFieldChange,
    },
    RecordItemAdded {
        index: usize,
    },
    RecordItemRemoved {
        index: usize,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserCharFieldChange {
    Level { old: i32, new: i32 },
    Strength { old: i32, new: i32 },
    AttackDamage { index: usize, old: i32, new: i32 },
    Resistance { index: usize, old: i16, new: i16 },
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserGameSaveFieldChange {
    PlayStageNum { old: i32, new: i32 },
    SkillInfo(UserSkillInfoFieldChange),
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserSkillInfoFieldChange {
    BSkillPoint { old: [u8; 20], new: [u8; 20] },
    WSkillMastery { index: usize, old: u16, new: u16 },
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserThrowItemFieldChange {
    Code { old: u32, new: u32 },
    Key { old: u32, new: u32 },
    Sum { old: u32, new: u32 },
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserRecordItemFieldChange {
    Count { old: i32, new: i32 },
    X { old: i32, new: i32 },
    Y { old: i32, new: i32 },
    Position { old: i32, new: i32 },
    Item(UserItemFieldChange),
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserItemFieldChange {
    Checksum { old: u32, new: u32 },
    Code { old: u32, new: u32 },
    ItemName { old: String, new: String },
    Price { old: i32, new: i32 },
    Resistance { index: usize, old: i16, new: i16 },
    Durability { index: usize, old: i16, new: i16 },
    JobItem(UserJobItemFieldChange),
    SwapItem(UserSwapItemFieldChange),
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserJobItemFieldChange {
    AddDefense { old: i32, new: i32 },
    AddResistance { index: usize, old: i16, new: i16 },
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserSwapItemFieldChange {
    Flag { old: u32, new: u32 },
    Code { old: u32, new: u32 },
}

macro_rules! push_if_ne {
    ($out:expr, $old:expr, $new:expr, $change:expr) => {
        if $old != $new {
            $out.push($change);
        }
    };
}

pub fn diff_user_player_data(
    old: &UserPlayerData,
    new: &UserPlayerData,
) -> Vec<UserPlayerDataFieldChange> {
    let mut changes = Vec::with_capacity(64);

    diff_user_char_into(&mut changes, &old.char_info, &new.char_info);
    diff_user_game_save_into(&mut changes, &old.game_save, &new.game_save);

    for (index, (old_throw, new_throw)) in
        old.throw_item.iter().zip(new.throw_item.iter()).enumerate()
    {
        diff_user_throw_item_into(&mut changes, index, old_throw, new_throw);
    }

    push_if_ne!(
        changes,
        old.throw_item_count,
        new.throw_item_count,
        UserPlayerDataFieldChange::ThrowItemCount {
            old: old.throw_item_count,
            new: new.throw_item_count
        }
    );
    push_if_ne!(
        changes,
        old.item_count,
        new.item_count,
        UserPlayerDataFieldChange::ItemCount {
            old: old.item_count,
            new: new.item_count
        }
    );
    push_if_ne!(
        changes,
        old.item_sub_start,
        new.item_sub_start,
        UserPlayerDataFieldChange::ItemSubStart {
            old: old.item_sub_start,
            new: new.item_sub_start
        }
    );

    diff_record_items_into(&mut changes, &old.record_items, &new.record_items);

    changes
}

fn diff_user_char_into(out: &mut Vec<UserPlayerDataFieldChange>, old: &UserChar, new: &UserChar) {
    push_if_ne!(
        out,
        old.level,
        new.level,
        UserPlayerDataFieldChange::Char(UserCharFieldChange::Level {
            old: old.level,
            new: new.level
        })
    );
    push_if_ne!(
        out,
        old.strength,
        new.strength,
        UserPlayerDataFieldChange::Char(UserCharFieldChange::Strength {
            old: old.strength,
            new: new.strength
        })
    );

    for (index, (&old_v, &new_v)) in old
        .attack_damage
        .iter()
        .zip(new.attack_damage.iter())
        .enumerate()
    {
        if old_v != new_v {
            out.push(UserPlayerDataFieldChange::Char(
                UserCharFieldChange::AttackDamage {
                    index,
                    old: old_v,
                    new: new_v,
                },
            ));
        }
    }
    for (index, (&old_v, &new_v)) in old.resistance.iter().zip(new.resistance.iter()).enumerate() {
        if old_v != new_v {
            out.push(UserPlayerDataFieldChange::Char(
                UserCharFieldChange::Resistance {
                    index,
                    old: old_v,
                    new: new_v,
                },
            ));
        }
    }
}

fn diff_user_game_save_into(
    out: &mut Vec<UserPlayerDataFieldChange>,
    old: &UserGameSave,
    new: &UserGameSave,
) {
    push_if_ne!(
        out,
        old.play_stage_num,
        new.play_stage_num,
        UserPlayerDataFieldChange::GameSave(UserGameSaveFieldChange::PlayStageNum {
            old: old.play_stage_num,
            new: new.play_stage_num
        })
    );
    diff_skill_info_into(out, &old.skill_info, &new.skill_info);
}

fn diff_skill_info_into(
    out: &mut Vec<UserPlayerDataFieldChange>,
    old: &UserSkillInfo,
    new: &UserSkillInfo,
) {
    push_if_ne!(
        out,
        old.b_skill_point,
        new.b_skill_point,
        UserPlayerDataFieldChange::GameSave(UserGameSaveFieldChange::SkillInfo(
            UserSkillInfoFieldChange::BSkillPoint {
                old: old.b_skill_point.clone(),
                new: new.b_skill_point.clone()
            }
        ))
    );

    for (index, (&old_v, &new_v)) in old
        .w_skill_mastery
        .iter()
        .zip(new.w_skill_mastery.iter())
        .enumerate()
    {
        if old_v != new_v {
            out.push(UserPlayerDataFieldChange::GameSave(
                UserGameSaveFieldChange::SkillInfo(UserSkillInfoFieldChange::WSkillMastery {
                    index,
                    old: old_v,
                    new: new_v,
                }),
            ));
        }
    }
}

fn diff_user_throw_item_into(
    out: &mut Vec<UserPlayerDataFieldChange>,
    index: usize,
    old: &UserThrowItem,
    new: &UserThrowItem,
) {
    if old.code != new.code {
        out.push(UserPlayerDataFieldChange::ThrowItem {
            index,
            change: UserThrowItemFieldChange::Code {
                old: old.code,
                new: new.code,
            },
        });
    }
    if old.key != new.key {
        out.push(UserPlayerDataFieldChange::ThrowItem {
            index,
            change: UserThrowItemFieldChange::Key {
                old: old.key,
                new: new.key,
            },
        });
    }
    if old.sum != new.sum {
        out.push(UserPlayerDataFieldChange::ThrowItem {
            index,
            change: UserThrowItemFieldChange::Sum {
                old: old.sum,
                new: new.sum,
            },
        });
    }
}

fn diff_record_items_into(
    out: &mut Vec<UserPlayerDataFieldChange>,
    old: &[UserRecordItem],
    new: &[UserRecordItem],
) {
    for (index, (old_item, new_item)) in old.iter().zip(new.iter()).enumerate() {
        diff_user_record_item_into(out, index, old_item, new_item);
    }

    if old.len() > new.len() {
        for index in new.len()..old.len() {
            out.push(UserPlayerDataFieldChange::RecordItemRemoved { index });
        }
    }

    if new.len() > old.len() {
        for index in old.len()..new.len() {
            out.push(UserPlayerDataFieldChange::RecordItemAdded { index });
        }
    }
}

fn diff_user_record_item_into(
    out: &mut Vec<UserPlayerDataFieldChange>,
    index: usize,
    old: &UserRecordItem,
    new: &UserRecordItem,
) {
    if old.count != new.count {
        out.push(UserPlayerDataFieldChange::RecordItem {
            index,
            change: UserRecordItemFieldChange::Count {
                old: old.count,
                new: new.count,
            },
        });
    }
    if old.x != new.x {
        out.push(UserPlayerDataFieldChange::RecordItem {
            index,
            change: UserRecordItemFieldChange::X {
                old: old.x,
                new: new.x,
            },
        });
    }
    if old.y != new.y {
        out.push(UserPlayerDataFieldChange::RecordItem {
            index,
            change: UserRecordItemFieldChange::Y {
                old: old.y,
                new: new.y,
            },
        });
    }
    if old.position != new.position {
        out.push(UserPlayerDataFieldChange::RecordItem {
            index,
            change: UserRecordItemFieldChange::Position {
                old: old.position,
                new: new.position,
            },
        });
    }

    if old.item.checksum != new.item.checksum {
        diff_user_item_into(out, index, &old.item, &new.item);
    }
}

fn push_item(out: &mut Vec<UserPlayerDataFieldChange>, index: usize, change: UserItemFieldChange) {
    out.push(UserPlayerDataFieldChange::RecordItem {
        index,
        change: UserRecordItemFieldChange::Item(change),
    });
}

fn diff_user_item_into(
    out: &mut Vec<UserPlayerDataFieldChange>,
    index: usize,
    old: &UserItem,
    new: &UserItem,
) {
    if old.checksum != new.checksum {
        push_item(
            out,
            index,
            UserItemFieldChange::Checksum {
                old: old.checksum,
                new: new.checksum,
            },
        );
    }
    if old.code != new.code {
        push_item(
            out,
            index,
            UserItemFieldChange::Code {
                old: old.code,
                new: new.code,
            },
        );
    }
    if old.item_name != new.item_name {
        push_item(
            out,
            index,
            UserItemFieldChange::ItemName {
                old: old.item_name.clone(),
                new: new.item_name.clone(),
            },
        );
    }
    if old.price != new.price {
        push_item(
            out,
            index,
            UserItemFieldChange::Price {
                old: old.price,
                new: new.price,
            },
        );
    }

    for (arr_index, (&old_v, &new_v)) in
        old.resistance.iter().zip(new.resistance.iter()).enumerate()
    {
        if old_v != new_v {
            push_item(
                out,
                index,
                UserItemFieldChange::Resistance {
                    index: arr_index,
                    old: old_v,
                    new: new_v,
                },
            );
        }
    }
    for (arr_index, (&old_v, &new_v)) in
        old.durability.iter().zip(new.durability.iter()).enumerate()
    {
        if old_v != new_v {
            push_item(
                out,
                index,
                UserItemFieldChange::Durability {
                    index: arr_index,
                    old: old_v,
                    new: new_v,
                },
            );
        }
    }

    diff_user_job_item_into(out, index, &old.job_item, &new.job_item);
    diff_user_swap_item_into(out, index, &old.swap_item, &new.swap_item);
}

fn diff_user_job_item_into(
    out: &mut Vec<UserPlayerDataFieldChange>,
    index: usize,
    old: &UserJobItem,
    new: &UserJobItem,
) {
    if old.add_defense != new.add_defense {
        push_item(
            out,
            index,
            UserItemFieldChange::JobItem(UserJobItemFieldChange::AddDefense {
                old: old.add_defense,
                new: new.add_defense,
            }),
        );
    }
    for (arr_index, (&old_v, &new_v)) in old
        .add_resistance
        .iter()
        .zip(new.add_resistance.iter())
        .enumerate()
    {
        if old_v != new_v {
            push_item(
                out,
                index,
                UserItemFieldChange::JobItem(UserJobItemFieldChange::AddResistance {
                    index: arr_index,
                    old: old_v,
                    new: new_v,
                }),
            );
        }
    }
}

fn diff_user_swap_item_into(
    out: &mut Vec<UserPlayerDataFieldChange>,
    index: usize,
    old: &UserSwapItem,
    new: &UserSwapItem,
) {
    if old.flag != new.flag {
        push_item(
            out,
            index,
            UserItemFieldChange::SwapItem(UserSwapItemFieldChange::Flag {
                old: old.flag,
                new: new.flag,
            }),
        );
    }
    if old.code != new.code {
        push_item(
            out,
            index,
            UserItemFieldChange::SwapItem(UserSwapItemFieldChange::Code {
                old: old.code,
                new: new.code,
            }),
        );
    }
}

impl fmt::Display for UserPlayerDataFieldChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserPlayerDataFieldChange::ThrowItemCount { old, new } => {
                write!(f, "throw_item_count: {old} -> {new}")
            }
            UserPlayerDataFieldChange::ItemCount { old, new } => {
                write!(f, "item_count: {old} -> {new}")
            }
            UserPlayerDataFieldChange::ItemSubStart { old, new } => {
                write!(f, "item_sub_start: {old} -> {new}")
            }
            UserPlayerDataFieldChange::Char(change) => write!(f, "char_info.{change}"),
            UserPlayerDataFieldChange::GameSave(change) => write!(f, "game_save.{change}"),
            UserPlayerDataFieldChange::ThrowItem { index, change } => {
                write!(f, "throw_item[{index}].{change}")
            }
            UserPlayerDataFieldChange::RecordItem { index, change } => {
                write!(f, "record_items[{index}].{change}")
            }
            UserPlayerDataFieldChange::RecordItemAdded { index } => {
                write!(f, "record_items[{index}] added")
            }
            UserPlayerDataFieldChange::RecordItemRemoved { index } => {
                write!(f, "record_items[{index}] removed")
            }
        }
    }
}

impl fmt::Display for UserCharFieldChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserCharFieldChange::Level { old, new } => write!(f, "level: {old} -> {new}"),
            UserCharFieldChange::Strength { old, new } => write!(f, "strength: {old} -> {new}"),
            UserCharFieldChange::AttackDamage { index, old, new } => {
                write!(f, "attack_damage[{index}]: {old} -> {new}")
            }
            UserCharFieldChange::Resistance { index, old, new } => {
                write!(f, "resistance[{index}]: {old} -> {new}")
            }
        }
    }
}

impl fmt::Display for UserGameSaveFieldChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserGameSaveFieldChange::PlayStageNum { old, new } => {
                write!(f, "play_stage_num: {old} -> {new}")
            }
            UserGameSaveFieldChange::SkillInfo(change) => write!(f, "skill_info.{change}"),
        }
    }
}

impl fmt::Display for UserSkillInfoFieldChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserSkillInfoFieldChange::BSkillPoint { old, new } => {
                let diffs = old
                    .iter()
                    .zip(new.iter())
                    .enumerate()
                    .filter_map(|(idx, (o, n))| {
                        if o != n {
                            Some(format!("{idx}:{o}->{n}"))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "b_skill_point: [{diffs}]")
            }
            UserSkillInfoFieldChange::WSkillMastery { index, old, new } => {
                write!(f, "w_skill_mastery[{index}]: {old} -> {new}")
            }
        }
    }
}

impl fmt::Display for UserThrowItemFieldChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserThrowItemFieldChange::Code { old, new } => write!(f, "code: {old} -> {new}"),
            UserThrowItemFieldChange::Key { old, new } => write!(f, "key: {old} -> {new}"),
            UserThrowItemFieldChange::Sum { old, new } => write!(f, "sum: {old} -> {new}"),
        }
    }
}

impl fmt::Display for UserRecordItemFieldChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserRecordItemFieldChange::Count { old, new } => write!(f, "count: {old} -> {new}"),
            UserRecordItemFieldChange::X { old, new } => write!(f, "x: {old} -> {new}"),
            UserRecordItemFieldChange::Y { old, new } => write!(f, "y: {old} -> {new}"),
            UserRecordItemFieldChange::Position { old, new } => {
                write!(f, "position: {old} -> {new}")
            }
            UserRecordItemFieldChange::Item(change) => write!(f, "item.{change}"),
        }
    }
}

impl fmt::Display for UserItemFieldChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserItemFieldChange::Checksum { old, new } => write!(f, "checksum: {old} -> {new}"),
            UserItemFieldChange::Code { old, new } => write!(f, "code: {old} -> {new}"),
            UserItemFieldChange::ItemName { old, new } => write!(f, "item_name: {old} -> {new}"),
            UserItemFieldChange::Price { old, new } => write!(f, "price: {old} -> {new}"),
            UserItemFieldChange::Resistance { index, old, new } => {
                write!(f, "resistance[{index}]: {old} -> {new}")
            }
            UserItemFieldChange::Durability { index, old, new } => {
                write!(f, "durability[{index}]: {old} -> {new}")
            }
            UserItemFieldChange::JobItem(change) => write!(f, "job_item.{change}"),
            UserItemFieldChange::SwapItem(change) => write!(f, "swap_item.{change}"),
        }
    }
}

impl fmt::Display for UserJobItemFieldChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserJobItemFieldChange::AddDefense { old, new } => {
                write!(f, "add_defense: {old} -> {new}")
            }
            UserJobItemFieldChange::AddResistance { index, old, new } => {
                write!(f, "add_resistance[{index}]: {old} -> {new}")
            }
        }
    }
}

impl fmt::Display for UserSwapItemFieldChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserSwapItemFieldChange::Flag { old, new } => write!(f, "flag: {old} -> {new}"),
            UserSwapItemFieldChange::Code { old, new } => write!(f, "code: {old} -> {new}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::item::dto::PlayerJobMask;
    use crate::domain::user_player::char_motion_state::CharMotionState;
    use crate::domain::user_player::dto::{
        ChangeJobTierState, PlayerJob, UserChar, UserGameSave, UserItem, UserItemHeader,
        UserJobItem, UserSkillInfo, UserSwapItem,
    };
    use std::array::from_fn;

    fn sample_player_data() -> UserPlayerData {
        let char_info = UserChar {
            state: CharMotionState::Unknown(4),
            job_code: PlayerJob::Priestess,
            level: 8,
            strength: 9,
            spirit: 10,
            talent: 11,
            dexterity: 12,
            health: 13,
            attack_rating: 15,
            attack_damage: [16, 17],
            attack_speed: 18,
            shooting_range: 19,
            critical_hit: 20,
            defence: 21,
            chance_block: 22,
            absorption: 23,
            move_speed: 24,
            sight: 25,
            weight: [26, 27],
            resistance: [28, 29, 30, 31, 32, 33, 34, 35],
            attack_resistance: [36, 37, 38, 39, 40, 41, 42, 43],
            life: [44, 45],
            mana: [46, 47],
            stamina: [48, 49],
            life_regen: 1.0,
            mana_regen: 2.0,
            stamina_regen: 3.0,
            state_point: 55,
            potion_space: 58,
            life_function: 59,
            mana_function: 60,
            stamina_function: 61,
            damage_function: [62, 63],
            change_job: ChangeJobTierState::Tier5,
            job_bit_mask: PlayerJobMask::PRIESTESS,
        };

        let game_save = UserGameSave {
            play_stage_num: 2,
            skill_info: UserSkillInfo {
                b_skill_point: from_fn(|i| i as u8),
                w_skill_mastery: from_fn(|i| i as u16),
                b_short_key: "s".to_string(),
                mouse_pos: "m".to_string(),
                w_select_skill: [1, 2],
            },
        };

        let base_item = UserItem {
            checksum: 500,
            size: 1,
            item_header: UserItemHeader {
                head: 1,
                version: 1,
                time: 1,
                checksum: 500,
            },
            durability: [1, 2],
            code: 10,
            item_name: "item".to_string().into(),
            weight: 1,
            price: 100,
            index: 0,
            potion_count: 0,
            resistance: [0; 8],
            sight: 0,
            temp0: 0,
            damage: [0, 0],
            shooting_range: 0,
            attack_speed: 0,
            attack_rating: 0,
            critical_hit: 0,
            f_absorb: 0.0,
            defense: 1,
            f_block_rating: 0.0,
            f_speed: 0.0,
            potion_space: 0,
            f_magic_mastery: 0.0,
            f_mana_regen: 0.0,
            f_life_regen: 0.0,
            f_stamina_regen: 0.0,
            f_increase_life: 0.0,
            f_increase_mana: 0.0,
            f_increase_stamina: 0.0,
            level: 0,
            strength: 0,
            spirit: 0,
            talent: 0,
            dexterity: 0,
            health: 0,
            mana: [0, 0],
            life: [0, 0],
            stamina: [0, 0],
            money: 0,
            not_use_flag: 0,
            back_up_key: 0,
            back_up_chk_sum: 0,
            scale_blink: [0, 0],
            unique_item: 0,
            effect_blink: [0, 0],
            effect_color: [0, 0, 0, 0],
            disp_effect: 0,
            job_code_mask: PlayerJobMask::default(),
            job_item: UserJobItem {
                add_defense: 5,
                ..UserJobItem::default()
            },
            item_kind_code: 0,
            item_kind_mask: 0,
            item_aging_num: [0, 0],
            item_aging_count: [0, 0],
            item_aging_protect: [0, 0],
            special_item_flag: [0, 0],
            swap_item: UserSwapItem { flag: 1, code: 2 },
            dw_create_time: 0,
            linked_item: 0,
            lock_item: 0,
            coin: 0,
            dw_temp: [0; 6],
        };
        let record_item = UserRecordItem {
            count: 1,
            x: 2,
            y: 3,
            position: 4,
            item: base_item,
        };

        UserPlayerData {
            char_info,
            game_save,
            throw_item: from_fn(|i| UserThrowItem {
                code: i as u32,
                key: (i + 1) as u32,
                sum: (i + 2) as u32,
            }),
            throw_item_count: 12,
            item_count: 13,
            item_sub_start: 14,
            record_items: vec![record_item.clone(), record_item],
        }
    }

    #[test]
    fn diff_user_player_data_sem_mudancas_retorna_vazio() {
        let old = sample_player_data();
        assert!(diff_user_player_data(&old, &old).is_empty());
    }

    #[test]
    fn diff_top_level_detecta_campos_basicos() {
        let old = sample_player_data();
        let mut new = old.clone();
        new.throw_item_count += 1;
        new.item_count += 1;
        new.item_sub_start += 1;
        let changes = diff_user_player_data(&old, &new);
        assert!(changes
            .iter()
            .any(|c| matches!(c, UserPlayerDataFieldChange::ThrowItemCount { .. })));
        assert!(changes
            .iter()
            .any(|c| matches!(c, UserPlayerDataFieldChange::ItemCount { .. })));
        assert!(changes
            .iter()
            .any(|c| matches!(c, UserPlayerDataFieldChange::ItemSubStart { .. })));
    }

    #[test]
    fn diff_char_detecta_scalar_e_array_com_index() {
        let old = sample_player_data();
        let mut new = old.clone();
        new.char_info.level += 1;
        new.char_info.strength += 1;
        new.char_info.attack_damage[1] += 1;
        new.char_info.resistance[2] += 1;
        let changes = diff_user_player_data(&old, &new);
        assert!(changes.iter().any(|c| matches!(
            c,
            UserPlayerDataFieldChange::Char(UserCharFieldChange::Level { .. })
        )));
        assert!(changes.iter().any(|c| matches!(
            c,
            UserPlayerDataFieldChange::Char(UserCharFieldChange::Strength { .. })
        )));
        assert!(changes.iter().any(|c| matches!(
            c,
            UserPlayerDataFieldChange::Char(UserCharFieldChange::AttackDamage { index: 1, .. })
        )));
        assert!(changes.iter().any(|c| matches!(
            c,
            UserPlayerDataFieldChange::Char(UserCharFieldChange::Resistance { index: 2, .. })
        )));
    }

    #[test]
    fn diff_game_save_detecta_scalar_array_e_matriz() {
        let old = sample_player_data();
        let mut new = old.clone();
        new.game_save.play_stage_num += 1;
        new.game_save.skill_info.w_skill_mastery[1] += 1;
        let changes = diff_user_player_data(&old, &new);
        assert!(changes.iter().any(|c| matches!(
            c,
            UserPlayerDataFieldChange::GameSave(UserGameSaveFieldChange::PlayStageNum { .. })
        )));
        assert!(changes.iter().any(|c| matches!(
            c,
            UserPlayerDataFieldChange::GameSave(UserGameSaveFieldChange::SkillInfo(
                UserSkillInfoFieldChange::WSkillMastery { index: 1, .. }
            ))
        )));
    }

    #[test]
    fn diff_game_save_detecta_skill_info_scalar() {
        let old = sample_player_data();
        let mut new = old.clone();
        new.game_save.skill_info.b_skill_point[3] += 1;
        let changes = diff_user_player_data(&old, &new);
        assert!(changes.iter().any(|c| matches!(
            c,
            UserPlayerDataFieldChange::GameSave(UserGameSaveFieldChange::SkillInfo(
                UserSkillInfoFieldChange::BSkillPoint { .. }
            ))
        )));
    }

    #[test]
    fn diff_throw_item_detecta_por_index() {
        let old = sample_player_data();
        let mut new = old.clone();
        new.throw_item[7].key += 1;
        let changes = diff_user_player_data(&old, &new);
        assert!(changes.iter().any(|c| matches!(
            c,
            UserPlayerDataFieldChange::ThrowItem {
                index: 7,
                change: UserThrowItemFieldChange::Key { .. }
            }
        )));
    }

    #[test]
    fn diff_record_items_profundo_detecta_metadados_e_item() {
        let old = sample_player_data();
        let mut new = old.clone();
        new.record_items[0].count += 1;
        new.record_items[0].item.checksum += 1;
        new.record_items[0].item.price += 1;
        new.record_items[0].item.resistance[3] += 1;
        let changes = diff_user_player_data(&old, &new);
        assert!(changes.iter().any(|c| matches!(
            c,
            UserPlayerDataFieldChange::RecordItem {
                index: 0,
                change: UserRecordItemFieldChange::Count { .. }
            }
        )));
        assert!(changes.iter().any(|c| matches!(
            c,
            UserPlayerDataFieldChange::RecordItem {
                index: 0,
                change: UserRecordItemFieldChange::Item(UserItemFieldChange::Price { .. })
            }
        )));
        assert!(changes.iter().any(|c| matches!(
            c,
            UserPlayerDataFieldChange::RecordItem {
                index: 0,
                change: UserRecordItemFieldChange::Item(UserItemFieldChange::Resistance {
                    index: 3,
                    ..
                })
            }
        )));
    }

    #[test]
    fn diff_record_items_detecta_tamanho() {
        let old = sample_player_data();
        let mut removed = old.clone();
        removed.record_items.pop();
        let removed_changes = diff_user_player_data(&old, &removed);
        assert!(removed_changes
            .iter()
            .any(|c| matches!(c, UserPlayerDataFieldChange::RecordItemRemoved { index: 1 })));

        let mut added = old.clone();
        added.record_items.push(added.record_items[0].clone());
        let added_changes = diff_user_player_data(&old, &added);
        assert!(added_changes
            .iter()
            .any(|c| matches!(c, UserPlayerDataFieldChange::RecordItemAdded { index: 2 })));
    }

    #[test]
    fn display_nao_aloca_e_formata_saida_esperada() {
        let rendered = UserPlayerDataFieldChange::Char(UserCharFieldChange::AttackDamage {
            index: 1,
            old: 10,
            new: 20,
        })
        .to_string();
        assert_eq!(rendered, "char_info.attack_damage[1]: 10 -> 20");
    }

    #[test]
    fn checksum_gate_item() {
        let old = sample_player_data();
        let mut new = old.clone();
        new.record_items[0].item.checksum += 1;
        new.record_items[0].item.price += 1;
        let changes = diff_user_player_data(&old, &new);
        assert!(changes.iter().any(|c| matches!(
            c,
            UserPlayerDataFieldChange::RecordItem {
                index: 0,
                change: UserRecordItemFieldChange::Item(UserItemFieldChange::Price { .. })
            }
        )));

        let same = old.clone();
        let changes_same = diff_user_player_data(&old, &same);
        assert!(!changes_same.iter().any(|c| matches!(
            c,
            UserPlayerDataFieldChange::RecordItem {
                change: UserRecordItemFieldChange::Item(_),
                ..
            }
        )));
    }

    #[test]
    fn diff_char_detecta_sem_checksum_gate() {
        let old = sample_player_data();
        let mut new = old.clone();
        new.char_info.level += 1;
        let changes = diff_user_player_data(&old, &new);
        assert!(changes.iter().any(|c| matches!(
            c,
            UserPlayerDataFieldChange::Char(UserCharFieldChange::Level { .. })
        )));
    }
}
