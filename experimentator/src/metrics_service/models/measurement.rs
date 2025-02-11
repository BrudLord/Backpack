use knapsack_library::models::knapsack::Knapsack;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::metrics_data::MetricsData;


/// Represents a single experiment measurement with its metrics
///
/// # Fields
/// * `experiment_name` - Name identifier for the experiment
/// * `knapsack` - The knapsack problem instance used in the experiment
/// * `metrics` - Collection of metrics for different algorithms
#[derive(Debug, Serialize, Deserialize)]
pub struct Measurement {
    pub experiment_name: String,
    pub knapsack: Knapsack,
    pub metrics: HashMap<String, MetricsData>,
}

impl Measurement {
    /// Creates a new Measurement instance
    ///
    /// # Arguments
    /// * `experiment_name` - Name identifier for the experiment
    /// * `knapsack` - Reference to the knapsack problem instance
    ///
    /// # Returns
    /// * `Self` - New Measurement instance with empty metrics
    pub fn new(experiment_name: Option<&str>, knapsack: &Knapsack) -> Self {
        let experiment_name = match experiment_name {
            Some(name) => name.to_string(),
            None => "Experiment".to_string(),
        };
        Self {
            experiment_name,
            knapsack: knapsack.clone(),
            metrics: HashMap::new(),
        }
    }

    /// Groups metrics by algorithm across multiple measurements
    ///
    /// # Arguments
    /// * `measurements` - Slice of measurements to analyze
    ///
    /// # Returns
    /// * `HashMap<String, Vec<&MetricsData>>` - Metrics grouped by algorithm name
    pub fn group_metrics_by_algorithm<'a>(
        measurements: &'a [Measurement]
    ) -> HashMap<String, Vec<&'a MetricsData>> {
        let mut metrics_by_algorithm: HashMap<String, Vec<&'a MetricsData>> = HashMap::new();
        
        for measurement in measurements {
            for (algo_name, metric) in &measurement.metrics {
                metrics_by_algorithm
                    .entry(algo_name.clone())
                    .or_default()
                    .push(metric);
            }
        }
        
        metrics_by_algorithm
    }

    /// Gets the best results from each measurement
    ///
    /// # Arguments
    /// * `measurements` - Slice of measurements to analyze
    ///
    /// # Returns
    /// * `Vec<u64>` - Vector of maximum results from each measurement
    pub fn get_bests_results(measurements: &[Measurement]) -> Vec<u64> {
        measurements.iter()
            .map(|measurement| {
                measurement.metrics.values()
                    // Преобразуем Result<u64, String> в Option<u64>, игнорируя ошибки
                    .filter_map(|metric| match &metric.result {
                        Ok(value) => Some(*value),       // Если Ok, добавляем значение
                        Err(_) => None,                  // Если Err, игнорируем
                    })
                    .max()
                    .unwrap_or(0)  // Если нет значений, возвращаем 0
            })
            .collect()
    }

}
