use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde::Serialize;

use crate::domain::skill::dtos::codes::{ChangeJobTier, SkillCode, SkillGroup, SkillSlot};
use crate::domain::skill::dtos::skills::{
    ArcherSkills, AssassinSkills, AtalantaSkills, FighterSkills, KnightSkills, MagicianSkills,
    MartialSkills, MechanicianSkills, PikemanSkills, PriestessSkills, ShamanSkills,
    SkillClassConfig, SkillEntry,
};
use crate::domain::user_player::dto::PlayerJob;
use crate::error::{AppError, IoErrorKind, ParseErrorKind};
use crate::protocol::game_server::field::{FieldMemory, StageAreaMemory};
use crate::protocol::game_server::monster::MonsterMemory;
use crate::protocol::game_server::npc::NpcMemory;
use crate::protocol::game_server::open_item::DefItemInfo;
use crate::state::GameServerWarmupSnapshot;

pub fn save_debug_json<T>(value: &T) -> Result<(), AppError>
where
    T: Serialize,
{
    let path = Path::new("debug/game_save.json");

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            crate::io_error!(
                IoErrorKind::CreateDir,
                e,
                "failed to create directory {}",
                parent.display()
            )
        })?;
    }

    let json = serde_json::to_string_pretty(value)
        .map_err(|e| crate::parse_error!(ParseErrorKind::Json, e, "failed to serialize JSON"))?;

    fs::write(path, json).map_err(|e| {
        crate::io_error!(
            IoErrorKind::WriteFile,
            e,
            "failed to write JSON file {}",
            path.display()
        )
    })?;

    Ok(())
}

fn write_pretty_json_file<T>(path: &Path, value: &T) -> Result<(), AppError>
where
    T: Serialize,
{
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            crate::io_error!(
                IoErrorKind::CreateDir,
                e,
                "failed to create directory {}",
                parent.display()
            )
        })?;
    }

    let json = serde_json::to_string_pretty(value)
        .map_err(|e| crate::parse_error!(ParseErrorKind::Json, e, "failed to serialize JSON"))?;

    fs::write(path, json).map_err(|e| {
        crate::io_error!(
            IoErrorKind::WriteFile,
            e,
            "failed to write JSON file {}",
            path.display()
        )
    })?;

    Ok(())
}

fn sanitize_file_stem(raw: &str) -> String {
    let mut sanitized = String::with_capacity(raw.len());

    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() {
            sanitized.push(ch.to_ascii_lowercase());
        } else if matches!(ch, ' ' | '-' | '_' | '.') {
            sanitized.push('_');
        } else if matches!(ch, '\\' | '/' | ':') {
            sanitized.push('_');
        }
    }

    let sanitized = sanitized.trim_matches('_');
    if sanitized.is_empty() {
        "unnamed".to_string()
    } else {
        sanitized.to_string()
    }
}

fn unique_json_path(
    root: &Path,
    directory: &str,
    name: &str,
    name_counts: &mut HashMap<String, usize>,
) -> std::path::PathBuf {
    let stem = sanitize_file_stem(name);
    let next_count = {
        let count = name_counts.entry(stem.clone()).or_insert(0);
        *count += 1;
        *count
    };

    let file_stem = if next_count == 1 {
        stem
    } else {
        format!("{stem}_{next_count}")
    };

    root.join(directory).join(format!("{file_stem}.json"))
}

fn field_json_path(
    root: &Path,
    field: &FieldMemory,
    name_counts: &mut HashMap<String, usize>,
) -> std::path::PathBuf {
    unique_json_path(root, "field", &field.sz_name, name_counts)
}

fn monster_json_path(
    root: &Path,
    monster: &MonsterMemory,
    name_counts: &mut HashMap<String, usize>,
) -> std::path::PathBuf {
    unique_json_path(root, "monsters", &monster.char_info.sz_name, name_counts)
}

fn npc_json_path(
    root: &Path,
    npc: &NpcMemory,
    name_counts: &mut HashMap<String, usize>,
) -> std::path::PathBuf {
    unique_json_path(root, "npcs", &npc.monster_info.sz_name, name_counts)
}

