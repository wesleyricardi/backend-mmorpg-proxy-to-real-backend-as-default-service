use std::mem::size_of;

use socket::{ReqHead, RequestVecBody, SocketPacket};

use crate::application::use_case::runtime::world_runtime::world_command::{
    RecordDataResponseContext, RecordDataWireSnapshot,
};
use crate::domain::user_player::dto::{
    ChangeJobTierState, UserChar, UserGameSave, UserItem, UserItemHeader, UserJobItem,
    UserLastQuestInfo, UserPlayerData, UserQuestInfo, UserRecordItem, UserSkillInfo, UserSwapItem,
    UserThrowItem,
};
use crate::error::{AppError, ParseErrorKind};
use crate::libs::decompress_record_items::encode_compress;
use crate::protocol::{
    char::Char,
    command::CharCommand,
    data::{GameSave, LastQuestInfo, QuestInfo, RecordData, SkillInfo, ThrowItem},
    item::{Header, Item, JobItem, RecordItem, SwapItem},
    transcode::route_code::Getrecorddata,
};

const REQUEST_HEAD_SIZE: usize = 8;
const TRANS_RECORD_LEN: usize = 32768 - 32;

pub fn build_fail_record_data_command(
    request: &CharCommand,
    object_serial: u32,
    login_safe_key: i32,
) -> CharCommand {
    CharCommand {
        l_param: login_safe_key,
        w_param: object_serial as i32,
        s_param: request.s_param,
        name: request.name.clone(),
    }
}

pub fn build_record_data_packets(
    player_data: &UserPlayerData,
    wire_snapshot: &RecordDataWireSnapshot,
    response_context: &RecordDataResponseContext,
) -> Result<Vec<RequestVecBody>, AppError> {
    let record_item_compress = encode_record_items(&player_data.record_items, wire_snapshot)?;
    let mut record_item_compress_fixed = [0u8; 31188];
    if record_item_compress.len() > record_item_compress_fixed.len() {
        return Err(crate::parse_error!(
            ParseErrorKind::RecordItem,
            "compressed record items exceed fixed buffer: {} > {}",
            record_item_compress.len(),
            record_item_compress_fixed.len()
        ));
    }
    record_item_compress_fixed[..record_item_compress.len()].copy_from_slice(&record_item_compress);

    let mut game_save = to_protocol_game_save(&player_data.char_info, &player_data.game_save);
    apply_response_context_to_game_save(&mut game_save, response_context);

    let mut char_info = to_protocol_char(
        &player_data.char_info,
        wire_snapshot,
        response_context.object_serial,
    );
    apply_response_context_to_char(&mut char_info, response_context);
    char_info.reform_code = compute_reform_code(&char_info, 10);

    let record_data = RecordData {
        size: compute_record_data_size(record_item_compress.len()) as i32,
        code: Getrecorddata - 1,
        header: resolve_packet_header(response_context),
        char_info,
        game_save,
        throw_item: player_data.throw_item.clone().map(to_protocol_throw_item),
        throw_item_count: player_data.throw_item_count,
        item_count: player_data
            .item_count
            .max(player_data.record_items.len() as i32),
        item_sub_start: player_data.item_sub_start,
        data_size: record_item_compress.len() as i32,
        record_item_compress: record_item_compress_fixed,
    };

    let mut record_bytes = record_data.to_bytes();
    let effective_size = usize::try_from(record_data.size.max(0)).unwrap_or(record_bytes.len());
    let effective_size = effective_size.min(record_bytes.len());
    record_bytes.truncate(effective_size);

    let packets = fragment_record_data(record_bytes);
    validate_fragmented_record_data(&packets)?;
    Ok(packets)
}

fn compute_record_data_size(compressed_items_len: usize) -> usize {
    size_of::<<RecordData as SocketPacket>::CRepr>() - 31188 + compressed_items_len
}

fn resolve_packet_header(response_context: &RecordDataResponseContext) -> String {
    response_context.packet_header.clone()
}

