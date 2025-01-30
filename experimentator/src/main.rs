mod data;
mod models;

use data::manager::generate_rnd_knapsacks;
use knapsack_library::algorithms_service::AlgorithmsService;
use knapsack_library::models::knapsack::Knapsack;
use metrics_service::metrics_service::MetricService;

fn main() {
    let config_path = "experiments.json";
    let knapsacks: Vec<Knapsack> =
        generate_rnd_knapsacks(config_path).expect("Failed to create knapsack");

    let algorithms_service = AlgorithmsService::new();
    let mut metric_service = MetricService::new(Some("results.txt"));

    let algorithms_names = AlgorithmsService::get_algorithms_names();
    let out = metric_service.conduct_batch_experiment(
        |s, k| algorithms_service.solve(s, k),
        knapsacks.iter().collect(),
        &algorithms_names,
    );

    metric_service.agreggate(out);

    println!("Results saved in results.txt");
}