fn open_item_json_path(
    root: &Path,
    open_item: &DefItemInfo,
    name_counts: &mut HashMap<String, usize>,
) -> std::path::PathBuf {
    unique_json_path(
        root,
        "open-itens",
        &open_item.item.item_name.to_string(),
        name_counts,
    )
}

fn stage_area_json_path(
    root: &Path,
    stage_area: &StageAreaMemory,
    name_counts: &mut HashMap<String, usize>,
) -> std::path::PathBuf {
    unique_json_path(
        root,
        "stage-areas",
        &stage_area.sz_char_monster_file,
        name_counts,
    )
}

pub fn save_game_server_warmup_json(snapshot: &GameServerWarmupSnapshot) -> Result<(), AppError> {
    let root = Path::new("game-server");
    let mut field_paths = HashMap::new();
    let mut monster_paths = HashMap::new();
    let mut npc_paths = HashMap::new();
    let mut open_item_paths = HashMap::new();
    let mut stage_area_paths = HashMap::new();

    if root.exists() {
        fs::remove_dir_all(root).map_err(|e| {
            crate::io_error!(
                IoErrorKind::RemoveDir,
                e,
                "failed to reset game-server directory {}",
                root.display()
            )
        })?;
    }

    for field in &snapshot.fields {
        let path = field_json_path(root, field, &mut field_paths);
        write_pretty_json_file(&path, field)?;
    }

    for monster in &snapshot.monsters {
        let path = monster_json_path(root, monster, &mut monster_paths);
        write_pretty_json_file(&path, monster)?;
    }

    for npc in &snapshot.npcs {
        let path = npc_json_path(root, npc, &mut npc_paths);
        write_pretty_json_file(&path, npc)?;
    }

    for open_item in snapshot
        .open_items
        .iter()
        .filter(|open_item| open_item.item.code != 0)
    {
        let path = open_item_json_path(root, open_item, &mut open_item_paths);
        write_pretty_json_file(&path, open_item)?;
    }

    for stage_area in &snapshot.stage_areas {
        let path = stage_area_json_path(root, stage_area, &mut stage_area_paths);
        write_pretty_json_file(&path, stage_area)?;
    }

    Ok(())
}

fn read_json_file_raw(path: &Path) -> Result<String, AppError> {
    std::fs::read_to_string(path).map_err(|error| {
        crate::io_error!(
            IoErrorKind::ReadFile,
            error,
            "failed to read JSON file: {}",
            path.display()
        )
    })
}

fn invalid_skill_config(path: &Path, reason: impl Into<String>) -> AppError {
    crate::parse_error!(
        ParseErrorKind::Json,
        "invalid skill config in {}: {}",
        path.display(),
        reason.into()
    )
}

fn skill_group_for_job(job: PlayerJob) -> SkillGroup {
    match job {
        PlayerJob::Mechanician => SkillGroup::Mechanician,
        PlayerJob::Fighter => SkillGroup::Fighter,
        PlayerJob::Pikeman => SkillGroup::Pikeman,
        PlayerJob::Archer => SkillGroup::Archer,
        PlayerJob::Knight => SkillGroup::Knight,
        PlayerJob::Atalanta => SkillGroup::Atalanta,
        PlayerJob::Priestess => SkillGroup::Priestess,
        PlayerJob::Magician => SkillGroup::Magician,
        PlayerJob::Assassine => SkillGroup::Assassin,
        PlayerJob::Shaman => SkillGroup::Shaman,
        PlayerJob::Martial => SkillGroup::Martial,
    }
}

fn skill_code_by_index(job: PlayerJob, idx: usize) -> Option<SkillCode> {
    if idx >= 20 {
        return None;
    }
    let tier = match idx / 4 {
        0 => ChangeJobTier::Tier1,
        1 => ChangeJobTier::Tier2,
        2 => ChangeJobTier::Tier3,
        3 => ChangeJobTier::Tier4,
        4 => ChangeJobTier::Tier5,
        _ => return None,
    };
    Some(SkillCode::new(
        skill_group_for_job(job),
        tier,
        SkillSlot((idx % 4 + 1) as u16),
    ))
}

