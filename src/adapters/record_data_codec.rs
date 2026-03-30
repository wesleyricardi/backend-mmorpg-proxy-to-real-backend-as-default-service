use crate::domain::user_player::dto::UserPlayerData;
use crate::application::use_case::runtime::world_runtime::world_command::{
    RecordDataResponseContext, RecordDataWireCharSnapshot, RecordDataWireItemSnapshot,
    RecordDataWireSnapshot,
};
use crate::error::AppError;
use crate::libs::decompress_record_items;
use crate::protocol::data::{RecordData, RecordDatas};
use crate::protocol::item::RecordItem;

const RECORD_ITEM_ITEM_NAME_OFFSET: usize = 44;
const RECORD_ITEM_ITEM_NAME_LEN: usize = 32;

fn clamp_i32_to_i16_range(value: i32) -> i32 {
    value.clamp(i16::MIN as i32, i16::MAX as i32)
}

fn normalize_legacy_resource_pair_i16(current: i16, max: i16) -> [i16; 2] {
    let normalized_max = max.max(0);
    [current.clamp(0, normalized_max), normalized_max]
}

fn normalize_legacy_char_info(char_info: &mut crate::protocol::char::Char) {
    let life_max = clamp_i32_to_i16_range(char_info.life[1]).max(0);
    let life_current = clamp_i32_to_i16_range(char_info.life[0]).clamp(0, life_max);
    char_info.life = [life_current, life_max];

    char_info.mana = normalize_legacy_resource_pair_i16(char_info.mana[0], char_info.mana[1]);
    char_info.stamina =
        normalize_legacy_resource_pair_i16(char_info.stamina[0], char_info.stamina[1]);
}

