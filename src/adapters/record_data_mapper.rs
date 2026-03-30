use crate::domain::user_player::char_motion_state::CharMotionState;
pub use crate::domain::user_player::dto::*;
use crate::error::{AppError, ParseErrorKind};
use crate::protocol;

impl TryFrom<protocol::job_code::RuntimeJobCode> for PlayerJob {
    type Error = InvalidPlayerJob;

    fn try_from(value: protocol::job_code::RuntimeJobCode) -> Result<Self, Self::Error> {
        match value.raw() {
            1 => Ok(Self::Fighter),
            2 => Ok(Self::Mechanician),
            3 => Ok(Self::Archer),
            4 => Ok(Self::Pikeman),
            5 => Ok(Self::Atalanta),
            6 => Ok(Self::Knight),
            7 => Ok(Self::Magician),
            8 => Ok(Self::Priestess),
            9 => Ok(Self::Assassine),
            10 => Ok(Self::Shaman),
            11 => Ok(Self::Martial),
            other => Err(InvalidPlayerJob(other)),
        }
    }
}

impl TryFrom<protocol::job_code::ClassJobCode> for PlayerJob {
    type Error = InvalidPlayerJob;

    fn try_from(value: protocol::job_code::ClassJobCode) -> Result<Self, Self::Error> {
        match value.raw() {
            0x100 => Ok(Self::Mechanician),
            0x200 => Ok(Self::Fighter),
            0x300 => Ok(Self::Pikeman),
            0x400 => Ok(Self::Archer),
            0x500 => Ok(Self::Knight),
            0x600 => Ok(Self::Atalanta),
            0x700 => Ok(Self::Priestess),
            0x800 => Ok(Self::Magician),
            0xA00 => Ok(Self::Assassine),
            0xB00 => Ok(Self::Shaman),
            0xC00 => Ok(Self::Martial),
            other => Err(InvalidPlayerJob(other)),
        }
    }
}

impl From<PlayerJob> for protocol::job_code::RuntimeJobCode {
    fn from(value: PlayerJob) -> Self {
        match value {
            PlayerJob::Fighter => Self(1),
            PlayerJob::Mechanician => Self(2),
            PlayerJob::Archer => Self(3),
            PlayerJob::Pikeman => Self(4),
            PlayerJob::Atalanta => Self(5),
            PlayerJob::Knight => Self(6),
            PlayerJob::Magician => Self(7),
            PlayerJob::Priestess => Self(8),
            PlayerJob::Assassine => Self(9),
            PlayerJob::Shaman => Self(10),
            PlayerJob::Martial => Self(11),
        }
    }
}

impl From<PlayerJob> for protocol::job_code::ClassJobCode {
    fn from(value: PlayerJob) -> Self {
        match value {
            PlayerJob::Mechanician => Self(0x100),
            PlayerJob::Fighter => Self(0x200),
            PlayerJob::Pikeman => Self(0x300),
            PlayerJob::Archer => Self(0x400),
            PlayerJob::Knight => Self(0x500),
            PlayerJob::Atalanta => Self(0x600),
            PlayerJob::Priestess => Self(0x700),
            PlayerJob::Magician => Self(0x800),
            PlayerJob::Assassine => Self(0xA00),
            PlayerJob::Shaman => Self(0xB00),
            PlayerJob::Martial => Self(0xC00),
        }
    }
}

pub fn map_dw_temp(raw: [u32; 9]) -> [u32; 11] {
    let mut mapped = [0_u32; 11];
    mapped[..raw.len()].copy_from_slice(&raw);
    mapped
}

impl TryFrom<protocol::char::Char> for UserChar {
    type Error = AppError;

