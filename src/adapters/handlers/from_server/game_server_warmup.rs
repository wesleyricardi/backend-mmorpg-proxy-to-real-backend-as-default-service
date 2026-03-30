use std::time::Instant;

use socket::{Data, Request, RequestVecBody, Responder};
use tokio::time::{sleep, Duration};

use crate::adapters::fileread::save_game_server_warmup_json;
use crate::application::use_case::runtime::world_runtime::game_server::dto::{
    FieldMonsterSpawnPoint, FieldMonsterSpawnRule, FieldNpcSpawn, GameField, GameFieldWarpGate,
    GameMonster, GameNpc, GameOpenItem, GameServerData, MonsterDrop, MonsterId, NpcId, NpcServices,
    WorldPoint, WorldPoint2D,
};
use crate::application::use_case::runtime::world_runtime::world_command::WorldCommand;
use crate::domain::field::dto::FieldId;
use crate::error::AppError;
use crate::protocol::command::Command;
use crate::protocol::game_server::{
    GameServerFieldChunk, GameServerMonsterChunk, GameServerNpcChunk, GameServerOpenItemChunk,
    GameServerStageAreaBossRuleChunk, GameServerStageAreaChunk,
    GameServerStageAreaMonsterRuleChunk, GameServerStageAreaNpcChunk,
    GameServerStageAreaStartPointChunk, GameServerWarmupDone, GameServerWarmupMeta,
};
use crate::state::ServerState;

fn world_sender() -> Option<&'static std::sync::mpsc::Sender<WorldCommand>> {
    crate::adapters::world::world_sender()
}