pub fn build_record_data_response_context(record_datas: &RecordDatas) -> RecordDataResponseContext {
    RecordDataResponseContext {
        packet_header: record_datas.data.header.clone(),
        char_name: record_datas.data.char_info.sz_name.clone(),
        char_model_name: record_datas.data.char_info.sz_model_name.clone(),
        char_model_name2: record_datas.data.char_info.sz_model_name2.to_string(),
        char_model_name_code2: record_datas.data.char_info.model_name_code2,
        game_save_head: record_datas.data.game_save.head,
        object_serial: record_datas.data.char_info.dw_object_serial,
        login_safe_key: record_datas.data.char_info.dw_login_server_safe_key as i32,
        accuracy: record_datas.data.char_info.accuracy,
        char_checksum: record_datas.data.game_save.check_sum_char_info,
        exp: record_datas.data.char_info.exp,
        next_exp: record_datas.data.char_info.next_exp,
        money: record_datas.data.char_info.money,
        lp_mon_info_addr: record_datas.data.char_info.lp_mon_info_addr,
        brood: record_datas.data.char_info.brood,
        update_info: record_datas.data.char_info.b_update_info.clone(),
        arrow_posi: record_datas.data.char_info.arrow_posi,
        class_clan: record_datas.data.char_info.class_clan,
        size_level: record_datas.data.char_info.size_level,
        char_sound_code: record_datas.data.char_info.dw_char_sound_code,
        exp_high: record_datas.data.char_info.exp_high,
        event_time_t: record_datas.data.char_info.dw_event_time_t,
        event_param: record_datas.data.char_info.s_event_param,
        present_item: record_datas.data.char_info.s_present_item,
        gravity_scroll_check: record_datas.data.char_info.gravity_scroll_check,
        player_killing: record_datas.data.char_info.player_killing,
        play_class: record_datas.data.char_info.play_class,
        char_temp: crate::adapters::record_data_mapper::map_dw_temp(
            record_datas.data.char_info.dw_temp,
        ),
        login_server_ip: record_datas.data.char_info.dw_login_server_ip,
        char_version: record_datas.data.char_info.w_version,
        pcr_no: record_datas.data.game_save.pcr_no,
        camera_mode: record_datas.data.game_save.camera_mode,
        pos_x: record_datas.data.game_save.pos_x,
        pos_z: record_datas.data.game_save.pos_z,
        last_money: record_datas.data.game_save.last_money,
        event_play: record_datas.data.game_save.event_play,
        pet_info: record_datas.data.game_save.pet_info,
        potion_update: record_datas.data.game_save.potion_update,
        potion_count: record_datas.data.game_save.potion_count,
        force_orb_using: record_datas.data.game_save.force_orb_using,
        level_quest_log: record_datas.data.game_save.level_quest_log,
        short_key_normal_attack: record_datas.data.game_save.short_key_normal_attack,
        bless_castle_tax: record_datas.data.game_save.bless_castle_tax,
        bless_castle_master: record_datas.data.game_save.bless_castle_master,
        elementary_quest_log: record_datas.data.game_save.elementary_quest_log,
        server_id: record_datas.data.game_save.server_id,
        save_time: record_datas.data.game_save.save_time,
        time_prime_item_x2: record_datas.data.game_save.time_prime_item_x2,
        time_prime_item_exp_up: record_datas.data.game_save.time_prime_item_exp_up,
        time_prime_item_vamp_cuspid: record_datas.data.game_save.time_prime_item_vamp_cuspid,
        time_prime_item_mana_recharg: record_datas.data.game_save.time_prime_item_mana_recharg,
        prime_item_package_code: record_datas.data.game_save.prime_item_package_code,
        time_prime_item_mightof_awell: record_datas.data.game_save.time_prime_item_mightof_awell,
        time_prime_item_mightof_awell2: record_datas.data.game_save.time_prime_item_mightof_awell2,
        time_prime_item_mana_reduce: record_datas.data.game_save.time_prime_item_mana_reduce,
        time_prime_item_phenix_pet: record_datas.data.game_save.time_prime_item_phenix_pet,
        time_prime_item_help_pet: record_datas.data.game_save.time_prime_item_help_pet,
        param_prime_item_help_pet: record_datas.data.game_save.param_prime_item_help_pet,
        time_prime_item_vamp_cuspid_ex: record_datas.data.game_save.time_prime_item_vamp_cuspid_ex,
        time_prime_item_stamina_reduce: record_datas.data.game_save.time_prime_item_stamina_reduce,
        total_sub_exp: record_datas.data.game_save.total_sub_exp,
        total_money: record_datas.data.game_save.total_money,
        total_exp: record_datas.data.game_save.total_exp,
        master_id: record_datas.data.game_save.master_id.clone(),
        quest_info: record_datas.data.game_save.quest_info.clone().into(),
        last_quest_info: record_datas.data.game_save.last_quest_info.clone().into(),
        life_booster_using: record_datas.data.game_save.life_booster_using,
        mana_booster_using: record_datas.data.game_save.mana_booster_using,
        stamina_booster_using: record_datas.data.game_save.stamina_booster_using,
        skill_delay_using: record_datas.data.game_save.skill_delay_using,
        time_prime_item_big_head: record_datas.data.game_save.time_prime_item_big_head,
        param_prime_item_big_head: record_datas.data.game_save.param_prime_item_big_head,
        time_prime_item_vip: record_datas.data.game_save.time_prime_item_vip,
        param_prime_item_vip: record_datas.data.game_save.param_prime_item_vip,
        quest_start_state: record_datas.data.game_save.quest_start_state,
        quest_start_param1: record_datas.data.game_save.quest_start_param1,
        quest_start_param2: record_datas.data.game_save.quest_start_param2,
        quest_lv150a_state: record_datas.data.game_save.quest_lv150a_state,
        quest_lv150a_param: record_datas.data.game_save.quest_lv150a_param,
        quest_lv150a_time: record_datas.data.game_save.quest_lv150a_time,
        quest_lv150b_state: record_datas.data.game_save.quest_lv150b_state,
        quest_lv150b_param: record_datas.data.game_save.quest_lv150b_param,
        quest_lv150b_time: record_datas.data.game_save.quest_lv150b_time,
        pv_p_time: record_datas.data.game_save.pv_p_time,
        pv_p_param: record_datas.data.game_save.pv_p_param,
        weapon_class: record_datas.data.game_save.weapon_class,
        armor_class: record_datas.data.game_save.armor_class,
        shield_class: record_datas.data.game_save.shield_class,
        bracelets_class: record_datas.data.game_save.bracelets_class,
        gauntlets_class: record_datas.data.game_save.gauntlets_class,
        boots_class: record_datas.data.game_save.boots_class,
        ring_class: record_datas.data.game_save.ring_class,
        amy_class: record_datas.data.game_save.amy_class,
        shel_class: record_datas.data.game_save.shel_class,
        time_class: record_datas.data.game_save.time_class,
        warehouse_page_time: record_datas.data.game_save.warehouse_page_time,
        quest_lv155a_state: record_datas.data.game_save.quest_lv155a_state,
        quest_lv155a_param: record_datas.data.game_save.quest_lv155a_param,
        quest_lv155a_time: record_datas.data.game_save.quest_lv155a_time,
        random_monster_param: record_datas.data.game_save.random_monster_param,
        random_monster_time: record_datas.data.game_save.random_monster_time,
        temp2: record_datas.data.game_save.temp2,
        game_save_checksum: record_datas.data.game_save.checksum,
    }
}