fn apply_response_context_to_char(value: &mut Char, response_context: &RecordDataResponseContext) {
    value.sz_name = response_context.char_name.clone();
    value.sz_model_name = response_context.char_model_name.clone();
    value.sz_model_name2 = response_context.char_model_name2.clone().into();
    value.model_name_code2 = response_context.char_model_name_code2;
    if response_context.object_serial != 0 {
        value.dw_object_serial = response_context.object_serial;
    }
    if response_context.login_safe_key != 0 {
        value.dw_login_server_safe_key = response_context.login_safe_key as u32;
    }
    value.accuracy = response_context.accuracy;
    value.exp = response_context.exp;
    value.next_exp = response_context.next_exp;
    value.money = response_context.money;
    value.lp_mon_info_addr = response_context.lp_mon_info_addr;
    value.brood = response_context.brood;
    value.b_update_info = response_context.update_info.clone();
    value.arrow_posi = response_context.arrow_posi;
    value.class_clan = response_context.class_clan;
    value.size_level = response_context.size_level;
    value.dw_char_sound_code = response_context.char_sound_code;
    value.exp_high = response_context.exp_high;
    value.dw_event_time_t = response_context.event_time_t;
    value.s_event_param = response_context.event_param;
    value.s_present_item = response_context.present_item;
    value.gravity_scroll_check = response_context.gravity_scroll_check;
    value.player_killing = response_context.player_killing;
    value.play_class = response_context.play_class;
    value.dw_temp = take_first_9(response_context.char_temp);
    value.dw_login_server_ip = response_context.login_server_ip;
    value.w_version = response_context.char_version;
}

fn apply_response_context_to_game_save(
    value: &mut GameSave,
    response_context: &RecordDataResponseContext,
) {
    value.head = response_context.game_save_head;
    value.check_sum_char_info = response_context.char_checksum;
    value.save_time = response_context.save_time;
    value.pcr_no = response_context.pcr_no;
    value.camera_mode = response_context.camera_mode;
    value.pos_x = response_context.pos_x;
    value.pos_z = response_context.pos_z;
    value.last_money = response_context.last_money;
    value.event_play = response_context.event_play;
    value.pet_info = response_context.pet_info;
    value.potion_update = response_context.potion_update;
    value.potion_count = response_context.potion_count;
    value.force_orb_using = response_context.force_orb_using;
    value.level_quest_log = response_context.level_quest_log;
    value.short_key_normal_attack = response_context.short_key_normal_attack;
    value.bless_castle_tax = response_context.bless_castle_tax;
    value.bless_castle_master = response_context.bless_castle_master;
    value.elementary_quest_log = response_context.elementary_quest_log;
    if response_context.server_id != 0 {
        value.server_id = response_context.server_id;
    }
    value.time_prime_item_x2 = response_context.time_prime_item_x2;
    value.time_prime_item_exp_up = response_context.time_prime_item_exp_up;
    value.time_prime_item_vamp_cuspid = response_context.time_prime_item_vamp_cuspid;
    value.time_prime_item_mana_recharg = response_context.time_prime_item_mana_recharg;
    value.prime_item_package_code = response_context.prime_item_package_code;
    value.time_prime_item_mightof_awell = response_context.time_prime_item_mightof_awell;
    value.time_prime_item_mightof_awell2 = response_context.time_prime_item_mightof_awell2;
    value.time_prime_item_mana_reduce = response_context.time_prime_item_mana_reduce;
    value.time_prime_item_phenix_pet = response_context.time_prime_item_phenix_pet;
    value.time_prime_item_help_pet = response_context.time_prime_item_help_pet;
    value.param_prime_item_help_pet = response_context.param_prime_item_help_pet;
    value.time_prime_item_vamp_cuspid_ex = response_context.time_prime_item_vamp_cuspid_ex;
    value.time_prime_item_stamina_reduce = response_context.time_prime_item_stamina_reduce;
    value.total_sub_exp = response_context.total_sub_exp;
    value.total_money = response_context.total_money;
    value.total_exp = response_context.total_exp;
    value.master_id = response_context.master_id.clone();
    value.quest_info = to_protocol_quest_info(&response_context.quest_info);
    value.last_quest_info = to_protocol_last_quest_info(&response_context.last_quest_info);
    value.life_booster_using = response_context.life_booster_using;
    value.mana_booster_using = response_context.mana_booster_using;
    value.stamina_booster_using = response_context.stamina_booster_using;
    value.skill_delay_using = response_context.skill_delay_using;
    value.time_prime_item_big_head = response_context.time_prime_item_big_head;
    value.param_prime_item_big_head = response_context.param_prime_item_big_head;
    value.time_prime_item_vip = response_context.time_prime_item_vip;
    value.param_prime_item_vip = response_context.param_prime_item_vip;
    value.quest_start_state = response_context.quest_start_state;
    value.quest_start_param1 = response_context.quest_start_param1;
    value.quest_start_param2 = response_context.quest_start_param2;
    value.quest_lv150a_state = response_context.quest_lv150a_state;
    value.quest_lv150a_param = response_context.quest_lv150a_param;
    value.quest_lv150a_time = response_context.quest_lv150a_time;
    value.quest_lv150b_state = response_context.quest_lv150b_state;
    value.quest_lv150b_param = response_context.quest_lv150b_param;
    value.quest_lv150b_time = response_context.quest_lv150b_time;
    value.pv_p_time = response_context.pv_p_time;
    value.pv_p_param = response_context.pv_p_param;
    value.weapon_class = response_context.weapon_class;
    value.armor_class = response_context.armor_class;
    value.shield_class = response_context.shield_class;
    value.bracelets_class = response_context.bracelets_class;
    value.gauntlets_class = response_context.gauntlets_class;
    value.boots_class = response_context.boots_class;
    value.ring_class = response_context.ring_class;
    value.amy_class = response_context.amy_class;
    value.shel_class = response_context.shel_class;
    value.time_class = response_context.time_class;
    value.warehouse_page_time = response_context.warehouse_page_time;
    value.quest_lv155a_state = response_context.quest_lv155a_state;
    value.quest_lv155a_param = response_context.quest_lv155a_param;
    value.quest_lv155a_time = response_context.quest_lv155a_time;
    value.random_monster_param = response_context.random_monster_param;
    value.random_monster_time = response_context.random_monster_time;
    value.temp2 = response_context.temp2;
    value.checksum = response_context.game_save_checksum;
}

