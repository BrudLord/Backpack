use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ExperimentConfig {
    pub num_items: usize,
    pub capacity: u32,
    pub weights_range: (u32, u32),
    pub costs_range: (u32, u32),
}
