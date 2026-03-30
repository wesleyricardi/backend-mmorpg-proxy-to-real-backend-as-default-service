use std::collections::HashMap;

use crate::domain::item::dto::EquippableItemType;
use crate::domain::skill::classification::{
    is_party_skill, skill_runtime_flag_rule, supports_runtime_kind, SkillRuntimeFlagRule,
    SkillRuntimeKind,
};
use crate::domain::skill::dtos::codes::{
    SkillCode, SkillGroup, SKILL_AMPLIFIED, SKILL_ANIMA, SKILL_A_MASTERY, SKILL_A_MIDRANDA,
    SKILL_BERSERKER, SKILL_BOOST_HEALTH, SKILL_B_BERSERKER, SKILL_B_CHECK, SKILL_COMPULSION,
    SKILL_CONCENTRATION, SKILL_CREED, SKILL_CRITICAL_MASTERY, SKILL_DIONS_EYE, SKILL_DISTORTION,
    SKILL_D_MASTERY, SKILL_D_MASTERY2, SKILL_ENERGY_SHIELD, SKILL_EVASION_MASTERY,
    SKILL_EXTREME_SHIELD, SKILL_FORCE_OF_NATURE, SKILL_F_MASTERY, SKILL_GODLY_SHIELD,
    SKILL_GOD_BLESS, SKILL_GOLDEN_FALCON, SKILL_HALL_OF_VALHALLA, SKILL_HOLY_BODY,
    SKILL_HOLY_INCANTATION, SKILL_HOLY_MIND, SKILL_HOLY_VALOR, SKILL_H_REGENE, SKILL_H_TRAINING,
    SKILL_INPES, SKILL_I_BULKUP, SKILL_KRISHNA, SKILL_MECHANIC_WEAPON, SKILL_MEDITATION,
    SKILL_MELEE_MASTERY, SKILL_MENTAL_MASTERY, SKILL_PATRIOT, SKILL_PERFECT_AIM,
    SKILL_PHOENIX_SHOT, SKILL_PHYSICAL_ABSORB, SKILL_PHYSICAL_TRAINING, SKILL_RAGE_UP,
    SKILL_REGENERATION_FIELD, SKILL_R_MAKER, SKILL_SHOOTING_MASTERY, SKILL_SPARK_SHIELD,
    SKILL_SPIRIT_ELEMENTAL, SKILL_SS_ATTACK, SKILL_STEELERS, SKILL_SWIFT_AXE, SKILL_SWORD_MASTERY,
    SKILL_S_MASTERY, SKILL_S_MASTERY2, SKILL_TALARIA, SKILL_TENUS, SKILL_THROWING_MASTERY,
    SKILL_TRIUMPH_OF_VALHALLA, SKILL_VANISH, SKILL_VENGEANCE, SKILL_VIRTUAL_LIFE,
    SKILL_WEAPON_DEFENSE_MASTERY, SKILL_WINDY,
};
use crate::domain::skill::dtos::skills::{SkillEntry, SkillLevel, SkillValueType};
use crate::domain::skill::state::SkillState;
use crate::domain::user_player::dto::{
    CalculatedCharStats, ChangeJobTierState, PlayerJob, UserContinueSkill, UserPassiveSkill,
};

#[derive(Clone, Copy)]
enum SkillEffect {
    AttackRatingFlat,
    AttackRatingPercent,
    AttackDamageFlat,
    AttackDamagePercent,
    AttackSpeedFlat,
    DefenceFlat,
    DefencePercent,
    AbsorptionFlat,
    AbsorptionPercent,
    ChanceBlockFlat,
    CriticalFlat,
    MoveSpeedFlat,
    MoveSpeedPercent,
    LifeFlat,
    LifePercent,
    ManaFlat,
    StaminaFlat,
    LifeRegenFlat,
    ManaRegenFlat,
    EvasionPercent,
}

#[derive(Clone, Copy)]
enum ValueSelector {
    MeleeMasteryDamagePercent,
    SwordMasteryDamagePercent,
    ShootingMasteryDamagePercent,
    ThrowingMasteryDamage,
    DionsEyeAttackRate,
    EvasionMasteryAddPercent,
    DMasteryDamagePercent,
    AMasteryAddPercent,
    AMasteryAddPercent2,
    FMasteryCritical,
    MentalMasteryMana,
    MeditationRegen,
    PhysicalTrainingStamina,
    WeaponDefenseMasteryBlock,
    CriticalMasteryCritical,
    MechanicWeaponMastery,
    SwordMasteryMartialDamage,
    SwordMastery2Stamina,
    SwordMastery2Rate,
    BoostHealthLife,
    ExtremeShieldBlockRate,
    PhysicalAbsorb,
    SparkShieldDefense,
    CompulsionAddAbsorb,
    HolyBodyAbsorb,
    HolyValorDamage,
    HolyIncantationAddLife,
    GodlyShieldAbsorbPercent,
    GodBlessAddDamage,
    ConcentrationAttackRate,
    SwiftAxeSpeed,
    BerserkerAddAttack,
    BerserkerSubAbsorb,
    BBerserkerAddAttack,
    BBerserkerSubAbsorb,
    WindyAttackRating,
    VengeanceAttackRate,
    VengeanceDamagePercent,
    VengeanceAddCritical,
    TalariaSpeed,
    VanishSpeed,
    AmplifiedAddAttack,
    AmplifiedSubAbsorb,
    SsAttackRating,
    SsAttackAbsorb,
    PerfectAimAttackRate,
    PerfectAimDamage,
    ForceOfNatureAddDamage,
    ForceOfNatureAddHit,
    GoldenFalconLifeRegen,
    PhoenixShotDamagePercent,
    HolyMindDecDamage,
    VirtualLifePercent,
    RegenerationFieldLifeRegen,
    RegenerationFieldManaRegen,
    KrishnaDefense,
    KrishnaAbs,
    EnergyShieldDecDamage,
    SpiritElementalRegenMana,
    TenusHp,
    TenusMana,
    AnimaMana,
    InpesSpeed,
    RageUpSpeed,
    BulkupHp,
    SteelersDefense,
    PatriotCritical,
    CheckCritical,
    RainMakerAbsorb,
    AdventMidrandaSpeed,
    DistortionDamageSubPercent,
    DistortionSpeedSubPercent,
    HRegeneAddHp,
    DMastery2AddAttackRate,
    CreedAddAttack,
    CreedAddAttackRate,
    HTrainingAddAttack,
    HTrainingAddAttackRate,
}

#[derive(Clone, Copy)]
struct SkillEffectRule {
    selector: ValueSelector,
    effect: SkillEffect,
}

fn make_level(point: i32) -> Option<SkillLevel> {
    if point <= 0 {
        return None;
    }
    let clamped = point.clamp(1, 10) as u8;
    SkillLevel::new(clamped)
}

fn scalar_i32(
    levels: &crate::domain::skill::dtos::skills::Levels<i32>,
    level: SkillLevel,
) -> (f32, SkillValueType) {
    (levels.at(level) as f32, levels.value_type())
}

fn scalar_f32(
    levels: &crate::domain::skill::dtos::skills::Levels<f32>,
    level: SkillLevel,
) -> (f32, SkillValueType) {
    (levels.at(level), levels.value_type())
}

fn dual_i32(
    levels: &crate::domain::skill::dtos::skills::DualLevels<i32>,
    level: SkillLevel,
) -> (f32, SkillValueType) {
    (levels.at(level).0 as f32, levels.value_type())
}

