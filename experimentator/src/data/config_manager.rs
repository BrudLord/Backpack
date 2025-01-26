use crate::models::experiment_config::ExperimentConfig;
use std::fs;
use std::path::Path;

pub struct DataConfigManager;

impl DataConfigManager {
    pub fn read_config<P: AsRef<Path>>(path: P) -> Result<ExperimentConfig, String> {
        let data = fs::read_to_string(path).map_err(|e| e.to_string())?;
        serde_json::from_str(&data).map_err(|e| e.to_string())
    }
}
