use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::domain::{item::dto::EquippableItemType, skill::dtos::codes::SkillCode};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SkillLevel(u8);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Skills {
    Pikeman(PikemanSkills),
    Mechanician(MechanicianSkills),
    Fighter(FighterSkills),
    Archer(ArcherSkills),
    Knight(KnightSkills),
    Atalanta(AtalantaSkills),
    Priestess(PriestessSkills),
    Magician(MagicianSkills),
    Assassin(AssassinSkills),
    Shaman(ShamanSkills),
    Martial(MartialSkills),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SkillClassConfig {
    pub by_code: HashMap<SkillCode, SkillEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SkillEntry {
    PikeWind(PikeWind),
    IceAttribute(IceAttribute),
    CriticalHit(CriticalHit),
    JumpingCrash(JumpingCrash),
    GroundPike(GroundPike),
    Tornado(Tornado),
    WeaponDefenseMastery(WeaponDefenseMastery),
    Expansion(Expansion),
    VenomSpear(VenomSpear),
    Vanish(Vanish),
    CriticalMastery(CriticalMastery),
    ChainLance(ChainLance),
    AssassinEye(AssassinEye),
    ChargingStrike(ChargingStrike),
    Vague(Vague),
    ShadowMaster(ShadowMaster),
    DReaper(DReaper),
    FSpear(FSpear),
    SsAttack(SsAttack),
    Amplified(Amplified),
    ExtremeShield(ExtremeShield),
    MechanicBomb(MechanicBomb),
    PhysicalAbsorb(PhysicalAbsorb),
    PoisonAttribute(PoisonAttribute),
    GreatSmash(GreatSmash),
    Maximize(Maximize),
    Automation(Automation),
    Spark(Spark),
    MetalArmor(MetalArmor),
    GrandSmash(GrandSmash),
    MechanicWeaponMastery(MechanicWeaponMastery),
    SparkShield(SparkShield),
    Impulsion(Impulsion),
    Compulsion(Compulsion),
    MagneticSphere(MagneticSphere),
    MetalGolem(MetalGolem),
    LandMine(LandMine),
    HyperSonic(HyperSonic),
    RSmash(RSmash),
    PhysicalEnhance(PhysicalEnhance),
    MeleeMastery(MeleeMastery),
    FireAttribute(FireAttribute),
    Raving(Raving),
    Impact(Impact),
    TripleImpact(TripleImpact),
    BrutalSwing(BrutalSwing),
    Roar(Roar),
    RageOfZecram(RageOfZecram),
    Concentration(Concentration),
    AvangingCrash(AvangingCrash),
    SwiftAxe(SwiftAxe),
    BoneCrash(BoneCrash),
    Destoryer(Destoryer),
    Berserker(Berserker),
    CycloneStrike(CycloneStrike),
    BoostHealth(BoostHealth),
    DHit(DHit),
    PDash(PDash),
    MBlow(MBlow),
    BBerserker(BBerserker),
    ScoutHawk(ScoutHawk),
    ShootingMastery(ShootingMastery),
    WindArrow(WindArrow),
    PerfectAim(PerfectAim),
    DionsEye(DionsEye),
    Falcon(Falcon),
    ArrowOfRage(ArrowOfRage),
    Avalanche(Avalanche),
    ElementalShot(ElementalShot),
    GoldenFalcon(GoldenFalcon),
    BombShot(BombShot),
    Perforation(Perforation),
    RecallWolverin(RecallWolverin),
    EvasionMastery(EvasionMastery),
    PhoenixShot(PhoenixShot),
    ForceOfNature(ForceOfNature),
    EShot(EShot),
    SRope(SRope),
    NSplash(NSplash),
    CTrap(CTrap),
    SwordBlast(SwordBlast),
    HolyBody(HolyBody),
    PhysicalTraining(PhysicalTraining),
    DoubleCrash(DoubleCrash),
    HolyValor(HolyValor),
    Brandish(Brandish),
    Piercing(Piercing),
    DrasticSpirit(DrasticSpirit),
    SwordMastery(SwordMastery),
    DivineInhalation(DivineInhalation),
    HolyIncantation(HolyIncantation),
    GrandCross(GrandCross),
    DivinePiercing(DivinePiercing),
    GodlyShield(GodlyShield),
    GodBless(GodBless),
    SwordOfJustice(SwordOfJustice),
    SBreaker(SBreaker),
    CMoon(CMoon),
    SBlade(SBlade),
    HBenedic(HBenedic),
    ShieldStrike(ShieldStrike),
    Farina(Farina),
    ThrowingMastery(ThrowingMastery),
    VigorSpear(VigorSpear),
    Windy(Windy),
    TwistJavelin(TwistJavelin),
    SoulSucker(SoulSucker),
    FireJavelin(FireJavelin),
    SplitJavelin(SplitJavelin),
    TriumphOfValhalla(TriumphOfValhalla),
    LightningJavelin(LightningJavelin),
    StormJavelin(StormJavelin),
    HallOfValhalla(HallOfValhalla),
    XRage(XRage),
    FrostJavelin(FrostJavelin),
    Vengeance(Vengeance),
    Talaria(Talaria),
    GCoup(GCoup),
    Arcuda(Arcuda),
    SFear(SFear),
    Healing(Healing),
    HolyBolt(HolyBolt),
    MultiSpark(MultiSpark),
    HolyMind(HolyMind),
    Meditation(Meditation),
    DivineLightning(DivineLightning),
    HolyReflection(HolyReflection),
    GrandHealing(GrandHealing),
    VigorBall(VigorBall),
    Resurrection(Resurrection),
    Extinction(Extinction),
    VirtualLife(VirtualLife),
    GlacialSpike(GlacialSpike),
    RegenerationField(RegenerationField),
    ChainLightning(ChainLightning),
    SummonMuspell(SummonMuspell),
    SImpact(SImpact),
    PIce(PIce),
    SummonRamiel(SummonRamiel),
    Krishna(Krishna),
    Agony(Agony),
    FireBolt(FireBolt),
    Zenith(Zenith),
    FireBall(FireBall),
    MentalMastery(MentalMastery),
    Watornado(Watornado),
    EnchantWeapon(EnchantWeapon),
    DeadRay(DeadRay),
    EnergyShield(EnergyShield),
    Diastrophism(Diastrophism),
    SpiritElemental(SpiritElemental),
    DancingSword(DancingSword),
    FireElemental(FireElemental),
    FlameWave(FlameWave),
    Distortion(Distortion),
    Meteo(Meteo),
    Silraphim(Silraphim),
    Tenus(Tenus),
    Ignis(Ignis),
    Anima(Anima),
    Stinger(Stinger),
    DoubleBlow(DoubleBlow),
    DMastery(DMastery),
    Wisp(Wisp),
    VenomThorn(VenomThorn),
    Alas(Alas),
    SoulShock(SoulShock),
    AMastery(AMastery),
    SoreSword(SoreSword),
    BeatUp(BeatUp),
    Inpes(Inpes),
    Blind(Blind),
    FrostWind(FrostWind),
    FMastery(FMastery),
    Polluted(Polluted),
    PastingShadow(PastingShadow),
    DarkBolt(DarkBolt),
    InnerPeace(InnerPeace),
    SoulManacle(SoulManacle),
    Haunt(Haunt),
    Scratch(Scratch),
    Judgement(Judgement),
    DarkWave(DarkWave),
    CurseLazy(CurseLazy),
    SpiritualFlare(SpiritualFlare),
    ChasingHunt(ChasingHunt),
    LandGhost(LandGhost),
    MourningOfPray(MourningOfPray),
    BloodyKnight(BloodyKnight),
    AdventMigal(AdventMigal),
    RainMaker(RainMaker),
    AdventMidranda(AdventMidranda),
    LowKick(LowKick),
    SwordMasteryMartial(SwordMasteryMartial),
    DoubleBlowMartial(DoubleBlowMartial),
    Straight(Straight),
    RageUp(RageUp),
    Patriot(Patriot),
    Elbow(Elbow),
    SwordMastery2(SwordMastery2),
    Bulkup(Bulkup),
    Warcry(Warcry),
    Cannon(Cannon),
    HeelKick(HeelKick),
    Combination(Combination),
    Steelers(Steelers),
    Check(Check),
    Typhoon(Typhoon),
    JBomb(JBomb),
    RSlash(RSlash),
    VStab(VStab),
    Storm(Storm),
    Creed(Creed),
    PDeity(PDeity),
    GNail(GNail),
    HRegene(HRegene),
    DMastery2(DMastery2),
    HHawk(HHawk),
    LBreaking(LBreaking),
    HTraining(HTraining),
}

impl SkillEntry {
    pub fn require_level(&self) -> u8 {
        match self {
            SkillEntry::PikeWind(v) => v.require_level,
            SkillEntry::IceAttribute(v) => v.require_level,
            SkillEntry::CriticalHit(v) => v.require_level,
            SkillEntry::JumpingCrash(v) => v.require_level,
            SkillEntry::GroundPike(v) => v.require_level,
            SkillEntry::Tornado(v) => v.require_level,
            SkillEntry::WeaponDefenseMastery(v) => v.require_level,
            SkillEntry::Expansion(v) => v.require_level,
            SkillEntry::VenomSpear(v) => v.require_level,
            SkillEntry::Vanish(v) => v.require_level,
            SkillEntry::CriticalMastery(v) => v.require_level,
            SkillEntry::ChainLance(v) => v.require_level,
            SkillEntry::AssassinEye(v) => v.require_level,
            SkillEntry::ChargingStrike(v) => v.require_level,
            SkillEntry::Vague(v) => v.require_level,
            SkillEntry::ShadowMaster(v) => v.require_level,
            SkillEntry::DReaper(v) => v.require_level,
            SkillEntry::FSpear(v) => v.require_level,
            SkillEntry::SsAttack(v) => v.require_level,
            SkillEntry::ExtremeShield(v) => v.require_level,
            SkillEntry::MechanicBomb(v) => v.require_level,
            SkillEntry::PhysicalAbsorb(v) => v.require_level,
            SkillEntry::PoisonAttribute(v) => v.require_level,
            SkillEntry::GreatSmash(v) => v.require_level,
            SkillEntry::Maximize(v) => v.require_level,
            SkillEntry::Automation(v) => v.require_level,
            SkillEntry::Spark(v) => v.require_level,
            SkillEntry::MetalArmor(v) => v.require_level,
            SkillEntry::GrandSmash(v) => v.require_level,
            SkillEntry::MechanicWeaponMastery(v) => v.require_level,
            SkillEntry::SparkShield(v) => v.require_level,
            SkillEntry::Impulsion(v) => v.require_level,
            SkillEntry::Compulsion(v) => v.require_level,
            SkillEntry::MagneticSphere(v) => v.require_level,
            SkillEntry::MetalGolem(v) => v.require_level,
            SkillEntry::LandMine(v) => v.require_level,
            SkillEntry::HyperSonic(v) => v.require_level,
            SkillEntry::RSmash(v) => v.require_level,
            SkillEntry::PhysicalEnhance(v) => v.require_level,
            SkillEntry::MeleeMastery(v) => v.require_level,
            SkillEntry::FireAttribute(v) => v.require_level,
            SkillEntry::Raving(v) => v.require_level,
            SkillEntry::Impact(v) => v.require_level,
            SkillEntry::TripleImpact(v) => v.require_level,
            SkillEntry::BrutalSwing(v) => v.require_level,
            SkillEntry::Roar(v) => v.require_level,
            SkillEntry::RageOfZecram(v) => v.require_level,
            SkillEntry::Concentration(v) => v.require_level,
            SkillEntry::AvangingCrash(v) => v.require_level,
            SkillEntry::SwiftAxe(v) => v.require_level,
            SkillEntry::BoneCrash(v) => v.require_level,
            SkillEntry::Destoryer(v) => v.require_level,
            SkillEntry::Berserker(v) => v.require_level,
            SkillEntry::CycloneStrike(_) => 0,
            SkillEntry::BoostHealth(v) => v.require_level,
            SkillEntry::DHit(v) => v.require_level,
            SkillEntry::PDash(v) => v.require_level,
            SkillEntry::MBlow(v) => v.require_level,
            SkillEntry::BBerserker(v) => v.require_level,
            SkillEntry::ScoutHawk(v) => v.require_level,
            SkillEntry::ShootingMastery(v) => v.require_level,
            SkillEntry::WindArrow(v) => v.require_level,
            SkillEntry::PerfectAim(v) => v.require_level,
            SkillEntry::DionsEye(v) => v.require_level,
            SkillEntry::Falcon(v) => v.require_level,
            SkillEntry::ArrowOfRage(v) => v.require_level,
            SkillEntry::Avalanche(v) => v.require_level,
            SkillEntry::ElementalShot(v) => v.require_level,
            SkillEntry::GoldenFalcon(v) => v.require_level,
            SkillEntry::BombShot(v) => v.require_level,
            SkillEntry::Perforation(v) => v.require_level,
            SkillEntry::RecallWolverin(v) => v.require_level,
            SkillEntry::EvasionMastery(v) => v.require_level,
            SkillEntry::PhoenixShot(v) => v.require_level,
            SkillEntry::ForceOfNature(v) => v.require_level,
            SkillEntry::EShot(v) => v.require_level,
            SkillEntry::SRope(v) => v.require_level,
            SkillEntry::NSplash(v) => v.require_level,
            SkillEntry::CTrap(v) => v.require_level,
            SkillEntry::SwordBlast(v) => v.require_level,
            SkillEntry::HolyBody(v) => v.require_level,
            SkillEntry::PhysicalTraining(v) => v.require_level,
            SkillEntry::DoubleCrash(v) => v.require_level,
            SkillEntry::HolyValor(v) => v.require_level,
            SkillEntry::Brandish(v) => v.require_level,
            SkillEntry::Piercing(v) => v.require_level,
            SkillEntry::DrasticSpirit(v) => v.require_level,
            SkillEntry::SwordMastery(v) => v.require_level,
            SkillEntry::DivineInhalation(v) => v.require_level,
            SkillEntry::HolyIncantation(v) => v.require_level,
            SkillEntry::GrandCross(v) => v.require_level,
            SkillEntry::DivinePiercing(v) => v.require_level,
            SkillEntry::GodlyShield(v) => v.require_level,
            SkillEntry::GodBless(v) => v.require_level,
            SkillEntry::SwordOfJustice(v) => v.require_level,
            SkillEntry::SBreaker(v) => v.require_level,
            SkillEntry::CMoon(v) => v.require_level,
            SkillEntry::SBlade(v) => v.require_level,
            SkillEntry::HBenedic(v) => v.require_level,
            SkillEntry::ShieldStrike(v) => v.require_level,
            SkillEntry::Farina(v) => v.require_level,
            SkillEntry::ThrowingMastery(v) => v.require_level,
            SkillEntry::VigorSpear(v) => v.require_level,
            SkillEntry::Windy(v) => v.require_level,
            SkillEntry::TwistJavelin(v) => v.require_level,
            SkillEntry::SoulSucker(v) => v.require_level,
            SkillEntry::FireJavelin(v) => v.require_level,
            SkillEntry::SplitJavelin(v) => v.require_level,
            SkillEntry::TriumphOfValhalla(v) => v.require_level,
            SkillEntry::LightningJavelin(v) => v.require_level,
            SkillEntry::StormJavelin(v) => v.require_level,
            SkillEntry::HallOfValhalla(v) => v.require_level,
            SkillEntry::XRage(v) => v.require_level,
            SkillEntry::FrostJavelin(v) => v.require_level,
            SkillEntry::Vengeance(v) => v.require_level,
            SkillEntry::Talaria(v) => v.require_level,
            SkillEntry::GCoup(v) => v.require_level,
            SkillEntry::Arcuda(v) => v.require_level,
            SkillEntry::SFear(v) => v.require_level,
            SkillEntry::Healing(v) => v.require_level,
            SkillEntry::HolyBolt(v) => v.require_level,
            SkillEntry::MultiSpark(v) => v.require_level,
            SkillEntry::HolyMind(v) => v.require_level,
            SkillEntry::Meditation(v) => v.require_level,
            SkillEntry::DivineLightning(v) => v.require_level,
            SkillEntry::HolyReflection(v) => v.require_level,
            SkillEntry::GrandHealing(v) => v.require_level,
            SkillEntry::VigorBall(v) => v.require_level,
            SkillEntry::Resurrection(v) => v.require_level,
            SkillEntry::Extinction(v) => v.require_level,
            SkillEntry::VirtualLife(v) => v.require_level,
            SkillEntry::GlacialSpike(v) => v.require_level,
            SkillEntry::RegenerationField(v) => v.require_level,
            SkillEntry::ChainLightning(v) => v.require_level,
            SkillEntry::SummonMuspell(v) => v.require_level,
            SkillEntry::SImpact(v) => v.require_level,
            SkillEntry::PIce(v) => v.require_level,
            SkillEntry::SummonRamiel(v) => v.require_level,
            SkillEntry::Krishna(v) => v.require_level,
            SkillEntry::Agony(v) => v.require_level,
            SkillEntry::FireBolt(v) => v.require_level,
            SkillEntry::Zenith(v) => v.require_level,
            SkillEntry::FireBall(v) => v.require_level,
            SkillEntry::MentalMastery(v) => v.require_level,
            SkillEntry::Watornado(v) => v.require_level,
            SkillEntry::EnchantWeapon(v) => v.require_level,
            SkillEntry::DeadRay(v) => v.require_level,
            SkillEntry::EnergyShield(v) => v.require_level,
            SkillEntry::Diastrophism(v) => v.require_level,
            SkillEntry::SpiritElemental(v) => v.require_level,
            SkillEntry::DancingSword(v) => v.require_level,
            SkillEntry::FireElemental(v) => v.require_level,
            SkillEntry::FlameWave(v) => v.require_level,
            SkillEntry::Distortion(v) => v.require_level,
            SkillEntry::Meteo(v) => v.require_level,
            SkillEntry::Silraphim(v) => v.require_level,
            SkillEntry::Tenus(v) => v.require_level,
            SkillEntry::Ignis(v) => v.require_level,
            SkillEntry::Anima(v) => v.require_level,
            SkillEntry::Stinger(v) => v.require_level,
            SkillEntry::DoubleBlow(v) => v.require_level,
            SkillEntry::DMastery(v) => v.require_level,
            SkillEntry::Wisp(v) => v.require_level,
            SkillEntry::VenomThorn(v) => v.require_level,
            SkillEntry::Alas(v) => v.require_level,
            SkillEntry::SoulShock(v) => v.require_level,
            SkillEntry::AMastery(v) => v.require_level,
            SkillEntry::SoreSword(v) => v.require_level,
            SkillEntry::BeatUp(v) => v.require_level,
            SkillEntry::Inpes(v) => v.require_level,
            SkillEntry::Blind(v) => v.require_level,
            SkillEntry::FrostWind(v) => v.require_level,
            SkillEntry::FMastery(v) => v.require_level,
            SkillEntry::Polluted(v) => v.require_level,
            SkillEntry::PastingShadow(v) => v.require_level,
            SkillEntry::DarkBolt(v) => v.require_level,
            SkillEntry::InnerPeace(v) => v.require_level,
            SkillEntry::SoulManacle(v) => v.require_level,
            SkillEntry::Haunt(v) => v.require_level,
            SkillEntry::Scratch(v) => v.require_level,
            SkillEntry::Judgement(v) => v.require_level,
            SkillEntry::DarkWave(v) => v.require_level,
            SkillEntry::CurseLazy(v) => v.require_level,
            SkillEntry::SpiritualFlare(v) => v.require_level,
            SkillEntry::ChasingHunt(v) => v.require_level,
            SkillEntry::LandGhost(v) => v.require_level,
            SkillEntry::MourningOfPray(v) => v.require_level,
            SkillEntry::BloodyKnight(v) => v.require_level,
            SkillEntry::AdventMigal(v) => v.require_level,
            SkillEntry::RainMaker(v) => v.require_level,
            SkillEntry::AdventMidranda(v) => v.require_level,
            SkillEntry::LowKick(v) => v.require_level,
            SkillEntry::SwordMasteryMartial(v) => v.require_level,
            SkillEntry::DoubleBlowMartial(v) => v.require_level,
            SkillEntry::Straight(v) => v.require_level,
            SkillEntry::RageUp(v) => v.require_level,
            SkillEntry::Patriot(v) => v.require_level,
            SkillEntry::Elbow(v) => v.require_level,
            SkillEntry::SwordMastery2(v) => v.require_level,
            SkillEntry::Bulkup(v) => v.require_level,
            SkillEntry::Warcry(v) => v.require_level,
            SkillEntry::Cannon(v) => v.require_level,
            SkillEntry::HeelKick(v) => v.require_level,
            SkillEntry::Combination(v) => v.require_level,
            SkillEntry::Steelers(v) => v.require_level,
            SkillEntry::Check(v) => v.require_level,
            SkillEntry::Typhoon(v) => v.require_level,
            SkillEntry::JBomb(v) => v.require_level,
            SkillEntry::RSlash(v) => v.require_level,
            SkillEntry::VStab(v) => v.require_level,
            SkillEntry::Storm(v) => v.require_level,
            SkillEntry::Creed(v) => v.require_level,
            SkillEntry::PDeity(v) => v.require_level,
            SkillEntry::GNail(v) => v.require_level,
            SkillEntry::HRegene(v) => v.require_level,
            SkillEntry::DMastery2(v) => v.require_level,
            SkillEntry::HHawk(v) => v.require_level,
            SkillEntry::LBreaking(v) => v.require_level,
            SkillEntry::HTraining(v) => v.require_level,
            SkillEntry::Amplified(_) => 0,
        }
    }

    pub fn require_mastery(&self) -> Option<(i32, i32)> {
        match self {
            SkillEntry::PikeWind(v) => v.require_mastery,
            SkillEntry::IceAttribute(v) => v.require_mastery,
            SkillEntry::CriticalHit(v) => v.require_mastery,
            SkillEntry::JumpingCrash(v) => v.require_mastery,
            SkillEntry::GroundPike(v) => v.require_mastery,
            SkillEntry::Tornado(v) => v.require_mastery,
            SkillEntry::WeaponDefenseMastery(v) => v.require_mastery,
            SkillEntry::Expansion(v) => v.require_mastery,
            SkillEntry::VenomSpear(v) => v.require_mastery,
            SkillEntry::Vanish(v) => v.require_mastery,
            SkillEntry::CriticalMastery(v) => v.require_mastery,
            SkillEntry::ChainLance(v) => v.require_mastery,
            SkillEntry::AssassinEye(v) => v.require_mastery,
            SkillEntry::ChargingStrike(v) => v.require_mastery,
            SkillEntry::Vague(v) => v.require_mastery,
            SkillEntry::ShadowMaster(v) => v.require_mastery,
            SkillEntry::DReaper(v) => v.require_mastery,
            SkillEntry::FSpear(v) => v.require_mastery,
            SkillEntry::SsAttack(v) => v.require_mastery,
            SkillEntry::ExtremeShield(v) => v.require_mastery,
            SkillEntry::MechanicBomb(v) => v.require_mastery,
            SkillEntry::PhysicalAbsorb(v) => v.require_mastery,
            SkillEntry::PoisonAttribute(v) => v.require_mastery,
            SkillEntry::GreatSmash(v) => v.require_mastery,
            SkillEntry::Maximize(v) => v.require_mastery,
            SkillEntry::Automation(v) => v.require_mastery,
            SkillEntry::Spark(v) => v.require_mastery,
            SkillEntry::MetalArmor(v) => v.require_mastery,
            SkillEntry::GrandSmash(v) => v.require_mastery,
            SkillEntry::MechanicWeaponMastery(v) => v.require_mastery,
            SkillEntry::SparkShield(v) => v.require_mastery,
            SkillEntry::Impulsion(v) => v.require_mastery,
            SkillEntry::Compulsion(v) => v.require_mastery,
            SkillEntry::MagneticSphere(v) => v.require_mastery,
            SkillEntry::MetalGolem(v) => v.require_mastery,
            SkillEntry::LandMine(v) => v.require_mastery,
            SkillEntry::HyperSonic(v) => v.require_mastery,
            SkillEntry::RSmash(v) => v.require_mastery,
            SkillEntry::PhysicalEnhance(v) => v.require_mastery,
            SkillEntry::MeleeMastery(v) => v.require_mastery,
            SkillEntry::FireAttribute(v) => v.require_mastery,
            SkillEntry::Raving(v) => v.require_mastery,
            SkillEntry::Impact(v) => v.require_mastery,
            SkillEntry::TripleImpact(v) => v.require_mastery,
            SkillEntry::BrutalSwing(v) => v.require_mastery,
            SkillEntry::Roar(v) => v.require_mastery,
            SkillEntry::RageOfZecram(v) => v.require_mastery,
            SkillEntry::Concentration(v) => v.require_mastery,
            SkillEntry::AvangingCrash(v) => v.require_mastery,
            SkillEntry::SwiftAxe(v) => v.require_mastery,
            SkillEntry::BoneCrash(v) => v.require_mastery,
            SkillEntry::Destoryer(v) => v.require_mastery,
            SkillEntry::Berserker(v) => v.require_mastery,
            SkillEntry::CycloneStrike(_) => None,
            SkillEntry::BoostHealth(v) => v.require_mastery,
            SkillEntry::DHit(v) => v.require_mastery,
            SkillEntry::PDash(v) => v.require_mastery,
            SkillEntry::MBlow(v) => v.require_mastery,
            SkillEntry::BBerserker(v) => v.require_mastery,
            SkillEntry::ScoutHawk(v) => v.require_mastery,
            SkillEntry::ShootingMastery(v) => v.require_mastery,
            SkillEntry::WindArrow(v) => v.require_mastery,
            SkillEntry::PerfectAim(v) => v.require_mastery,
            SkillEntry::DionsEye(v) => v.require_mastery,
            SkillEntry::Falcon(v) => v.require_mastery,
            SkillEntry::ArrowOfRage(v) => v.require_mastery,
            SkillEntry::Avalanche(v) => v.require_mastery,
            SkillEntry::ElementalShot(v) => v.require_mastery,
            SkillEntry::GoldenFalcon(v) => v.require_mastery,
            SkillEntry::BombShot(v) => v.require_mastery,
            SkillEntry::Perforation(v) => v.require_mastery,
            SkillEntry::RecallWolverin(v) => v.require_mastery,
            SkillEntry::EvasionMastery(v) => v.require_mastery,
            SkillEntry::PhoenixShot(v) => v.require_mastery,
            SkillEntry::ForceOfNature(v) => v.require_mastery,
            SkillEntry::EShot(v) => v.require_mastery,
            SkillEntry::SRope(v) => v.require_mastery,
            SkillEntry::NSplash(v) => v.require_mastery,
            SkillEntry::CTrap(v) => v.require_mastery,
            SkillEntry::SwordBlast(v) => v.require_mastery,
            SkillEntry::HolyBody(v) => v.require_mastery,
            SkillEntry::PhysicalTraining(v) => v.require_mastery,
            SkillEntry::DoubleCrash(v) => v.require_mastery,
            SkillEntry::HolyValor(v) => v.require_mastery,
            SkillEntry::Brandish(v) => v.require_mastery,
            SkillEntry::Piercing(v) => v.require_mastery,
            SkillEntry::DrasticSpirit(v) => v.require_mastery,
            SkillEntry::SwordMastery(v) => v.require_mastery,
            SkillEntry::DivineInhalation(v) => v.require_mastery,
            SkillEntry::HolyIncantation(v) => v.require_mastery,
            SkillEntry::GrandCross(v) => v.require_mastery,
            SkillEntry::DivinePiercing(v) => v.require_mastery,
            SkillEntry::GodlyShield(v) => v.require_mastery,
            SkillEntry::GodBless(v) => v.require_mastery,
            SkillEntry::SwordOfJustice(v) => v.require_mastery,
            SkillEntry::SBreaker(v) => v.require_mastery,
            SkillEntry::CMoon(v) => v.require_mastery,
            SkillEntry::SBlade(v) => v.require_mastery,
            SkillEntry::HBenedic(v) => v.require_mastery,
            SkillEntry::ShieldStrike(v) => v.require_mastery,
            SkillEntry::Farina(v) => v.require_mastery,
            SkillEntry::ThrowingMastery(v) => v.require_mastery,
            SkillEntry::VigorSpear(v) => v.require_mastery,
            SkillEntry::Windy(v) => v.require_mastery,
            SkillEntry::TwistJavelin(v) => v.require_mastery,
            SkillEntry::SoulSucker(v) => v.require_mastery,
            SkillEntry::FireJavelin(v) => v.require_mastery,
            SkillEntry::SplitJavelin(v) => v.require_mastery,
            SkillEntry::TriumphOfValhalla(v) => v.require_mastery,
            SkillEntry::LightningJavelin(v) => v.require_mastery,
            SkillEntry::StormJavelin(v) => v.require_mastery,
            SkillEntry::HallOfValhalla(v) => v.require_mastery,
            SkillEntry::XRage(v) => v.require_mastery,
            SkillEntry::FrostJavelin(v) => v.require_mastery,
            SkillEntry::Vengeance(v) => v.require_mastery,
            SkillEntry::Talaria(v) => v.require_mastery,
            SkillEntry::GCoup(v) => v.require_mastery,
            SkillEntry::Arcuda(v) => v.require_mastery,
            SkillEntry::SFear(v) => v.require_mastery,
            SkillEntry::Healing(v) => v.require_mastery,
            SkillEntry::HolyBolt(v) => v.require_mastery,
            SkillEntry::MultiSpark(v) => v.require_mastery,
            SkillEntry::HolyMind(v) => v.require_mastery,
            SkillEntry::Meditation(v) => v.require_mastery,
            SkillEntry::DivineLightning(v) => v.require_mastery,
            SkillEntry::HolyReflection(v) => v.require_mastery,
            SkillEntry::GrandHealing(v) => v.require_mastery,
            SkillEntry::VigorBall(v) => v.require_mastery,
            SkillEntry::Resurrection(v) => v.require_mastery,
            SkillEntry::Extinction(v) => v.require_mastery,
            SkillEntry::VirtualLife(v) => v.require_mastery,
            SkillEntry::GlacialSpike(v) => v.require_mastery,
            SkillEntry::RegenerationField(v) => v.require_mastery,
            SkillEntry::ChainLightning(v) => v.require_mastery,
            SkillEntry::SummonMuspell(v) => v.require_mastery,
            SkillEntry::SImpact(v) => v.require_mastery,
            SkillEntry::PIce(v) => v.require_mastery,
            SkillEntry::SummonRamiel(v) => v.require_mastery,
            SkillEntry::Krishna(v) => v.require_mastery,
            SkillEntry::Agony(v) => v.require_mastery,
            SkillEntry::FireBolt(v) => v.require_mastery,
            SkillEntry::Zenith(v) => v.require_mastery,
            SkillEntry::FireBall(v) => v.require_mastery,
            SkillEntry::MentalMastery(v) => v.require_mastery,
            SkillEntry::Watornado(v) => v.require_mastery,
            SkillEntry::EnchantWeapon(v) => v.require_mastery,
            SkillEntry::DeadRay(v) => v.require_mastery,
            SkillEntry::EnergyShield(v) => v.require_mastery,
            SkillEntry::Diastrophism(v) => v.require_mastery,
            SkillEntry::SpiritElemental(v) => v.require_mastery,
            SkillEntry::DancingSword(v) => v.require_mastery,
            SkillEntry::FireElemental(v) => v.require_mastery,
            SkillEntry::FlameWave(v) => v.require_mastery,
            SkillEntry::Distortion(v) => v.require_mastery,
            SkillEntry::Meteo(v) => v.require_mastery,
            SkillEntry::Silraphim(v) => v.require_mastery,
            SkillEntry::Tenus(v) => v.require_mastery,
            SkillEntry::Ignis(v) => v.require_mastery,
            SkillEntry::Anima(v) => v.require_mastery,
            SkillEntry::Stinger(v) => v.require_mastery,
            SkillEntry::DoubleBlow(v) => v.require_mastery,
            SkillEntry::DMastery(v) => v.require_mastery,
            SkillEntry::Wisp(v) => v.require_mastery,
            SkillEntry::VenomThorn(v) => v.require_mastery,
            SkillEntry::Alas(v) => v.require_mastery,
            SkillEntry::SoulShock(v) => v.require_mastery,
            SkillEntry::AMastery(v) => v.require_mastery,
            SkillEntry::SoreSword(v) => v.require_mastery,
            SkillEntry::BeatUp(v) => v.require_mastery,
            SkillEntry::Inpes(v) => v.require_mastery,
            SkillEntry::Blind(v) => v.require_mastery,
            SkillEntry::FrostWind(v) => v.require_mastery,
            SkillEntry::FMastery(v) => v.require_mastery,
            SkillEntry::Polluted(v) => v.require_mastery,
            SkillEntry::PastingShadow(v) => v.require_mastery,
            SkillEntry::DarkBolt(v) => v.require_mastery,
            SkillEntry::InnerPeace(v) => v.require_mastery,
            SkillEntry::SoulManacle(v) => v.require_mastery,
            SkillEntry::Haunt(v) => v.require_mastery,
            SkillEntry::Scratch(v) => v.require_mastery,
            SkillEntry::Judgement(v) => v.require_mastery,
            SkillEntry::DarkWave(v) => v.require_mastery,
            SkillEntry::CurseLazy(v) => v.require_mastery,
            SkillEntry::SpiritualFlare(v) => v.require_mastery,
            SkillEntry::ChasingHunt(v) => v.require_mastery,
            SkillEntry::LandGhost(v) => v.require_mastery,
            SkillEntry::MourningOfPray(v) => v.require_mastery,
            SkillEntry::BloodyKnight(v) => v.require_mastery,
            SkillEntry::AdventMigal(v) => v.require_mastery,
            SkillEntry::RainMaker(v) => v.require_mastery,
            SkillEntry::AdventMidranda(v) => v.require_mastery,
            SkillEntry::LowKick(v) => v.require_mastery,
            SkillEntry::SwordMasteryMartial(v) => v.require_mastery,
            SkillEntry::DoubleBlowMartial(v) => v.require_mastery,
            SkillEntry::Straight(v) => v.require_mastery,
            SkillEntry::RageUp(v) => v.require_mastery,
            SkillEntry::Patriot(v) => v.require_mastery,
            SkillEntry::Elbow(v) => v.require_mastery,
            SkillEntry::SwordMastery2(v) => v.require_mastery,
            SkillEntry::Bulkup(v) => v.require_mastery,
            SkillEntry::Warcry(v) => v.require_mastery,
            SkillEntry::Cannon(v) => v.require_mastery,
            SkillEntry::HeelKick(v) => v.require_mastery,
            SkillEntry::Combination(v) => v.require_mastery,
            SkillEntry::Steelers(v) => v.require_mastery,
            SkillEntry::Check(v) => v.require_mastery,
            SkillEntry::Typhoon(v) => v.require_mastery,
            SkillEntry::JBomb(v) => v.require_mastery,
            SkillEntry::RSlash(v) => v.require_mastery,
            SkillEntry::VStab(v) => v.require_mastery,
            SkillEntry::Storm(v) => v.require_mastery,
            SkillEntry::Creed(v) => v.require_mastery,
            SkillEntry::PDeity(v) => v.require_mastery,
            SkillEntry::GNail(v) => v.require_mastery,
            SkillEntry::HRegene(v) => v.require_mastery,
            SkillEntry::DMastery2(v) => v.require_mastery,
            SkillEntry::HHawk(v) => v.require_mastery,
            SkillEntry::LBreaking(v) => v.require_mastery,
            SkillEntry::HTraining(v) => v.require_mastery,
            SkillEntry::Amplified(_) => None,
        }
    }

    pub fn use_stamina(&self) -> Option<(i32, i32)> {
        match self {
            SkillEntry::PikeWind(v) => v.use_stamina,
            SkillEntry::IceAttribute(v) => v.use_stamina,
            SkillEntry::CriticalHit(v) => v.use_stamina,
            SkillEntry::JumpingCrash(v) => v.use_stamina,
            SkillEntry::GroundPike(v) => v.use_stamina,
            SkillEntry::Tornado(v) => v.use_stamina,
            SkillEntry::WeaponDefenseMastery(v) => v.use_stamina,
            SkillEntry::Expansion(v) => v.use_stamina,
            SkillEntry::VenomSpear(v) => v.use_stamina,
            SkillEntry::Vanish(v) => v.use_stamina,
            SkillEntry::CriticalMastery(v) => v.use_stamina,
            SkillEntry::ChainLance(v) => v.use_stamina,
            SkillEntry::AssassinEye(v) => v.use_stamina,
            SkillEntry::ChargingStrike(v) => v.use_stamina,
            SkillEntry::Vague(v) => v.use_stamina,
            SkillEntry::ShadowMaster(v) => v.use_stamina,
            SkillEntry::DReaper(v) => v.use_stamina,
            SkillEntry::FSpear(v) => v.use_stamina,
            SkillEntry::SsAttack(v) => v.use_stamina,
            SkillEntry::ExtremeShield(v) => v.use_stamina,
            SkillEntry::MechanicBomb(v) => v.use_stamina,
            SkillEntry::PhysicalAbsorb(v) => v.use_stamina,
            SkillEntry::PoisonAttribute(v) => v.use_stamina,
            SkillEntry::GreatSmash(v) => v.use_stamina,
            SkillEntry::Maximize(v) => v.use_stamina,
            SkillEntry::Automation(v) => v.use_stamina,
            SkillEntry::Spark(v) => v.use_stamina,
            SkillEntry::MetalArmor(v) => v.use_stamina,
            SkillEntry::GrandSmash(v) => v.use_stamina,
            SkillEntry::MechanicWeaponMastery(v) => v.use_stamina,
            SkillEntry::SparkShield(v) => v.use_stamina,
            SkillEntry::Impulsion(v) => v.use_stamina,
            SkillEntry::Compulsion(v) => v.use_stamina,
            SkillEntry::MagneticSphere(v) => v.use_stamina,
            SkillEntry::MetalGolem(v) => v.use_stamina,
            SkillEntry::LandMine(v) => v.use_stamina,
            SkillEntry::HyperSonic(v) => v.use_stamina,
            SkillEntry::RSmash(v) => v.use_stamina,
            SkillEntry::PhysicalEnhance(v) => v.use_stamina,
            SkillEntry::MeleeMastery(v) => v.use_stamina,
            SkillEntry::FireAttribute(v) => v.use_stamina,
            SkillEntry::Raving(v) => v.use_stamina,
            SkillEntry::Impact(v) => v.use_stamina,
            SkillEntry::TripleImpact(v) => v.use_stamina,
            SkillEntry::BrutalSwing(v) => v.use_stamina,
            SkillEntry::Roar(v) => v.use_stamina,
            SkillEntry::RageOfZecram(v) => v.use_stamina,
            SkillEntry::Concentration(v) => v.use_stamina,
            SkillEntry::AvangingCrash(v) => v.use_stamina,
            SkillEntry::SwiftAxe(v) => v.use_stamina,
            SkillEntry::BoneCrash(v) => v.use_stamina,
            SkillEntry::Destoryer(v) => v.use_stamina,
            SkillEntry::Berserker(v) => v.use_stamina,
            SkillEntry::CycloneStrike(_) => None,
            SkillEntry::BoostHealth(v) => v.use_stamina,
            SkillEntry::DHit(v) => v.use_stamina,
            SkillEntry::PDash(v) => v.use_stamina,
            SkillEntry::MBlow(v) => v.use_stamina,
            SkillEntry::BBerserker(v) => v.use_stamina,
            SkillEntry::ScoutHawk(v) => v.use_stamina,
            SkillEntry::ShootingMastery(v) => v.use_stamina,
            SkillEntry::WindArrow(v) => v.use_stamina,
            SkillEntry::PerfectAim(v) => v.use_stamina,
            SkillEntry::DionsEye(v) => v.use_stamina,
            SkillEntry::Falcon(v) => v.use_stamina,
            SkillEntry::ArrowOfRage(v) => v.use_stamina,
            SkillEntry::Avalanche(v) => v.use_stamina,
            SkillEntry::ElementalShot(v) => v.use_stamina,
            SkillEntry::GoldenFalcon(v) => v.use_stamina,
            SkillEntry::BombShot(v) => v.use_stamina,
            SkillEntry::Perforation(v) => v.use_stamina,
            SkillEntry::RecallWolverin(v) => v.use_stamina,
            SkillEntry::EvasionMastery(v) => v.use_stamina,
            SkillEntry::PhoenixShot(v) => v.use_stamina,
            SkillEntry::ForceOfNature(v) => v.use_stamina,
            SkillEntry::EShot(v) => v.use_stamina,
            SkillEntry::SRope(v) => v.use_stamina,
            SkillEntry::NSplash(v) => v.use_stamina,
            SkillEntry::CTrap(v) => v.use_stamina,
            SkillEntry::SwordBlast(v) => v.use_stamina,
            SkillEntry::HolyBody(v) => v.use_stamina,
            SkillEntry::PhysicalTraining(v) => v.use_stamina,
            SkillEntry::DoubleCrash(v) => v.use_stamina,
            SkillEntry::HolyValor(v) => v.use_stamina,
            SkillEntry::Brandish(v) => v.use_stamina,
            SkillEntry::Piercing(v) => v.use_stamina,
            SkillEntry::DrasticSpirit(v) => v.use_stamina,
            SkillEntry::SwordMastery(v) => v.use_stamina,
            SkillEntry::DivineInhalation(v) => v.use_stamina,
            SkillEntry::HolyIncantation(v) => v.use_stamina,
            SkillEntry::GrandCross(v) => v.use_stamina,
            SkillEntry::DivinePiercing(v) => v.use_stamina,
            SkillEntry::GodlyShield(v) => v.use_stamina,
            SkillEntry::GodBless(v) => v.use_stamina,
            SkillEntry::SwordOfJustice(v) => v.use_stamina,
            SkillEntry::SBreaker(v) => v.use_stamina,
            SkillEntry::CMoon(v) => v.use_stamina,
            SkillEntry::SBlade(v) => v.use_stamina,
            SkillEntry::HBenedic(v) => v.use_stamina,
            SkillEntry::ShieldStrike(v) => v.use_stamina,
            SkillEntry::Farina(v) => v.use_stamina,
            SkillEntry::ThrowingMastery(v) => v.use_stamina,
            SkillEntry::VigorSpear(v) => v.use_stamina,
            SkillEntry::Windy(v) => v.use_stamina,
            SkillEntry::TwistJavelin(v) => v.use_stamina,
            SkillEntry::SoulSucker(v) => v.use_stamina,
            SkillEntry::FireJavelin(v) => v.use_stamina,
            SkillEntry::SplitJavelin(v) => v.use_stamina,
            SkillEntry::TriumphOfValhalla(v) => v.use_stamina,
            SkillEntry::LightningJavelin(v) => v.use_stamina,
            SkillEntry::StormJavelin(v) => v.use_stamina,
            SkillEntry::HallOfValhalla(v) => v.use_stamina,
            SkillEntry::XRage(v) => v.use_stamina,
            SkillEntry::FrostJavelin(v) => v.use_stamina,
            SkillEntry::Vengeance(v) => v.use_stamina,
            SkillEntry::Talaria(v) => v.use_stamina,
            SkillEntry::GCoup(v) => v.use_stamina,
            SkillEntry::Arcuda(v) => v.use_stamina,
            SkillEntry::SFear(v) => v.use_stamina,
            SkillEntry::Healing(v) => v.use_stamina,
            SkillEntry::HolyBolt(v) => v.use_stamina,
            SkillEntry::MultiSpark(v) => v.use_stamina,
            SkillEntry::HolyMind(v) => v.use_stamina,
            SkillEntry::Meditation(v) => v.use_stamina,
            SkillEntry::DivineLightning(v) => v.use_stamina,
            SkillEntry::HolyReflection(v) => v.use_stamina,
            SkillEntry::GrandHealing(v) => v.use_stamina,
            SkillEntry::VigorBall(v) => v.use_stamina,
            SkillEntry::Resurrection(v) => v.use_stamina,
            SkillEntry::Extinction(v) => v.use_stamina,
            SkillEntry::VirtualLife(v) => v.use_stamina,
            SkillEntry::GlacialSpike(v) => v.use_stamina,
            SkillEntry::RegenerationField(v) => v.use_stamina,
            SkillEntry::ChainLightning(v) => v.use_stamina,
            SkillEntry::SummonMuspell(v) => v.use_stamina,
            SkillEntry::SImpact(v) => v.use_stamina,
            SkillEntry::PIce(v) => v.use_stamina,
            SkillEntry::SummonRamiel(v) => v.use_stamina,
            SkillEntry::Krishna(v) => v.use_stamina,
            SkillEntry::Agony(v) => v.use_stamina,
            SkillEntry::FireBolt(v) => v.use_stamina,
            SkillEntry::Zenith(v) => v.use_stamina,
            SkillEntry::FireBall(v) => v.use_stamina,
            SkillEntry::MentalMastery(v) => v.use_stamina,
            SkillEntry::Watornado(v) => v.use_stamina,
            SkillEntry::EnchantWeapon(v) => v.use_stamina,
            SkillEntry::DeadRay(v) => v.use_stamina,
            SkillEntry::EnergyShield(v) => v.use_stamina,
            SkillEntry::Diastrophism(v) => v.use_stamina,
            SkillEntry::SpiritElemental(v) => v.use_stamina,
            SkillEntry::DancingSword(v) => v.use_stamina,
            SkillEntry::FireElemental(v) => v.use_stamina,
            SkillEntry::FlameWave(v) => v.use_stamina,
            SkillEntry::Distortion(v) => v.use_stamina,
            SkillEntry::Meteo(v) => v.use_stamina,
            SkillEntry::Silraphim(v) => v.use_stamina,
            SkillEntry::Tenus(v) => v.use_stamina,
            SkillEntry::Ignis(v) => v.use_stamina,
            SkillEntry::Anima(v) => v.use_stamina,
            SkillEntry::Stinger(v) => v.use_stamina,
            SkillEntry::DoubleBlow(v) => v.use_stamina,
            SkillEntry::DMastery(v) => v.use_stamina,
            SkillEntry::Wisp(v) => v.use_stamina,
            SkillEntry::VenomThorn(v) => v.use_stamina,
            SkillEntry::Alas(v) => v.use_stamina,
            SkillEntry::SoulShock(v) => v.use_stamina,
            SkillEntry::AMastery(v) => v.use_stamina,
            SkillEntry::SoreSword(v) => v.use_stamina,
            SkillEntry::BeatUp(v) => v.use_stamina,
            SkillEntry::Inpes(v) => v.use_stamina,
            SkillEntry::Blind(v) => v.use_stamina,
            SkillEntry::FrostWind(v) => v.use_stamina,
            SkillEntry::FMastery(v) => v.use_stamina,
            SkillEntry::Polluted(v) => v.use_stamina,
            SkillEntry::PastingShadow(v) => v.use_stamina,
            SkillEntry::DarkBolt(v) => v.use_stamina,
            SkillEntry::InnerPeace(v) => v.use_stamina,
            SkillEntry::SoulManacle(v) => v.use_stamina,
            SkillEntry::Haunt(v) => v.use_stamina,
            SkillEntry::Scratch(v) => v.use_stamina,
            SkillEntry::Judgement(v) => v.use_stamina,
            SkillEntry::DarkWave(v) => v.use_stamina,
            SkillEntry::CurseLazy(v) => v.use_stamina,
            SkillEntry::SpiritualFlare(v) => v.use_stamina,
            SkillEntry::ChasingHunt(v) => v.use_stamina,
            SkillEntry::LandGhost(v) => v.use_stamina,
            SkillEntry::MourningOfPray(v) => v.use_stamina,
            SkillEntry::BloodyKnight(v) => v.use_stamina,
            SkillEntry::AdventMigal(v) => v.use_stamina,
            SkillEntry::RainMaker(v) => v.use_stamina,
            SkillEntry::AdventMidranda(v) => v.use_stamina,
            SkillEntry::LowKick(v) => v.use_stamina,
            SkillEntry::SwordMasteryMartial(v) => v.use_stamina,
            SkillEntry::DoubleBlowMartial(v) => v.use_stamina,
            SkillEntry::Straight(v) => v.use_stamina,
            SkillEntry::RageUp(v) => v.use_stamina,
            SkillEntry::Patriot(v) => v.use_stamina,
            SkillEntry::Elbow(v) => v.use_stamina,
            SkillEntry::SwordMastery2(v) => v.use_stamina,
            SkillEntry::Bulkup(v) => v.use_stamina,
            SkillEntry::Warcry(v) => v.use_stamina,
            SkillEntry::Cannon(v) => v.use_stamina,
            SkillEntry::HeelKick(v) => v.use_stamina,
            SkillEntry::Combination(v) => v.use_stamina,
            SkillEntry::Steelers(v) => v.use_stamina,
            SkillEntry::Check(v) => v.use_stamina,
            SkillEntry::Typhoon(v) => v.use_stamina,
            SkillEntry::JBomb(v) => v.use_stamina,
            SkillEntry::RSlash(v) => v.use_stamina,
            SkillEntry::VStab(v) => v.use_stamina,
            SkillEntry::Storm(v) => v.use_stamina,
            SkillEntry::Creed(v) => v.use_stamina,
            SkillEntry::PDeity(v) => v.use_stamina,
            SkillEntry::GNail(v) => v.use_stamina,
            SkillEntry::HRegene(v) => v.use_stamina,
            SkillEntry::DMastery2(v) => v.use_stamina,
            SkillEntry::HHawk(v) => v.use_stamina,
            SkillEntry::LBreaking(v) => v.use_stamina,
            SkillEntry::HTraining(v) => v.use_stamina,
            SkillEntry::Amplified(_) => None,
        }
    }

    pub fn use_mana(&self, level: SkillLevel) -> Option<i32> {
        match self {
            SkillEntry::PikeWind(v) => Some(v.use_mana.at(level)),
            SkillEntry::IceAttribute(_) => None,
            SkillEntry::CriticalHit(v) => Some(v.use_mana.at(level)),
            SkillEntry::JumpingCrash(v) => Some(v.use_mana.at(level)),
            SkillEntry::GroundPike(v) => Some(v.use_mana.at(level)),
            SkillEntry::Tornado(v) => Some(v.use_mana.at(level)),
            SkillEntry::WeaponDefenseMastery(_) => None,
            SkillEntry::Expansion(v) => Some(v.use_mana.at(level)),
            SkillEntry::VenomSpear(v) => Some(v.use_mana.at(level)),
            SkillEntry::Vanish(v) => Some(v.use_mana.at(level)),
            SkillEntry::CriticalMastery(_) => None,
            SkillEntry::ChainLance(v) => Some(v.use_mana.at(level)),
            SkillEntry::AssassinEye(v) => Some(v.use_mana.at(level)),
            SkillEntry::ChargingStrike(v) => Some(v.use_mana.at(level)),
            SkillEntry::Vague(v) => Some(v.use_mana.at(level)),
            SkillEntry::ShadowMaster(v) => Some(v.use_mana.at(level)),
            SkillEntry::DReaper(v) => Some(v.use_mana.at(level)),
            SkillEntry::FSpear(v) => Some(v.use_mana.at(level)),
            SkillEntry::SsAttack(v) => Some(v.use_mana.at(level)),
            SkillEntry::ExtremeShield(v) => Some(v.use_mana.at(level)),
            SkillEntry::MechanicBomb(v) => Some(v.use_mana.at(level)),
            SkillEntry::PhysicalAbsorb(v) => Some(v.use_mana.at(level)),
            SkillEntry::PoisonAttribute(_) => None,
            SkillEntry::GreatSmash(v) => Some(v.use_mana.at(level)),
            SkillEntry::Maximize(v) => Some(v.use_mana.at(level)),
            SkillEntry::Automation(v) => Some(v.use_mana.at(level)),
            SkillEntry::Spark(v) => Some(v.use_mana.at(level)),
            SkillEntry::MetalArmor(v) => Some(v.use_mana.at(level)),
            SkillEntry::GrandSmash(v) => Some(v.use_mana.at(level)),
            SkillEntry::MechanicWeaponMastery(_) => None,
            SkillEntry::SparkShield(v) => Some(v.use_mana.at(level)),
            SkillEntry::Impulsion(v) => Some(v.use_mana.at(level)),
            SkillEntry::Compulsion(v) => Some(v.use_mana.at(level)),
            SkillEntry::MagneticSphere(v) => Some(v.use_mana.at(level)),
            SkillEntry::MetalGolem(v) => Some(v.use_mana.at(level)),
            SkillEntry::LandMine(v) => Some(v.use_mana.at(level)),
            SkillEntry::HyperSonic(v) => Some(v.use_mana.at(level)),
            SkillEntry::RSmash(v) => Some(v.use_mana.at(level)),
            SkillEntry::PhysicalEnhance(v) => Some(v.use_mana.at(level)),
            SkillEntry::MeleeMastery(_) => None,
            SkillEntry::FireAttribute(_) => None,
            SkillEntry::Raving(v) => Some(v.use_mana.at(level)),
            SkillEntry::Impact(v) => Some(v.use_mana.at(level)),
            SkillEntry::TripleImpact(v) => Some(v.use_mana.at(level)),
            SkillEntry::BrutalSwing(v) => Some(v.use_mana.at(level)),
            SkillEntry::Roar(v) => Some(v.use_mana.at(level)),
            SkillEntry::RageOfZecram(v) => Some(v.use_mana.at(level)),
            SkillEntry::Concentration(v) => Some(v.use_mana.at(level)),
            SkillEntry::AvangingCrash(v) => Some(v.use_mana.at(level)),
            SkillEntry::SwiftAxe(v) => Some(v.use_mana.at(level)),
            SkillEntry::BoneCrash(v) => Some(v.use_mana.at(level)),
            SkillEntry::Destoryer(v) => Some(v.use_mana.at(level)),
            SkillEntry::Berserker(v) => Some(v.use_mana.at(level)),
            SkillEntry::CycloneStrike(v) => Some(v.use_mana.at(level)),
            SkillEntry::BoostHealth(_) => None,
            SkillEntry::DHit(v) => Some(v.use_mana.at(level)),
            SkillEntry::PDash(v) => Some(v.use_mana.at(level)),
            SkillEntry::MBlow(v) => Some(v.use_mana.at(level)),
            SkillEntry::BBerserker(v) => Some(v.use_mana.at(level)),
            SkillEntry::ScoutHawk(v) => Some(v.use_mana.at(level)),
            SkillEntry::ShootingMastery(_) => None,
            SkillEntry::WindArrow(v) => Some(v.use_mana.at(level)),
            SkillEntry::PerfectAim(v) => Some(v.use_mana.at(level)),
            SkillEntry::DionsEye(_) => None,
            SkillEntry::Falcon(v) => Some(v.use_mana.at(level)),
            SkillEntry::ArrowOfRage(v) => Some(v.use_mana.at(level)),
            SkillEntry::Avalanche(v) => Some(v.use_mana.at(level)),
            SkillEntry::ElementalShot(v) => Some(v.use_mana.at(level)),
            SkillEntry::GoldenFalcon(v) => Some(v.use_mana.at(level)),
            SkillEntry::BombShot(v) => Some(v.use_mana.at(level)),
            SkillEntry::Perforation(v) => Some(v.use_mana.at(level)),
            SkillEntry::RecallWolverin(v) => Some(v.use_mana.at(level)),
            SkillEntry::EvasionMastery(_) => None,
            SkillEntry::PhoenixShot(v) => Some(v.use_mana.at(level)),
            SkillEntry::ForceOfNature(v) => Some(v.use_mana.at(level)),
            SkillEntry::EShot(v) => Some(v.use_mana.at(level)),
            SkillEntry::SRope(v) => Some(v.use_mana.at(level)),
            SkillEntry::NSplash(v) => Some(v.use_mana.at(level)),
            SkillEntry::CTrap(v) => Some(v.use_mana.at(level)),
            SkillEntry::SwordBlast(v) => Some(v.use_mana.at(level)),
            SkillEntry::HolyBody(v) => Some(v.use_mana.at(level)),
            SkillEntry::PhysicalTraining(_) => None,
            SkillEntry::DoubleCrash(v) => Some(v.use_mana.at(level)),
            SkillEntry::HolyValor(v) => Some(v.use_mana.at(level)),
            SkillEntry::Brandish(v) => Some(v.use_mana.at(level)),
            SkillEntry::Piercing(v) => Some(v.use_mana.at(level)),
            SkillEntry::DrasticSpirit(v) => Some(v.use_mana.at(level)),
            SkillEntry::SwordMastery(_) => None,
            SkillEntry::DivineInhalation(v) => Some(v.use_mana.at(level)),
            SkillEntry::HolyIncantation(v) => Some(v.use_mana.at(level)),
            SkillEntry::GrandCross(v) => Some(v.use_mana.at(level)),
            SkillEntry::DivinePiercing(v) => Some(v.use_mana.at(level)),
            SkillEntry::GodlyShield(v) => Some(v.use_mana.at(level)),
            SkillEntry::GodBless(v) => Some(v.use_mana.at(level)),
            SkillEntry::SwordOfJustice(v) => Some(v.use_mana.at(level)),
            SkillEntry::SBreaker(v) => Some(v.use_mana.at(level)),
            SkillEntry::CMoon(v) => Some(v.use_mana.at(level)),
            SkillEntry::SBlade(v) => Some(v.use_mana.at(level)),
            SkillEntry::HBenedic(v) => Some(v.use_mana.at(level)),
            SkillEntry::ShieldStrike(v) => Some(v.use_mana.at(level)),
            SkillEntry::Farina(v) => Some(v.use_mana.at(level)),
            SkillEntry::ThrowingMastery(_) => None,
            SkillEntry::VigorSpear(v) => Some(v.use_mana.at(level)),
            SkillEntry::Windy(v) => Some(v.use_mana.at(level)),
            SkillEntry::TwistJavelin(v) => Some(v.use_mana.at(level)),
            SkillEntry::SoulSucker(v) => Some(v.use_mana.at(level)),
            SkillEntry::FireJavelin(v) => Some(v.use_mana.at(level)),
            SkillEntry::SplitJavelin(v) => Some(v.use_mana.at(level)),
            SkillEntry::TriumphOfValhalla(v) => Some(v.use_mana.at(level)),
            SkillEntry::LightningJavelin(v) => Some(v.use_mana.at(level)),
            SkillEntry::StormJavelin(v) => Some(v.use_mana.at(level)),
            SkillEntry::HallOfValhalla(v) => Some(v.use_mana.at(level)),
            SkillEntry::XRage(v) => Some(v.use_mana.at(level)),
            SkillEntry::FrostJavelin(v) => Some(v.use_mana.at(level)),
            SkillEntry::Vengeance(v) => Some(v.use_mana.at(level)),
            SkillEntry::Talaria(v) => Some(v.use_mana.at(level)),
            SkillEntry::GCoup(v) => Some(v.use_mana.at(level)),
            SkillEntry::Arcuda(v) => Some(v.use_mana.at(level)),
            SkillEntry::SFear(v) => Some(v.use_mana.at(level)),
            SkillEntry::Healing(v) => Some(v.use_mana.at(level)),
            SkillEntry::HolyBolt(v) => Some(v.use_mana.at(level)),
            SkillEntry::MultiSpark(v) => Some(v.use_mana.at(level)),
            SkillEntry::HolyMind(v) => Some(v.use_mana.at(level)),
            SkillEntry::Meditation(_) => None,
            SkillEntry::DivineLightning(v) => Some(v.use_mana.at(level)),
            SkillEntry::HolyReflection(v) => Some(v.use_mana.at(level)),
            SkillEntry::GrandHealing(v) => Some(v.use_mana.at(level)),
            SkillEntry::VigorBall(v) => Some(v.use_mana.at(level)),
            SkillEntry::Resurrection(v) => Some(v.use_mana.at(level)),
            SkillEntry::Extinction(v) => Some(v.use_mana.at(level)),
            SkillEntry::VirtualLife(v) => Some(v.use_mana.at(level)),
            SkillEntry::GlacialSpike(v) => Some(v.use_mana.at(level)),
            SkillEntry::RegenerationField(v) => Some(v.use_mana.at(level)),
            SkillEntry::ChainLightning(v) => Some(v.use_mana.at(level)),
            SkillEntry::SummonMuspell(v) => Some(v.use_mana.at(level)),
            SkillEntry::SImpact(v) => Some(v.use_mana.at(level)),
            SkillEntry::PIce(v) => Some(v.use_mana.at(level)),
            SkillEntry::SummonRamiel(v) => Some(v.use_mana.at(level)),
            SkillEntry::Krishna(v) => Some(v.use_mana.at(level)),
            SkillEntry::Agony(v) => Some(v.use_mana.at(level)),
            SkillEntry::FireBolt(v) => Some(v.use_mana.at(level)),
            SkillEntry::Zenith(v) => Some(v.use_mana.at(level)),
            SkillEntry::FireBall(v) => Some(v.use_mana.at(level)),
            SkillEntry::MentalMastery(_) => None,
            SkillEntry::Watornado(v) => Some(v.use_mana.at(level)),
            SkillEntry::EnchantWeapon(v) => Some(v.use_mana.at(level)),
            SkillEntry::DeadRay(v) => Some(v.use_mana.at(level)),
            SkillEntry::EnergyShield(v) => Some(v.use_mana.at(level)),
            SkillEntry::Diastrophism(v) => Some(v.use_mana.at(level)),
            SkillEntry::SpiritElemental(v) => Some(v.use_mana.at(level)),
            SkillEntry::DancingSword(v) => Some(v.use_mana.at(level)),
            SkillEntry::FireElemental(v) => Some(v.use_mana.at(level)),
            SkillEntry::FlameWave(v) => Some(v.use_mana.at(level)),
            SkillEntry::Distortion(v) => Some(v.use_mana.at(level)),
            SkillEntry::Meteo(v) => Some(v.use_mana.at(level)),
            SkillEntry::Silraphim(v) => Some(v.use_mana.at(level)),
            SkillEntry::Tenus(v) => Some(v.use_mana.at(level)),
            SkillEntry::Ignis(v) => Some(v.use_mana.at(level)),
            SkillEntry::Anima(v) => Some(v.use_mana.at(level)),
            SkillEntry::Stinger(v) => Some(v.use_mana.at(level)),
            SkillEntry::DoubleBlow(v) => Some(v.use_mana.at(level)),
            SkillEntry::DMastery(_) => None,
            SkillEntry::Wisp(v) => Some(v.use_mana.at(level)),
            SkillEntry::VenomThorn(v) => Some(v.use_mana.at(level)),
            SkillEntry::Alas(v) => Some(v.use_mana.at(level)),
            SkillEntry::SoulShock(v) => Some(v.use_mana.at(level)),
            SkillEntry::AMastery(_) => None,
            SkillEntry::SoreSword(v) => Some(v.use_mana.at(level)),
            SkillEntry::BeatUp(v) => Some(v.use_mana.at(level)),
            SkillEntry::Inpes(v) => Some(v.use_mana.at(level)),
            SkillEntry::Blind(v) => Some(v.use_mana.at(level)),
            SkillEntry::FrostWind(v) => Some(v.use_mana.at(level)),
            SkillEntry::FMastery(_) => None,
            SkillEntry::Polluted(v) => Some(v.use_mana.at(level)),
            SkillEntry::PastingShadow(v) => Some(v.use_mana.at(level)),
            SkillEntry::DarkBolt(v) => Some(v.use_mana.at(level)),
            SkillEntry::InnerPeace(_) => None,
            SkillEntry::SoulManacle(v) => Some(v.use_mana.at(level)),
            SkillEntry::Haunt(v) => Some(v.use_mana.at(level)),
            SkillEntry::Scratch(v) => Some(v.use_mana.at(level)),
            SkillEntry::Judgement(v) => Some(v.use_mana.at(level)),
            SkillEntry::DarkWave(v) => Some(v.use_mana.at(level)),
            SkillEntry::CurseLazy(v) => Some(v.use_mana.at(level)),
            SkillEntry::SpiritualFlare(v) => Some(v.use_mana.at(level)),
            SkillEntry::ChasingHunt(v) => Some(v.use_mana.at(level)),
            SkillEntry::LandGhost(v) => Some(v.use_mana.at(level)),
            SkillEntry::MourningOfPray(v) => Some(v.use_mana.at(level)),
            SkillEntry::BloodyKnight(v) => Some(v.use_mana.at(level)),
            SkillEntry::AdventMigal(v) => Some(v.use_mana.at(level)),
            SkillEntry::RainMaker(v) => Some(v.use_mana.at(level)),
            SkillEntry::AdventMidranda(v) => Some(v.use_mana.at(level)),
            SkillEntry::LowKick(v) => Some(v.use_mana.at(level)),
            SkillEntry::SwordMasteryMartial(_) => None,
            SkillEntry::DoubleBlowMartial(v) => Some(v.use_mana.at(level)),
            SkillEntry::Straight(v) => Some(v.use_mana.at(level)),
            SkillEntry::RageUp(v) => Some(v.use_mana.at(level)),
            SkillEntry::Patriot(v) => Some(v.use_mana.at(level)),
            SkillEntry::Elbow(v) => Some(v.use_mana.at(level)),
            SkillEntry::SwordMastery2(_) => None,
            SkillEntry::Bulkup(v) => Some(v.use_mana.at(level)),
            SkillEntry::Warcry(v) => Some(v.use_mana.at(level)),
            SkillEntry::Cannon(v) => Some(v.use_mana.at(level)),
            SkillEntry::HeelKick(v) => Some(v.use_mana.at(level)),
            SkillEntry::Combination(v) => Some(v.use_mana.at(level)),
            SkillEntry::Steelers(v) => Some(v.use_mana.at(level)),
            SkillEntry::Check(v) => Some(v.use_mana.at(level)),
            SkillEntry::Typhoon(v) => Some(v.use_mana.at(level)),
            SkillEntry::JBomb(v) => Some(v.use_mana.at(level)),
            SkillEntry::RSlash(v) => Some(v.use_mana.at(level)),
            SkillEntry::VStab(v) => Some(v.use_mana.at(level)),
            SkillEntry::Storm(v) => Some(v.use_mana.at(level)),
            SkillEntry::Creed(v) => Some(v.use_mana.at(level)),
            SkillEntry::PDeity(v) => Some(v.use_mana.at(level)),
            SkillEntry::GNail(v) => Some(v.use_mana.at(level)),
            SkillEntry::HRegene(_) => None,
            SkillEntry::DMastery2(_) => None,
            SkillEntry::HHawk(v) => Some(v.use_mana.at(level)),
            SkillEntry::LBreaking(v) => Some(v.use_mana.at(level)),
            SkillEntry::HTraining(v) => Some(v.use_mana.at(level)),
            SkillEntry::Amplified(_) => None,
        }
    }

    pub fn duration_secs(&self, level: SkillLevel) -> Option<i32> {
        match self {
            SkillEntry::Concentration(v) => Some(v.time.at(level)),
            SkillEntry::SparkShield(v) => Some(v.time.at(level)),
            SkillEntry::Compulsion(v) => Some(v.time.at(level)),
            SkillEntry::HolyBody(v) => Some(v.time.at(level)),
            SkillEntry::HolyValor(v) => Some(v.time.at(level)),
            SkillEntry::SwiftAxe(v) => Some(v.time.at(level)),
            SkillEntry::Berserker(v) => Some(v.time.at(level)),
            SkillEntry::BBerserker(v) => Some(v.time.at(level)),
            SkillEntry::Windy(v) => Some(v.time.at(level)),
            SkillEntry::Talaria(v) => Some(v.time.at(level)),
            SkillEntry::Vanish(v) => Some(v.time.at(level)),
            SkillEntry::ForceOfNature(v) => Some(v.time.at(level)),
            SkillEntry::GoldenFalcon(v) => Some(v.time.at(level)),
            SkillEntry::HolyMind(v) => Some(v.time.at(level)),
            SkillEntry::HolyReflection(v) => Some(v.time.at(level)),
            SkillEntry::VirtualLife(v) => Some(v.time.at(level)),
            SkillEntry::RegenerationField(v) => Some(v.time.at(level)),
            SkillEntry::Krishna(v) => Some(v.time.at(level)),
            SkillEntry::EnergyShield(v) => Some(v.time.at(level)),
            SkillEntry::SpiritElemental(v) => Some(v.time.at(level)),
            SkillEntry::Tenus(v) => Some(v.time.at(level)),
            SkillEntry::Anima(v) => Some(v.time.at(level)),
            SkillEntry::RageUp(v) => Some(v.time.at(level)),
            SkillEntry::Bulkup(v) => Some(v.time.at(level)),
            SkillEntry::Steelers(v) => Some(v.time.at(level)),
            SkillEntry::RainMaker(v) => Some(v.time.at(level)),
            SkillEntry::AdventMidranda(v) => Some(v.time.at(level)),
            SkillEntry::Distortion(v) => Some(v.time.at(level)),
            SkillEntry::Creed(v) => Some(v.time.at(level)),
            SkillEntry::HTraining(v) => Some(v.time.at(level)),
            _ => None,
        }
    }

    pub fn element(&self) -> Option<(i32, i32, i32)> {
        match self {
            SkillEntry::PikeWind(v) => v.element,
            SkillEntry::IceAttribute(v) => v.element,
            SkillEntry::CriticalHit(v) => v.element,
            SkillEntry::JumpingCrash(v) => v.element,
            SkillEntry::GroundPike(v) => v.element,
            SkillEntry::Tornado(v) => v.element,
            SkillEntry::WeaponDefenseMastery(v) => v.element,
            SkillEntry::Expansion(v) => v.element,
            SkillEntry::VenomSpear(v) => v.element,
            SkillEntry::Vanish(v) => v.element,
            SkillEntry::CriticalMastery(v) => v.element,
            SkillEntry::ChainLance(v) => v.element,
            SkillEntry::AssassinEye(v) => v.element,
            SkillEntry::ChargingStrike(v) => v.element,
            SkillEntry::Vague(v) => v.element,
            SkillEntry::ShadowMaster(v) => v.element,
            SkillEntry::DReaper(v) => v.element,
            SkillEntry::FSpear(v) => v.element,
            SkillEntry::SsAttack(v) => v.element,
            SkillEntry::ExtremeShield(v) => v.element,
            SkillEntry::MechanicBomb(v) => v.element,
            SkillEntry::PhysicalAbsorb(v) => v.element,
            SkillEntry::PoisonAttribute(v) => v.element,
            SkillEntry::GreatSmash(v) => v.element,
            SkillEntry::Maximize(v) => v.element,
            SkillEntry::Automation(v) => v.element,
            SkillEntry::Spark(v) => v.element,
            SkillEntry::MetalArmor(v) => v.element,
            SkillEntry::GrandSmash(v) => v.element,
            SkillEntry::MechanicWeaponMastery(v) => v.element,
            SkillEntry::SparkShield(v) => v.element,
            SkillEntry::Impulsion(v) => v.element,
            SkillEntry::Compulsion(v) => v.element,
            SkillEntry::MagneticSphere(v) => v.element,
            SkillEntry::MetalGolem(v) => v.element,
            SkillEntry::LandMine(v) => v.element,
            SkillEntry::HyperSonic(v) => v.element,
            SkillEntry::RSmash(v) => v.element,
            SkillEntry::PhysicalEnhance(v) => v.element,
            SkillEntry::MeleeMastery(v) => v.element,
            SkillEntry::FireAttribute(v) => v.element,
            SkillEntry::Raving(v) => v.element,
            SkillEntry::Impact(v) => v.element,
            SkillEntry::TripleImpact(v) => v.element,
            SkillEntry::BrutalSwing(v) => v.element,
            SkillEntry::Roar(v) => v.element,
            SkillEntry::RageOfZecram(v) => v.element,
            SkillEntry::Concentration(v) => v.element,
            SkillEntry::AvangingCrash(v) => v.element,
            SkillEntry::SwiftAxe(v) => v.element,
            SkillEntry::BoneCrash(v) => v.element,
            SkillEntry::Destoryer(v) => v.element,
            SkillEntry::Berserker(v) => v.element,
            SkillEntry::CycloneStrike(_) => None,
            SkillEntry::BoostHealth(v) => v.element,
            SkillEntry::DHit(v) => v.element,
            SkillEntry::PDash(v) => v.element,
            SkillEntry::MBlow(v) => v.element,
            SkillEntry::BBerserker(v) => v.element,
            SkillEntry::ScoutHawk(v) => v.element,
            SkillEntry::ShootingMastery(v) => v.element,
            SkillEntry::WindArrow(v) => v.element,
            SkillEntry::PerfectAim(v) => v.element,
            SkillEntry::DionsEye(v) => v.element,
            SkillEntry::Falcon(v) => v.element,
            SkillEntry::ArrowOfRage(v) => v.element,
            SkillEntry::Avalanche(v) => v.element,
            SkillEntry::ElementalShot(v) => v.element,
            SkillEntry::GoldenFalcon(v) => v.element,
            SkillEntry::BombShot(v) => v.element,
            SkillEntry::Perforation(v) => v.element,
            SkillEntry::RecallWolverin(v) => v.element,
            SkillEntry::EvasionMastery(v) => v.element,
            SkillEntry::PhoenixShot(v) => v.element,
            SkillEntry::ForceOfNature(v) => v.element,
            SkillEntry::EShot(v) => v.element,
            SkillEntry::SRope(v) => v.element,
            SkillEntry::NSplash(v) => v.element,
            SkillEntry::CTrap(v) => v.element,
            SkillEntry::SwordBlast(v) => v.element,
            SkillEntry::HolyBody(v) => v.element,
            SkillEntry::PhysicalTraining(v) => v.element,
            SkillEntry::DoubleCrash(v) => v.element,
            SkillEntry::HolyValor(v) => v.element,
            SkillEntry::Brandish(v) => v.element,
            SkillEntry::Piercing(v) => v.element,
            SkillEntry::DrasticSpirit(v) => v.element,
            SkillEntry::SwordMastery(v) => v.element,
            SkillEntry::DivineInhalation(v) => v.element,
            SkillEntry::HolyIncantation(v) => v.element,
            SkillEntry::GrandCross(v) => v.element,
            SkillEntry::DivinePiercing(v) => v.element,
            SkillEntry::GodlyShield(v) => v.element,
            SkillEntry::GodBless(v) => v.element,
            SkillEntry::SwordOfJustice(v) => v.element,
            SkillEntry::SBreaker(v) => v.element,
            SkillEntry::CMoon(v) => v.element,
            SkillEntry::SBlade(v) => v.element,
            SkillEntry::HBenedic(v) => v.element,
            SkillEntry::ShieldStrike(v) => v.element,
            SkillEntry::Farina(v) => v.element,
            SkillEntry::ThrowingMastery(v) => v.element,
            SkillEntry::VigorSpear(v) => v.element,
            SkillEntry::Windy(v) => v.element,
            SkillEntry::TwistJavelin(v) => v.element,
            SkillEntry::SoulSucker(v) => v.element,
            SkillEntry::FireJavelin(v) => v.element,
            SkillEntry::SplitJavelin(v) => v.element,
            SkillEntry::TriumphOfValhalla(v) => v.element,
            SkillEntry::LightningJavelin(v) => v.element,
            SkillEntry::StormJavelin(v) => v.element,
            SkillEntry::HallOfValhalla(v) => v.element,
            SkillEntry::XRage(v) => v.element,
            SkillEntry::FrostJavelin(v) => v.element,
            SkillEntry::Vengeance(v) => v.element,
            SkillEntry::Talaria(v) => v.element,
            SkillEntry::GCoup(v) => v.element,
            SkillEntry::Arcuda(v) => v.element,
            SkillEntry::SFear(v) => v.element,
            SkillEntry::Healing(v) => v.element,
            SkillEntry::HolyBolt(v) => v.element,
            SkillEntry::MultiSpark(v) => v.element,
            SkillEntry::HolyMind(v) => v.element,
            SkillEntry::Meditation(v) => v.element,
            SkillEntry::DivineLightning(v) => v.element,
            SkillEntry::HolyReflection(v) => v.element,
            SkillEntry::GrandHealing(v) => v.element,
            SkillEntry::VigorBall(v) => v.element,
            SkillEntry::Resurrection(v) => v.element,
            SkillEntry::Extinction(v) => v.element,
            SkillEntry::VirtualLife(v) => v.element,
            SkillEntry::GlacialSpike(v) => v.element,
            SkillEntry::RegenerationField(v) => v.element,
            SkillEntry::ChainLightning(v) => v.element,
            SkillEntry::SummonMuspell(v) => v.element,
            SkillEntry::SImpact(v) => v.element,
            SkillEntry::PIce(v) => v.element,
            SkillEntry::SummonRamiel(v) => v.element,
            SkillEntry::Krishna(v) => v.element,
            SkillEntry::Agony(v) => v.element,
            SkillEntry::FireBolt(v) => v.element,
            SkillEntry::Zenith(v) => v.element,
            SkillEntry::FireBall(v) => v.element,
            SkillEntry::MentalMastery(v) => v.element,
            SkillEntry::Watornado(v) => v.element,
            SkillEntry::EnchantWeapon(v) => v.element,
            SkillEntry::DeadRay(v) => v.element,
            SkillEntry::EnergyShield(v) => v.element,
            SkillEntry::Diastrophism(v) => v.element,
            SkillEntry::SpiritElemental(v) => v.element,
            SkillEntry::DancingSword(v) => v.element,
            SkillEntry::FireElemental(v) => v.element,
            SkillEntry::FlameWave(v) => v.element,
            SkillEntry::Distortion(v) => v.element,
            SkillEntry::Meteo(v) => v.element,
            SkillEntry::Silraphim(v) => v.element,
            SkillEntry::Tenus(v) => v.element,
            SkillEntry::Ignis(v) => v.element,
            SkillEntry::Anima(v) => v.element,
            SkillEntry::Stinger(v) => v.element,
            SkillEntry::DoubleBlow(v) => v.element,
            SkillEntry::DMastery(v) => v.element,
            SkillEntry::Wisp(v) => v.element,
            SkillEntry::VenomThorn(v) => v.element,
            SkillEntry::Alas(v) => v.element,
            SkillEntry::SoulShock(v) => v.element,
            SkillEntry::AMastery(v) => v.element,
            SkillEntry::SoreSword(v) => v.element,
            SkillEntry::BeatUp(v) => v.element,
            SkillEntry::Inpes(v) => v.element,
            SkillEntry::Blind(v) => v.element,
            SkillEntry::FrostWind(v) => v.element,
            SkillEntry::FMastery(v) => v.element,
            SkillEntry::Polluted(v) => v.element,
            SkillEntry::PastingShadow(v) => v.element,
            SkillEntry::DarkBolt(v) => v.element,
            SkillEntry::InnerPeace(v) => v.element,
            SkillEntry::SoulManacle(v) => v.element,
            SkillEntry::Haunt(v) => v.element,
            SkillEntry::Scratch(v) => v.element,
            SkillEntry::Judgement(v) => v.element,
            SkillEntry::DarkWave(v) => v.element,
            SkillEntry::CurseLazy(v) => v.element,
            SkillEntry::SpiritualFlare(v) => v.element,
            SkillEntry::ChasingHunt(v) => v.element,
            SkillEntry::LandGhost(v) => v.element,
            SkillEntry::MourningOfPray(v) => v.element,
            SkillEntry::BloodyKnight(v) => v.element,
            SkillEntry::AdventMigal(v) => v.element,
            SkillEntry::RainMaker(v) => v.element,
            SkillEntry::AdventMidranda(v) => v.element,
            SkillEntry::LowKick(v) => v.element,
            SkillEntry::SwordMasteryMartial(v) => v.element,
            SkillEntry::DoubleBlowMartial(v) => v.element,
            SkillEntry::Straight(v) => v.element,
            SkillEntry::RageUp(v) => v.element,
            SkillEntry::Patriot(v) => v.element,
            SkillEntry::Elbow(v) => v.element,
            SkillEntry::SwordMastery2(v) => v.element,
            SkillEntry::Bulkup(v) => v.element,
            SkillEntry::Warcry(v) => v.element,
            SkillEntry::Cannon(v) => v.element,
            SkillEntry::HeelKick(v) => v.element,
            SkillEntry::Combination(v) => v.element,
            SkillEntry::Steelers(v) => v.element,
            SkillEntry::Check(v) => v.element,
            SkillEntry::Typhoon(v) => v.element,
            SkillEntry::JBomb(v) => v.element,
            SkillEntry::RSlash(v) => v.element,
            SkillEntry::VStab(v) => v.element,
            SkillEntry::Storm(v) => v.element,
            SkillEntry::Creed(v) => v.element,
            SkillEntry::PDeity(v) => v.element,
            SkillEntry::GNail(v) => v.element,
            SkillEntry::HRegene(v) => v.element,
            SkillEntry::DMastery2(v) => v.element,
            SkillEntry::HHawk(v) => v.element,
            SkillEntry::LBreaking(v) => v.element,
            SkillEntry::HTraining(v) => v.element,
            SkillEntry::Amplified(_) => None,
        }
    }

    pub fn use_weapon_code(&self) -> &[EquippableItemType] {
        match self {
            SkillEntry::PikeWind(v) => &v.use_weapon_code,
            SkillEntry::IceAttribute(v) => &v.use_weapon_code,
            SkillEntry::CriticalHit(v) => &v.use_weapon_code,
            SkillEntry::JumpingCrash(v) => &v.use_weapon_code,
            SkillEntry::GroundPike(v) => &v.use_weapon_code,
            SkillEntry::Tornado(v) => &v.use_weapon_code,
            SkillEntry::WeaponDefenseMastery(v) => &v.use_weapon_code,
            SkillEntry::Expansion(v) => &v.use_weapon_code,
            SkillEntry::VenomSpear(v) => &v.use_weapon_code,
            SkillEntry::Vanish(v) => &v.use_weapon_code,
            SkillEntry::CriticalMastery(v) => &v.use_weapon_code,
            SkillEntry::ChainLance(v) => &v.use_weapon_code,
            SkillEntry::AssassinEye(v) => &v.use_weapon_code,
            SkillEntry::ChargingStrike(v) => &v.use_weapon_code,
            SkillEntry::Vague(v) => &v.use_weapon_code,
            SkillEntry::ShadowMaster(v) => &v.use_weapon_code,
            SkillEntry::DReaper(v) => &v.use_weapon_code,
            SkillEntry::FSpear(v) => &v.use_weapon_code,
            SkillEntry::SsAttack(v) => &v.use_weapon_code,
            SkillEntry::ExtremeShield(v) => &v.use_weapon_code,
            SkillEntry::MechanicBomb(v) => &v.use_weapon_code,
            SkillEntry::PhysicalAbsorb(v) => &v.use_weapon_code,
            SkillEntry::PoisonAttribute(v) => &v.use_weapon_code,
            SkillEntry::GreatSmash(v) => &v.use_weapon_code,
            SkillEntry::Maximize(v) => &v.use_weapon_code,
            SkillEntry::Automation(v) => &v.use_weapon_code,
            SkillEntry::Spark(v) => &v.use_weapon_code,
            SkillEntry::MetalArmor(v) => &v.use_weapon_code,
            SkillEntry::GrandSmash(v) => &v.use_weapon_code,
            SkillEntry::MechanicWeaponMastery(v) => &v.use_weapon_code,
            SkillEntry::SparkShield(v) => &v.use_weapon_code,
            SkillEntry::Impulsion(v) => &v.use_weapon_code,
            SkillEntry::Compulsion(v) => &v.use_weapon_code,
            SkillEntry::MagneticSphere(v) => &v.use_weapon_code,
            SkillEntry::MetalGolem(v) => &v.use_weapon_code,
            SkillEntry::LandMine(v) => &v.use_weapon_code,
            SkillEntry::HyperSonic(v) => &v.use_weapon_code,
            SkillEntry::RSmash(v) => &v.use_weapon_code,
            SkillEntry::PhysicalEnhance(v) => &v.use_weapon_code,
            SkillEntry::MeleeMastery(v) => &v.use_weapon_code,
            SkillEntry::FireAttribute(v) => &v.use_weapon_code,
            SkillEntry::Raving(v) => &v.use_weapon_code,
            SkillEntry::Impact(v) => &v.use_weapon_code,
            SkillEntry::TripleImpact(v) => &v.use_weapon_code,
            SkillEntry::BrutalSwing(v) => &v.use_weapon_code,
            SkillEntry::Roar(v) => &v.use_weapon_code,
            SkillEntry::RageOfZecram(v) => &v.use_weapon_code,
            SkillEntry::Concentration(v) => &v.use_weapon_code,
            SkillEntry::AvangingCrash(v) => &v.use_weapon_code,
            SkillEntry::SwiftAxe(v) => &v.use_weapon_code,
            SkillEntry::BoneCrash(v) => &v.use_weapon_code,
            SkillEntry::Destoryer(v) => &v.use_weapon_code,
            SkillEntry::Berserker(v) => &v.use_weapon_code,
            SkillEntry::CycloneStrike(_) => &[],
            SkillEntry::BoostHealth(v) => &v.use_weapon_code,
            SkillEntry::DHit(v) => &v.use_weapon_code,
            SkillEntry::PDash(v) => &v.use_weapon_code,
            SkillEntry::MBlow(v) => &v.use_weapon_code,
            SkillEntry::BBerserker(v) => &v.use_weapon_code,
            SkillEntry::ScoutHawk(v) => &v.use_weapon_code,
            SkillEntry::ShootingMastery(v) => &v.use_weapon_code,
            SkillEntry::WindArrow(v) => &v.use_weapon_code,
            SkillEntry::PerfectAim(v) => &v.use_weapon_code,
            SkillEntry::DionsEye(v) => &v.use_weapon_code,
            SkillEntry::Falcon(v) => &v.use_weapon_code,
            SkillEntry::ArrowOfRage(v) => &v.use_weapon_code,
            SkillEntry::Avalanche(v) => &v.use_weapon_code,
            SkillEntry::ElementalShot(v) => &v.use_weapon_code,
            SkillEntry::GoldenFalcon(v) => &v.use_weapon_code,
            SkillEntry::BombShot(v) => &v.use_weapon_code,
            SkillEntry::Perforation(v) => &v.use_weapon_code,
            SkillEntry::RecallWolverin(v) => &v.use_weapon_code,
            SkillEntry::EvasionMastery(v) => &v.use_weapon_code,
            SkillEntry::PhoenixShot(v) => &v.use_weapon_code,
            SkillEntry::ForceOfNature(v) => &v.use_weapon_code,
            SkillEntry::EShot(v) => &v.use_weapon_code,
            SkillEntry::SRope(v) => &v.use_weapon_code,
            SkillEntry::NSplash(v) => &v.use_weapon_code,
            SkillEntry::CTrap(v) => &v.use_weapon_code,
            SkillEntry::SwordBlast(v) => &v.use_weapon_code,
            SkillEntry::HolyBody(v) => &v.use_weapon_code,
            SkillEntry::PhysicalTraining(v) => &v.use_weapon_code,
            SkillEntry::DoubleCrash(v) => &v.use_weapon_code,
            SkillEntry::HolyValor(v) => &v.use_weapon_code,
            SkillEntry::Brandish(v) => &v.use_weapon_code,
            SkillEntry::Piercing(v) => &v.use_weapon_code,
            SkillEntry::DrasticSpirit(v) => &v.use_weapon_code,
            SkillEntry::SwordMastery(v) => &v.use_weapon_code,
            SkillEntry::DivineInhalation(v) => &v.use_weapon_code,
            SkillEntry::HolyIncantation(v) => &v.use_weapon_code,
            SkillEntry::GrandCross(v) => &v.use_weapon_code,
            SkillEntry::DivinePiercing(v) => &v.use_weapon_code,
            SkillEntry::GodlyShield(v) => &v.use_weapon_code,
            SkillEntry::GodBless(v) => &v.use_weapon_code,
            SkillEntry::SwordOfJustice(v) => &v.use_weapon_code,
            SkillEntry::SBreaker(v) => &v.use_weapon_code,
            SkillEntry::CMoon(v) => &v.use_weapon_code,
            SkillEntry::SBlade(v) => &v.use_weapon_code,
            SkillEntry::HBenedic(v) => &v.use_weapon_code,
            SkillEntry::ShieldStrike(v) => &v.use_weapon_code,
            SkillEntry::Farina(v) => &v.use_weapon_code,
            SkillEntry::ThrowingMastery(v) => &v.use_weapon_code,
            SkillEntry::VigorSpear(v) => &v.use_weapon_code,
            SkillEntry::Windy(v) => &v.use_weapon_code,
            SkillEntry::TwistJavelin(v) => &v.use_weapon_code,
            SkillEntry::SoulSucker(v) => &v.use_weapon_code,
            SkillEntry::FireJavelin(v) => &v.use_weapon_code,
            SkillEntry::SplitJavelin(v) => &v.use_weapon_code,
            SkillEntry::TriumphOfValhalla(v) => &v.use_weapon_code,
            SkillEntry::LightningJavelin(v) => &v.use_weapon_code,
            SkillEntry::StormJavelin(v) => &v.use_weapon_code,
            SkillEntry::HallOfValhalla(v) => &v.use_weapon_code,
            SkillEntry::XRage(v) => &v.use_weapon_code,
            SkillEntry::FrostJavelin(v) => &v.use_weapon_code,
            SkillEntry::Vengeance(v) => &v.use_weapon_code,
            SkillEntry::Talaria(v) => &v.use_weapon_code,
            SkillEntry::GCoup(v) => &v.use_weapon_code,
            SkillEntry::Arcuda(v) => &v.use_weapon_code,
            SkillEntry::SFear(v) => &v.use_weapon_code,
            SkillEntry::Healing(v) => &v.use_weapon_code,
            SkillEntry::HolyBolt(v) => &v.use_weapon_code,
            SkillEntry::MultiSpark(v) => &v.use_weapon_code,
            SkillEntry::HolyMind(v) => &v.use_weapon_code,
            SkillEntry::Meditation(v) => &v.use_weapon_code,
            SkillEntry::DivineLightning(v) => &v.use_weapon_code,
            SkillEntry::HolyReflection(v) => &v.use_weapon_code,
            SkillEntry::GrandHealing(v) => &v.use_weapon_code,
            SkillEntry::VigorBall(v) => &v.use_weapon_code,
            SkillEntry::Resurrection(v) => &v.use_weapon_code,
            SkillEntry::Extinction(v) => &v.use_weapon_code,
            SkillEntry::VirtualLife(v) => &v.use_weapon_code,
            SkillEntry::GlacialSpike(v) => &v.use_weapon_code,
            SkillEntry::RegenerationField(v) => &v.use_weapon_code,
            SkillEntry::ChainLightning(v) => &v.use_weapon_code,
            SkillEntry::SummonMuspell(v) => &v.use_weapon_code,
            SkillEntry::SImpact(v) => &v.use_weapon_code,
            SkillEntry::PIce(v) => &v.use_weapon_code,
            SkillEntry::SummonRamiel(v) => &v.use_weapon_code,
            SkillEntry::Krishna(v) => &v.use_weapon_code,
            SkillEntry::Agony(v) => &v.use_weapon_code,
            SkillEntry::FireBolt(v) => &v.use_weapon_code,
            SkillEntry::Zenith(v) => &v.use_weapon_code,
            SkillEntry::FireBall(v) => &v.use_weapon_code,
            SkillEntry::MentalMastery(v) => &v.use_weapon_code,
            SkillEntry::Watornado(v) => &v.use_weapon_code,
            SkillEntry::EnchantWeapon(v) => &v.use_weapon_code,
            SkillEntry::DeadRay(v) => &v.use_weapon_code,
            SkillEntry::EnergyShield(v) => &v.use_weapon_code,
            SkillEntry::Diastrophism(v) => &v.use_weapon_code,
            SkillEntry::SpiritElemental(v) => &v.use_weapon_code,
            SkillEntry::DancingSword(v) => &v.use_weapon_code,
            SkillEntry::FireElemental(v) => &v.use_weapon_code,
            SkillEntry::FlameWave(v) => &v.use_weapon_code,
            SkillEntry::Distortion(v) => &v.use_weapon_code,
            SkillEntry::Meteo(v) => &v.use_weapon_code,
            SkillEntry::Silraphim(v) => &v.use_weapon_code,
            SkillEntry::Tenus(v) => &v.use_weapon_code,
            SkillEntry::Ignis(v) => &v.use_weapon_code,
            SkillEntry::Anima(v) => &v.use_weapon_code,
            SkillEntry::Stinger(v) => &v.use_weapon_code,
            SkillEntry::DoubleBlow(v) => &v.use_weapon_code,
            SkillEntry::DMastery(v) => &v.use_weapon_code,
            SkillEntry::Wisp(v) => &v.use_weapon_code,
            SkillEntry::VenomThorn(v) => &v.use_weapon_code,
            SkillEntry::Alas(v) => &v.use_weapon_code,
            SkillEntry::SoulShock(v) => &v.use_weapon_code,
            SkillEntry::AMastery(v) => &v.use_weapon_code,
            SkillEntry::SoreSword(v) => &v.use_weapon_code,
            SkillEntry::BeatUp(v) => &v.use_weapon_code,
            SkillEntry::Inpes(v) => &v.use_weapon_code,
            SkillEntry::Blind(v) => &v.use_weapon_code,
            SkillEntry::FrostWind(v) => &v.use_weapon_code,
            SkillEntry::FMastery(v) => &v.use_weapon_code,
            SkillEntry::Polluted(v) => &v.use_weapon_code,
            SkillEntry::PastingShadow(v) => &v.use_weapon_code,
            SkillEntry::DarkBolt(v) => &v.use_weapon_code,
            SkillEntry::InnerPeace(v) => &v.use_weapon_code,
            SkillEntry::SoulManacle(v) => &v.use_weapon_code,
            SkillEntry::Haunt(v) => &v.use_weapon_code,
            SkillEntry::Scratch(v) => &v.use_weapon_code,
            SkillEntry::Judgement(v) => &v.use_weapon_code,
            SkillEntry::DarkWave(v) => &v.use_weapon_code,
            SkillEntry::CurseLazy(v) => &v.use_weapon_code,
            SkillEntry::SpiritualFlare(v) => &v.use_weapon_code,
            SkillEntry::ChasingHunt(v) => &v.use_weapon_code,
            SkillEntry::LandGhost(v) => &v.use_weapon_code,
            SkillEntry::MourningOfPray(v) => &v.use_weapon_code,
            SkillEntry::BloodyKnight(v) => &v.use_weapon_code,
            SkillEntry::AdventMigal(v) => &v.use_weapon_code,
            SkillEntry::RainMaker(v) => &v.use_weapon_code,
            SkillEntry::AdventMidranda(v) => &v.use_weapon_code,
            SkillEntry::LowKick(v) => &v.use_weapon_code,
            SkillEntry::SwordMasteryMartial(v) => &v.use_weapon_code,
            SkillEntry::DoubleBlowMartial(v) => &v.use_weapon_code,
            SkillEntry::Straight(v) => &v.use_weapon_code,
            SkillEntry::RageUp(v) => &v.use_weapon_code,
            SkillEntry::Patriot(v) => &v.use_weapon_code,
            SkillEntry::Elbow(v) => &v.use_weapon_code,
            SkillEntry::SwordMastery2(v) => &v.use_weapon_code,
            SkillEntry::Bulkup(v) => &v.use_weapon_code,
            SkillEntry::Warcry(v) => &v.use_weapon_code,
            SkillEntry::Cannon(v) => &v.use_weapon_code,
            SkillEntry::HeelKick(v) => &v.use_weapon_code,
            SkillEntry::Combination(v) => &v.use_weapon_code,
            SkillEntry::Steelers(v) => &v.use_weapon_code,
            SkillEntry::Check(v) => &v.use_weapon_code,
            SkillEntry::Typhoon(v) => &v.use_weapon_code,
            SkillEntry::JBomb(v) => &v.use_weapon_code,
            SkillEntry::RSlash(v) => &v.use_weapon_code,
            SkillEntry::VStab(v) => &v.use_weapon_code,
            SkillEntry::Storm(v) => &v.use_weapon_code,
            SkillEntry::Creed(v) => &v.use_weapon_code,
            SkillEntry::PDeity(v) => &v.use_weapon_code,
            SkillEntry::GNail(v) => &v.use_weapon_code,
            SkillEntry::HRegene(v) => &v.use_weapon_code,
            SkillEntry::DMastery2(v) => &v.use_weapon_code,
            SkillEntry::HHawk(v) => &v.use_weapon_code,
            SkillEntry::LBreaking(v) => &v.use_weapon_code,
            SkillEntry::HTraining(v) => &v.use_weapon_code,
            SkillEntry::Amplified(_) => &[],
        }
    }

    pub fn get_info(&self) -> (&str, &str, u8) {
        match self {
            SkillEntry::PikeWind(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::IceAttribute(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::CriticalHit(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::JumpingCrash(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::GroundPike(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Tornado(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::WeaponDefenseMastery(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Expansion(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::VenomSpear(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Vanish(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::CriticalMastery(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::ChainLance(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::AssassinEye(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::ChargingStrike(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Vague(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::ShadowMaster(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::DReaper(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::FSpear(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::SsAttack(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::ExtremeShield(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::MechanicBomb(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::PhysicalAbsorb(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::PoisonAttribute(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::GreatSmash(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Maximize(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Automation(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Spark(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::MetalArmor(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::GrandSmash(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::MechanicWeaponMastery(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::SparkShield(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Impulsion(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Compulsion(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::MagneticSphere(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::MetalGolem(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::LandMine(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::HyperSonic(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::RSmash(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::PhysicalEnhance(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::MeleeMastery(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::FireAttribute(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Raving(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Impact(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::TripleImpact(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::BrutalSwing(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Roar(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::RageOfZecram(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Concentration(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::AvangingCrash(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::SwiftAxe(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::BoneCrash(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Destoryer(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Berserker(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::CycloneStrike(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::BoostHealth(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::DHit(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::PDash(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::MBlow(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::BBerserker(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::ScoutHawk(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::ShootingMastery(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::WindArrow(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::PerfectAim(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::DionsEye(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Falcon(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::ArrowOfRage(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Avalanche(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::ElementalShot(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::GoldenFalcon(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::BombShot(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Perforation(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::RecallWolverin(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::EvasionMastery(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::PhoenixShot(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::ForceOfNature(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::EShot(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::SRope(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::NSplash(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::CTrap(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::SwordBlast(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::HolyBody(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::PhysicalTraining(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::DoubleCrash(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::HolyValor(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Brandish(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Piercing(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::DrasticSpirit(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::SwordMastery(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::DivineInhalation(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::HolyIncantation(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::GrandCross(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::DivinePiercing(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::GodlyShield(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::GodBless(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::SwordOfJustice(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::SBreaker(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::CMoon(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::SBlade(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::HBenedic(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::ShieldStrike(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Farina(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::ThrowingMastery(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::VigorSpear(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Windy(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::TwistJavelin(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::SoulSucker(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::FireJavelin(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::SplitJavelin(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::TriumphOfValhalla(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::LightningJavelin(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::StormJavelin(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::HallOfValhalla(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::XRage(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::FrostJavelin(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Vengeance(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Talaria(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::GCoup(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Arcuda(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::SFear(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Healing(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::HolyBolt(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::MultiSpark(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::HolyMind(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Meditation(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::DivineLightning(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::HolyReflection(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::GrandHealing(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::VigorBall(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Resurrection(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Extinction(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::VirtualLife(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::GlacialSpike(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::RegenerationField(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::ChainLightning(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::SummonMuspell(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::SImpact(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::PIce(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::SummonRamiel(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Krishna(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Agony(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::FireBolt(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Zenith(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::FireBall(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::MentalMastery(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Watornado(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::EnchantWeapon(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::DeadRay(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::EnergyShield(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Diastrophism(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::SpiritElemental(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::DancingSword(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::FireElemental(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::FlameWave(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Distortion(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Meteo(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Silraphim(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Tenus(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Ignis(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Anima(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Stinger(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::DoubleBlow(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::DMastery(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Wisp(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::VenomThorn(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Alas(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::SoulShock(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::AMastery(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::SoreSword(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::BeatUp(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Inpes(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Blind(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::FrostWind(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::FMastery(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Polluted(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::PastingShadow(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::DarkBolt(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::InnerPeace(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::SoulManacle(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Haunt(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Scratch(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Judgement(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::DarkWave(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::CurseLazy(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::SpiritualFlare(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::ChasingHunt(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::LandGhost(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::MourningOfPray(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::BloodyKnight(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::AdventMigal(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::RainMaker(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::AdventMidranda(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::LowKick(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::SwordMasteryMartial(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::DoubleBlowMartial(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Straight(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::RageUp(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Patriot(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Elbow(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::SwordMastery2(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Bulkup(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Warcry(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Cannon(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::HeelKick(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Combination(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Steelers(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Check(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Typhoon(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::JBomb(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::RSlash(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::VStab(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Storm(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Creed(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::PDeity(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::GNail(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::HRegene(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::DMastery2(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::HHawk(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::LBreaking(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::HTraining(v) => (&v.name, &v.description, v.require_level),
            SkillEntry::Amplified(v) => (&v.name, &v.description, v.require_level),
        }
    }

    pub fn use_weapon_codes_array(&self) -> [Option<EquippableItemType>; 8] {
        let mut out = [None; 8];
        for (idx, item_type) in self.use_weapon_code().iter().take(8).enumerate() {
            out[idx] = Some(*item_type);
        }
        out
    }
}

impl SkillLevel {
    pub const MIN: u8 = 1;
    pub const MAX: u8 = 10;

    pub const fn new(level: u8) -> Option<Self> {
        if level >= Self::MIN && level <= Self::MAX {
            Some(Self(level))
        } else {
            None
        }
    }

    pub const fn new_unchecked(level: u8) -> Self {
        Self(level)
    }

    #[inline(always)]
    pub const fn index(self) -> usize {
        (self.0 - 1) as usize
    }

    pub const fn get(self) -> u8 {
        self.0
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillValueType {
    Permille,
    Fixed2,
}

pub const SKILL_LEVELS: usize = 10;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Levels<T> {
    value_type: SkillValueType,
    value: [T; SKILL_LEVELS],
}

impl<T: Copy> Levels<T> {
    pub const fn new(value_type: SkillValueType, value: [T; SKILL_LEVELS]) -> Self {
        Self { value_type, value }
    }

    #[inline(always)]
    pub fn at(&self, level: SkillLevel) -> T {
        self.value[level.index()]
    }

    pub const fn value_type(&self) -> SkillValueType {
        self.value_type
    }

    pub const fn values(&self) -> &[T; SKILL_LEVELS] {
        &self.value
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DualLevels<T> {
    value_type: SkillValueType,
    value: [[T; SKILL_LEVELS]; 2],
}

impl<T: Copy> DualLevels<T> {
    pub const fn new(value_type: SkillValueType, value: [[T; SKILL_LEVELS]; 2]) -> Self {
        Self { value_type, value }
    }

    #[inline(always)]
    pub fn at(&self, level: SkillLevel) -> (T, T) {
        (self.value[0][level.index()], self.value[1][level.index()])
    }

    pub const fn value_type(&self) -> SkillValueType {
        self.value_type
    }

    pub const fn values(&self) -> &[[T; SKILL_LEVELS]; 2] {
        &self.value
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PikemanSkills {
    pub pike_wind: PikeWind,
    pub ice_attribute: IceAttribute,
    pub critical_hit: CriticalHit,
    pub jumping_crash: JumpingCrash,
    pub ground_pike: GroundPike,
    pub tornado: Tornado,
    pub weapon_defense_mastery: WeaponDefenseMastery,
    pub expansion: Expansion,
    pub venom_spear: VenomSpear,
    pub vanish: Vanish,
    pub critical_mastery: CriticalMastery,
    pub chain_lance: ChainLance,
    pub assassin_eye: AssassinEye,
    pub charging_strike: ChargingStrike,
    pub vague: Vague,
    pub shadow_master: ShadowMaster,
    pub d_reaper: DReaper,
    pub f_spear: FSpear,
    pub ss_attack: SsAttack,
    pub amplified: Amplified,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PikeWind {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: DualLevels<i32>,
    pub push_length: Levels<i32>,
    pub throw_height: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct IceAttribute {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub plus_ice: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CriticalHit {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub critical: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct JumpingCrash {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub use_mana: Levels<i32>,
    pub attack_rating: Levels<i32>,
    pub damage: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct GroundPike {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: DualLevels<i32>,
    pub range: Levels<i32>,
    pub use_mana: Levels<i32>,
    pub time: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Tornado {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub stun: Levels<i32>,
    pub range: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct WeaponDefenseMastery {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub block: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Expansion {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub size: Levels<i32>,
    pub use_mana: Levels<i32>,
    pub damage: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct VenomSpear {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: DualLevels<i32>,
    pub num: Levels<i32>,
    pub time: Levels<i32>,
    pub range: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Vanish {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub speed: Levels<f32>,
    pub damage: Levels<i32>,
    pub range: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CriticalMastery {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub critical: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ChainLance {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct AssassinEye {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub add_critical: Levels<i32>,
    pub m_sub_critical: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ChargingStrike {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage_percent: Levels<i32>,
    pub charing_damage_percent: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Vague {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub evasion_percent: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ShadowMaster {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage_percent: Levels<i32>,
    pub add_hit: Levels<i32>,
    pub shadow_num: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DReaper {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub add_atk: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct FSpear {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: DualLevels<i32>,
    pub num: Levels<i32>,
    pub area: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SsAttack {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub absorb: Levels<i32>,
    pub rating: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Amplified {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub add_attack: Levels<i32>,
    pub add_min_attack: Levels<i32>,
    pub sub_absorb: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MechanicianSkills {
    pub extreme_shield: ExtremeShield,
    pub mechanic_bomb: MechanicBomb,
    pub physical_absorb: PhysicalAbsorb,
    pub poison_attribute: PoisonAttribute,
    pub great_smash: GreatSmash,
    pub maximize: Maximize,
    pub automation: Automation,
    pub spark: Spark,
    pub metal_armor: MetalArmor,
    pub grand_smash: GrandSmash,
    pub mechanic_weapon_mastery: MechanicWeaponMastery,
    pub spark_shield: SparkShield,
    pub impulsion: Impulsion,
    pub compulsion: Compulsion,
    pub magnetic_sphere: MagneticSphere,
    pub metal_golem: MetalGolem,
    pub land_mine: LandMine,
    pub hyper_sonic: HyperSonic,
    pub r_smash: RSmash,
    pub physical_enhance: PhysicalEnhance,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ExtremeShield {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub block_rate: Levels<i32>,
    pub use_time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MechanicBomb {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: DualLevels<i32>,
    pub attack_range: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PhysicalAbsorb {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub absorb: DualLevels<i32>,
    pub use_time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PoisonAttribute {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub plus_poison: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct GreatSmash {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub attack_rate: Levels<i32>,
    pub attack_range: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Maximize {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub weapon_speed: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Automation {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub speed: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Spark {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: DualLevels<i32>,
    pub area: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MetalArmor {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub defense: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct GrandSmash {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub attack_rate: Levels<i32>,
    pub damage: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MechanicWeaponMastery {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub mastery: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SparkShield {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub defense: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Impulsion {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub lightning_damage: DualLevels<i32>,
    pub range: Levels<i32>,
    pub lightning_num: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Compulsion {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub add_absorb: Levels<i32>,
    pub area: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MagneticSphere {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub add_damage: DualLevels<i32>,
    pub area: Levels<i32>,
    pub attack_delay: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MetalGolem {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: DualLevels<i32>,
    pub hit: Levels<i32>,
    pub defense: Levels<i32>,
    pub life: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct LandMine {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage_percent: Levels<i32>,
    pub area_damage: DualLevels<i32>,
    pub attack_num: Levels<i32>,
    pub area: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct HyperSonic {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub speed: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RSmash {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub rate: Levels<i32>,
    pub damage: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PhysicalEnhance {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: DualLevels<i32>,
    pub area: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct FighterSkills {
    pub melee_mastery: MeleeMastery,
    pub fire_attribute: FireAttribute,
    pub raving: Raving,
    pub impact: Impact,
    pub triple_impact: TripleImpact,
    pub brutal_swing: BrutalSwing,
    pub roar: Roar,
    pub rage_of_zecram: RageOfZecram,
    pub concentration: Concentration,
    pub avanging_crash: AvangingCrash,
    pub swift_axe: SwiftAxe,
    pub bone_crash: BoneCrash,
    pub destoryer: Destoryer,
    pub berserker: Berserker,
    pub cyclone_strike: CycloneStrike,
    pub boost_health: BoostHealth,
    pub d_hit: DHit,
    pub p_dash: PDash,
    pub m_blow: MBlow,
    pub b_berserker: BBerserker,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MeleeMastery {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage_percent: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct FireAttribute {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub plus_fire: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Raving {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub speed: Levels<i32>,
    pub use_life: Levels<f32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Impact {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub attack_rating: Levels<i32>,
    pub damage: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct TripleImpact {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub hit: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct BrutalSwing {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub critical: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Roar {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub range: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RageOfZecram {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Concentration {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub attack_rate: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct AvangingCrash {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub attack_rate: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SwiftAxe {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub speed: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct BoneCrash {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub demon_damage: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Destoryer {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage_percent: Levels<i32>,
    pub add_critical: Levels<i32>,
    pub attack_rate: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Berserker {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub add_attack: Levels<i32>,
    pub sub_absorb: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CycloneStrike {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    pub damage_percent: Levels<i32>,
    pub area_damage: DualLevels<i32>,
    pub attack_num: Levels<i32>,
    pub area: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct BoostHealth {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub life: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DHit {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PDash {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub range: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MBlow {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub range_damage: DualLevels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct BBerserker {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub add_attack: Levels<i32>,
    pub sub_absorb: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ArcherSkills {
    pub scout_hawk: ScoutHawk,
    pub shooting_mastery: ShootingMastery,
    pub wind_arrow: WindArrow,
    pub perfect_aim: PerfectAim,
    pub dions_eye: DionsEye,
    pub falcon: Falcon,
    pub arrow_of_rage: ArrowOfRage,
    pub avalanche: Avalanche,
    pub elemental_shot: ElementalShot,
    pub golden_falcon: GoldenFalcon,
    pub bomb_shot: BombShot,
    pub perforation: Perforation,
    pub recall_wolverin: RecallWolverin,
    pub evasion_mastery: EvasionMastery,
    pub phoenix_shot: PhoenixShot,
    pub force_of_nature: ForceOfNature,
    pub e_shot: EShot,
    pub s_rope: SRope,
    pub n_splash: NSplash,
    pub c_trap: CTrap,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ScoutHawk {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub attack_rate: Levels<i32>,
    pub use_time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ShootingMastery {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage_percent: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct WindArrow {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub speed: Levels<i32>,
    pub damage: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PerfectAim {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub attack_rate: Levels<i32>,
    pub damage: Levels<i32>,
    pub damage_lv: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DionsEye {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub attack_rate: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Falcon {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: DualLevels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ArrowOfRage {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub arrow_num: Levels<i32>,
    pub damage: Levels<i32>,
    pub area: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Avalanche {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub speed: Levels<i32>,
    pub damage: Levels<i32>,
    pub arrow_num: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ElementalShot {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub ice: DualLevels<i32>,
    pub fire: DualLevels<i32>,
    pub lightning: DualLevels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct GoldenFalcon {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub life_regen: Levels<f32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct BombShot {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub demon_damage: Levels<i32>,
    pub add_damage: DualLevels<i32>,
    pub area: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Perforation {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub critical: Levels<i32>,
    pub use_mana: Levels<i32>,
    pub attack_range: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RecallWolverin {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: DualLevels<i32>,
    pub hit: Levels<i32>,
    pub defense: Levels<i32>,
    pub life: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct EvasionMastery {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub add_percent: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PhoenixShot {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage_percent: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ForceOfNature {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub add_damage: Levels<i32>,
    pub add_hit: Levels<i32>,
    pub falcon_add_damage: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct EShot {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SRope {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub range: Levels<i32>,
    pub percent: Levels<i32>,
    pub damage: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct NSplash {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub range: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CTrap {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub range: Levels<i32>,
    pub delay1: Levels<i32>,
    pub delay2: Levels<f32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct KnightSkills {
    pub sword_blast: SwordBlast,
    pub holy_body: HolyBody,
    pub physical_training: PhysicalTraining,
    pub double_crash: DoubleCrash,
    pub holy_valor: HolyValor,
    pub brandish: Brandish,
    pub piercing: Piercing,
    pub drastic_spirit: DrasticSpirit,
    pub sword_mastery: SwordMastery,
    pub divine_inhalation: DivineInhalation,
    pub holy_incantation: HolyIncantation,
    pub grand_cross: GrandCross,
    pub divine_piercing: DivinePiercing,
    pub godly_shield: GodlyShield,
    pub god_bless: GodBless,
    pub sword_of_justice: SwordOfJustice,
    pub s_breaker: SBreaker,
    pub c_moon: CMoon,
    pub s_blade: SBlade,
    pub h_benedic: HBenedic,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SwordBlast {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub use_mana: Levels<i32>,
    pub damage: DualLevels<i32>,
    pub shooting_range: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct HolyBody {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub use_mana: Levels<i32>,
    pub absorb: Levels<i32>,
    pub time: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PhysicalTraining {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub stamina: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DoubleCrash {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub use_mana: Levels<i32>,
    pub damage: Levels<i32>,
    pub critical: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct HolyValor {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Brandish {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub range: Levels<i32>,
    pub damage: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Piercing {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub attack_rating: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DrasticSpirit {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub defense: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SwordMastery {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage_percent: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DivineInhalation {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub life: Levels<i32>,
    pub block: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct HolyIncantation {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub success: Levels<i32>,
    pub add_life: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct GrandCross {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub attack_rate: Levels<i32>,
    pub undead_damage: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DivinePiercing {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage_percent: Levels<i32>,
    pub add_hit: Levels<i32>,
    pub attack_num: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct GodlyShield {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub absorb_percent: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct GodBless {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub add_damage: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SwordOfJustice {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage_percent: Levels<i32>,
    pub area: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SBreaker {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub attack: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CMoon {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub range: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SBlade {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub undead_damage: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct HBenedic {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub undead_damage: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct AtalantaSkills {
    pub shield_strike: ShieldStrike,
    pub farina: Farina,
    pub throwing_mastery: ThrowingMastery,
    pub vigor_spear: VigorSpear,
    pub windy: Windy,
    pub twist_javelin: TwistJavelin,
    pub soul_sucker: SoulSucker,
    pub fire_javelin: FireJavelin,
    pub split_javelin: SplitJavelin,
    pub triumph_of_valhalla: TriumphOfValhalla,
    pub lightning_javelin: LightningJavelin,
    pub storm_javelin: StormJavelin,
    pub hall_of_valhalla: HallOfValhalla,
    pub x_rage: XRage,
    pub frost_javelin: FrostJavelin,
    pub vengeance: Vengeance,
    pub talaria: Talaria,
    pub g_coup: GCoup,
    pub arcuda: Arcuda,
    pub s_fear: SFear,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ShieldStrike {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub use_mana: Levels<i32>,
    pub damage: DualLevels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Farina {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub use_mana: Levels<i32>,
    pub speed: Levels<i32>,
    pub attack_rate: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ThrowingMastery {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct VigorSpear {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub use_mana: Levels<i32>,
    pub damage: DualLevels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Windy {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub attack_rating: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct TwistJavelin {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub attack_rating: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SoulSucker {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub absorb: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct FireJavelin {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: DualLevels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SplitJavelin {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub attack_num: Levels<i32>,
    pub damage: Levels<i32>,
    pub attack_rate: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct TriumphOfValhalla {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct LightningJavelin {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: DualLevels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct StormJavelin {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct HallOfValhalla {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub add_evasion: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct XRage {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage_percent: Levels<i32>,
    pub area: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct FrostJavelin {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub ice_add_damage: DualLevels<i32>,
    pub speed_sub_percent: Levels<i32>,
    pub ice_time: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Vengeance {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage_percent: Levels<i32>,
    pub attack_rate: Levels<i32>,
    pub add_critical: Levels<i32>,
    pub range: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Talaria {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub speed: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct GCoup {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub area: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Arcuda {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: DualLevels<i32>,
    pub hit: Levels<i32>,
    pub defense: Levels<i32>,
    pub life: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SFear {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage_percent: Levels<i32>,
    pub range: Levels<i32>,
    pub use_mana: Levels<i32>,
    pub add_critical: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PriestessSkills {
    pub healing: Healing,
    pub holy_bolt: HolyBolt,
    pub multi_spark: MultiSpark,
    pub holy_mind: HolyMind,
    pub meditation: Meditation,
    pub divine_lightning: DivineLightning,
    pub holy_reflection: HolyReflection,
    pub grand_healing: GrandHealing,
    pub vigor_ball: VigorBall,
    pub resurrection: Resurrection,
    pub extinction: Extinction,
    pub virtual_life: VirtualLife,
    pub glacial_spike: GlacialSpike,
    pub regeneration_field: RegenerationField,
    pub chain_lightning: ChainLightning,
    pub summon_muspell: SummonMuspell,
    pub s_impact: SImpact,
    pub p_ice: PIce,
    pub summon_ramiel: SummonRamiel,
    pub krishna: Krishna,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Healing {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub use_mana: Levels<i32>,
    pub heal: DualLevels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct HolyBolt {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub use_mana: Levels<i32>,
    pub damage: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MultiSpark {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub use_mana: Levels<i32>,
    pub damage: Levels<i32>,
    pub num: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct HolyMind {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub use_mana: Levels<i32>,
    pub dec_damage: Levels<i32>,
    pub time: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Meditation {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub regen: Levels<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DivineLightning {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: DualLevels<i32>,
    pub num: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct HolyReflection {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub return_damage: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct GrandHealing {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub heal: DualLevels<i32>,
    pub range: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct VigorBall {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Resurrection {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub percent: Levels<i32>,
    pub exp: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Extinction {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub percent: Levels<i32>,
    pub amount: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct VirtualLife {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub percent: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct GlacialSpike {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub area: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RegenerationField {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub life_regen: Levels<f32>,
    pub mana_regen: Levels<f32>,
    pub time: Levels<i32>,
    pub area: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ChainLightning {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: DualLevels<i32>,
    pub num: Levels<i32>,
    pub range: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SummonMuspell {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: DualLevels<i32>,
    pub attack_delay: Levels<i32>,
    pub undead_absorb_percent: Levels<i32>,
    pub block_percent: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SImpact {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: DualLevels<i32>,
    pub area: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PIce {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SummonRamiel {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: DualLevels<i32>,
    pub attack_delay: Levels<i32>,
    pub undead_absorb_percent: Levels<i32>,
    pub block_percent: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Krishna {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub defense: Levels<i32>,
    pub absorb: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MagicianSkills {
    pub agony: Agony,
    pub fire_bolt: FireBolt,
    pub zenith: Zenith,
    pub fire_ball: FireBall,
    pub mental_mastery: MentalMastery,
    pub watornado: Watornado,
    pub enchant_weapon: EnchantWeapon,
    pub dead_ray: DeadRay,
    pub energy_shield: EnergyShield,
    pub diastrophism: Diastrophism,
    pub spirit_elemental: SpiritElemental,
    pub dancing_sword: DancingSword,
    pub fire_elemental: FireElemental,
    pub flame_wave: FlameWave,
    pub distortion: Distortion,
    pub meteo: Meteo,
    pub silraphim: Silraphim,
    pub tenus: Tenus,
    pub ignis: Ignis,
    pub anima: Anima,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Agony {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub use_mana: Levels<i32>,
    pub conv_life: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct FireBolt {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub use_mana: Levels<i32>,
    pub damage: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Zenith {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub use_mana: Levels<i32>,
    pub element_damage: Levels<i32>,
    pub time: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct FireBall {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub use_mana: Levels<i32>,
    pub damage: Levels<i32>,
    pub range: Levels<i32>,
    pub area: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MentalMastery {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Watornado {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub range: Levels<i32>,
    pub use_mana: Levels<i32>,
    pub area: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct EnchantWeapon {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage_ice: DualLevels<i32>,
    pub damage_fire: DualLevels<i32>,
    pub damage_lightning: DualLevels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DeadRay {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub range: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct EnergyShield {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub dec_damage: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Diastrophism {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: DualLevels<i32>,
    pub area: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SpiritElemental {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub regen_mana: Levels<f32>,
    pub damage: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DancingSword {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub ice_damage: DualLevels<i32>,
    pub fire_damage: DualLevels<i32>,
    pub attack_delay: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct FireElemental {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: DualLevels<i32>,
    pub hit: Levels<i32>,
    pub life: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct FlameWave {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub fire_damage: Levels<i32>,
    pub area: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Distortion {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub speed_sub_percent: Levels<i32>,
    pub damage_sub_percent: Levels<i32>,
    pub area: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Meteo {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: DualLevels<i32>,
    pub area: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Silraphim {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub area: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Tenus {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub hp: Levels<i32>,
    pub damage: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Ignis {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Anima {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub mana: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct AssassinSkills {
    pub stinger: Stinger,
    pub double_blow: DoubleBlow,
    pub d_mastery: DMastery,
    pub wisp: Wisp,
    pub venom_thorn: VenomThorn,
    pub alas: Alas,
    pub soul_shock: SoulShock,
    pub a_mastery: AMastery,
    pub sore_sword: SoreSword,
    pub beat_up: BeatUp,
    pub inpes: Inpes,
    pub blind: Blind,
    pub frost_wind: FrostWind,
    pub f_mastery: FMastery,
    pub polluted: Polluted,
    pub pasting_shadow: PastingShadow,
    pub j_bomb: JBomb,
    pub r_slash: RSlash,
    pub v_stab: VStab,
    pub storm: Storm,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Stinger {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DoubleBlow {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DMastery {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage_percent: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Wisp {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub use_time: Levels<i32>,
    pub reduction: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct VenomThorn {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub time: Levels<i32>,
    pub chance: Levels<i32>,
    pub damage: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Alas {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
    pub evasion_percent: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SoulShock {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub range: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct AMastery {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub add_percent: Levels<i32>,
    pub add_percent2: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SoreSword {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct BeatUp {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub area: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Inpes {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub time: Levels<i32>,
    pub speed: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Blind {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub time: Levels<i32>,
    pub damage: Levels<i32>,
    pub range: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct FrostWind {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct FMastery {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub critical: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Polluted {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub damage2: DualLevels<i32>,
    pub area: Levels<i32>,
    pub time: Levels<i32>,
    pub attack_num: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PastingShadow {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct JBomb {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub area: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RSlash {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct VStab {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub attack_num: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Storm {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub area: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ShamanSkills {
    pub dark_bolt: DarkBolt,
    pub inner_peace: InnerPeace,
    pub soul_manacle: SoulManacle,
    pub haunt: Haunt,
    pub scratch: Scratch,
    pub judgement: Judgement,
    pub dark_wave: DarkWave,
    pub curse_lazy: CurseLazy,
    pub spiritual_flare: SpiritualFlare,
    pub chasing_hunt: ChasingHunt,
    pub land_ghost: LandGhost,
    pub mourning_of_pray: MourningOfPray,
    pub bloody_knight: BloodyKnight,
    pub advent_migal: AdventMigal,
    pub rain_maker: RainMaker,
    pub advent_midranda: AdventMidranda,
    pub creed: Creed,
    pub p_deity: PDeity,
    pub g_nail: GNail,
    pub h_regene: HRegene,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DarkBolt {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct InnerPeace {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub add: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SoulManacle {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Haunt {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Scratch {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Judgement {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub range: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DarkWave {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub area: Levels<i32>,
    pub range: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CurseLazy {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub speed_sub_percent: Levels<i32>,
    pub damage_sub_percent: Levels<i32>,
    pub area: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SpiritualFlare {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub area: Levels<i32>,
    pub range: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ChasingHunt {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub time: Levels<i32>,
    pub area: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct LandGhost {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub range: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MourningOfPray {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub range: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct BloodyKnight {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: DualLevels<i32>,
    pub hit: Levels<i32>,
    pub defense: Levels<i32>,
    pub life: Levels<i32>,
    pub use_mana: Levels<i32>,
    pub time: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct AdventMigal {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub attack_power: Levels<i32>,
    pub use_mana: Levels<i32>,
    pub time: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RainMaker {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub absorb: Levels<i32>,
    pub use_mana: Levels<i32>,
    pub time: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct AdventMidranda {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub use_mana: Levels<i32>,
    pub speed: Levels<i32>,
    pub time: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Creed {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub add_attack: Levels<i32>,
    pub add_attack_rate: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PDeity {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct GNail {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub attack_num: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct HRegene {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub add_hp: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MartialSkills {
    pub low_kick: LowKick,
    pub sword_mastery: SwordMasteryMartial,
    pub double_blow: DoubleBlowMartial,
    pub straight: Straight,
    pub rage_up: RageUp,
    pub patriot: Patriot,
    pub elbow: Elbow,
    pub sword_mastery2: SwordMastery2,
    pub bulkup: Bulkup,
    pub warcry: Warcry,
    pub cannon: Cannon,
    pub heel_kick: HeelKick,
    pub combination: Combination,
    pub steelers: Steelers,
    pub check: Check,
    pub typhoon: Typhoon,
    pub d_mastery2: DMastery2,
    pub h_hawk: HHawk,
    pub l_breaking: LBreaking,
    pub h_training: HTraining,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct LowKick {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: DualLevels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SwordMasteryMartial {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DoubleBlowMartial {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Straight {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub rate: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RageUp {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub speed: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Patriot {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub critical: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Elbow {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SwordMastery2 {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub stamina: Levels<i32>,
    pub rate: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Bulkup {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub hp: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Warcry {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub reduce: Levels<i32>,
    pub range: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Cannon {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct HeelKick {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Combination {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Steelers {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub defense: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Check {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub critical: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Typhoon {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub rate: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DMastery2 {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub add_attack_rate: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct HHawk {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub attack_num: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct LBreaking {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub damage: Levels<i32>,
    pub area: Levels<i32>,
    pub use_mana: Levels<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct HTraining {
    pub name: String,
    pub description: String,
    pub require_level: u8,
    pub use_stamina: Option<(i32, i32)>,
    pub require_mastery: Option<(i32, i32)>,
    pub element: Option<(i32, i32, i32)>,
    pub use_weapon_code: Vec<EquippableItemType>,
    #[serde(default)]
    pub skill_code: SkillCode,
    pub add_attack: Levels<i32>,
    pub add_attack_rate: Levels<i32>,
    pub time: Levels<i32>,
    pub use_mana: Levels<i32>,
}
