mod data;
mod models;

use data::manager::generate_rnd_knapsacks;
use knapsack_library::algorithms_service::AlgorithmsService;
use knapsack_library::models::knapsack::Knapsack;
use metrics_service::metrics_service::MetricService;

fn main() {
    let config_path = "experiments.json";
    // Generates knapsacks based on the configuration file
    // You can find the example of this file is "experiments_example.json"
    let knapsacks: Vec<Knapsack> =
        generate_rnd_knapsacks(config_path).expect("Failed to create knapsack");

    let mut metric_service = MetricService::new(Some("results.txt"));

    let algorithms_names = AlgorithmsService::get_algorithms_names();
    let out = metric_service.conduct_batch_experiment(
        |s, k| AlgorithmsService::solve(s, k),
        knapsacks.iter().collect(),
        &algorithms_names,
    );

    // Aggregates the results of the batch experiment
    metric_service.agreggate(out);

    println!("Results saved in results.txt");
}