fn parse_entries(job: PlayerJob, raw: &str, path: &Path) -> Result<Vec<SkillEntry>, AppError> {
    match job {
        PlayerJob::Pikeman => {
            let parsed: PikemanSkills = serde_json::from_str(raw).map_err(|error| {
                invalid_skill_config(path, format!("serde parse error: {error}"))
            })?;
            Ok(vec![
                SkillEntry::PikeWind(parsed.pike_wind),
                SkillEntry::IceAttribute(parsed.ice_attribute),
                SkillEntry::CriticalHit(parsed.critical_hit),
                SkillEntry::JumpingCrash(parsed.jumping_crash),
                SkillEntry::GroundPike(parsed.ground_pike),
                SkillEntry::Tornado(parsed.tornado),
                SkillEntry::WeaponDefenseMastery(parsed.weapon_defense_mastery),
                SkillEntry::Expansion(parsed.expansion),
                SkillEntry::VenomSpear(parsed.venom_spear),
                SkillEntry::Vanish(parsed.vanish),
                SkillEntry::CriticalMastery(parsed.critical_mastery),
                SkillEntry::ChainLance(parsed.chain_lance),
                SkillEntry::AssassinEye(parsed.assassin_eye),
                SkillEntry::ChargingStrike(parsed.charging_strike),
                SkillEntry::Vague(parsed.vague),
                SkillEntry::ShadowMaster(parsed.shadow_master),
                SkillEntry::DReaper(parsed.d_reaper),
                SkillEntry::FSpear(parsed.f_spear),
                SkillEntry::SsAttack(parsed.ss_attack),
                SkillEntry::Amplified(parsed.amplified),
            ])
        }
        PlayerJob::Mechanician => {
            let parsed: MechanicianSkills = serde_json::from_str(raw).map_err(|error| {
                invalid_skill_config(path, format!("serde parse error: {error}"))
            })?;
            Ok(vec![
                SkillEntry::ExtremeShield(parsed.extreme_shield),
                SkillEntry::MechanicBomb(parsed.mechanic_bomb),
                SkillEntry::PhysicalAbsorb(parsed.physical_absorb),
                SkillEntry::PoisonAttribute(parsed.poison_attribute),
                SkillEntry::GreatSmash(parsed.great_smash),
                SkillEntry::Maximize(parsed.maximize),
                SkillEntry::Automation(parsed.automation),
                SkillEntry::Spark(parsed.spark),
                SkillEntry::MetalArmor(parsed.metal_armor),
                SkillEntry::GrandSmash(parsed.grand_smash),
                SkillEntry::MechanicWeaponMastery(parsed.mechanic_weapon_mastery),
                SkillEntry::SparkShield(parsed.spark_shield),
                SkillEntry::Impulsion(parsed.impulsion),
                SkillEntry::Compulsion(parsed.compulsion),
                SkillEntry::MagneticSphere(parsed.magnetic_sphere),
                SkillEntry::MetalGolem(parsed.metal_golem),
                SkillEntry::LandMine(parsed.land_mine),
                SkillEntry::HyperSonic(parsed.hyper_sonic),
                SkillEntry::RSmash(parsed.r_smash),
                SkillEntry::PhysicalEnhance(parsed.physical_enhance),
            ])
        }
        PlayerJob::Fighter => {
            let parsed: FighterSkills = serde_json::from_str(raw).map_err(|error| {
                invalid_skill_config(path, format!("serde parse error: {error}"))
            })?;
            Ok(vec![
                SkillEntry::MeleeMastery(parsed.melee_mastery),
                SkillEntry::FireAttribute(parsed.fire_attribute),
                SkillEntry::Raving(parsed.raving),
                SkillEntry::Impact(parsed.impact),
                SkillEntry::TripleImpact(parsed.triple_impact),
                SkillEntry::BrutalSwing(parsed.brutal_swing),
                SkillEntry::Roar(parsed.roar),
                SkillEntry::RageOfZecram(parsed.rage_of_zecram),
                SkillEntry::Concentration(parsed.concentration),
                SkillEntry::AvangingCrash(parsed.avanging_crash),
                SkillEntry::SwiftAxe(parsed.swift_axe),
                SkillEntry::BoneCrash(parsed.bone_crash),
                SkillEntry::Destoryer(parsed.destoryer),
                SkillEntry::Berserker(parsed.berserker),
                SkillEntry::CycloneStrike(parsed.cyclone_strike),
                SkillEntry::BoostHealth(parsed.boost_health),
                SkillEntry::DHit(parsed.d_hit),
                SkillEntry::PDash(parsed.p_dash),
                SkillEntry::MBlow(parsed.m_blow),
                SkillEntry::BBerserker(parsed.b_berserker),
            ])
        }
        PlayerJob::Archer => {
            let parsed: ArcherSkills = serde_json::from_str(raw).map_err(|error| {
                invalid_skill_config(path, format!("serde parse error: {error}"))
            })?;
            Ok(vec![
                SkillEntry::ScoutHawk(parsed.scout_hawk),
                SkillEntry::ShootingMastery(parsed.shooting_mastery),
                SkillEntry::WindArrow(parsed.wind_arrow),
                SkillEntry::PerfectAim(parsed.perfect_aim),
                SkillEntry::DionsEye(parsed.dions_eye),
                SkillEntry::Falcon(parsed.falcon),
                SkillEntry::ArrowOfRage(parsed.arrow_of_rage),
                SkillEntry::Avalanche(parsed.avalanche),
                SkillEntry::ElementalShot(parsed.elemental_shot),
                SkillEntry::GoldenFalcon(parsed.golden_falcon),
                SkillEntry::BombShot(parsed.bomb_shot),
                SkillEntry::Perforation(parsed.perforation),
                SkillEntry::RecallWolverin(parsed.recall_wolverin),
                SkillEntry::EvasionMastery(parsed.evasion_mastery),
                SkillEntry::PhoenixShot(parsed.phoenix_shot),
                SkillEntry::ForceOfNature(parsed.force_of_nature),
                SkillEntry::EShot(parsed.e_shot),
                SkillEntry::SRope(parsed.s_rope),
                SkillEntry::NSplash(parsed.n_splash),
                SkillEntry::CTrap(parsed.c_trap),
            ])
        }
        PlayerJob::Knight => {
            let parsed: KnightSkills = serde_json::from_str(raw).map_err(|error| {
                invalid_skill_config(path, format!("serde parse error: {error}"))
            })?;
            Ok(vec![
                SkillEntry::SwordBlast(parsed.sword_blast),
                SkillEntry::HolyBody(parsed.holy_body),
                SkillEntry::PhysicalTraining(parsed.physical_training),
                SkillEntry::DoubleCrash(parsed.double_crash),
                SkillEntry::HolyValor(parsed.holy_valor),
                SkillEntry::Brandish(parsed.brandish),
                SkillEntry::Piercing(parsed.piercing),
                SkillEntry::DrasticSpirit(parsed.drastic_spirit),
                SkillEntry::SwordMastery(parsed.sword_mastery),
                SkillEntry::DivineInhalation(parsed.divine_inhalation),
                SkillEntry::HolyIncantation(parsed.holy_incantation),
                SkillEntry::GrandCross(parsed.grand_cross),
                SkillEntry::DivinePiercing(parsed.divine_piercing),
                SkillEntry::GodlyShield(parsed.godly_shield),
                SkillEntry::GodBless(parsed.god_bless),
                SkillEntry::SwordOfJustice(parsed.sword_of_justice),
                SkillEntry::SBreaker(parsed.s_breaker),
                SkillEntry::CMoon(parsed.c_moon),
                SkillEntry::SBlade(parsed.s_blade),
                SkillEntry::HBenedic(parsed.h_benedic),
            ])
        }
        PlayerJob::Atalanta => {
            let parsed: AtalantaSkills = serde_json::from_str(raw).map_err(|error| {
                invalid_skill_config(path, format!("serde parse error: {error}"))
            })?;
            Ok(vec![
                SkillEntry::ShieldStrike(parsed.shield_strike),
                SkillEntry::Farina(parsed.farina),
                SkillEntry::ThrowingMastery(parsed.throwing_mastery),
                SkillEntry::VigorSpear(parsed.vigor_spear),
                SkillEntry::Windy(parsed.windy),
                SkillEntry::TwistJavelin(parsed.twist_javelin),
                SkillEntry::SoulSucker(parsed.soul_sucker),
                SkillEntry::FireJavelin(parsed.fire_javelin),
                SkillEntry::SplitJavelin(parsed.split_javelin),
                SkillEntry::TriumphOfValhalla(parsed.triumph_of_valhalla),
                SkillEntry::LightningJavelin(parsed.lightning_javelin),
                SkillEntry::StormJavelin(parsed.storm_javelin),
                SkillEntry::HallOfValhalla(parsed.hall_of_valhalla),
                SkillEntry::XRage(parsed.x_rage),
                SkillEntry::FrostJavelin(parsed.frost_javelin),
                SkillEntry::Vengeance(parsed.vengeance),
                SkillEntry::Talaria(parsed.talaria),
                SkillEntry::GCoup(parsed.g_coup),
                SkillEntry::Arcuda(parsed.arcuda),
                SkillEntry::SFear(parsed.s_fear),
            ])
        }
        PlayerJob::Priestess => {
            let parsed: PriestessSkills = serde_json::from_str(raw).map_err(|error| {
                invalid_skill_config(path, format!("serde parse error: {error}"))
            })?;
            Ok(vec![
                SkillEntry::Healing(parsed.healing),
                SkillEntry::HolyBolt(parsed.holy_bolt),
                SkillEntry::MultiSpark(parsed.multi_spark),
                SkillEntry::HolyMind(parsed.holy_mind),
                SkillEntry::Meditation(parsed.meditation),
                SkillEntry::DivineLightning(parsed.divine_lightning),
                SkillEntry::HolyReflection(parsed.holy_reflection),
                SkillEntry::GrandHealing(parsed.grand_healing),
                SkillEntry::VigorBall(parsed.vigor_ball),
                SkillEntry::Resurrection(parsed.resurrection),
                SkillEntry::Extinction(parsed.extinction),
                SkillEntry::VirtualLife(parsed.virtual_life),
                SkillEntry::GlacialSpike(parsed.glacial_spike),
                SkillEntry::RegenerationField(parsed.regeneration_field),
                SkillEntry::ChainLightning(parsed.chain_lightning),
                SkillEntry::SummonMuspell(parsed.summon_muspell),
                SkillEntry::SImpact(parsed.s_impact),
                SkillEntry::PIce(parsed.p_ice),
                SkillEntry::SummonRamiel(parsed.summon_ramiel),
                SkillEntry::Krishna(parsed.krishna),
            ])
        }
        PlayerJob::Magician => {
            let parsed: MagicianSkills = serde_json::from_str(raw).map_err(|error| {
                invalid_skill_config(path, format!("serde parse error: {error}"))
            })?;
            Ok(vec![
                SkillEntry::Agony(parsed.agony),
                SkillEntry::FireBolt(parsed.fire_bolt),
                SkillEntry::Zenith(parsed.zenith),
                SkillEntry::FireBall(parsed.fire_ball),
                SkillEntry::MentalMastery(parsed.mental_mastery),
                SkillEntry::Watornado(parsed.watornado),
                SkillEntry::EnchantWeapon(parsed.enchant_weapon),
                SkillEntry::DeadRay(parsed.dead_ray),
                SkillEntry::EnergyShield(parsed.energy_shield),
                SkillEntry::Diastrophism(parsed.diastrophism),
                SkillEntry::SpiritElemental(parsed.spirit_elemental),
                SkillEntry::DancingSword(parsed.dancing_sword),
                SkillEntry::FireElemental(parsed.fire_elemental),
                SkillEntry::FlameWave(parsed.flame_wave),
                SkillEntry::Distortion(parsed.distortion),
                SkillEntry::Meteo(parsed.meteo),
                SkillEntry::Silraphim(parsed.silraphim),
                SkillEntry::Tenus(parsed.tenus),
                SkillEntry::Ignis(parsed.ignis),
                SkillEntry::Anima(parsed.anima),
            ])
        }
        PlayerJob::Assassine => {
            let parsed: AssassinSkills = serde_json::from_str(raw).map_err(|error| {
                invalid_skill_config(path, format!("serde parse error: {error}"))
            })?;
            Ok(vec![
                SkillEntry::Stinger(parsed.stinger),
                SkillEntry::DoubleBlow(parsed.double_blow),
                SkillEntry::DMastery(parsed.d_mastery),
                SkillEntry::Wisp(parsed.wisp),
                SkillEntry::VenomThorn(parsed.venom_thorn),
                SkillEntry::Alas(parsed.alas),
                SkillEntry::SoulShock(parsed.soul_shock),
                SkillEntry::AMastery(parsed.a_mastery),
                SkillEntry::SoreSword(parsed.sore_sword),
                SkillEntry::BeatUp(parsed.beat_up),
                SkillEntry::Inpes(parsed.inpes),
                SkillEntry::Blind(parsed.blind),
                SkillEntry::FrostWind(parsed.frost_wind),
                SkillEntry::FMastery(parsed.f_mastery),
                SkillEntry::Polluted(parsed.polluted),
                SkillEntry::PastingShadow(parsed.pasting_shadow),
                SkillEntry::JBomb(parsed.j_bomb),
                SkillEntry::RSlash(parsed.r_slash),
                SkillEntry::VStab(parsed.v_stab),
                SkillEntry::Storm(parsed.storm),
            ])
        }
        PlayerJob::Shaman => {
            let parsed: ShamanSkills = serde_json::from_str(raw).map_err(|error| {
                invalid_skill_config(path, format!("serde parse error: {error}"))
            })?;
            Ok(vec![
                SkillEntry::DarkBolt(parsed.dark_bolt),
                SkillEntry::InnerPeace(parsed.inner_peace),
                SkillEntry::SoulManacle(parsed.soul_manacle),
                SkillEntry::Haunt(parsed.haunt),
                SkillEntry::Scratch(parsed.scratch),
                SkillEntry::Judgement(parsed.judgement),
                SkillEntry::DarkWave(parsed.dark_wave),
                SkillEntry::CurseLazy(parsed.curse_lazy),
                SkillEntry::SpiritualFlare(parsed.spiritual_flare),
                SkillEntry::ChasingHunt(parsed.chasing_hunt),
                SkillEntry::LandGhost(parsed.land_ghost),
                SkillEntry::MourningOfPray(parsed.mourning_of_pray),
                SkillEntry::BloodyKnight(parsed.bloody_knight),
                SkillEntry::AdventMigal(parsed.advent_migal),
                SkillEntry::RainMaker(parsed.rain_maker),
                SkillEntry::AdventMidranda(parsed.advent_midranda),
                SkillEntry::Creed(parsed.creed),
                SkillEntry::PDeity(parsed.p_deity),
                SkillEntry::GNail(parsed.g_nail),
                SkillEntry::HRegene(parsed.h_regene),
            ])
        }
        PlayerJob::Martial => {
            let parsed: MartialSkills = serde_json::from_str(raw).map_err(|error| {
                invalid_skill_config(path, format!("serde parse error: {error}"))
            })?;
            Ok(vec![
                SkillEntry::LowKick(parsed.low_kick),
                SkillEntry::SwordMasteryMartial(parsed.sword_mastery),
                SkillEntry::DoubleBlowMartial(parsed.double_blow),
                SkillEntry::Straight(parsed.straight),
                SkillEntry::RageUp(parsed.rage_up),
                SkillEntry::Patriot(parsed.patriot),
                SkillEntry::Elbow(parsed.elbow),
                SkillEntry::SwordMastery2(parsed.sword_mastery2),
                SkillEntry::Bulkup(parsed.bulkup),
                SkillEntry::Warcry(parsed.warcry),
                SkillEntry::Cannon(parsed.cannon),
                SkillEntry::HeelKick(parsed.heel_kick),
                SkillEntry::Combination(parsed.combination),
                SkillEntry::Steelers(parsed.steelers),
                SkillEntry::Check(parsed.check),
                SkillEntry::Typhoon(parsed.typhoon),
                SkillEntry::DMastery2(parsed.d_mastery2),
                SkillEntry::HHawk(parsed.h_hawk),
                SkillEntry::LBreaking(parsed.l_breaking),
                SkillEntry::HTraining(parsed.h_training),
            ])
        }
    }
}