fn get_rule_value(
    skill_state: &SkillState,
    player_job: PlayerJob,
    code: SkillCode,
    selector: ValueSelector,
    level: SkillLevel,
) -> Option<(f32, SkillValueType)> {
    let entry = skill_state.get_skill(player_job, code)?;
    match (code, selector, entry) {
        (
            SKILL_MELEE_MASTERY,
            ValueSelector::MeleeMasteryDamagePercent,
            SkillEntry::MeleeMastery(v),
        ) => Some(scalar_i32(&v.damage_percent, level)),
        (
            SKILL_SWORD_MASTERY,
            ValueSelector::SwordMasteryDamagePercent,
            SkillEntry::SwordMastery(v),
        ) => Some(scalar_i32(&v.damage_percent, level)),
        (
            SKILL_SHOOTING_MASTERY,
            ValueSelector::ShootingMasteryDamagePercent,
            SkillEntry::ShootingMastery(v),
        ) => Some(scalar_i32(&v.damage_percent, level)),
        (
            SKILL_THROWING_MASTERY,
            ValueSelector::ThrowingMasteryDamage,
            SkillEntry::ThrowingMastery(v),
        ) => Some(scalar_i32(&v.damage, level)),
        (SKILL_DIONS_EYE, ValueSelector::DionsEyeAttackRate, SkillEntry::DionsEye(v)) => {
            Some(scalar_i32(&v.attack_rate, level))
        }
        (
            SKILL_EVASION_MASTERY,
            ValueSelector::EvasionMasteryAddPercent,
            SkillEntry::EvasionMastery(v),
        ) => Some(scalar_i32(&v.add_percent, level)),
        (SKILL_D_MASTERY, ValueSelector::DMasteryDamagePercent, SkillEntry::DMastery(v)) => {
            Some(scalar_i32(&v.damage_percent, level))
        }
        (SKILL_A_MASTERY, ValueSelector::AMasteryAddPercent, SkillEntry::AMastery(v)) => {
            Some(scalar_i32(&v.add_percent, level))
        }
        (SKILL_A_MASTERY, ValueSelector::AMasteryAddPercent2, SkillEntry::AMastery(v)) => {
            Some(scalar_i32(&v.add_percent2, level))
        }
        (SKILL_F_MASTERY, ValueSelector::FMasteryCritical, SkillEntry::FMastery(v)) => {
            Some(scalar_i32(&v.critical, level))
        }
        (SKILL_MENTAL_MASTERY, ValueSelector::MentalMasteryMana, SkillEntry::MentalMastery(v)) => {
            Some(scalar_i32(&v.mana, level))
        }
        (SKILL_MEDITATION, ValueSelector::MeditationRegen, SkillEntry::Meditation(v)) => {
            Some(scalar_f32(&v.regen, level))
        }
        (
            SKILL_PHYSICAL_TRAINING,
            ValueSelector::PhysicalTrainingStamina,
            SkillEntry::PhysicalTraining(v),
        ) => Some(scalar_i32(&v.stamina, level)),
        (
            SKILL_WEAPON_DEFENSE_MASTERY,
            ValueSelector::WeaponDefenseMasteryBlock,
            SkillEntry::WeaponDefenseMastery(v),
        ) => Some(scalar_i32(&v.block, level)),
        (
            SKILL_CRITICAL_MASTERY,
            ValueSelector::CriticalMasteryCritical,
            SkillEntry::CriticalMastery(v),
        ) => Some(scalar_i32(&v.critical, level)),
        (
            SKILL_MECHANIC_WEAPON,
            ValueSelector::MechanicWeaponMastery,
            SkillEntry::MechanicWeaponMastery(v),
        ) => Some(scalar_i32(&v.mastery, level)),
        (
            SKILL_S_MASTERY,
            ValueSelector::SwordMasteryMartialDamage,
            SkillEntry::SwordMasteryMartial(v),
        ) => Some(scalar_i32(&v.damage, level)),
        (SKILL_S_MASTERY2, ValueSelector::SwordMastery2Stamina, SkillEntry::SwordMastery2(v)) => {
            Some(scalar_i32(&v.stamina, level))
        }
        (SKILL_S_MASTERY2, ValueSelector::SwordMastery2Rate, SkillEntry::SwordMastery2(v)) => {
            Some(scalar_i32(&v.rate, level))
        }
        (SKILL_BOOST_HEALTH, ValueSelector::BoostHealthLife, SkillEntry::BoostHealth(v)) => {
            Some(scalar_i32(&v.life, level))
        }
        (
            SKILL_EXTREME_SHIELD,
            ValueSelector::ExtremeShieldBlockRate,
            SkillEntry::ExtremeShield(v),
        ) => Some(scalar_i32(&v.block_rate, level)),
        (SKILL_PHYSICAL_ABSORB, ValueSelector::PhysicalAbsorb, SkillEntry::PhysicalAbsorb(v)) => {
            Some(dual_i32(&v.absorb, level))
        }
        (SKILL_SPARK_SHIELD, ValueSelector::SparkShieldDefense, SkillEntry::SparkShield(v)) => {
            Some(scalar_i32(&v.defense, level))
        }
        (SKILL_COMPULSION, ValueSelector::CompulsionAddAbsorb, SkillEntry::Compulsion(v)) => {
            Some(scalar_i32(&v.add_absorb, level))
        }
        (SKILL_HOLY_BODY, ValueSelector::HolyBodyAbsorb, SkillEntry::HolyBody(v)) => {
            Some(scalar_i32(&v.absorb, level))
        }
        (SKILL_HOLY_VALOR, ValueSelector::HolyValorDamage, SkillEntry::HolyValor(v)) => {
            Some(scalar_i32(&v.damage, level))
        }
        (
            SKILL_HOLY_INCANTATION,
            ValueSelector::HolyIncantationAddLife,
            SkillEntry::HolyIncantation(v),
        ) => Some(scalar_i32(&v.add_life, level)),
        (
            SKILL_GODLY_SHIELD,
            ValueSelector::GodlyShieldAbsorbPercent,
            SkillEntry::GodlyShield(v),
        ) => Some(scalar_i32(&v.absorb_percent, level)),
        (SKILL_GOD_BLESS, ValueSelector::GodBlessAddDamage, SkillEntry::GodBless(v)) => {
            Some(scalar_i32(&v.add_damage, level))
        }
        (
            SKILL_CONCENTRATION,
            ValueSelector::ConcentrationAttackRate,
            SkillEntry::Concentration(v),
        ) => Some(scalar_i32(&v.attack_rate, level)),
        (SKILL_SWIFT_AXE, ValueSelector::SwiftAxeSpeed, SkillEntry::SwiftAxe(v)) => {
            Some(scalar_i32(&v.speed, level))
        }
        (SKILL_BERSERKER, ValueSelector::BerserkerAddAttack, SkillEntry::Berserker(v)) => {
            Some(scalar_i32(&v.add_attack, level))
        }
        (SKILL_BERSERKER, ValueSelector::BerserkerSubAbsorb, SkillEntry::Berserker(v)) => {
            Some(scalar_i32(&v.sub_absorb, level))
        }
        (SKILL_B_BERSERKER, ValueSelector::BBerserkerAddAttack, SkillEntry::BBerserker(v)) => {
            Some(scalar_i32(&v.add_attack, level))
        }
        (SKILL_B_BERSERKER, ValueSelector::BBerserkerSubAbsorb, SkillEntry::BBerserker(v)) => {
            Some(scalar_i32(&v.sub_absorb, level))
        }
        (SKILL_WINDY, ValueSelector::WindyAttackRating, SkillEntry::Windy(v)) => {
            Some(scalar_i32(&v.attack_rating, level))
        }
        (SKILL_VENGEANCE, ValueSelector::VengeanceAttackRate, SkillEntry::Vengeance(v)) => {
            Some(scalar_i32(&v.attack_rate, level))
        }
        (SKILL_VENGEANCE, ValueSelector::VengeanceDamagePercent, SkillEntry::Vengeance(v)) => {
            Some(scalar_i32(&v.damage_percent, level))
        }
        (SKILL_VENGEANCE, ValueSelector::VengeanceAddCritical, SkillEntry::Vengeance(v)) => {
            Some(scalar_i32(&v.add_critical, level))
        }
        (SKILL_TALARIA, ValueSelector::TalariaSpeed, SkillEntry::Talaria(v)) => {
            Some(scalar_i32(&v.speed, level))
        }
        (SKILL_VANISH, ValueSelector::VanishSpeed, SkillEntry::Vanish(v)) => {
            Some(scalar_f32(&v.speed, level))
        }
        (SKILL_AMPLIFIED, ValueSelector::AmplifiedAddAttack, SkillEntry::Amplified(v)) => {
            Some(scalar_i32(&v.add_attack, level))
        }
        (SKILL_AMPLIFIED, ValueSelector::AmplifiedSubAbsorb, SkillEntry::Amplified(v)) => {
            Some(scalar_i32(&v.sub_absorb, level))
        }
        (SKILL_SS_ATTACK, ValueSelector::SsAttackRating, SkillEntry::SsAttack(v)) => {
            Some(scalar_i32(&v.rating, level))
        }
        (SKILL_SS_ATTACK, ValueSelector::SsAttackAbsorb, SkillEntry::SsAttack(v)) => {
            Some(scalar_i32(&v.absorb, level))
        }
        (SKILL_PERFECT_AIM, ValueSelector::PerfectAimAttackRate, SkillEntry::PerfectAim(v)) => {
            Some(scalar_i32(&v.attack_rate, level))
        }
        (SKILL_PERFECT_AIM, ValueSelector::PerfectAimDamage, SkillEntry::PerfectAim(v)) => {
            Some(scalar_i32(&v.damage, level))
        }
        (
            SKILL_FORCE_OF_NATURE,
            ValueSelector::ForceOfNatureAddDamage,
            SkillEntry::ForceOfNature(v),
        ) => Some(scalar_i32(&v.add_damage, level)),
        (
            SKILL_FORCE_OF_NATURE,
            ValueSelector::ForceOfNatureAddHit,
            SkillEntry::ForceOfNature(v),
        ) => Some(scalar_i32(&v.add_hit, level)),
        (
            SKILL_GOLDEN_FALCON,
            ValueSelector::GoldenFalconLifeRegen,
            SkillEntry::GoldenFalcon(v),
        ) => Some(scalar_f32(&v.life_regen, level)),
        (
            SKILL_PHOENIX_SHOT,
            ValueSelector::PhoenixShotDamagePercent,
            SkillEntry::PhoenixShot(v),
        ) => Some(scalar_i32(&v.damage_percent, level)),
        (SKILL_HOLY_MIND, ValueSelector::HolyMindDecDamage, SkillEntry::HolyMind(v)) => {
            Some(scalar_i32(&v.dec_damage, level))
        }
        (SKILL_VIRTUAL_LIFE, ValueSelector::VirtualLifePercent, SkillEntry::VirtualLife(v)) => {
            Some(scalar_i32(&v.percent, level))
        }
        (
            SKILL_REGENERATION_FIELD,
            ValueSelector::RegenerationFieldLifeRegen,
            SkillEntry::RegenerationField(v),
        ) => Some(scalar_f32(&v.life_regen, level)),
        (
            SKILL_REGENERATION_FIELD,
            ValueSelector::RegenerationFieldManaRegen,
            SkillEntry::RegenerationField(v),
        ) => Some(scalar_f32(&v.mana_regen, level)),
        (SKILL_KRISHNA, ValueSelector::KrishnaDefense, SkillEntry::Krishna(v)) => {
            Some(scalar_i32(&v.defense, level))
        }
        (SKILL_KRISHNA, ValueSelector::KrishnaAbs, SkillEntry::Krishna(v)) => {
            Some(scalar_i32(&v.absorb, level))
        }
        (
            SKILL_ENERGY_SHIELD,
            ValueSelector::EnergyShieldDecDamage,
            SkillEntry::EnergyShield(v),
        ) => Some(scalar_i32(&v.dec_damage, level)),
        (
            SKILL_SPIRIT_ELEMENTAL,
            ValueSelector::SpiritElementalRegenMana,
            SkillEntry::SpiritElemental(v),
        ) => Some(scalar_f32(&v.regen_mana, level)),
        (SKILL_TENUS, ValueSelector::TenusHp, SkillEntry::Tenus(v)) => {
            Some(scalar_i32(&v.hp, level))
        }
        (SKILL_TENUS, ValueSelector::TenusMana, SkillEntry::Tenus(v)) => {
            Some(scalar_i32(&v.damage, level))
        }
        (SKILL_ANIMA, ValueSelector::AnimaMana, SkillEntry::Anima(v)) => {
            Some(scalar_i32(&v.mana, level))
        }
        (SKILL_INPES, ValueSelector::InpesSpeed, SkillEntry::Inpes(v)) => {
            Some(scalar_i32(&v.speed, level))
        }
        (SKILL_RAGE_UP, ValueSelector::RageUpSpeed, SkillEntry::RageUp(v)) => {
            Some(scalar_i32(&v.speed, level))
        }
        (SKILL_I_BULKUP, ValueSelector::BulkupHp, SkillEntry::Bulkup(v)) => {
            Some(scalar_i32(&v.hp, level))
        }
        (SKILL_STEELERS, ValueSelector::SteelersDefense, SkillEntry::Steelers(v)) => {
            Some(scalar_i32(&v.defense, level))
        }
        (SKILL_PATRIOT, ValueSelector::PatriotCritical, SkillEntry::Patriot(v)) => {
            Some(scalar_i32(&v.critical, level))
        }
        (SKILL_B_CHECK, ValueSelector::CheckCritical, SkillEntry::Check(v)) => {
            Some(scalar_i32(&v.critical, level))
        }
        (SKILL_R_MAKER, ValueSelector::RainMakerAbsorb, SkillEntry::RainMaker(v)) => {
            Some(scalar_i32(&v.absorb, level))
        }
        (SKILL_A_MIDRANDA, ValueSelector::AdventMidrandaSpeed, SkillEntry::AdventMidranda(v)) => {
            Some(scalar_i32(&v.speed, level))
        }
        (
            SKILL_DISTORTION,
            ValueSelector::DistortionDamageSubPercent,
            SkillEntry::Distortion(v),
        ) => Some(scalar_i32(&v.damage_sub_percent, level)),
        (SKILL_DISTORTION, ValueSelector::DistortionSpeedSubPercent, SkillEntry::Distortion(v)) => {
            Some(scalar_i32(&v.speed_sub_percent, level))
        }
        (SKILL_H_REGENE, ValueSelector::HRegeneAddHp, SkillEntry::HRegene(v)) => {
            Some(scalar_i32(&v.add_hp, level))
        }
        (SKILL_D_MASTERY2, ValueSelector::DMastery2AddAttackRate, SkillEntry::DMastery2(v)) => {
            Some(scalar_i32(&v.add_attack_rate, level))
        }
        (SKILL_CREED, ValueSelector::CreedAddAttack, SkillEntry::Creed(v)) => {
            Some(scalar_i32(&v.add_attack, level))
        }
        (SKILL_CREED, ValueSelector::CreedAddAttackRate, SkillEntry::Creed(v)) => {
            Some(scalar_i32(&v.add_attack_rate, level))
        }
        (SKILL_H_TRAINING, ValueSelector::HTrainingAddAttack, SkillEntry::HTraining(v)) => {
            Some(scalar_i32(&v.add_attack, level))
        }
        (SKILL_H_TRAINING, ValueSelector::HTrainingAddAttackRate, SkillEntry::HTraining(v)) => {
            Some(scalar_i32(&v.add_attack_rate, level))
        }
        _ => None,
    }
}

