mod data;
mod metrics_service;
mod models;

use data::manager::generate_rnd_knapsacks;
use knapsack_library::algorithms_service::AlgorithmsService;
use metrics_service::bencher::Bencher;

fn main() {
    let config_path = "experiment.json";
    let os_string = "linux";

    let bencher = Bencher::new(Some(os_string), true).unwrap();

    let (knapsacks, algorithms_names) =
        generate_rnd_knapsacks(config_path).expect("Failed to create knapsack");
    let algorithms = AlgorithmsService::get_algorithms_by_names(algorithms_names);
    bencher.conduct_experiment( &algorithms, &knapsacks);
}