fn compute_reform_code(value: &Char, key: u32) -> u32 {
    let mut form_code = 0u32;
    let mut form_count = key;

    reform_state_code(
        &mut form_code,
        &mut form_count,
        &char_model_name2_and_code_bytes(value),
    );
    reform_state_code(&mut form_code, &mut form_count, &value.level.to_le_bytes());
    reform_state_code(
        &mut form_code,
        &mut form_count,
        &value.strength.to_le_bytes(),
    );
    reform_state_code(&mut form_code, &mut form_count, &value.spirit.to_le_bytes());
    reform_state_code(&mut form_code, &mut form_count, &value.talent.to_le_bytes());
    reform_state_code(
        &mut form_code,
        &mut form_count,
        &value.dexterity.to_le_bytes(),
    );
    reform_state_code(&mut form_code, &mut form_count, &value.health.to_le_bytes());
    reform_state_code(&mut form_code, &mut form_count, &value.level.to_le_bytes());
    reform_state_code(
        &mut form_code,
        &mut form_count,
        &value.accuracy.to_le_bytes(),
    );
    reform_state_code(
        &mut form_code,
        &mut form_count,
        &value.attack_rating.to_le_bytes(),
    );
    for attack_damage in value.attack_damage {
        reform_state_code(
            &mut form_code,
            &mut form_count,
            &attack_damage.to_le_bytes(),
        );
    }
    reform_state_code(
        &mut form_code,
        &mut form_count,
        &value.attack_speed.to_le_bytes(),
    );
    reform_state_code(
        &mut form_code,
        &mut form_count,
        &value.shooting_range.to_le_bytes(),
    );
    reform_state_code(
        &mut form_code,
        &mut form_count,
        &value.critical_hit.to_le_bytes(),
    );
    reform_state_code(
        &mut form_code,
        &mut form_count,
        &value.defence.to_le_bytes(),
    );
    reform_state_code(
        &mut form_code,
        &mut form_count,
        &value.chance_block.to_le_bytes(),
    );
    reform_state_code(
        &mut form_code,
        &mut form_count,
        &value.absorption.to_le_bytes(),
    );
    reform_state_code(
        &mut form_code,
        &mut form_count,
        &value.move_speed.to_le_bytes(),
    );
    reform_state_code(&mut form_code, &mut form_count, &value.sight.to_le_bytes());
    for weight in value.weight {
        reform_state_code(&mut form_code, &mut form_count, &weight.to_le_bytes());
    }
    for resistance in value.resistance {
        reform_state_code(&mut form_code, &mut form_count, &resistance.to_le_bytes());
    }
    for attack_resistance in value.attack_resistance {
        reform_state_code(
            &mut form_code,
            &mut form_count,
            &attack_resistance.to_le_bytes(),
        );
    }
    reform_state_code(
        &mut form_code,
        &mut form_count,
        &value.life[0].to_le_bytes(),
    );
    for mana in value.mana {
        reform_state_code(&mut form_code, &mut form_count, &mana.to_le_bytes());
    }
    for stamina in value.stamina {
        reform_state_code(&mut form_code, &mut form_count, &stamina.to_le_bytes());
    }
    reform_state_code(&mut form_code, &mut form_count, &value.exp.to_le_bytes());
    reform_state_code(
        &mut form_code,
        &mut form_count,
        &value.next_exp.to_le_bytes(),
    );
    reform_state_code(&mut form_code, &mut form_count, &value.money.to_le_bytes());
    reform_state_code(
        &mut form_code,
        &mut form_count,
        &value.state_point.to_le_bytes(),
    );
    reform_state_code(
        &mut form_code,
        &mut form_count,
        &value.exp_high.to_le_bytes(),
    );

    form_code
}