fn percent_ratio(value: f32, value_type: SkillValueType) -> f32 {
    match value_type {
        SkillValueType::Permille => value / 100.0,
        SkillValueType::Fixed2 => value / 100.0,
    }
}

fn add_or_percent(base: &mut f32, value: f32, value_type: SkillValueType) {
    if value_type == SkillValueType::Permille {
        *base += *base * percent_ratio(value, value_type);
    } else {
        *base += value;
    }
}

fn apply_effect(
    stats: &mut CalculatedCharStats,
    effect: SkillEffect,
    value: f32,
    value_type: SkillValueType,
) {
    match effect {
        SkillEffect::AttackRatingFlat | SkillEffect::AttackRatingPercent => {
            add_or_percent(&mut stats.attack_rating, value, value_type);
        }
        SkillEffect::AttackDamageFlat | SkillEffect::AttackDamagePercent => {
            add_or_percent(&mut stats.attack_damage[0], value, value_type);
            add_or_percent(&mut stats.attack_damage[1], value, value_type);
        }
        SkillEffect::AttackSpeedFlat => add_or_percent(&mut stats.attack_speed, value, value_type),
        SkillEffect::DefenceFlat | SkillEffect::DefencePercent => {
            add_or_percent(&mut stats.defence, value, value_type)
        }
        SkillEffect::AbsorptionFlat | SkillEffect::AbsorptionPercent => {
            add_or_percent(&mut stats.absorption, value, value_type)
        }
        SkillEffect::ChanceBlockFlat => add_or_percent(&mut stats.chance_block, value, value_type),
        SkillEffect::CriticalFlat => add_or_percent(&mut stats.critical_hit, value, value_type),
        SkillEffect::MoveSpeedFlat | SkillEffect::MoveSpeedPercent => {
            add_or_percent(&mut stats.move_speed, value, value_type)
        }
        SkillEffect::LifeFlat | SkillEffect::LifePercent => {
            add_or_percent(&mut stats.life[1], value, value_type)
        }
        SkillEffect::ManaFlat => add_or_percent(&mut stats.mana[1], value, value_type),
        SkillEffect::StaminaFlat => add_or_percent(&mut stats.stamina[1], value, value_type),
        SkillEffect::LifeRegenFlat => add_or_percent(&mut stats.life_regen, value, value_type),
        SkillEffect::ManaRegenFlat => add_or_percent(&mut stats.mana_regen, value, value_type),
        SkillEffect::EvasionPercent => {
            if value_type == SkillValueType::Permille {
                stats.evade += 100.0 * percent_ratio(value, value_type);
            } else {
                stats.evade += value;
            }
        }
    }
}

