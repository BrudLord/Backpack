mod data;
mod metrics_service;
mod models;

use data::manager::generate_rnd_knapsacks;
use knapsack_library::algorithms_service::AlgorithmsService;
use metrics_service::bencher::Bencher;

fn main() {
    let config_path = "experiment.json";

    let numbers_of_items = vec![20];

    let mut bencher = Bencher::new(Some("experiment_results.txt")).unwrap();

    for num_items in &numbers_of_items {
        let (knapsacks, algorithms_names) =
            generate_rnd_knapsacks(config_path).expect("Failed to create knapsack");
        let mut algorithms = AlgorithmsService::get_all_algorithms();
        algorithms.retain(|solver| algorithms_names.contains(&solver.as_ref().get_name()));

        bencher.conduct_experiment(*num_items, &algorithms, &knapsacks);
    }
}