fn reform_state_code(form_code: &mut u32, form_count: &mut u32, bytes: &[u8]) {
    for &byte in bytes {
        let weight = (*form_count & 0x1FFF) + 1;
        let signed_byte = i32::from(i8::from_ne_bytes([byte])) as u32;
        *form_code = form_code.wrapping_add(signed_byte.wrapping_mul(weight));
        *form_count = form_count.wrapping_add(1);
    }
}

fn char_model_name2_and_code_bytes(value: &Char) -> [u8; 64] {
    let mut out = [0u8; 64];
    out[..60].copy_from_slice(value.sz_model_name2.raw());
    out[60..64].copy_from_slice(&value.model_name_code2.to_le_bytes());
    out
}

fn encode_record_items(
    record_items: &[UserRecordItem],
    wire_snapshot: &RecordDataWireSnapshot,
) -> Result<Vec<u8>, AppError> {
    let mut out = Vec::new();

    for (index, record_item) in record_items.iter().enumerate() {
        let wire_item = wire_snapshot.items.get(index);
        let bytes = to_protocol_record_item_bytes(record_item, wire_item);
        let compressed = encode_compress(&bytes, bytes.len());
        out.extend_from_slice(&compressed);
    }

    Ok(out)
}

fn fragment_record_data(record_bytes: Vec<u8>) -> Vec<RequestVecBody> {
    let size = record_bytes.len();
    let mut part_total = size / TRANS_RECORD_LEN;
    if (size % TRANS_RECORD_LEN) != 0 && size > TRANS_RECORD_LEN {
        part_total += 1;
    }

    let mut packets = Vec::new();
    let mut count = 0usize;
    let mut total_len = 0usize;

    loop {
        let remaining = size.saturating_sub(total_len);
        let len = remaining.min(TRANS_RECORD_LEN);

        let mut body = Vec::with_capacity(12 + len);
        body.extend_from_slice(&(count as i32).to_le_bytes());
        body.extend_from_slice(&(part_total as i32).to_le_bytes());
        body.extend_from_slice(&(len as i32).to_le_bytes());
        body.extend_from_slice(&record_bytes[total_len..total_len + len]);

        packets.push(RequestVecBody {
            head: ReqHead {
                size: REQUEST_HEAD_SIZE + body.len(),
                code: Getrecorddata - 1,
            },
            body: body.into(),
        });

        count += 1;
        total_len += len;
        if total_len >= size {
            break;
        }
    }

    packets
}

