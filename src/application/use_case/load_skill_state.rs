// src/application/use_case/load_skill_state.rs
use std::collections::HashMap;
use std::path::PathBuf;

use crate::adapters::fileread::read_skill_class_config_from_json_file;
use crate::domain::skill::state::SkillState;
use crate::domain::user_player::dto::PlayerJob;
use crate::error::AppError;

/// Use-case: orquestra o carregamento do estado de skills a partir de uma lista de arquivos.
/// - NÃO depende de serde_json
/// - NÃO faz parse/validação de JSON
/// - NÃO monta path base nem "descobre" arquivos
/// Apenas chama o fileread e constrói o SkillState.
pub fn load_skill_state_from_files(
    files: Vec<(PlayerJob, PathBuf)>,
) -> Result<SkillState, AppError> {
    let mut state = HashMap::with_capacity(files.len());

    for (job, path) in files {
        let config = read_skill_class_config_from_json_file(path.as_path(), job)?;
        state.insert(job, config);
    }

    Ok(SkillState { state })
}