    fn try_from(value: protocol::char::Char) -> Result<Self, Self::Error> {
        let job_code = PlayerJob::try_from(value.job_code).map_err(|_| {
            log::error!(
                "invalid job_code in ClientRecordData char_info: raw_job_code={}",
                value.job_code.raw()
            );
            crate::parse_error!(
                ParseErrorKind::IntegerConversion,
                "invalid char_info.job_code in ClientRecordData: {}",
                value.job_code.raw()
            )
        })?;
        let change_job =
            ChangeJobTierState::try_from((value.change_job.saturating_add(1)).clamp(1, 5))
                .unwrap_or(ChangeJobTierState::Tier1);
        let life_0 = i16::try_from(value.life[0]).map_err(|_| {
            crate::parse_error!(
                ParseErrorKind::IntegerConversion,
                "invalid char_info.life[0] in ClientRecordData: {}",
                value.life[0]
            )
        })?;
        let life_1 = i16::try_from(value.life[1]).map_err(|_| {
            crate::parse_error!(
                ParseErrorKind::IntegerConversion,
                "invalid char_info.life[1] in ClientRecordData: {}",
                value.life[1]
            )
        })?;
        Ok(Self {
            state: CharMotionState::from_raw(value.state),
            job_code,
            level: value.level,
            strength: value.strength,
            spirit: value.spirit,
            talent: value.talent,
            dexterity: value.dexterity,
            health: value.health,
            attack_rating: value.attack_rating,
            attack_damage: value.attack_damage,
            attack_speed: value.attack_speed,
            shooting_range: value.shooting_range,
            critical_hit: value.critical_hit,
            defence: value.defence,
            chance_block: value.chance_block,
            absorption: value.absorption,
            move_speed: value.move_speed,
            sight: value.sight,
            weight: value.weight,
            resistance: value.resistance,
            attack_resistance: value.attack_resistance,
            life: [life_0, life_1],
            mana: value.mana,
            stamina: value.stamina,
            life_regen: value.life_regen,
            mana_regen: value.mana_regen,
            stamina_regen: value.stamina_regen,
            state_point: value.state_point,
            potion_space: value.potion_space,
            life_function: value.life_function,
            mana_function: value.mana_function,
            stamina_function: value.stamina_function,
            damage_function: value.damage_function,
            change_job,
            job_bit_mask: value.job_bit_mask.into(),
        })
    }
}

impl From<protocol::data::SkillInfo> for UserSkillInfo {
    fn from(value: protocol::data::SkillInfo) -> Self {
        Self {
            b_skill_point: value.b_skill_point,
            w_skill_mastery: value.w_skill_mastery,
            b_short_key: value.b_short_key,
            mouse_pos: value.mouse_pos,
            w_select_skill: value.w_select_skill,
        }
    }
}

impl From<protocol::data::QuestInfo> for UserQuestInfo {
    fn from(value: protocol::data::QuestInfo) -> Self {
        Self {
            quest_code: value.quest_code,
            data: value.data,
        }
    }
}

impl From<protocol::data::LastQuestInfo> for UserLastQuestInfo {
    fn from(value: protocol::data::LastQuestInfo) -> Self {
        Self {
            last_quest: value.last_quest,
            last_quest_count: value.last_quest_count,
        }
    }
}

impl From<protocol::data::ThrowItem> for UserThrowItem {
    fn from(value: protocol::data::ThrowItem) -> Self {
        Self {
            code: value.code,
            key: value.key,
            sum: value.sum,
        }
    }
}

impl From<protocol::data::GameSave> for UserGameSave {
    fn from(value: protocol::data::GameSave) -> Self {
        Self {
            play_stage_num: value.play_stage_num,
            skill_info: value.skill_info.into(),
        }
    }
}

impl From<protocol::item::Header> for UserItemHeader {
    fn from(value: protocol::item::Header) -> Self {
        Self {
            head: value.head,
            version: value.version,
            time: value.time,
            checksum: value.checksum,
        }
    }
}