fn validate_fragmented_record_data(packets: &[RequestVecBody]) -> Result<(), AppError> {
    if packets.is_empty() {
        return Err(crate::parse_error!(
            ParseErrorKind::RecordItem,
            "record data adapter produced no packets"
        ));
    }

    let expected_total = if packets.len() <= 1 {
        0
    } else {
        packets.len() as i32
    };

    let mut total_payload_size = 0usize;
    for (index, packet) in packets.iter().enumerate() {
        if packet.head.code != Getrecorddata - 1 {
            return Err(crate::parse_error!(
                ParseErrorKind::RecordItem,
                "record data adapter produced invalid opcode: {}",
                packet.head.code
            ));
        }

        if packet.body.len() < 12 {
            return Err(crate::parse_error!(
                ParseErrorKind::RecordItem,
                "record data adapter produced invalid body length: {}",
                packet.body.len()
            ));
        }

        let body = packet.body.as_ref();
        let count = i32::from_le_bytes(body[0..4].try_into().unwrap_or_default());
        let total = i32::from_le_bytes(body[4..8].try_into().unwrap_or_default());
        let trans_size = i32::from_le_bytes(body[8..12].try_into().unwrap_or_default());
        let payload_len = body.len() - 12;

        if count != index as i32 {
            return Err(crate::parse_error!(
                ParseErrorKind::RecordItem,
                "record data adapter produced invalid fragment count: expected {} got {}",
                index,
                count
            ));
        }

        if total != expected_total {
            return Err(crate::parse_error!(
                ParseErrorKind::RecordItem,
                "record data adapter produced invalid fragment total: expected {} got {}",
                expected_total,
                total
            ));
        }

        if trans_size < 0 || trans_size as usize != payload_len {
            return Err(crate::parse_error!(
                ParseErrorKind::RecordItem,
                "record data adapter produced invalid trans_size: expected {} got {}",
                payload_len,
                trans_size
            ));
        }

        if packet.head.size != REQUEST_HEAD_SIZE + packet.body.len() {
            return Err(crate::parse_error!(
                ParseErrorKind::RecordItem,
                "record data adapter produced invalid packet size: expected {} got {}",
                REQUEST_HEAD_SIZE + packet.body.len(),
                packet.head.size
            ));
        }

        total_payload_size += payload_len;
    }

    if total_payload_size == 0 {
        return Err(crate::parse_error!(
            ParseErrorKind::RecordItem,
            "record data adapter produced empty payload"
        ));
    }

    Ok(())
}

fn to_protocol_char(
    value: &UserChar,
    wire_snapshot: &RecordDataWireSnapshot,
    object_serial: u32,
) -> Char {
    Char {
        sz_name: String::new(),
        sz_model_name: String::new(),
        sz_model_name2: String::new().into(),
        model_name_code2: 0,
        dw_object_serial: object_serial,
        class_clan: 0,
        state: value.state.raw(),
        size_level: 0,
        dw_char_sound_code: 0,
        job_code: value.job_code.into(),
        level: value.level,
        strength: value.strength,
        spirit: value.spirit,
        talent: value.talent,
        dexterity: value.dexterity,
        health: value.health,
        accuracy: 0,
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
        unused_life: wire_snapshot.char_info.unused_life,
        mana: value.mana,
        stamina: value.stamina,
        life_regen: value.life_regen,
        mana_regen: value.mana_regen,
        stamina_regen: value.stamina_regen,
        exp: 0,
        next_exp: 0,
        money: 0,
        lp_mon_info_addr: 0,
        brood: 0,
        state_point: value.state_point,
        b_update_info: String::new(),
        arrow_posi: [0, 0],
        potion_space: value.potion_space,
        life_function: value.life_function,
        mana_function: value.mana_function,
        stamina_function: value.stamina_function,
        damage_function: value.damage_function,
        reform_code: 0,
        change_job: encode_change_job(value.change_job),
        job_bit_mask: value.job_bit_mask.into(),
        player_killing: [0, 0],
        play_class: [0, 0],
        exp_high: 0,
        dw_event_time_t: 0,
        s_event_param: [0, 0],
        s_present_item: [0, 0],
        gravity_scroll_check: [0, 0],
        life: [i32::from(value.life[0]), i32::from(value.life[1])],
        dw_temp: [0; 9],
        dw_login_server_ip: 0,
        dw_login_server_safe_key: 0,
        w_version: [0, 0],
    }
}