pub fn build_user_player_data(record_datas: &RecordDatas) -> Result<UserPlayerData, AppError> {
    let mut record_data = record_datas.data.clone();
    let data_size = record_data.data_size.max(0) as usize;
    let used_size = record_data.record_item_compress.len().min(data_size);
    let record_item_compress = record_data.record_item_compress[..used_size].to_vec();

    let item_count = record_data.item_count.max(0) as usize;
    let record_items = if record_item_compress.is_empty() || item_count == 0 {
        Vec::new()
    } else {
        let decompressed_chunks =
            decompress_record_items::get_record_item_chunks(&record_item_compress, item_count)?;

        decompressed_chunks
            .into_iter()
            .map(|chunk| {
                RecordItem::from_bytes(&chunk)
                    .map_err(|err| {
                        log::error!(
                            "Failed to parse record item from decompressed data: {:?}",
                            err
                        );
                        crate::parse_error!(
                            crate::error::ParseErrorKind::RecordItem,
                            "failed to parse record item"
                        )
                    })
                    .map(Into::into)
            })
            .collect::<Result<Vec<_>, AppError>>()?
    };

    let game_save: crate::domain::user_player::dto::UserGameSave = record_data.game_save.into();
    normalize_legacy_char_info(&mut record_data.char_info);
    let char_info = crate::domain::user_player::dto::UserChar::try_from(record_data.char_info)?;

    Ok(UserPlayerData {
        char_info,
        game_save,
        throw_item: record_data.throw_item.map(|item| item.into()),
        throw_item_count: record_data.throw_item_count,
        item_count: record_data.item_count,
        item_sub_start: record_data.item_sub_start,
        record_items,
    })
}

pub fn build_record_data_wire_snapshot(
    record_datas: &RecordDatas,
) -> Result<RecordDataWireSnapshot, AppError> {
    let data_size = record_datas.data.data_size.max(0) as usize;
    let used_size = record_datas.data.record_item_compress.len().min(data_size);
    let record_item_compress = &record_datas.data.record_item_compress[..used_size];
    let item_count = record_datas.data.item_count.max(0) as usize;

    let items = if record_item_compress.is_empty() || item_count == 0 {
        Vec::new()
    } else {
        decompress_record_items::get_record_item_chunks(record_item_compress, item_count)?
            .into_iter()
            .map(|chunk| {
                let mut item_name_bytes = [0u8; RECORD_ITEM_ITEM_NAME_LEN];
                if chunk.len() >= RECORD_ITEM_ITEM_NAME_OFFSET + RECORD_ITEM_ITEM_NAME_LEN {
                    item_name_bytes.copy_from_slice(
                        &chunk[RECORD_ITEM_ITEM_NAME_OFFSET
                            ..RECORD_ITEM_ITEM_NAME_OFFSET + RECORD_ITEM_ITEM_NAME_LEN],
                    );
                }

                Ok(RecordDataWireItemSnapshot { item_name_bytes })
            })
            .collect::<Result<Vec<_>, AppError>>()?
    };

    Ok(RecordDataWireSnapshot {
        char_info: RecordDataWireCharSnapshot {
            unused_life: record_datas.data.char_info.unused_life,
        },
        items,
    })
}

pub fn decode_protocol_record_items(
    record_datas: &RecordDatas,
) -> Result<Vec<RecordItem>, AppError> {
    let data_size = record_datas.data.data_size.max(0) as usize;
    let used_size = record_datas.data.record_item_compress.len().min(data_size);
    let record_item_compress = &record_datas.data.record_item_compress[..used_size];
    let item_count = record_datas.data.item_count.max(0) as usize;

    if record_item_compress.is_empty() || item_count == 0 {
        return Ok(Vec::new());
    }

    decompress_record_items::get_record_items(record_item_compress, item_count)
}

pub fn parse_record_data_from_payload(payload: &[u8]) -> Result<RecordData, AppError> {
    let mut normalized = payload.to_vec();
    normalized.resize(
        std::mem::size_of::<<RecordData as socket::SocketPacket>::CRepr>(),
        0,
    );
    RecordData::from_bytes(&normalized).map_err(|err| {
        log::error!("Failed to parse RecordData payload: {:?}", err);
        crate::parse_error!(
            crate::error::ParseErrorKind::RecordItem,
            "failed to parse RecordData payload"
        )
    })
}