impl From<protocol::item::JobItem> for UserJobItem {
    fn from(value: protocol::item::JobItem) -> Self {
        Self {
            add_f_absorb: value.add_f_absorb,
            add_defense: value.add_defense,
            add_f_speed: value.add_f_speed,
            add_f_block_rating: value.add_f_block_rating,
            add_attack_speed: value.add_attack_speed,
            add_critical_hit: value.add_critical_hit,
            add_shooting_range: value.add_shooting_range,
            add_f_magic_mastery: value.add_f_magic_mastery,
            add_resistance: value.add_resistance,
            lev_attack_resistance: value.lev_attack_resistance,
            lev_mana: value.lev_mana,
            lev_life: value.lev_life,
            lev_attack_rating: value.lev_attack_rating,
            lev_damage: value.lev_damage,
            per_mana_regen: value.per_mana_regen,
            per_life_regen: value.per_life_regen,
            per_stamina_regen: value.per_stamina_regen,
            temp: value.temp,
        }
    }
}

impl From<protocol::item::SwapItem> for UserSwapItem {
    fn from(value: protocol::item::SwapItem) -> Self {
        Self {
            flag: value.flag,
            code: value.code,
        }
    }
}

impl From<protocol::item::Item> for UserItem {
    fn from(value: protocol::item::Item) -> Self {
        let item_header: UserItemHeader = value.item_header.into();
        Self {
            checksum: item_header.checksum,
            size: value.size,
            item_header,
            durability: value.durability,
            code: value.code,
            item_name: value.item_name.to_string(),
            weight: value.weight,
            price: value.price,
            index: value.index,
            potion_count: value.potion_count,
            resistance: value.resistance,
            sight: value.sight,
            temp0: value.temp0,
            damage: value.damage,
            shooting_range: value.shooting_range,
            attack_speed: value.attack_speed,
            attack_rating: value.attack_rating,
            critical_hit: value.critical_hit,
            f_absorb: value.f_absorb,
            defense: value.defense,
            f_block_rating: value.f_block_rating,
            f_speed: value.f_speed,
            potion_space: value.potion_space,
            f_magic_mastery: value.f_magic_mastery,
            f_mana_regen: value.f_mana_regen,
            f_life_regen: value.f_life_regen,
            f_stamina_regen: value.f_stamina_regen,
            f_increase_life: value.f_increase_life,
            f_increase_mana: value.f_increase_mana,
            f_increase_stamina: value.f_increase_stamina,
            level: value.level,
            strength: value.strength,
            spirit: value.spirit,
            talent: value.talent,
            dexterity: value.dexterity,
            health: value.health,
            mana: value.mana,
            life: value.life,
            stamina: value.stamina,
            money: value.money,
            not_use_flag: value.not_use_flag,
            back_up_key: value.back_up_key,
            back_up_chk_sum: value.back_up_chk_sum,
            scale_blink: value.scale_blink,
            unique_item: value.unique_item,
            effect_blink: value.effect_blink,
            effect_color: value.effect_color,
            disp_effect: value.disp_effect,
            job_code_mask: value.job_code_mask.into(),
            job_item: value.job_item.into(),
            item_kind_code: value.item_kind_code,
            item_kind_mask: value.item_kind_mask,
            item_aging_num: value.item_aging_num,
            item_aging_count: value.item_aging_count,
            item_aging_protect: value.item_aging_protect,
            special_item_flag: value.special_item_flag,
            swap_item: value.swap_item.into(),
            dw_create_time: value.dw_create_time,
            linked_item: value.linked_item,
            lock_item: value.lock_item,
            coin: value.coin,
            dw_temp: value.dw_temp,
        }
    }
}