fn encode_change_job(value: ChangeJobTierState) -> u32 {
    value.as_i32().saturating_sub(1) as u32
}

fn take_first_9(value: [u32; 11]) -> [u32; 9] {
    let mut out = [0u32; 9];
    out.copy_from_slice(&value[..9]);
    out
}

fn to_protocol_game_save(_char_info: &UserChar, value: &UserGameSave) -> GameSave {
    GameSave {
        head: 0,
        play_stage_num: value.play_stage_num,
        camera_mode: 0,
        pos_x: 0,
        pos_z: 0,
        last_money: 0,
        check_sum_char_info: 0,
        skill_info: to_protocol_skill_info(&value.skill_info),
        save_time: 0,
        pcr_no: 0,
        event_play: [0, 0],
        pet_info: [0, 0],
        potion_update: [0, 0],
        potion_count: [[0; 3]; 4],
        force_orb_using: [0, 0],
        level_quest_log: 0,
        short_key_normal_attack: 0,
        bless_castle_tax: 0,
        bless_castle_master: 0,
        elementary_quest_log: 0,
        time_prime_item_x2: 0,
        time_prime_item_exp_up: 0,
        time_prime_item_vamp_cuspid: 0,
        time_prime_item_mana_recharg: 0,
        prime_item_package_code: 0,
        time_prime_item_mightof_awell: 0,
        time_prime_item_mightof_awell2: 0,
        time_prime_item_mana_reduce: 0,
        time_prime_item_phenix_pet: 0,
        time_prime_item_help_pet: 0,
        param_prime_item_help_pet: 0,
        time_prime_item_vamp_cuspid_ex: 0,
        time_prime_item_stamina_reduce: 0,
        total_sub_exp: 0,
        total_money: 0,
        total_exp: 0,
        master_id: String::new(),
        quest_info: QuestInfo {
            quest_code: [0, 0],
            data: [0; 7],
        },
        last_quest_info: LastQuestInfo {
            last_quest: [0; 32],
            last_quest_count: 0,
        },
        server_id: 0,
        life_booster_using: [0, 0],
        mana_booster_using: [0, 0],
        stamina_booster_using: [0, 0],
        skill_delay_using: [0, 0],
        time_prime_item_big_head: 0,
        param_prime_item_big_head: 0,
        time_prime_item_vip: 0,
        param_prime_item_vip: 0,
        quest_start_state: 0,
        quest_start_param1: 0,
        quest_start_param2: 0,
        quest_lv150a_state: 0,
        quest_lv150a_param: 0,
        quest_lv150a_time: 0,
        quest_lv150b_state: 0,
        quest_lv150b_param: 0,
        quest_lv150b_time: 0,
        pv_p_time: 0,
        pv_p_param: 0,
        checksum: 0,
        weapon_class: 0,
        armor_class: 0,
        shield_class: 0,
        bracelets_class: 0,
        gauntlets_class: 0,
        boots_class: 0,
        ring_class: 0,
        amy_class: 0,
        shel_class: 0,
        time_class: 0,
        warehouse_page_time: 0,
        quest_lv155a_state: 0,
        quest_lv155a_param: 0,
        quest_lv155a_time: 0,
        random_monster_param: 0,
        random_monster_time: 0,
        temp2: [0, 0],
    }
}

fn to_protocol_skill_info(value: &UserSkillInfo) -> SkillInfo {
    SkillInfo {
        b_skill_point: value.b_skill_point,
        w_skill_mastery: value.w_skill_mastery,
        b_short_key: value.b_short_key.clone(),
        mouse_pos: value.mouse_pos.clone(),
        w_select_skill: value.w_select_skill,
    }
}

fn to_protocol_quest_info(value: &UserQuestInfo) -> QuestInfo {
    QuestInfo {
        quest_code: value.quest_code,
        data: value.data,
    }
}

fn to_protocol_last_quest_info(value: &UserLastQuestInfo) -> LastQuestInfo {
    LastQuestInfo {
        last_quest: value.last_quest,
        last_quest_count: value.last_quest_count,
    }
}

