use std::collections::HashMap;

use crate::domain::{
    skill::dtos::{
        codes::SkillCode,
        skills::{SkillClassConfig, SkillEntry},
    },
    user_player::dto::PlayerJob,
};

#[derive(Debug, Clone, Default)]
pub struct SkillState {
    pub state: HashMap<PlayerJob, SkillClassConfig>,
}

impl SkillState {
    pub fn get_job_config(&self, job: PlayerJob) -> Option<&SkillClassConfig> {
        self.state.get(&job)
    }

    pub fn get_skill(&self, job: PlayerJob, code: SkillCode) -> Option<&SkillEntry> {
        self.state.get(&job)?.by_code.get(&code)
    }
}
