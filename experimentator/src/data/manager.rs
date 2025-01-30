use crate::data::config_manager::DataConfigManager;
use crate::models::experiment_config::ExperimentConfig;
use knapsack_library::models::item::Item;
use knapsack_library::models::knapsack::Knapsack;
use rand::Rng;
use std::path::Path;

pub struct DataManager;

impl DataManager {
    pub fn generate_rnd_knapsacks<P: AsRef<Path>>(path: P) -> Result<Vec<Knapsack>, String> {
        let config: Vec<ExperimentConfig> = DataConfigManager::read_rand_config(path)?;

        let mut knapsacks: Vec<Knapsack> = Vec::new();

        for exp_conf in config {
            knapsacks.push(DataManager::generate_knapsack(exp_conf));
        }

        Ok(knapsacks)
    }

    fn generate_knapsack(config: ExperimentConfig) -> Knapsack {
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