fn to_protocol_throw_item(value: UserThrowItem) -> ThrowItem {
    ThrowItem {
        code: value.code,
        key: value.key,
        sum: value.sum,
    }
}

fn to_protocol_record_item(value: &UserRecordItem) -> RecordItem {
    RecordItem {
        count: value.count,
        x: value.x,
        y: value.y,
        position: value.position,
        item: to_protocol_item(&value.item),
    }
}

fn to_protocol_record_item_bytes(
    value: &UserRecordItem,
    wire_item: Option<&crate::application::use_case::runtime::world_runtime::world_command::RecordDataWireItemSnapshot>,
) -> Vec<u8> {
    let mut bytes = to_protocol_record_item(value).to_bytes();
    const RECORD_ITEM_ITEM_NAME_OFFSET: usize = 44;
    const RECORD_ITEM_ITEM_NAME_LEN: usize = 32;

    if bytes.len() >= RECORD_ITEM_ITEM_NAME_OFFSET + RECORD_ITEM_ITEM_NAME_LEN {
        let item_name_bytes = wire_item
            .map(|item| item.item_name_bytes)
            .unwrap_or_else(|| socket::string_to_fixed::<32>(&value.item.item_name));
        bytes[RECORD_ITEM_ITEM_NAME_OFFSET
            ..RECORD_ITEM_ITEM_NAME_OFFSET + RECORD_ITEM_ITEM_NAME_LEN]
            .copy_from_slice(&item_name_bytes);
    }

    bytes
}

fn to_protocol_item(value: &UserItem) -> Item {
    Item {
        size: value.size,
        item_header: to_protocol_item_header(&value.item_header),
        durability: value.durability,
        code: value.code,
        item_name: value.item_name.clone().into(),
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
        job_item: to_protocol_job_item(&value.job_item),
        item_kind_code: value.item_kind_code,
        item_kind_mask: value.item_kind_mask,
        item_aging_num: value.item_aging_num,
        item_aging_count: value.item_aging_count,
        item_aging_protect: value.item_aging_protect,
        special_item_flag: value.special_item_flag,
        swap_item: to_protocol_swap_item(&value.swap_item),
        dw_create_time: value.dw_create_time,
        linked_item: value.linked_item,
        lock_item: value.lock_item,
        coin: value.coin,
        dw_temp: value.dw_temp,
    }
}

fn to_protocol_item_header(value: &UserItemHeader) -> Header {
    Header {
        head: value.head,
        version: value.version,
        time: value.time,
        checksum: value.checksum,
    }
}

