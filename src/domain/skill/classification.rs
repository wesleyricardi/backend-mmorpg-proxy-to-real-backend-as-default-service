use crate::domain::skill::dtos::codes::{
    SkillCode, SKILL_AMPLIFIED, SKILL_ANIMA, SKILL_A_MIDRANDA, SKILL_BERSERKER, SKILL_B_BERSERKER,
    SKILL_B_CHECK, SKILL_COMPULSION, SKILL_CONCENTRATION, SKILL_CREED, SKILL_DISTORTION,
    SKILL_ENERGY_SHIELD, SKILL_EXTREME_SHIELD, SKILL_FORCE_OF_NATURE, SKILL_GOD_BLESS,
    SKILL_GOLDEN_FALCON, SKILL_HALL_OF_VALHALLA, SKILL_HOLY_BODY, SKILL_HOLY_INCANTATION,
    SKILL_HOLY_MIND, SKILL_HOLY_VALOR, SKILL_H_TRAINING, SKILL_INPES, SKILL_I_BULKUP,
    SKILL_KRISHNA, SKILL_PATRIOT, SKILL_PERFECT_AIM, SKILL_PHOENIX_SHOT, SKILL_PHYSICAL_ABSORB,
    SKILL_RAGE_UP, SKILL_REGENERATION_FIELD, SKILL_R_MAKER, SKILL_SPARK_SHIELD,
    SKILL_SPIRIT_ELEMENTAL, SKILL_SS_ATTACK, SKILL_STEELERS, SKILL_SWIFT_AXE, SKILL_TALARIA,
    SKILL_TENUS, SKILL_TRIUMPH_OF_VALHALLA, SKILL_VANISH, SKILL_VENGEANCE, SKILL_VIRTUAL_LIFE,
    SKILL_WINDY,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SkillRuntimeKind {
    ContinueSelfBuff,
    TargetBuff,
    PartyBuff,
    SingleTargetAttack,
    AreaAttack,
    Passive,
    Special,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillRuntimeFlagRule {
    AnyPositive,
    Exact(i32),
    OneOf(&'static [i32]),
}

pub fn supports_runtime_kind(code: SkillCode, kind: SkillRuntimeKind) -> bool {
    match kind {
        SkillRuntimeKind::ContinueSelfBuff => matches!(
            code,
            SKILL_EXTREME_SHIELD
                | SKILL_PHYSICAL_ABSORB
                | SKILL_SPARK_SHIELD
                | SKILL_COMPULSION
                | SKILL_HOLY_BODY
                | SKILL_HOLY_VALOR
                | SKILL_CONCENTRATION
                | SKILL_SWIFT_AXE
                | SKILL_BERSERKER
                | SKILL_B_BERSERKER
                | SKILL_WINDY
                | SKILL_VENGEANCE
                | SKILL_TALARIA
                | SKILL_VANISH
                | SKILL_AMPLIFIED
                | SKILL_SS_ATTACK
                | SKILL_PERFECT_AIM
                | SKILL_FORCE_OF_NATURE
                | SKILL_GOLDEN_FALCON
                | SKILL_PHOENIX_SHOT
                | SKILL_HOLY_MIND
                | SKILL_KRISHNA
                | SKILL_ENERGY_SHIELD
                | SKILL_SPIRIT_ELEMENTAL
                | SKILL_TENUS
                | SKILL_ANIMA
                | SKILL_INPES
                | SKILL_RAGE_UP
                | SKILL_I_BULKUP
                | SKILL_STEELERS
                | SKILL_PATRIOT
                | SKILL_B_CHECK
                | SKILL_R_MAKER
                | SKILL_A_MIDRANDA
                | SKILL_DISTORTION
                | SKILL_CREED
                | SKILL_H_TRAINING
                | SKILL_VIRTUAL_LIFE
        ),
        SkillRuntimeKind::TargetBuff => {
            matches!(code, SKILL_TRIUMPH_OF_VALHALLA | SKILL_VIRTUAL_LIFE)
        }
        SkillRuntimeKind::PartyBuff => matches!(
            code,
            SKILL_HOLY_INCANTATION
                | SKILL_GOD_BLESS
                | SKILL_TRIUMPH_OF_VALHALLA
                | SKILL_HALL_OF_VALHALLA
                | SKILL_REGENERATION_FIELD
                | SKILL_VIRTUAL_LIFE
        ),
        SkillRuntimeKind::SingleTargetAttack => false,
        SkillRuntimeKind::AreaAttack => false,
        SkillRuntimeKind::Passive => code.is_passive(),
        SkillRuntimeKind::Special => false,
    }
}

pub fn is_party_skill(code: SkillCode) -> bool {
    supports_runtime_kind(code, SkillRuntimeKind::PartyBuff)
}

pub fn is_target_buff_skill(code: SkillCode) -> bool {
    supports_runtime_kind(code, SkillRuntimeKind::TargetBuff)
}

pub fn skill_runtime_flag_rule(code: SkillCode) -> SkillRuntimeFlagRule {
    if is_party_skill(code) {
        return SkillRuntimeFlagRule::OneOf(&[1, 2]);
    }

    if supports_runtime_kind(code, SkillRuntimeKind::ContinueSelfBuff) {
        return SkillRuntimeFlagRule::Exact(1);
    }

    SkillRuntimeFlagRule::AnyPositive
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::skill::dtos::codes::{
        SKILL_CONCENTRATION, SKILL_HALL_OF_VALHALLA, SKILL_NORMAL_ATTACK,
        SKILL_TRIUMPH_OF_VALHALLA, SKILL_VIRTUAL_LIFE,
    };

    #[test]
    fn party_skill_classification_matches_known_party_buffs() {
        assert!(supports_runtime_kind(
            SKILL_HALL_OF_VALHALLA,
            SkillRuntimeKind::PartyBuff
        ));
        assert!(is_party_skill(SKILL_HALL_OF_VALHALLA));
        assert!(!supports_runtime_kind(
            SKILL_CONCENTRATION,
            SkillRuntimeKind::PartyBuff
        ));
    }

    #[test]
    fn virtual_life_supports_multiple_runtime_kinds() {
        assert!(supports_runtime_kind(
            SKILL_VIRTUAL_LIFE,
            SkillRuntimeKind::ContinueSelfBuff
        ));
        assert!(supports_runtime_kind(
            SKILL_VIRTUAL_LIFE,
            SkillRuntimeKind::TargetBuff
        ));
        assert!(supports_runtime_kind(
            SKILL_VIRTUAL_LIFE,
            SkillRuntimeKind::PartyBuff
        ));
    }

    #[test]
    fn target_buff_classification_matches_known_target_buffs() {
        assert!(supports_runtime_kind(
            SKILL_TRIUMPH_OF_VALHALLA,
            SkillRuntimeKind::TargetBuff
        ));
        assert!(is_target_buff_skill(SKILL_TRIUMPH_OF_VALHALLA));
        assert!(!is_target_buff_skill(SKILL_CONCENTRATION));
    }

    #[test]
    fn runtime_flag_rule_uses_semantic_classification() {
        assert_eq!(
            skill_runtime_flag_rule(SKILL_HALL_OF_VALHALLA),
            SkillRuntimeFlagRule::OneOf(&[1, 2])
        );
        assert_eq!(
            skill_runtime_flag_rule(SKILL_CONCENTRATION),
            SkillRuntimeFlagRule::Exact(1)
        );
        assert_eq!(
            skill_runtime_flag_rule(SKILL_NORMAL_ATTACK),
            SkillRuntimeFlagRule::AnyPositive
        );
    }
}
