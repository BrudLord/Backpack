use crate::data::config_manager::DataConfigManager;
use crate::models::experiment_config::ExperimentConfig;
use knapsack_library::models::item::Item;
use knapsack_library::models::knapsack::Knapsack;
use rand::Rng;
use std::path::Path;

pub struct DataManager;

impl DataManager {
    pub fn create_knapsack_from_config<P: AsRef<Path>>(path: P) -> Result<Knapsack, String> {
        let config = DataConfigManager::read_config(path)?;
        Ok(DataManager::generate_knapsack(config))
    }

    pub fn generate_knapsack(config: ExperimentConfig) -> Knapsack {
        let mut rng = rand::thread_rng();
        let items: Vec<Item> = (0..config.num_items)
            .map(|_| {
                Item::new(
                    rng.gen_range(config.weights_range.0..=config.weights_range.1),
                    rng.gen_range(config.costs_range.0..=config.costs_range.1),
                )
            })
            .collect();

        Knapsack::new(config.capacity, items)
    }
}
