mod data;
mod models;

use data::manager::DataManager;
use knapsack_library::algorithms_api::AlgorithmsAPI;

fn main() {
    let config_path = "experiment.json";
    let mut knapsack =
        DataManager::create_knapsack_from_config(config_path).expect("Failed to create knapsack");

    let api = AlgorithmsAPI::new();
    let best_value = api.solve_knapsack(&mut knapsack);

    println!("Maximum value in knapsack: {}", best_value);
}