impl From<protocol::item::RecordItem> for UserRecordItem {
    fn from(value: protocol::item::RecordItem) -> Self {
        Self {
            count: value.count,
            x: value.x,
            y: value.y,
            position: value.position,
            item: value.item.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::job_code::RuntimeJobCode;

    #[test]
    fn from_protocol_char_maps_job_and_change_job_to_typed_domain_fields() {
        let raw = protocol::char::Char {
            sz_name: "name".to_string(),
            sz_model_name: "m1".to_string(),
            sz_model_name2: "m2".to_string().into(),
            model_name_code2: 0,
            dw_object_serial: 0,
            class_clan: 0,
            state: 0,
            size_level: 0,
            dw_char_sound_code: 0,
            job_code: RuntimeJobCode(8),
            level: 0,
            strength: 0,
            spirit: 0,
            talent: 0,
            dexterity: 0,
            health: 0,
            accuracy: 0,
            attack_rating: 0,
            attack_damage: [0, 0],
            attack_speed: 0,
            shooting_range: 0,
            critical_hit: 0,
            defence: 0,
            chance_block: 0,
            absorption: 0,
            move_speed: 0,
            sight: 0,
            weight: [0, 0],
            resistance: [0; 8],
            attack_resistance: [0; 8],
            unused_life: [0, 0],
            mana: [0, 0],
            stamina: [0, 0],
            life_regen: 0.0,
            mana_regen: 0.0,
            stamina_regen: 0.0,
            exp: 0,
            next_exp: 0,
            money: 0,
            lp_mon_info_addr: 0,
            brood: 0,
            state_point: 0,
            b_update_info: String::new(),
            arrow_posi: [0, 0],
            potion_space: 0,
            life_function: 0,
            mana_function: 0,
            stamina_function: 0,
            damage_function: [0, 0],
            reform_code: 0,
            change_job: 4,
            job_bit_mask: 0,
            player_killing: [0, 0],
            play_class: [0, 0],
            exp_high: 0,
            dw_event_time_t: 0,
            s_event_param: [0, 0],
            s_present_item: [0, 0],
            gravity_scroll_check: [0, 0],
            life: [0, 0],
            dw_temp: [0; 9],
            dw_login_server_ip: 0,
            dw_login_server_safe_key: 0,
            w_version: [0, 0],
        };

        let mapped = UserChar::try_from(raw).expect("mapping should succeed");
        assert_eq!(mapped.job_code, PlayerJob::Priestess);
        assert_eq!(mapped.change_job, ChangeJobTierState::Tier5);
        assert_eq!(mapped.state, CharMotionState::Unknown(0));
    }

    #[test]
    fn from_protocol_char_maps_runtime_job_code() {
        let raw = protocol::char::Char {
            job_code: RuntimeJobCode(4),
            ..protocol::char::Char {
                sz_name: "name".to_string(),
                sz_model_name: "m1".to_string(),
                sz_model_name2: "m2".to_string().into(),
                model_name_code2: 0,
                dw_object_serial: 0,
                class_clan: 0,
                state: 0,
                size_level: 0,
                dw_char_sound_code: 0,
                job_code: RuntimeJobCode(0),
                level: 0,
                strength: 0,
                spirit: 0,
                talent: 0,
                dexterity: 0,
                health: 0,
                accuracy: 0,
                attack_rating: 0,
                attack_damage: [0, 0],
                attack_speed: 0,
                shooting_range: 0,
                critical_hit: 0,
                defence: 0,
                chance_block: 0,
                absorption: 0,
                move_speed: 0,
                sight: 0,
                weight: [0, 0],
                resistance: [0; 8],
                attack_resistance: [0; 8],
                unused_life: [0, 0],
                mana: [0, 0],
                stamina: [0, 0],
                life_regen: 0.0,
                mana_regen: 0.0,
                stamina_regen: 0.0,
                exp: 0,
                next_exp: 0,
                money: 0,
                lp_mon_info_addr: 0,
                brood: 0,
                state_point: 0,
                b_update_info: String::new(),
                arrow_posi: [0, 0],
                potion_space: 0,
                life_function: 0,
                mana_function: 0,
                stamina_function: 0,
                damage_function: [0, 0],
                reform_code: 0,
                change_job: 0,
                job_bit_mask: 0,
                player_killing: [0, 0],
                play_class: [0, 0],
                exp_high: 0,
                dw_event_time_t: 0,
                s_event_param: [0, 0],
                s_present_item: [0, 0],
                gravity_scroll_check: [0, 0],
                life: [0, 0],
                dw_temp: [0; 9],
                dw_login_server_ip: 0,
                dw_login_server_safe_key: 0,
                w_version: [0, 0],
            }
        };

        let mapped = UserChar::try_from(raw).expect("mapping should succeed");
        assert_eq!(mapped.job_code, PlayerJob::Pikeman);
    }
}
