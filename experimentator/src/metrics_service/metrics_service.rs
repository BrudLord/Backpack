//! Metrics Service module provides functionality for collecting and analyzing algorithm performance metrics.
//!
//! This module is responsible for:
//! - Conducting experiments with different algorithms
//! - Collecting performance metrics (execution time, memory usage)
//! - Aggregating and analyzing results
//! - Generating reports

use std::collections::HashMap;
use std::io;
use std::time::Instant;

use crate::metrics_service::models::measurement::Measurement;
use crate::metrics_service::models::metrics_data::MetricsData;
use crate::metrics_service::models::stats::{AggregatedMetric, Stats};
use crate::metrics_service::reporter::Reporter;
use knapsack_library::models::knapsack::Knapsack;
use knapsack_library::models::knapsack_solver::KnapsackSolver;

/// Service for collecting and analyzing algorithm performance metrics
pub struct MetricService {
    reporter: Reporter,
}

impl MetricService {
    /// Creates a new MetricService instance
    ///
    /// # Arguments
    /// * `file_path` - Optional path to output file for metrics reporting
    ///
    /// # Returns
    /// * `io::Result<Self>` - New MetricService instance or IO error
    pub fn new(file_path: Option<&str>) -> io::Result<Self> {
        Ok(Self {
            reporter: Reporter::new(file_path)?,
        })
    }

    /// Conducts a single experiment with given algorithm and input
    ///
    /// # Arguments
    /// * `knapsack_solvers`
    /// * `knapsack` - Input data for the algorithm
    /// * `algorithm_names` - Names of algorithms to test
    /// * `experiment_name` - Optional name for the experiment
    ///
    /// # Returns
    /// * `Measurement` - Results of the experiment
    pub fn conduct_experiment(
        &self,
        knapsack_solvers: &Vec<Box<dyn KnapsackSolver>>,
        knapsack: &Knapsack,
        experiment_name: Option<&str>,
    ) -> Measurement {
        let mut experiment_unit: Measurement = Measurement::new(experiment_name, knapsack);
        let maps: HashMap<String, MetricsData> = knapsack_solvers
            .iter()
            .map(|solver| {
                println!("Running algorithm: {}", solver.get_name()); // Add algorithm logging
                let start_time = Instant::now();
                let result = solver.solve(&knapsack);
                let execution_time_ns = start_time.elapsed().as_nanos();
                println!("Completed algorithm: {}", solver.get_name()); // Add completion logging
                return (
                    solver.get_name().clone(),
                    (Some(result), Some(execution_time_ns), None).into(),
                );
            })
            .collect();
        experiment_unit.metrics = maps;
        experiment_unit
    }

    /// Conducts batch experiments with multiple inputs
    pub fn conduct_batch_experiment(
        &self,
        knapsack_solvers: &Vec<Box<dyn KnapsackSolver>>,
        knapsacks: Vec<&Knapsack>,
    ) -> Vec<Measurement> {
        println!(
            "Starting batch experiment with {} knapsacks",
            knapsacks.len()
        );
        let mut measurements = Vec::new();
        for knapsack in knapsacks {
            println!("Processing knapsack");
            let measurement = self.conduct_experiment(&knapsack_solvers, knapsack, None);
            measurements.push(measurement);
        }
        println!("Batch experiment completed");
        measurements
    }

    /// Writes an experiment result to the reporter
    ///
    /// # Arguments
    /// * `measurement` - The measurement to write
    #[allow(dead_code)]
    pub fn write_measurement(&self, measurement: &Measurement) {
        self.reporter.report_json(measurement).unwrap_or_else(|e| {
            eprintln!("Failed to report measurement: {}", e);
        });
    }

    /// Writes a batch of experiment results to the reporter
    ///
    /// # Arguments
    /// * `measurements` - The measurements to write    
    #[allow(dead_code)]
    pub fn write_batch_measurement(&self, measurements: &Vec<Measurement>) {
        self.reporter
            .report_batch(measurements)
            .unwrap_or_else(|e| {
                eprintln!("Failed to report batch measurements: {}", e);
            });
    }

    /// Writes the aggregated metrics to the reporter
    ///
    /// # Arguments
    /// * `metrics` - The aggregated metrics to write
    fn report_metrics(&self, metrics: &[AggregatedMetric]) {
        let mut output = String::new();

        let delimeter =
            "----------+---------------------+----------------------------+---------------------\n";
        // Add header
        output.push_str(
            "Algorithm | Success Rate        | Execution Time (ms)        | Memory Usage (MB)\n",
        );
        output.push_str(
            "          |                     | mean/median/p95            | mean/median/p95\n",
        );
        output.push_str(&delimeter);

        // Add data rows
        for metric in metrics {
            output.push_str(&format!(
                "{:<9} | {:>6.1}%             | {:>6.3}/{:>6.3}/{:>6.3}       | {:>6.3}/{:>6.3}/{:>6.3}\n",
                metric.algorithm_name,
                metric.correct_rate * 100.0,
                metric.execution_time.mean / 1_000_000.0,
                metric.execution_time.median / 1_000_000.0,
                metric.execution_time.percentile95 / 1_000_000.0,
                metric.memory_usage.mean / (1024.0 * 1024.0),
                metric.memory_usage.median / (1024.0 * 1024.0),
                metric.memory_usage.percentile95 / (1024.0 * 1024.0)
            ));
            output.push_str(&delimeter);
        }

        self.reporter.report(&output).unwrap_or_else(|e| {
            eprintln!("Failed to report metrics: {}", e);
        });
    }

    /// Aggregates measurements and generates metrics report
    ///
    /// # Arguments
    /// * `measurements` - The measurements to aggregate
    ///
    /// # Returns
    /// * `io::Result<Vec<AggregatedMetric>>` - Aggregated metrics or IO error
    pub fn aggregate(&self, measurements: Vec<Measurement>) -> Vec<AggregatedMetric> {
        let grouped_metrics = Measurement::group_metrics_by_algorithm(&measurements);
        let answers = Measurement::get_bests_results(&measurements);

        // Calculate aggregated metrics for each algorithm
        let aggregated_metrics: Vec<AggregatedMetric> = grouped_metrics
            .iter()
            .map(|(algo_name, metrics)| {
                // Calculate correct rate by comparing each result with its corresponding answer
                let correct_rate = metrics
                    .iter()
                    .zip(answers.iter())
                    .filter(|(m, &answer)| m.result.is_some() && m.result.unwrap() == answer)
                    .count() as f64
                    / metrics.len() as f64;

                // Extract and sort execution times
                let mut exec_times: Vec<u128> =
                    metrics.iter().filter_map(|m| m.execution_time_ns).collect();
                exec_times.sort_unstable();

                // Extract and sort memory usage
                let mut memory_usage: Vec<usize> =
                    metrics.iter().filter_map(|m| m.memory_usage).collect();
                memory_usage.sort_unstable();

                AggregatedMetric::new(
                    algo_name.clone(),
                    correct_rate,
                    Stats::calculate_stats(
                        &exec_times.iter().map(|&x| x as f64).collect::<Vec<f64>>(),
                    ),
                    Stats::calculate_stats(
                        &memory_usage.iter().map(|&x| x as f64).collect::<Vec<f64>>(),
                    ),
                )
            })
            .collect();

        // Report results
        self.report_metrics(&aggregated_metrics);
        aggregated_metrics
    }
}
