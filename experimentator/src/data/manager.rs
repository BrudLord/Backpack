use crate::data::config_manager::read_rand_config;
use crate::models::experiment_config::ExperimentConfig;
use knapsack_library::models::item::Item;
use knapsack_library::models::knapsack::Knapsack;
use rand::Rng;
use std::path::Path;

/// Generates a vector of `Knapsack` instances based on configurations read from a file.
///
/// This function reads experiment configurations from the specified path using `read_rand_config`.
/// For each configuration, it generates a `Knapsack` instance using `generate_knapsack` and collects
/// them into a vector.
///
/// # Arguments
///
/// * `path`: A path to the JSON configuration file.
///
/// # Returns
///
/// * `Result<Vec<Knapsack>, String>`: A `Result` containing a vector of `Knapsack` instances if successful,
///   or an error string if reading or parsing the configuration fails.
///
/// # Errors
///
/// Returns an error if `read_rand_config` fails.
pub fn generate_rnd_knapsacks<P: AsRef<Path>>(path: P) -> Result<Vec<Knapsack>, String> {
    let config: Vec<ExperimentConfig> = read_rand_config(path)?;

    let mut knapsacks: Vec<Knapsack> = Vec::new();

    for exp_conf in config {
        knapsacks.push(generate_knapsack(exp_conf));
    }

    Ok(knapsacks)
}

/// Generates a single `Knapsack` instance based on the provided configuration.
///
/// This function uses the `ExperimentConfig` to create a `Knapsack` and its items.
///
/// # Arguments
///
/// * `config`: An `ExperimentConfig` struct containing the parameters for the knapsack.
///
/// # Returns
///
/// A `Knapsack` instance.
fn generate_knapsack(config: ExperimentConfig) -> Knapsack {
    let mut rng = rand::thread_rng();
    let items: Vec<Item> = (0..config.num_items)
        .map(|_| {
            Item::new(
                rng.gen_range(config.weights_range.0..=config.weights_range.1) as u64,
                rng.gen_range(config.costs_range.0..=config.costs_range.1) as u64,
            )
        })
        .collect();

        Knapsack::new(config.capacity as u64, items)
}