const RULE_EMPTY: &[SkillEffectRule] = &[];

fn passive_rules_for(code: SkillCode) -> &'static [SkillEffectRule] {
    match code {
        SKILL_MELEE_MASTERY => &[SkillEffectRule {
            selector: ValueSelector::MeleeMasteryDamagePercent,
            effect: SkillEffect::AttackDamagePercent,
        }],
        SKILL_SWORD_MASTERY => &[SkillEffectRule {
            selector: ValueSelector::SwordMasteryDamagePercent,
            effect: SkillEffect::AttackDamagePercent,
        }],
        SKILL_SHOOTING_MASTERY => &[SkillEffectRule {
            selector: ValueSelector::ShootingMasteryDamagePercent,
            effect: SkillEffect::AttackDamagePercent,
        }],
        SKILL_THROWING_MASTERY => &[SkillEffectRule {
            selector: ValueSelector::ThrowingMasteryDamage,
            effect: SkillEffect::AttackDamageFlat,
        }],
        SKILL_DIONS_EYE => &[SkillEffectRule {
            selector: ValueSelector::DionsEyeAttackRate,
            effect: SkillEffect::AttackRatingFlat,
        }],
        SKILL_EVASION_MASTERY => &[SkillEffectRule {
            selector: ValueSelector::EvasionMasteryAddPercent,
            effect: SkillEffect::EvasionPercent,
        }],
        SKILL_D_MASTERY => &[SkillEffectRule {
            selector: ValueSelector::DMasteryDamagePercent,
            effect: SkillEffect::AttackDamagePercent,
        }],
        SKILL_A_MASTERY => &[
            SkillEffectRule {
                selector: ValueSelector::AMasteryAddPercent,
                effect: SkillEffect::AttackRatingPercent,
            },
            SkillEffectRule {
                selector: ValueSelector::AMasteryAddPercent2,
                effect: SkillEffect::AttackDamagePercent,
            },
        ],
        SKILL_F_MASTERY => &[SkillEffectRule {
            selector: ValueSelector::FMasteryCritical,
            effect: SkillEffect::CriticalFlat,
        }],
        SKILL_MENTAL_MASTERY => &[SkillEffectRule {
            selector: ValueSelector::MentalMasteryMana,
            effect: SkillEffect::ManaFlat,
        }],
        SKILL_MEDITATION => &[SkillEffectRule {
            selector: ValueSelector::MeditationRegen,
            effect: SkillEffect::ManaRegenFlat,
        }],
        SKILL_PHYSICAL_TRAINING => &[SkillEffectRule {
            selector: ValueSelector::PhysicalTrainingStamina,
            effect: SkillEffect::StaminaFlat,
        }],
        SKILL_WEAPON_DEFENSE_MASTERY => &[SkillEffectRule {
            selector: ValueSelector::WeaponDefenseMasteryBlock,
            effect: SkillEffect::ChanceBlockFlat,
        }],
        SKILL_CRITICAL_MASTERY => &[SkillEffectRule {
            selector: ValueSelector::CriticalMasteryCritical,
            effect: SkillEffect::CriticalFlat,
        }],
        SKILL_MECHANIC_WEAPON => &[SkillEffectRule {
            selector: ValueSelector::MechanicWeaponMastery,
            effect: SkillEffect::AttackDamageFlat,
        }],
        SKILL_S_MASTERY => &[SkillEffectRule {
            selector: ValueSelector::SwordMasteryMartialDamage,
            effect: SkillEffect::AttackDamageFlat,
        }],
        SKILL_S_MASTERY2 => &[
            SkillEffectRule {
                selector: ValueSelector::SwordMastery2Stamina,
                effect: SkillEffect::StaminaFlat,
            },
            SkillEffectRule {
                selector: ValueSelector::SwordMastery2Rate,
                effect: SkillEffect::AttackRatingFlat,
            },
        ],
        SKILL_BOOST_HEALTH => &[SkillEffectRule {
            selector: ValueSelector::BoostHealthLife,
            effect: SkillEffect::LifeFlat,
        }],
        SKILL_H_REGENE => &[SkillEffectRule {
            selector: ValueSelector::HRegeneAddHp,
            effect: SkillEffect::LifeFlat,
        }],
        SKILL_D_MASTERY2 => &[SkillEffectRule {
            selector: ValueSelector::DMastery2AddAttackRate,
            effect: SkillEffect::AttackRatingFlat,
        }],
        _ => RULE_EMPTY,
    }
}
fn active_rules_for(code: SkillCode) -> &'static [SkillEffectRule] {
    match code {
        SKILL_EXTREME_SHIELD => &[SkillEffectRule {
            selector: ValueSelector::ExtremeShieldBlockRate,
            effect: SkillEffect::ChanceBlockFlat,
        }],
        SKILL_PHYSICAL_ABSORB => &[SkillEffectRule {
            selector: ValueSelector::PhysicalAbsorb,
            effect: SkillEffect::AbsorptionFlat,
        }],
        SKILL_SPARK_SHIELD => &[SkillEffectRule {
            selector: ValueSelector::SparkShieldDefense,
            effect: SkillEffect::DefenceFlat,
        }],
        SKILL_COMPULSION => &[SkillEffectRule {
            selector: ValueSelector::CompulsionAddAbsorb,
            effect: SkillEffect::AbsorptionFlat,
        }],
        SKILL_HOLY_BODY => &[SkillEffectRule {
            selector: ValueSelector::HolyBodyAbsorb,
            effect: SkillEffect::AbsorptionFlat,
        }],
        SKILL_HOLY_VALOR => &[SkillEffectRule {
            selector: ValueSelector::HolyValorDamage,
            effect: SkillEffect::AttackDamageFlat,
        }],
        SKILL_HOLY_INCANTATION => &[SkillEffectRule {
            selector: ValueSelector::HolyIncantationAddLife,
            effect: SkillEffect::LifeFlat,
        }],
        SKILL_GODLY_SHIELD => &[SkillEffectRule {
            selector: ValueSelector::GodlyShieldAbsorbPercent,
            effect: SkillEffect::AbsorptionPercent,
        }],
        SKILL_GOD_BLESS => &[SkillEffectRule {
            selector: ValueSelector::GodBlessAddDamage,
            effect: SkillEffect::AttackDamageFlat,
        }],
        SKILL_CONCENTRATION => &[SkillEffectRule {
            selector: ValueSelector::ConcentrationAttackRate,
            effect: SkillEffect::AttackRatingFlat,
        }],
        SKILL_SWIFT_AXE => &[SkillEffectRule {
            selector: ValueSelector::SwiftAxeSpeed,
            effect: SkillEffect::AttackSpeedFlat,
        }],
        SKILL_BERSERKER => &[
            SkillEffectRule {
                selector: ValueSelector::BerserkerAddAttack,
                effect: SkillEffect::AttackDamageFlat,
            },
            SkillEffectRule {
                selector: ValueSelector::BerserkerSubAbsorb,
                effect: SkillEffect::AbsorptionPercent,
            },
        ],
        SKILL_B_BERSERKER => &[
            SkillEffectRule {
                selector: ValueSelector::BBerserkerAddAttack,
                effect: SkillEffect::AttackDamageFlat,
            },
            SkillEffectRule {
                selector: ValueSelector::BBerserkerSubAbsorb,
                effect: SkillEffect::AbsorptionPercent,
            },
        ],
        SKILL_WINDY => &[SkillEffectRule {
            selector: ValueSelector::WindyAttackRating,
            effect: SkillEffect::AttackRatingFlat,
        }],
        SKILL_VENGEANCE => &[
            SkillEffectRule {
                selector: ValueSelector::VengeanceAttackRate,
                effect: SkillEffect::AttackRatingFlat,
            },
            SkillEffectRule {
                selector: ValueSelector::VengeanceDamagePercent,
                effect: SkillEffect::AttackDamagePercent,
            },
            SkillEffectRule {
                selector: ValueSelector::VengeanceAddCritical,
                effect: SkillEffect::CriticalFlat,
            },
        ],
        SKILL_TALARIA => &[SkillEffectRule {
            selector: ValueSelector::TalariaSpeed,
            effect: SkillEffect::MoveSpeedFlat,
        }],
        SKILL_VANISH => &[SkillEffectRule {
            selector: ValueSelector::VanishSpeed,
            effect: SkillEffect::MoveSpeedFlat,
        }],
        SKILL_AMPLIFIED => &[
            SkillEffectRule {
                selector: ValueSelector::AmplifiedAddAttack,
                effect: SkillEffect::AttackDamageFlat,
            },
            SkillEffectRule {
                selector: ValueSelector::AmplifiedSubAbsorb,
                effect: SkillEffect::AbsorptionPercent,
            },
        ],
        SKILL_SS_ATTACK => &[
            SkillEffectRule {
                selector: ValueSelector::SsAttackRating,
                effect: SkillEffect::AttackRatingFlat,
            },
            SkillEffectRule {
                selector: ValueSelector::SsAttackAbsorb,
                effect: SkillEffect::AbsorptionFlat,
            },
        ],
        SKILL_PERFECT_AIM => &[
            SkillEffectRule {
                selector: ValueSelector::PerfectAimAttackRate,
                effect: SkillEffect::AttackRatingFlat,
            },
            SkillEffectRule {
                selector: ValueSelector::PerfectAimDamage,
                effect: SkillEffect::AttackDamagePercent,
            },
        ],
        SKILL_FORCE_OF_NATURE => &[
            SkillEffectRule {
                selector: ValueSelector::ForceOfNatureAddDamage,
                effect: SkillEffect::AttackDamageFlat,
            },
            SkillEffectRule {
                selector: ValueSelector::ForceOfNatureAddHit,
                effect: SkillEffect::AttackRatingFlat,
            },
        ],
        SKILL_GOLDEN_FALCON => &[SkillEffectRule {
            selector: ValueSelector::GoldenFalconLifeRegen,
            effect: SkillEffect::LifeRegenFlat,
        }],
        SKILL_PHOENIX_SHOT => &[SkillEffectRule {
            selector: ValueSelector::PhoenixShotDamagePercent,
            effect: SkillEffect::AttackDamagePercent,
        }],
        SKILL_HOLY_MIND => &[SkillEffectRule {
            selector: ValueSelector::HolyMindDecDamage,
            effect: SkillEffect::AbsorptionPercent,
        }],
        SKILL_VIRTUAL_LIFE => &[SkillEffectRule {
            selector: ValueSelector::VirtualLifePercent,
            effect: SkillEffect::LifePercent,
        }],
        SKILL_REGENERATION_FIELD => &[
            SkillEffectRule {
                selector: ValueSelector::RegenerationFieldLifeRegen,
                effect: SkillEffect::LifeRegenFlat,
            },
            SkillEffectRule {
                selector: ValueSelector::RegenerationFieldManaRegen,
                effect: SkillEffect::ManaRegenFlat,
            },
        ],
        SKILL_KRISHNA => &[
            SkillEffectRule {
                selector: ValueSelector::KrishnaDefense,
                effect: SkillEffect::DefenceFlat,
            },
            SkillEffectRule {
                selector: ValueSelector::KrishnaAbs,
                effect: SkillEffect::AbsorptionFlat,
            },
        ],
        SKILL_ENERGY_SHIELD => &[SkillEffectRule {
            selector: ValueSelector::EnergyShieldDecDamage,
            effect: SkillEffect::AbsorptionPercent,
        }],
        SKILL_SPIRIT_ELEMENTAL => &[SkillEffectRule {
            selector: ValueSelector::SpiritElementalRegenMana,
            effect: SkillEffect::ManaRegenFlat,
        }],
        SKILL_TENUS => &[
            SkillEffectRule {
                selector: ValueSelector::TenusHp,
                effect: SkillEffect::LifeFlat,
            },
            SkillEffectRule {
                selector: ValueSelector::TenusMana,
                effect: SkillEffect::ManaFlat,
            },
        ],
        SKILL_ANIMA => &[SkillEffectRule {
            selector: ValueSelector::AnimaMana,
            effect: SkillEffect::ManaFlat,
        }],
        SKILL_INPES => &[SkillEffectRule {
            selector: ValueSelector::InpesSpeed,
            effect: SkillEffect::MoveSpeedFlat,
        }],
        SKILL_RAGE_UP => &[SkillEffectRule {
            selector: ValueSelector::RageUpSpeed,
            effect: SkillEffect::MoveSpeedFlat,
        }],
        SKILL_I_BULKUP => &[SkillEffectRule {
            selector: ValueSelector::BulkupHp,
            effect: SkillEffect::LifeFlat,
        }],
        SKILL_STEELERS => &[SkillEffectRule {
            selector: ValueSelector::SteelersDefense,
            effect: SkillEffect::DefenceFlat,
        }],
        SKILL_PATRIOT => &[SkillEffectRule {
            selector: ValueSelector::PatriotCritical,
            effect: SkillEffect::CriticalFlat,
        }],
        SKILL_B_CHECK => &[SkillEffectRule {
            selector: ValueSelector::CheckCritical,
            effect: SkillEffect::CriticalFlat,
        }],
        SKILL_R_MAKER => &[SkillEffectRule {
            selector: ValueSelector::RainMakerAbsorb,
            effect: SkillEffect::AbsorptionFlat,
        }],
        SKILL_A_MIDRANDA => &[SkillEffectRule {
            selector: ValueSelector::AdventMidrandaSpeed,
            effect: SkillEffect::MoveSpeedFlat,
        }],
        SKILL_DISTORTION => &[
            SkillEffectRule {
                selector: ValueSelector::DistortionDamageSubPercent,
                effect: SkillEffect::DefencePercent,
            },
            SkillEffectRule {
                selector: ValueSelector::DistortionSpeedSubPercent,
                effect: SkillEffect::MoveSpeedPercent,
            },
        ],
        SKILL_CREED => &[
            SkillEffectRule {
                selector: ValueSelector::CreedAddAttack,
                effect: SkillEffect::AttackDamageFlat,
            },
            SkillEffectRule {
                selector: ValueSelector::CreedAddAttackRate,
                effect: SkillEffect::AttackRatingFlat,
            },
        ],
        SKILL_H_TRAINING => &[
            SkillEffectRule {
                selector: ValueSelector::HTrainingAddAttack,
                effect: SkillEffect::AttackDamageFlat,
            },
            SkillEffectRule {
                selector: ValueSelector::HTrainingAddAttackRate,
                effect: SkillEffect::AttackRatingFlat,
            },
        ],
        _ => RULE_EMPTY,
    }
}

