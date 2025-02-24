mod data;
mod metrics_service;
mod models;

use data::manager::generate_rnd_knapsacks;
use knapsack_library::algorithms_service::AlgorithmsService;

fn main() {
    let config_path = "experiments.json";

    let numbers_of_items = vec![20];
    let num_knapsacks_per_size = 100;

    let mut bencher =
        metrics_service::bencher::Bencher::new(Some("experiment_results.txt")).unwrap();

    for num_items in &numbers_of_items {
        let mut knapsacks = Vec::new();
        for _ in 0..num_knapsacks_per_size {
            let knapsack =
                generate_rnd_knapsacks(config_path).expect("Failed to create knapsack")[0].clone();
            knapsacks.push(knapsack);
        }

        let algorithms = AlgorithmsService::get_all_algorithms();

        bencher.conduct_experiment(*num_items, &algorithms, &knapsacks);
    }
}