pub fn read_skill_class_config_from_json_file(
    path: &Path,
    job: PlayerJob,
) -> Result<SkillClassConfig, AppError> {
    let raw = read_json_file_raw(path)?;
    let entries = parse_entries(job, &raw, path)?;

    let mut by_code = HashMap::with_capacity(entries.len());
    for (idx, entry) in entries.into_iter().enumerate() {
        if let Some(code) = skill_code_by_index(job, idx) {
            by_code.insert(code, entry);
        }
    }

    Ok(SkillClassConfig { by_code })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_json_file_missing_file_returns_explicit_error() {
        let path = std::env::temp_dir().join("skill_read_json_file_missing.json");
        let error = read_json_file_raw(&path);
        assert!(error.is_err());
        let error = match error {
            Err(error) => error,
            Ok(_) => {
                assert!(false, "expected read_json_file_raw to fail");
                return;
            }
        };
        assert!(error.message.contains("failed to read JSON file"));
    }

    #[test]
    fn parse_rejects_invalid_weapon_token() {
        let path = std::env::temp_dir().join("skill_parse_invalid_weapon.json");
        std::fs::write(
            &path,
            r#"{
                "scout_hawk": {"name":"x","description":"x","require_level":1,"use_stamina":[0,0],"require_mastery":[0,0],"element":[0,0,0],"use_weapon_code":[],"attack_rate":{"value_type":"Fixed2","value":[1,1,1,1,1,1,1,1,1,1]},"use_time":{"value_type":"Fixed2","value":[1,1,1,1,1,1,1,1,1,1]},"use_mana":{"value_type":"Fixed2","value":[1,1,1,1,1,1,1,1,1,1]}},
                "shooting_mastery": {"name":"x","description":"x","require_level":1,"use_stamina":[0,0],"require_mastery":[0,0],"element":[0,0,0],"use_weapon_code":[""],"damage_percent":{"value_type":"Percent","value":[1,1,1,1,1,1,1,1,1,1]}},
                "wind_arrow": {"name":"x","description":"x","require_level":1,"use_stamina":[0,0],"require_mastery":[0,0],"element":[0,0,0],"use_weapon_code":[],"speed":{"value_type":"Fixed2","value":[1,1,1,1,1,1,1,1,1,1]},"damage":{"value_type":"Fixed2","value":[1,1,1,1,1,1,1,1,1,1]},"use_mana":{"value_type":"Fixed2","value":[1,1,1,1,1,1,1,1,1,1]}},
                "perfect_aim": {"name":"x","description":"x","require_level":1,"use_stamina":[0,0],"require_mastery":[0,0],"element":[0,0,0],"use_weapon_code":[],"attack_rate":{"value_type":"Fixed2","value":[1,1,1,1,1,1,1,1,1,1]},"damage":{"value_type":"Percent","value":[1,1,1,1,1,1,1,1,1,1]},"damage_lv":{"value_type":"Fixed2","value":[1,1,1,1,1,1,1,1,1,1]},"use_mana":{"value_type":"Fixed2","value":[1,1,1,1,1,1,1,1,1,1]}}
            }"#,
        )
        .unwrap_or_else(|error| {
            assert!(false, "must write fixture: {error}");
        });

        let err = read_skill_class_config_from_json_file(&path, PlayerJob::Archer);
        assert!(err.is_err());
        let err = match err {
            Err(err) => err,
            Ok(_) => {
                assert!(false, "expected invalid skill config");
                return;
            }
        };
        assert!(err.message.contains("invalid skill config"));
        let _ = std::fs::remove_file(path);
    }
}
