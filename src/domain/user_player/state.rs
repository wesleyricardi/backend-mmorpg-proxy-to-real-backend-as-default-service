use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::domain::skill::dtos::codes::SkillCode;
use crate::domain::user_player::dto::{
    UserCalcOutput, UserChar, UserContinueSkill, UserGameSave, UserPassiveSkill, UserPlayerData,
    UserRecordItem, UserThrowItem,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserPlayerId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ContinueSkillCode(pub SkillCode);

impl From<SkillCode> for ContinueSkillCode {
    fn from(value: SkillCode) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone)]
pub struct ContinueSkillInstance {
    pub skill: UserContinueSkill,
    pub expires_at: Instant,
    pub instance_id: u64,
    pub cooldown_ready_at: Instant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerStateSource {
    ServerClientRecordData,
    HandlerUpdate,
    WorldLoopTick,
}

// TODO: revover isso, essa informacao nao é necessaria no estado do player.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerIdentityState {
    pub connection_id: usize,
    pub server_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerCoreState {
    pub char_info: UserChar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerInventoryState {
    #[serde(with = "serde_arrays")]
    pub throw_item: [UserThrowItem; 64],
    pub throw_item_count: i32,
    pub item_count: i32,
    pub item_sub_start: i32,
    pub record_items: Vec<UserRecordItem>,
    pub equipped_positions: Vec<i32>,
    pub potion_space: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerProgressionState {
    pub game_save: UserGameSave,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlayerRuntimeState {
    pub active_buff_codes: Vec<SkillCode>,
    pub passive_skill_codes: Vec<SkillCode>,
    #[serde(skip, default)]
    pub active_continue_skills: HashMap<ContinueSkillCode, ContinueSkillInstance>,
    pub passive_skills: Vec<UserPassiveSkill>,
    pub cooldowns: HashMap<SkillCode, u64>,
    pub premium_timers: Vec<(u32, u32)>,
    pub map_position: Option<(i32, i32, i32)>,
    pub target_object_serial: Option<u32>,
    #[serde(skip, default)]
    pub regen_acc_life: f32,
    #[serde(skip, default)]
    pub regen_acc_mana: f32,
    #[serde(skip, default)]
    pub regen_acc_stamina: f32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlayerSocialRuntimeState {
    pub party_id: Option<u32>,
    pub party_members: Vec<UserPlayerId>,
    pub clan_code: Option<u32>,
    pub last_chat_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStateMeta {
    pub state_version: u64,
    pub last_update_unix_secs: u64,
    pub source: PlayerStateSource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPlayerState {
    pub identity: PlayerIdentityState,
    pub core: PlayerCoreState,
    pub inventory: PlayerInventoryState,
    pub progression: PlayerProgressionState,
    pub runtime: PlayerRuntimeState,
    pub social_runtime: PlayerSocialRuntimeState,
    pub meta: PlayerStateMeta,
}

impl UserPlayerState {
    fn now_unix_secs() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|v| v.as_secs())
            .unwrap_or(0)
    }

    fn now_unix_millis() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|v| v.as_millis() as u64)
            .unwrap_or(0)
    }

    fn normalize_equipped_positions(record_items: &[UserRecordItem]) -> Vec<i32> {
        let mut seen = [false; 17];
        let mut out = Vec::with_capacity(17);
        for item in record_items {
            if item.position <= 0 {
                continue;
            }
            let pos = item.position as usize;
            if pos >= seen.len() || seen[pos] {
                continue;
            }
            seen[pos] = true;
            out.push(item.position);
        }
        out
    }

    pub fn from_record_data(
        connection_id: usize,
        user_player_data: UserPlayerData,
        calc_output: UserCalcOutput,
    ) -> Self {
        let mut state = Self {
            identity: PlayerIdentityState {
                connection_id,
                server_id: None,
            },
            core: PlayerCoreState {
                char_info: user_player_data.char_info,
            },
            inventory: PlayerInventoryState {
                throw_item: user_player_data.throw_item,
                throw_item_count: user_player_data.throw_item_count,
                item_count: user_player_data.item_count,
                item_sub_start: user_player_data.item_sub_start,
                equipped_positions: Self::normalize_equipped_positions(
                    &user_player_data.record_items,
                ),
                potion_space: 2,
                record_items: user_player_data.record_items,
            },
            progression: PlayerProgressionState {
                game_save: user_player_data.game_save,
            },
            runtime: PlayerRuntimeState::default(),
            social_runtime: PlayerSocialRuntimeState::default(),
            meta: PlayerStateMeta {
                state_version: 1,
                last_update_unix_secs: Self::now_unix_secs(),
                source: PlayerStateSource::ServerClientRecordData,
            },
        };
        state.apply_calc_output(calc_output);
        state
    }

    pub fn apply_record_snapshot(&mut self, user_player_data: UserPlayerData) {
        self.core.char_info = user_player_data.char_info;

        self.progression.game_save = user_player_data.game_save;

        self.inventory.throw_item = user_player_data.throw_item;
        self.inventory.throw_item_count = user_player_data.throw_item_count;
        self.inventory.item_count = user_player_data.item_count;
        self.inventory.item_sub_start = user_player_data.item_sub_start;
        self.inventory.equipped_positions =
            Self::normalize_equipped_positions(&user_player_data.record_items);
        self.inventory.record_items = user_player_data.record_items;

        self.touch_update(PlayerStateSource::ServerClientRecordData);
    }

    pub fn apply_calc_output(&mut self, calc_output: UserCalcOutput) {
        self.core.char_info = calc_output.updated_char_info;
        self.inventory.potion_space = self.core.char_info.potion_space.max(2);
        self.touch_update(PlayerStateSource::ServerClientRecordData);
    }

    pub fn upsert_active_continue_skill(&mut self, skill: UserContinueSkill) {
        let key = ContinueSkillCode::from(skill.code);
        let now = Instant::now();
        self.runtime.active_continue_skills.insert(
            key,
            ContinueSkillInstance {
                skill: skill.clone(),
                expires_at: now,
                instance_id: 0,
                cooldown_ready_at: now,
            },
        );
        if !self.runtime.active_buff_codes.contains(&skill.code) {
            self.runtime.active_buff_codes.push(skill.code);
        }
        self.touch_update(PlayerStateSource::HandlerUpdate);
    }

    pub fn upsert_continue_skill_instance(&mut self, instance: ContinueSkillInstance) {
        let key = ContinueSkillCode::from(instance.skill.code);
        if !self
            .runtime
            .active_buff_codes
            .contains(&instance.skill.code)
        {
            self.runtime.active_buff_codes.push(instance.skill.code);
        }
        self.runtime.active_continue_skills.insert(key, instance);
        self.touch_update(PlayerStateSource::WorldLoopTick);
    }

    pub fn remove_active_continue_skill(&mut self, code: SkillCode) -> Option<UserContinueSkill> {
        let removed = self
            .runtime
            .active_continue_skills
            .remove(&ContinueSkillCode::from(code))
            .map(|instance| instance.skill);
        self.runtime.active_buff_codes.retain(|v| *v != code);
        self.touch_update(PlayerStateSource::HandlerUpdate);
        removed
    }

    pub fn clear_active_continue_skills(&mut self) -> bool {
        if self.runtime.active_continue_skills.is_empty()
            && self.runtime.active_buff_codes.is_empty()
        {
            return false;
        }

        self.runtime.active_continue_skills.clear();
        self.runtime.active_buff_codes.clear();
        self.touch_update(PlayerStateSource::HandlerUpdate);
        true
    }

    pub fn active_continue_skills_vec(&self) -> Vec<UserContinueSkill> {
        self.runtime
            .active_continue_skills
            .values()
            .map(|v| v.skill.clone())
            .collect()
    }

    pub fn is_skill_on_cooldown(&self, skill_code: SkillCode) -> bool {
        self.runtime
            .cooldowns
            .get(&skill_code)
            .copied()
            .is_some_and(|ready_at_unix_millis| Self::now_unix_millis() < ready_at_unix_millis)
    }

    pub fn set_skill_cooldown(&mut self, skill_code: SkillCode, cooldown_ready_at: Instant) {
        let remaining = cooldown_ready_at
            .saturating_duration_since(Instant::now())
            .as_millis() as u64;
        let ready_at_unix_millis = Self::now_unix_millis().saturating_add(remaining);
        self.runtime
            .cooldowns
            .insert(skill_code, ready_at_unix_millis);
        self.touch_update(PlayerStateSource::WorldLoopTick);
    }

    pub fn clear_skill_cooldown_if_ready(&mut self, skill_code: SkillCode) {
        let now_unix_millis = Self::now_unix_millis();
        let should_remove = self
            .runtime
            .cooldowns
            .get(&skill_code)
            .copied()
            .is_some_and(|ready_at_unix_millis| ready_at_unix_millis <= now_unix_millis);
        if should_remove {
            self.runtime.cooldowns.remove(&skill_code);
            self.touch_update(PlayerStateSource::WorldLoopTick);
        }
    }

    pub fn touch_update(&mut self, source: PlayerStateSource) {
        self.meta.state_version = self.meta.state_version.saturating_add(1);
        self.meta.last_update_unix_secs = Self::now_unix_secs();
        self.meta.source = source;
    }

    pub fn to_user_player_data_snapshot(&self) -> UserPlayerData {
        UserPlayerData {
            char_info: self.core.char_info.clone(),
            game_save: self.progression.game_save.clone(),
            throw_item: self.inventory.throw_item.clone(),
            throw_item_count: self.inventory.throw_item_count,
            item_count: self.inventory.item_count,
            item_sub_start: self.inventory.item_sub_start,
            record_items: self.inventory.record_items.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConnectionPlayerData {
    pub connection_id: usize,
    pub player_state: UserPlayerState,
    pub verification_lock: Arc<Mutex<()>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::user_player::dto::{UserCalcOutput, UserPlayerData};

    #[test]
    fn from_record_data_creates_single_state() {
        let mut data = UserPlayerData::default();
        data.char_info.level = 10;
        data.char_info.attack_rating = 55;
        let mut output = UserCalcOutput::default();
        output.updated_char_info = data.char_info.clone();

        let state = UserPlayerState::from_record_data(7, data, output);
        assert_eq!(state.identity.connection_id, 7);
        assert_eq!(state.core.char_info.level, 10);
        assert_eq!(state.core.char_info.attack_rating, 55);
        assert!(!state.inventory.equipped_positions.contains(&0));
    }

    #[test]
    fn apply_record_and_calc_increment_version() {
        let mut data = UserPlayerData::default();
        data.char_info.level = 10;
        let mut output = UserCalcOutput::default();
        output.updated_char_info = data.char_info.clone();

        let mut state = UserPlayerState::from_record_data(1, data.clone(), output.clone());
        let initial = state.meta.state_version;

        data.char_info.level = 11;
        output.updated_char_info.level = 11;
        state.apply_record_snapshot(data);
        state.apply_calc_output(output);
        assert!(state.meta.state_version > initial);
        assert_eq!(state.core.char_info.level, 11);
    }
}