fn non_empty_text(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn build_filtered_game_server_data(
    snapshot: &crate::state::GameServerWarmupSnapshot,
) -> GameServerData {
    let monsters = snapshot
        .monsters
        .iter()
        .enumerate()
        .map(|(index, monster)| GameMonster {
            index,
            id: MonsterId::from_legacy_auto_char_code(monster.monster_info.dw_auto_char_code),
            auto_char_code: monster.monster_info.dw_auto_char_code,
            name: monster.char_info.sz_name.clone(),
            model_file: monster.char_info.sz_model_name.clone(),
            level: monster.char_info.level,
            life: monster.char_info.life[0],
            attack_rating: monster.char_info.attack_rating,
            attack_damage_min: monster.char_info.attack_damage[0],
            attack_damage_max: monster.char_info.attack_damage[1],
            defense: monster.char_info.defence,
            absorption: monster.char_info.absorption,
            move_speed: monster.char_info.move_speed,
            sight: monster.char_info.sight,
            move_range: monster.monster_info.move_range,
            exp: monster.monster_info.get_exp,
            nature: monster.monster_info.nature,
            undead: monster.monster_info.undead,
            class_code: monster.monster_info.class_code,
            is_boss: monster.monster_info.boss != 0,
            drops: monster
                .monster_info
                .fall_items
                .iter()
                .take(monster.monster_info.fall_item_count.max(0) as usize)
                .filter(|drop| drop.dw_item_code != 0 && drop.percentage > 0)
                .map(|drop| MonsterDrop {
                    item_code: drop.dw_item_code,
                    percentage: drop.percentage,
                })
                .collect(),
        })
        .collect::<Vec<_>>();

    let npcs = snapshot
        .npcs
        .iter()
        .enumerate()
        .map(|(index, npc)| GameNpc {
            index,
            id: NpcId::from_file_name(npc.file_name.clone()),
            file_name: npc.file_name.clone(),
            name: npc.char_info.sz_name.clone(),
            model_file: npc.char_info.sz_model_name.clone(),
            dialog_title: non_empty_text(&npc.monster_info.sz_name),
            quest_code: npc.monster_info.quest_code,
            quest_param: npc.monster_info.quest_param,
            services: NpcServices {
                teleport: npc.monster_info.tele_port_npc != 0,
                warehouse: npc.monster_info.ware_house_master != 0,
                item_mix: npc.monster_info.item_mix != 0,
                aging: npc.monster_info.item_aging != 0,
                collect_money: npc.monster_info.collect_money != 0,
                event_gift: npc.monster_info.event_gift != 0,
                smelting: npc.monster_info.smelting != 0,
                manufacture: npc.monster_info.manufacture != 0,
                mixture_reset: npc.monster_info.mixture_reset != 0,
                remodel: npc.monster_info.remodel != 0,
                coin_shop: npc.monster_info.coin_shop != 0,
                time_shop: npc.monster_info.time_shop != 0,
                clan_service: npc.monster_info.clan_npc != 0,
                gift_express: npc.monster_info.gift_express != 0,
                wing_quest: npc.monster_info.wing_quest_npc != 0,
                star_point: npc.monster_info.star_point_npc != 0,
                give_money: npc.monster_info.give_money_npc != 0,
                bless_castle: npc.monster_info.bless_castle_npc != 0,
                polling: npc.monster_info.polling_npc != 0,
            },
        })
        .collect::<Vec<_>>();

    let monster_ids_by_name = monsters.iter().fold(
        std::collections::HashMap::<String, Vec<MonsterId>>::new(),
        |mut acc, monster| {
            acc.entry(monster.name.to_ascii_lowercase())
                .or_default()
                .push(monster.id);
            acc
        },
    );
    let npc_ids_by_exact = npcs.iter().fold(
        std::collections::HashMap::<(String, String), Vec<NpcId>>::new(),
        |mut acc, npc| {
            acc.entry((
                npc.name.to_ascii_lowercase(),
                npc.model_file.to_ascii_lowercase(),
            ))
            .or_default()
            .push(npc.id.clone());
            acc
        },
    );
    let npc_ids_by_name = npcs.iter().fold(
        std::collections::HashMap::<String, Vec<NpcId>>::new(),
        |mut acc, npc| {
            acc.entry(npc.name.to_ascii_lowercase())
                .or_default()
                .push(npc.id.clone());
            acc
        },
    );

    let mut field_npc_spawns: Vec<Vec<FieldNpcSpawn>> = vec![Vec::new(); snapshot.fields.len()];
    let mut field_monster_spawn_points: Vec<Vec<FieldMonsterSpawnPoint>> =
        vec![Vec::new(); snapshot.fields.len()];
    let mut field_monster_spawn_rules: Vec<Vec<FieldMonsterSpawnRule>> =
        vec![Vec::new(); snapshot.fields.len()];
    let mut field_boss_spawn_rules: Vec<Vec<crate::application::use_case::runtime::world_runtime::game_server::dto::FieldBossSpawnRule>> =
        vec![Vec::new(); snapshot.fields.len()];

    for (field_index, stage_area) in snapshot.stage_areas.iter().enumerate() {
        if field_index >= snapshot.fields.len() {
            break;
        }

        field_npc_spawns[field_index] = stage_area
            .trans_char_fixed
            .iter()
            .filter(|npc| npc.x != 0 || npc.y != 0 || npc.z != 0 || !npc.sm_char_info.sz_name.is_empty())
            .map(|npc| {
                let name_key = npc.sm_char_info.sz_name.to_ascii_lowercase();
                let model_key = npc.sm_char_info.sz_model_name.to_ascii_lowercase();
                let resolved_npc_id = match npc_ids_by_exact.get(&(name_key.clone(), model_key)) {
                    Some(ids) if ids.len() == 1 => Some(ids[0].clone()),
                    Some(ids) => {
                        log::warn!(
                            "ambiguous npc spawn resolution by name+model for field_index={} name='{}' matches={}",
                            field_index,
                            npc.sm_char_info.sz_name,
                            ids.len()
                        );
                        None
                    }
                    None => match npc_ids_by_name.get(&name_key) {
                        Some(ids) if ids.len() == 1 => Some(ids[0].clone()),
                        Some(ids) => {
                            log::warn!(
                                "ambiguous npc spawn resolution by name fallback for field_index={} name='{}' matches={}",
                                field_index,
                                npc.sm_char_info.sz_name,
                                ids.len()
                            );
                            None
                        }
                        None => {
                            log::warn!(
                                "failed to resolve npc spawn for field_index={} name='{}' model='{}'",
                                field_index,
                                npc.sm_char_info.sz_name,
                                npc.sm_char_info.sz_model_name
                            );
                            None
                        }
                    },
                };

                FieldNpcSpawn {
                    npc_id: resolved_npc_id,
                    name: npc.sm_char_info.sz_name.clone(),
                    position: WorldPoint {
                        x: npc.x,
                        y: npc.y,
                        z: npc.z,
                    },
                    direction: WorldPoint {
                        x: npc.ax,
                        y: npc.ay,
                        z: npc.az,
                    },
                    dialog_title: non_empty_text(&npc.sm_char_info.sz_name),
                }
            })
            .collect();

        field_monster_spawn_points[field_index] = stage_area
            .start_point
            .iter()
            .enumerate()
            .take(stage_area.start_point_cnt.max(0) as usize)
            .map(|(spawn_index, point)| FieldMonsterSpawnPoint {
                index: spawn_index,
                position: WorldPoint2D {
                    x: point.x,
                    z: point.z,
                },
                state: point.state,
                near_play_count: stage_area.start_point_near_play[spawn_index],
                monster_count: stage_area.start_point_mon_count[spawn_index],
            })
            .collect();

        field_monster_spawn_rules[field_index] = stage_area
            .rs_monster_list
            .rs_monster
            .iter()
            .filter(|rule| !rule.sz_monster_name.trim().is_empty())
            .map(|rule| {
                let resolved_monster_id = match monster_ids_by_name
                    .get(&rule.sz_monster_name.to_ascii_lowercase())
                {
                    Some(ids) if ids.len() == 1 => Some(ids[0]),
                    Some(ids) => {
                        log::warn!(
                            "ambiguous monster spawn rule resolution for field_index={} name='{}' matches={}",
                            field_index,
                            rule.sz_monster_name,
                            ids.len()
                        );
                        None
                    }
                    None => {
                        log::warn!(
                            "failed to resolve monster spawn rule for field_index={} name='{}'",
                            field_index,
                            rule.sz_monster_name
                        );
                        None
                    }
                };

                FieldMonsterSpawnRule {
                    monster_id: resolved_monster_id,
                    monster_name: rule.sz_monster_name.clone(),
                    open_percentage: rule.open_percentage,
                    spawn_start_index: rule.num_open_start,
                }
            })
            .collect();

        field_boss_spawn_rules[field_index] = stage_area
            .rs_monster_list
            .s_boss_monsters
            .iter()
            .take(stage_area.rs_monster_list.boss_monster_count.max(0) as usize)
            .filter(|boss| {
                !boss.master_monster_name.trim().is_empty()
                    || boss.master_monster_auto_code != 0
                    || !boss.slave_monster_name.trim().is_empty()
                    || boss.slave_monster_auto_code != 0
            })
            .map(|boss| crate::application::use_case::runtime::world_runtime::game_server::dto::FieldBossSpawnRule {
                master_monster_id: if boss.master_monster_auto_code != 0 {
                    Some(MonsterId::from_legacy_auto_char_code(
                        boss.master_monster_auto_code,
                    ))
                } else {
                    None
                },
                master_monster_name: boss.master_monster_name.clone(),
                slave_monster_id: if boss.slave_monster_auto_code != 0 {
                    Some(MonsterId::from_legacy_auto_char_code(
                        boss.slave_monster_auto_code,
                    ))
                } else {
                    None
                },
                slave_monster_name: boss.slave_monster_name.clone(),
                slave_count: boss.slave_count,
                open_hours: boss
                    .b_open_time
                    .iter()
                    .take(boss.open_time_count.max(0) as usize)
                    .copied()
                    .collect(),
            })
            .collect();
    }

    let fields = snapshot
        .fields
        .iter()
        .enumerate()
        .map(|(index, field)| {
            let start_points = field
                .start_point
                .iter()
                .take(field.start_point_cnt.max(0) as usize)
                .filter(|point| point.x != 0 || point.y != 0)
                .map(|point| WorldPoint2D {
                    x: point.x,
                    z: point.y,
                })
                .collect();

            let warp_gates = field
                .warp_gate
                .iter()
                .take(field.warp_gate_count.max(0) as usize)
                .filter(|gate| gate.size > 0 || gate.height > 0 || gate.out_gate_count > 0)
                .map(|gate| GameFieldWarpGate {
                    position: WorldPoint {
                        x: gate.x,
                        y: gate.y,
                        z: gate.z,
                    },
                    height: gate.height,
                    size: gate.size,
                    limit_level: gate.limit_level,
                    special_effect: gate.special_effect,
                    destinations: gate
                        .out_gate
                        .iter()
                        .take(gate.out_gate_count.max(0) as usize)
                        .map(|out| WorldPoint {
                            x: out.x,
                            y: out.y,
                            z: out.z,
                        })
                        .collect(),
                })
                .collect();

            GameField {
                index,
                field_id: FieldId::from_raw(field.field_code),
                stage_file: field.sz_name.clone(),
                map_image_file: field.sz_name_map.clone(),
                title_image_file: field.sz_name_title.clone(),
                state: field.state,
                event_code: field.field_event,
                back_music_code: field.back_music_code,
                limit_level: field.limit_level,
                field_code: field.field_code,
                server_code: field.server_code,
                center: WorldPoint2D {
                    x: field.c_x,
                    z: field.c_z,
                },
                start_points,
                warp_gates,
                npc_spawns: field_npc_spawns.get(index).cloned().unwrap_or_default(),
                monster_spawn_points: field_monster_spawn_points
                    .get(index)
                    .cloned()
                    .unwrap_or_default(),
                monster_spawn_rules: field_monster_spawn_rules
                    .get(index)
                    .cloned()
                    .unwrap_or_default(),
                boss_spawn_rules: field_boss_spawn_rules
                    .get(index)
                    .cloned()
                    .unwrap_or_default(),
            }
        })
        .collect();

    let open_items = snapshot
        .open_items
        .iter()
        .enumerate()
        .map(|(index, item)| GameOpenItem {
            index,
            code: item.item.code,
            name: item.item.item_name.to_string(),
            level_requirement: item.item.level,
            price: item.item.price,
            weight: item.item.weight,
            attack_speed: item.item.attack_speed,
            attack_rating: item.s_attack_rating[0] as i32,
            critical_hit: item.item.critical_hit,
            defense: item.s_defence[0] as i32,
            block_rating: item.f_block_rating[0],
            absorb: item.f_absorb[0],
            min_damage: item.s_damage[0],
            max_damage: item.s_damage[2],
            durability_min: item.s_durability[0],
            durability_max: item.s_durability[1],
            job_code_mask: item.item.job_code_mask,
            unique_item: item.item.unique_item,
            disp_effect: item.item.disp_effect,
        })
        .collect();

    GameServerData {
        fields,
        monsters,
        npcs,
        open_items,
    }
}

fn warmup_snapshot_is_complete(snapshot: &crate::state::GameServerWarmupSnapshot) -> bool {
    let Some(meta) = snapshot.meta.as_ref() else {
        return false;
    };

    snapshot.fields.len() >= meta.field_count as usize
        && snapshot.monsters.len() >= meta.monster_count as usize
        && snapshot.npcs.len() >= meta.npc_count as usize
        && snapshot.open_items.len() >= meta.open_item_count as usize
        && snapshot.stage_areas.len() >= meta.stage_area_count as usize
}

pub async fn warmup_meta(
    request: Request<GameServerWarmupMeta>,
    _responder: Responder,
    data: Data<ServerState>,
) -> Result<(), AppError> {
    let _ordered = data.game_server_warmup_order.lock().await;
    log::info!(
        "warmup meta received: fields={} monsters={} npcs={} open_items={} stage_areas={}",
        request.body.field_count,
        request.body.monster_count,
        request.body.npc_count,
        request.body.open_item_count,
        request.body.stage_area_count
    );
    data.game_server_warmup
        .lock()
        .await
        .begin(request.body.clone());
    Ok(())
}

pub async fn warmup_field(
    request: Request<GameServerFieldChunk>,
    _responder: Responder,
    data: Data<ServerState>,
) -> Result<(), AppError> {
    let _ordered = data.game_server_warmup_order.lock().await;
    data.game_server_warmup
        .lock()
        .await
        .push_field(request.body.data.clone());
    Ok(())
}

pub async fn warmup_monster(
    request: Request<GameServerMonsterChunk>,
    _responder: Responder,
    data: Data<ServerState>,
) -> Result<(), AppError> {
    let _ordered = data.game_server_warmup_order.lock().await;
    data.game_server_warmup
        .lock()
        .await
        .push_monster(request.body.data.clone());
    Ok(())
}

pub async fn warmup_npc(
    request: Request<GameServerNpcChunk>,
    _responder: Responder,
    data: Data<ServerState>,
) -> Result<(), AppError> {
    let _ordered = data.game_server_warmup_order.lock().await;
    data.game_server_warmup
        .lock()
        .await
        .push_npc(request.body.data.clone());
    Ok(())
}

pub async fn warmup_open_item(
    request: Request<GameServerOpenItemChunk>,
    _responder: Responder,
    data: Data<ServerState>,
) -> Result<(), AppError> {
    let _ordered = data.game_server_warmup_order.lock().await;
    data.game_server_warmup
        .lock()
        .await
        .push_open_item(request.body.data.clone());
    Ok(())
}

pub async fn warmup_stage_area(
    request: Request<GameServerStageAreaChunk>,
    _responder: Responder,
    data: Data<ServerState>,
) -> Result<(), AppError> {
    let _ordered = data.game_server_warmup_order.lock().await;
    data.game_server_warmup
        .lock()
        .await
        .apply_stage_area_summary(
            request.body.index.max(0) as usize,
            request.body.data.clone(),
        );
    Ok(())
}

pub async fn warmup_stage_area_npcs(
    request: Request<GameServerStageAreaNpcChunk>,
    _responder: Responder,
    data: Data<ServerState>,
) -> Result<(), AppError> {
    let _ordered = data.game_server_warmup_order.lock().await;
    data.game_server_warmup
        .lock()
        .await
        .apply_stage_area_npcs(request.body.data.clone());
    Ok(())
}

pub async fn warmup_stage_area_start_points(
    request: Request<GameServerStageAreaStartPointChunk>,
    _responder: Responder,
    data: Data<ServerState>,
) -> Result<(), AppError> {
    let _ordered = data.game_server_warmup_order.lock().await;
    data.game_server_warmup
        .lock()
        .await
        .apply_stage_area_start_points(request.body.data.clone());
    Ok(())
}

pub async fn warmup_stage_area_monster_rules(
    request: Request<GameServerStageAreaMonsterRuleChunk>,
    _responder: Responder,
    data: Data<ServerState>,
) -> Result<(), AppError> {
    let _ordered = data.game_server_warmup_order.lock().await;
    data.game_server_warmup
        .lock()
        .await
        .apply_stage_area_monster_rules(request.body.data.clone());
    Ok(())
}

pub async fn warmup_stage_area_boss_rules(
    request: Request<GameServerStageAreaBossRuleChunk>,
    _responder: Responder,
    data: Data<ServerState>,
) -> Result<(), AppError> {
    let _ordered = data.game_server_warmup_order.lock().await;
    data.game_server_warmup
        .lock()
        .await
        .apply_stage_area_boss_rules(request.body.data.clone());
    Ok(())
}

pub async fn warmup_done(
    _request: Request<GameServerWarmupDone>,
    responder: Responder,
    data: Data<ServerState>,
) -> Result<(), AppError> {
    let _ordered = data.game_server_warmup_order.lock().await;
    let mut attempts = 0usize;
    loop {
        let snapshot = data.game_server_warmup.lock().await.snapshot();
        if warmup_snapshot_is_complete(&snapshot) {
            break;
        }

        attempts += 1;
        if attempts >= 200 {
            let expected = snapshot.meta.as_ref();
            log::warn!(
                "warmup_done arrived before all chunks were processed. proceeding with partial snapshot: fields={}/{:?}, monsters={}/{:?}, npcs={}/{:?}, open_items={}/{:?}, stage_areas={}/{:?}",
                snapshot.fields.len(),
                expected.map(|meta| meta.field_count),
                snapshot.monsters.len(),
                expected.map(|meta| meta.monster_count),
                snapshot.npcs.len(),
                expected.map(|meta| meta.npc_count),
                snapshot.open_items.len(),
                expected.map(|meta| meta.open_item_count),
                snapshot.stage_areas.len()
                ,
                expected.map(|meta| meta.stage_area_count)
            );
            break;
        }

        sleep(Duration::from_millis(10)).await;
    }

    let snapshot = data.game_server_warmup.lock().await.take();

    let snapshot_for_disk = snapshot.clone();
    std::thread::spawn(move || {
        if let Err(error) = save_game_server_warmup_json(&snapshot_for_disk) {
            log::warn!(
                "failed to persist game-server warmup json files: {:?}",
                error
            );
        }
    });

    let filtered_data = build_filtered_game_server_data(&snapshot);

    if let Some(tx) = world_sender() {
        let _ = tx.send(WorldCommand::LoadGameServerWarmup {
            data: filtered_data,
            observed_at: Instant::now(),
        });
    } else {
        log::error!("world sender not initialized for GameServer warmup");
    }

    if let Err(error) = responder.close_connection().await {
        log::warn!("failed to close warmup legacy connection: {:?}", error);
    }
    Ok(())
}

pub async fn ignore_unexpected_warmup_packet(
    request: RequestVecBody,
    _responder: Responder,
    _data: Data<ServerState>,
) -> Result<(), AppError> {
    log::warn!(
        "unexpected packet received on dedicated warmup connection: code=0x{:X}, size={}",
        request.head.code,
        request.head.size
    );
    Ok(())
}

pub async fn warmup_server_version(
    request: Request<Command>,
    responder: Responder,
    _data: Data<ServerState>,
) -> Result<(), AppError> {
    log::trace!(
        "warmup server version packet: responder_id={} version={} player_count_hint={}",
        responder.id,
        request.body.w_param,
        request.body.l_param
    );
    Ok(())
}

pub async fn warmup_disconnect(responder: Responder, data: Data<ServerState>) {
    let mut collector = data.game_server_warmup.lock().await;
    collector.mark_disconnected();
    let completed = collector.is_completed();
    let snapshot = collector.snapshot();

    if completed {
        log::info!(
            "warmup connection {} closed after successful warmup transfer",
            responder.id
        );
        return;
    }

    match snapshot.meta {
        None => {
            log::warn!(
                "warmup connection {} closed before GameServerWarmupMeta. the running legacy server likely does not have GetGameServerAllData implemented",
                responder.id
            );
        }
        Some(meta) => {
            log::info!(
                "warmup connection {} closed. received so far: fields={}/{}, monsters={}/{}, npcs={}/{}, open_items={}/{}, stage_areas={}/{}",
                responder.id,
                snapshot.fields.len(),
                meta.field_count as usize,
                snapshot.monsters.len(),
                meta.monster_count as usize,
                snapshot.npcs.len(),
                meta.npc_count as usize,
                snapshot.open_items.len(),
                meta.open_item_count as usize,
                snapshot.stage_areas.len(),
                meta.stage_area_count as usize
            );
        }
    }
}