fn apply_rules(
    code: SkillCode,
    rules: &[SkillEffectRule],
    skill_state: &SkillState,
    player_job: PlayerJob,
    level: SkillLevel,
    stats: &mut CalculatedCharStats,
) -> bool {
    let mut applied = false;
    for rule in rules {
        if let Some((value, value_type)) =
            get_rule_value(skill_state, player_job, code, rule.selector, level)
        {
            apply_effect(stats, rule.effect, value, value_type);
            applied = true;
        }
    }
    applied
}

fn player_job_group(job: PlayerJob) -> SkillGroup {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum GateReason {
    Allowed,
    JobTier,
    Weapon,
    Flag,
    PartyFlag,
    Element,
    PlusState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct GateResult {
    allowed: bool,
    reason: GateReason,
}

impl GateResult {
    fn allowed() -> Self {
        Self {
            allowed: true,
            reason: GateReason::Allowed,
        }
    }

    fn blocked(reason: GateReason) -> Self {
        Self {
            allowed: false,
            reason,
        }
    }
}

fn check_job_tier(
    code: SkillCode,
    player_job: PlayerJob,
    player_change_job: ChangeJobTierState,
) -> GateResult {
    let Some(group) = code.group_enum() else {
        return GateResult::blocked(GateReason::JobTier);
    };
    if group != SkillGroup::Other && group != player_job_group(player_job) {
        return GateResult::blocked(GateReason::JobTier);
    }
    let Some(tier) = code.tier_enum() else {
        return GateResult::blocked(GateReason::JobTier);
    };
    let required = match tier {
        crate::domain::skill::dtos::codes::ChangeJobTier::Tier1 => 1,
        crate::domain::skill::dtos::codes::ChangeJobTier::Tier2 => 2,
        crate::domain::skill::dtos::codes::ChangeJobTier::Tier3 => 3,
        crate::domain::skill::dtos::codes::ChangeJobTier::Tier4 => 4,
        crate::domain::skill::dtos::codes::ChangeJobTier::Tier5 => 5,
    };
    if required <= player_change_job.as_i32() as u32 {
        GateResult::allowed()
    } else {
        GateResult::blocked(GateReason::JobTier)
    }
}

fn check_active_job_tier(
    code: SkillCode,
    player_job: PlayerJob,
    player_change_job: ChangeJobTierState,
) -> GateResult {
    let Some(group) = code.group_enum() else {
        return GateResult::blocked(GateReason::JobTier);
    };

    if group != SkillGroup::Other
        && group != player_job_group(player_job)
        && skill_requires_party(player_job, code)
    {
        return GateResult::allowed();
    }

    check_job_tier(code, player_job, player_change_job)
}

fn check_weapon(
    use_weapon_codes: &[Option<EquippableItemType>; 8],
    equipped_weapon_code: Option<EquippableItemType>,
) -> GateResult {
    if use_weapon_codes.iter().all(|v| v.is_none()) {
        return GateResult::allowed();
    }
    let Some(weapon_code) = equipped_weapon_code else {
        return GateResult::blocked(GateReason::Weapon);
    };
    if use_weapon_codes
        .iter()
        .flatten()
        .any(|code| *code == weapon_code)
    {
        GateResult::allowed()
    } else {
        GateResult::blocked(GateReason::Weapon)
    }
}

fn skill_requires_party(player_job: PlayerJob, code: SkillCode) -> bool {
    if !is_party_skill(code) {
        return false;
    }

    match code.group_enum() {
        Some(SkillGroup::Other) | None => true,
        Some(group) => group != player_job_group(player_job),
    }
}

fn check_flag_rule(code: SkillCode, flag: i32) -> GateResult {
    let allowed = match skill_runtime_flag_rule(code) {
        SkillRuntimeFlagRule::AnyPositive => flag > 0,
        SkillRuntimeFlagRule::Exact(expected) => flag == expected,
        SkillRuntimeFlagRule::OneOf(values) => values.iter().any(|v| *v == flag),
    };
    if allowed {
        GateResult::allowed()
    } else {
        GateResult::blocked(GateReason::Flag)
    }
}

fn check_flag_party(
    player_job: PlayerJob,
    code: SkillCode,
    flag: i32,
    party_flag: bool,
) -> GateResult {
    let flag_check = check_flag_rule(code, flag);
    if !flag_check.allowed {
        return flag_check;
    }
    if skill_requires_party(player_job, code) && !party_flag {
        return GateResult::blocked(GateReason::PartyFlag);
    }
    GateResult::allowed()
}

fn check_element_optional(entry: Option<&SkillEntry>, element_index: Option<i32>) -> GateResult {
    let Some((primary, secondary, _)) = entry.and_then(SkillEntry::element) else {
        return GateResult::allowed();
    };

    let mut allowed_values = [None, None];
    if primary > 0 {
        allowed_values[0] = Some(primary);
    }
    if secondary > 0 && secondary != primary {
        allowed_values[1] = Some(secondary);
    }
    if allowed_values.iter().all(|v| v.is_none()) {
        return GateResult::allowed();
    }

    let Some(runtime_element) = element_index else {
        return GateResult::blocked(GateReason::Element);
    };
    if runtime_element < 0 {
        return GateResult::blocked(GateReason::Element);
    }

    if allowed_values
        .iter()
        .flatten()
        .any(|allowed| *allowed == runtime_element)
    {
        GateResult::allowed()
    } else {
        GateResult::blocked(GateReason::Element)
    }
}

fn check_plus_state(code: SkillCode, plus_state: [i32; 2]) -> GateResult {
    if plus_state[0] < 0 || plus_state[1] < 0 {
        return GateResult::blocked(GateReason::PlusState);
    }

    // Skills without configured active formula depend on fallback (+state damage/rating),
    // so a zeroed payload is considered invalid runtime signal for application.
    if active_rules_for(code).is_empty() && plus_state[0] == 0 && plus_state[1] == 0 {
        return GateResult::blocked(GateReason::PlusState);
    }

    GateResult::allowed()
}

fn check_all(results: &[GateResult]) -> GateResult {
    for result in results {
        if !result.allowed {
            return *result;
        }
    }
    GateResult::allowed()
}

fn active_gate(
    buff: &UserContinueSkill,
    skill_entry: Option<&SkillEntry>,
    player_job: PlayerJob,
    player_change_job: ChangeJobTierState,
    equipped_weapon_code: Option<EquippableItemType>,
) -> GateResult {
    check_all(&[
        check_active_job_tier(buff.code, player_job, player_change_job),
        check_weapon(&buff.use_weapon_codes, equipped_weapon_code),
        check_flag_party(player_job, buff.code, buff.flag, buff.party_flag),
        check_element_optional(skill_entry, Some(buff.element_index)),
        check_plus_state(buff.code, buff.plus_state),
    ])
}

fn passive_gate(
    passive: &UserPassiveSkill,
    _skill_entry: Option<&SkillEntry>,
    player_job: PlayerJob,
    player_change_job: ChangeJobTierState,
    equipped_weapon_code: Option<EquippableItemType>,
) -> GateResult {
    check_all(&[
        check_job_tier(passive.code, player_job, player_change_job),
        check_weapon(&passive.use_weapon_codes, equipped_weapon_code),
    ])
}

fn bump_gate_reason_counter(reason: GateReason, counters: &mut HashMap<GateReason, u32>) {
    if reason == GateReason::Allowed {
        return;
    }
    counters
        .entry(reason)
        .and_modify(|count| *count += 1)
        .or_insert(1);
}

pub fn apply_skill_stat_formulas(
    skill_state: Option<&SkillState>,
    player_job: PlayerJob,
    player_change_job: ChangeJobTierState,
    equipped_weapon_code: Option<EquippableItemType>,
    active_continue_skills: &[UserContinueSkill],
    passive_skills: &[UserPassiveSkill],
    stats: &mut CalculatedCharStats,
    debug_breakdown: &mut Vec<String>,
) {
    let mut configured_applied = 0_u32;
    let mut fallback_applied = 0_u32;
    let mut gated_blocked = 0_u32;
    let mut blocked_by_reason: HashMap<GateReason, u32> = HashMap::new();

    for buff in active_continue_skills {
        let skill_entry = skill_state.and_then(|state| state.get_skill(player_job, buff.code));
        let gate = active_gate(
            buff,
            skill_entry,
            player_job,
            player_change_job,
            equipped_weapon_code,
        );
        if !gate.allowed {
            gated_blocked += 1;
            bump_gate_reason_counter(gate.reason, &mut blocked_by_reason);
            continue;
        }

        let mut applied = false;
        if let (Some(state), Some(level)) = (skill_state, make_level(buff.point)) {
            applied = apply_rules(
                buff.code,
                active_rules_for(buff.code),
                state,
                player_job,
                level,
                stats,
            );
            if applied {
                configured_applied += 1;
            }
        }

        if !applied {
            if buff.plus_state[0] != 0 {
                stats.attack_damage[0] += buff.plus_state[0] as f32;
                stats.attack_damage[1] += buff.plus_state[0] as f32;
            }
            if buff.plus_state[1] != 0 {
                stats.attack_rating += buff.plus_state[1] as f32;
            }
            if buff.plus_state[0] != 0 || buff.plus_state[1] != 0 {
                fallback_applied += 1;
            }
        }
    }

    for passive in passive_skills {
        let skill_entry = skill_state.and_then(|state| state.get_skill(player_job, passive.code));
        let gate = passive_gate(
            passive,
            skill_entry,
            player_job,
            player_change_job,
            equipped_weapon_code,
        );
        if !gate.allowed {
            gated_blocked += 1;
            bump_gate_reason_counter(gate.reason, &mut blocked_by_reason);
            continue;
        }

        let mut applied = false;
        if let (Some(state), Some(level)) = (skill_state, make_level(passive.point)) {
            applied = apply_rules(
                passive.code,
                passive_rules_for(passive.code),
                state,
                player_job,
                level,
                stats,
            );
            if applied {
                configured_applied += 1;
            }
        }

        if !applied && passive.point != 0 {
            stats.attack_rating += passive.point as f32;
            fallback_applied += 1;
        }
    }

    debug_breakdown.push(format!(
        "skill_formula_configured_applied={}",
        configured_applied
    ));
    debug_breakdown.push(format!(
        "skill_formula_fallback_applied={}",
        fallback_applied
    ));
    debug_breakdown.push(format!("skill_formula_gated_blocked={}", gated_blocked));
    debug_breakdown.push(format!(
        "skill_formula_gate_blocked_job_tier={}",
        blocked_by_reason
            .get(&GateReason::JobTier)
            .copied()
            .unwrap_or_default()
    ));
    debug_breakdown.push(format!(
        "skill_formula_gate_blocked_weapon={}",
        blocked_by_reason
            .get(&GateReason::Weapon)
            .copied()
            .unwrap_or_default()
    ));
    debug_breakdown.push(format!(
        "skill_formula_gate_blocked_flag={}",
        blocked_by_reason
            .get(&GateReason::Flag)
            .copied()
            .unwrap_or_default()
    ));
    debug_breakdown.push(format!(
        "skill_formula_gate_blocked_party={}",
        blocked_by_reason
            .get(&GateReason::PartyFlag)
            .copied()
            .unwrap_or_default()
    ));
    debug_breakdown.push(format!(
        "skill_formula_gate_blocked_element={}",
        blocked_by_reason
            .get(&GateReason::Element)
            .copied()
            .unwrap_or_default()
    ));
    debug_breakdown.push(format!(
        "skill_formula_gate_blocked_plus_state={}",
        blocked_by_reason
            .get(&GateReason::PlusState)
            .copied()
            .unwrap_or_default()
    ));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::item::dto::EquippableItemType;
    use crate::domain::skill::dtos::codes::{
        SKILL_CONCENTRATION, SKILL_HOLY_INCANTATION, SKILL_MELEE_MASTERY,
        SKILL_TRIUMPH_OF_VALHALLA, SKILL_VIRTUAL_LIFE,
    };
    use crate::domain::skill::dtos::skills::{
        Concentration, Levels, MeleeMastery, SkillClassConfig, SkillEntry, SkillValueType,
    };
    use std::collections::HashMap;

    fn mk_levels_i32(v: i32) -> Levels<i32> {
        Levels::new(SkillValueType::Fixed2, [v; 10])
    }

    fn mk_concentration_entry(element: Option<(i32, i32, i32)>) -> SkillEntry {
        SkillEntry::Concentration(Concentration {
            name: "Concentration".to_string(),
            description: "test".to_string(),
            require_level: 1,
            use_stamina: None,
            require_mastery: None,
            element,
            use_weapon_code: vec![],
            skill_code: SKILL_CONCENTRATION,
            attack_rate: mk_levels_i32(100),
            time: mk_levels_i32(1),
            use_mana: mk_levels_i32(1),
        })
    }

    fn mk_melee_mastery_entry() -> SkillEntry {
        SkillEntry::MeleeMastery(MeleeMastery {
            name: "MeleeMastery".to_string(),
            description: "test".to_string(),
            require_level: 1,
            use_stamina: None,
            require_mastery: None,
            element: None,
            use_weapon_code: vec![],
            skill_code: SKILL_MELEE_MASTERY,
            damage_percent: mk_levels_i32(10),
        })
    }

    fn mk_state(entries: Vec<(SkillCode, SkillEntry)>) -> SkillState {
        let by_code = entries.into_iter().collect::<HashMap<_, _>>();
        SkillState {
            state: HashMap::from([(PlayerJob::Fighter, SkillClassConfig { by_code })]),
        }
    }

    fn mk_active(code: SkillCode) -> UserContinueSkill {
        UserContinueSkill {
            code,
            point: 1,
            flag: 1,
            party_flag: true,
            element_index: 1,
            plus_state: [1, 1],
            use_weapon_codes: [None; 8],
        }
    }

    fn debug_value(debug: &[String], key: &str) -> Option<i32> {
        debug
            .iter()
            .find_map(|line| line.strip_prefix(&format!("{key}=")))
            .and_then(|v| v.parse::<i32>().ok())
    }

    #[test]
    fn gate_blocks_job_tier() {
        let mut stats = CalculatedCharStats::default();
        let mut debug = Vec::new();
        apply_skill_stat_formulas(
            None,
            PlayerJob::Archer,
            ChangeJobTierState::Tier5,
            None,
            &[mk_active(SKILL_CONCENTRATION)],
            &[],
            &mut stats,
            &mut debug,
        );
        assert_eq!(
            debug_value(&debug, "skill_formula_gate_blocked_job_tier"),
            Some(1)
        );
    }

    #[test]
    fn gate_blocks_weapon() {
        let mut stats = CalculatedCharStats::default();
        let mut debug = Vec::new();
        let mut buff = mk_active(SKILL_CONCENTRATION);
        buff.use_weapon_codes = [
            Some(EquippableItemType::WA1),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ];
        apply_skill_stat_formulas(
            None,
            PlayerJob::Fighter,
            ChangeJobTierState::Tier5,
            Some(EquippableItemType::WS1),
            &[buff],
            &[],
            &mut stats,
            &mut debug,
        );
        assert_eq!(
            debug_value(&debug, "skill_formula_gate_blocked_weapon"),
            Some(1)
        );
    }

    #[test]
    fn gate_blocks_flag() {
        let mut stats = CalculatedCharStats::default();
        let mut debug = Vec::new();
        let mut buff = mk_active(SKILL_CONCENTRATION);
        buff.flag = 2; // concentration expects exact active-state flag.
        apply_skill_stat_formulas(
            None,
            PlayerJob::Fighter,
            ChangeJobTierState::Tier5,
            None,
            &[buff],
            &[],
            &mut stats,
            &mut debug,
        );
        assert_eq!(
            debug_value(&debug, "skill_formula_gate_blocked_flag"),
            Some(1)
        );
    }

    #[test]
    fn gate_blocks_party_flag() {
        let mut stats = CalculatedCharStats::default();
        let mut debug = Vec::new();
        let mut buff = mk_active(SKILL_HOLY_INCANTATION);
        buff.party_flag = false;
        apply_skill_stat_formulas(
            None,
            PlayerJob::Fighter,
            ChangeJobTierState::Tier5,
            None,
            &[buff],
            &[],
            &mut stats,
            &mut debug,
        );
        assert_eq!(
            debug_value(&debug, "skill_formula_gate_blocked_party"),
            Some(1)
        );
    }

    #[test]
    fn gate_allows_party_skill_for_owner_job_without_party_flag() {
        let mut stats = CalculatedCharStats::default();
        let mut debug = Vec::new();
        let mut buff = mk_active(SKILL_TRIUMPH_OF_VALHALLA);
        buff.party_flag = false;
        apply_skill_stat_formulas(
            None,
            PlayerJob::Atalanta,
            ChangeJobTierState::Tier5,
            None,
            &[buff],
            &[],
            &mut stats,
            &mut debug,
        );
        assert_eq!(
            debug_value(&debug, "skill_formula_gate_blocked_party"),
            Some(0)
        );
        assert_eq!(
            debug_value(&debug, "skill_formula_fallback_applied"),
            Some(1)
        );
    }

    #[test]
    fn gate_blocks_party_skill_for_foreign_job_without_party_flag() {
        let mut stats = CalculatedCharStats::default();
        let mut debug = Vec::new();
        let mut buff = mk_active(SKILL_TRIUMPH_OF_VALHALLA);
        buff.party_flag = false;
        apply_skill_stat_formulas(
            None,
            PlayerJob::Pikeman,
            ChangeJobTierState::Tier5,
            None,
            &[buff],
            &[],
            &mut stats,
            &mut debug,
        );
        assert_eq!(
            debug_value(&debug, "skill_formula_gate_blocked_party"),
            Some(1)
        );
    }

    #[test]
    fn gate_allows_party_skill_for_foreign_job_with_party_flag() {
        let mut stats = CalculatedCharStats::default();
        let mut debug = Vec::new();
        let mut buff = mk_active(SKILL_TRIUMPH_OF_VALHALLA);
        buff.party_flag = true;
        apply_skill_stat_formulas(
            None,
            PlayerJob::Pikeman,
            ChangeJobTierState::Tier5,
            None,
            &[buff],
            &[],
            &mut stats,
            &mut debug,
        );
        assert_eq!(
            debug_value(&debug, "skill_formula_gate_blocked_party"),
            Some(0)
        );
        assert_eq!(
            debug_value(&debug, "skill_formula_gate_blocked_job_tier"),
            Some(0)
        );
        assert_eq!(
            debug_value(&debug, "skill_formula_fallback_applied"),
            Some(1)
        );
    }

    #[test]
    fn gate_blocks_element_mismatch() {
        let state = mk_state(vec![(
            SKILL_CONCENTRATION,
            mk_concentration_entry(Some((1, 0, 0))),
        )]);
        let mut stats = CalculatedCharStats::default();
        let mut debug = Vec::new();
        let mut buff = mk_active(SKILL_CONCENTRATION);
        buff.element_index = 2;
        apply_skill_stat_formulas(
            Some(&state),
            PlayerJob::Fighter,
            ChangeJobTierState::Tier5,
            None,
            &[buff],
            &[],
            &mut stats,
            &mut debug,
        );
        assert_eq!(
            debug_value(&debug, "skill_formula_gate_blocked_element"),
            Some(1)
        );
    }

    #[test]
    fn gate_blocks_plus_state_dependency_for_unconfigured_active() {
        let mut stats = CalculatedCharStats::default();
        let mut debug = Vec::new();
        let mut buff = mk_active(SKILL_MELEE_MASTERY); // no active formula configured
        buff.plus_state = [0, 0];
        apply_skill_stat_formulas(
            None,
            PlayerJob::Fighter,
            ChangeJobTierState::Tier5,
            None,
            &[buff],
            &[],
            &mut stats,
            &mut debug,
        );
        assert_eq!(
            debug_value(&debug, "skill_formula_gate_blocked_plus_state"),
            Some(1)
        );
    }

    #[test]
    fn gate_allows_and_applies_configured_rule() {
        let state = mk_state(vec![
            (SKILL_CONCENTRATION, mk_concentration_entry(None)),
            (SKILL_MELEE_MASTERY, mk_melee_mastery_entry()),
        ]);
        let mut stats = CalculatedCharStats::default();
        let mut debug = Vec::new();
        apply_skill_stat_formulas(
            Some(&state),
            PlayerJob::Fighter,
            ChangeJobTierState::Tier5,
            None,
            &[mk_active(SKILL_CONCENTRATION)],
            &[],
            &mut stats,
            &mut debug,
        );
        assert!(stats.attack_rating > 0.0);
        assert_eq!(
            debug_value(&debug, "skill_formula_configured_applied"),
            Some(1)
        );
        assert_eq!(debug_value(&debug, "skill_formula_gated_blocked"), Some(0));
    }
}
