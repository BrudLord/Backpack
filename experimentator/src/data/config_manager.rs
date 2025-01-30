use crate::models::experiment_config::ExperimentConfig;
use std::fs;
use std::path::Path;

pub struct DataConfigManager;

impl DataConfigManager {
    pub fn read_rand_config<P: AsRef<Path>>(path: P) -> Result<Vec<ExperimentConfig>, String> {
        let data = fs::read_to_string(path).map_err(|e| e.to_string())?;
        serde_json::from_str(&data).map_err(|e| e.to_string())
    }
}