fn to_protocol_job_item(value: &UserJobItem) -> JobItem {
    JobItem {
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

fn to_protocol_swap_item(value: &UserSwapItem) -> SwapItem {
    SwapItem {
        flag: value.flag,
        code: value.code,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        apply_response_context_to_char, encode_record_items, to_protocol_char,
        to_protocol_record_item,
    };
    use crate::application::use_case::runtime::world_runtime::world_command::RecordDataResponseContext;
    use crate::application::use_case::runtime::world_runtime::world_command::{
        RecordDataWireCharSnapshot, RecordDataWireItemSnapshot, RecordDataWireSnapshot,
    };
    use crate::domain::item::dto::PlayerJobMask;
    use crate::domain::user_player::char_motion_state::CharMotionState;
    use crate::domain::user_player::dto::{
        ChangeJobTierState, PlayerJob, UserChar, UserGameSave, UserItem, UserItemHeader,
        UserJobItem, UserRecordItem, UserSkillInfo, UserSwapItem,
    };
    use crate::libs::decompress_record_items::get_record_items;

    fn sample_record_item() -> UserRecordItem {
        UserRecordItem {
            count: 1,
            x: 2,
            y: 3,
            position: 4,
            item: UserItem {
                checksum: 123,
                size: 1,
                item_header: UserItemHeader {
                    head: 10,
                    version: 20,
                    time: 30,
                    checksum: 123,
                },
                durability: [1, 2],
                code: 99,
                item_name: "test_item".to_string(),
                weight: 5,
                price: 6,
                index: 7,
                potion_count: 8,
                resistance: [0; 8],
                sight: 9,
                temp0: 10,
                damage: [11, 12],
                shooting_range: 13,
                attack_speed: 14,
                attack_rating: 15,
                critical_hit: 16,
                f_absorb: 1.0,
                defense: 17,
                f_block_rating: 2.0,
                f_speed: 3.0,
                potion_space: 18,
                f_magic_mastery: 4.0,
                f_mana_regen: 5.0,
                f_life_regen: 6.0,
                f_stamina_regen: 7.0,
                f_increase_life: 8.0,
                f_increase_mana: 9.0,
                f_increase_stamina: 10.0,
                level: 19,
                strength: 20,
                spirit: 21,
                talent: 22,
                dexterity: 23,
                health: 24,
                mana: [25, 26],
                life: [27, 28],
                stamina: [29, 30],
                money: 31,
                not_use_flag: 32,
                back_up_key: 33,
                back_up_chk_sum: 34,
                scale_blink: [35, 36],
                unique_item: 37,
                effect_blink: [38, 39],
                effect_color: [40, 41, 42, 43],
                disp_effect: 44,
                job_code_mask: PlayerJobMask::PRIESTESS,
                job_item: UserJobItem::default(),
                item_kind_code: 45,
                item_kind_mask: 46,
                item_aging_num: [47, 48],
                item_aging_count: [49, 50],
                item_aging_protect: [51, 52],
                special_item_flag: [53, 54],
                swap_item: UserSwapItem { flag: 55, code: 56 },
                dw_create_time: 57,
                linked_item: 58,
                lock_item: 59,
                coin: 60,
                dw_temp: [61, 62, 63, 64, 65, 66],
            },
        }
    }

    #[test]
    fn encoded_record_items_roundtrip_with_decoder() {
        let record_item = sample_record_item();
        let wire_snapshot = RecordDataWireSnapshot {
            char_info: RecordDataWireCharSnapshot::default(),
            items: vec![RecordDataWireItemSnapshot {
                item_name_bytes: socket::string_to_fixed::<32>("test_item"),
            }],
        };
        let encoded = encode_record_items(std::slice::from_ref(&record_item), &wire_snapshot)
            .expect("encode");
        let decoded = get_record_items(&encoded, 1).expect("decode");

        assert_eq!(decoded.len(), 1);
        assert_eq!(decoded[0].count, record_item.count);
        assert_eq!(decoded[0].x, record_item.x);
        assert_eq!(decoded[0].y, record_item.y);
        assert_eq!(decoded[0].position, record_item.position);
        assert_eq!(
            decoded[0].to_bytes(),
            to_protocol_record_item(&record_item).to_bytes()
        );
    }

    #[test]
    fn rebuilt_char_preserves_bootstrapped_runtime_job_code() {
        let char_info = UserChar {
            state: CharMotionState::Stand,
            job_code: PlayerJob::Priestess,
            level: 1,
            strength: 2,
            spirit: 3,
            talent: 4,
            dexterity: 5,
            health: 6,
            attack_rating: 7,
            attack_damage: [8, 9],
            attack_speed: 10,
            shooting_range: 11,
            critical_hit: 12,
            defence: 13,
            chance_block: 14,
            absorption: 15,
            move_speed: 16,
            sight: 17,
            weight: [18, 19],
            resistance: [0; 8],
            attack_resistance: [0; 8],
            life: [20, 21],
            mana: [22, 23],
            stamina: [24, 25],
            life_regen: 0.0,
            mana_regen: 0.0,
            stamina_regen: 0.0,
            state_point: 26,
            potion_space: 27,
            life_function: 28,
            mana_function: 29,
            stamina_function: 30,
            damage_function: [31, 32],
            change_job: ChangeJobTierState::Tier4,
            job_bit_mask: PlayerJobMask::PRIESTESS,
        };
        let mut protocol_char =
            to_protocol_char(&char_info, &RecordDataWireSnapshot::default(), 77);
        let response_context = RecordDataResponseContext::default();

        apply_response_context_to_char(&mut protocol_char, &response_context);

        assert_eq!(protocol_char.job_code.raw(), 8);
    }
}
