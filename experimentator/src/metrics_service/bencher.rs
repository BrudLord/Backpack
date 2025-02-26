use crate::data;
use crate::metrics_service::models::measurement::Measurement;
use crate::metrics_service::{data_collector, reporter::Reporter};
use criterion::{BenchmarkId, Criterion};
use knapsack_library::models::{knapsack::Knapsack, knapsack_solver::KnapsackSolver};
use std::fmt::write;
use std::{collections::HashMap, io, time::Duration};
use std::{env, ffi::OsStr, fs, path::PathBuf};

/// A benchmarking utility for evaluating the performance of different knapsack solvers.
///
/// This struct provides functionality to benchmark knapsack problem solvers
/// using the Criterion benchmarking library and report the results.
pub struct Bencher {
    reporter: Reporter,
}

impl Bencher {
    /// Default sample size for benchmarking.
    const SAMPLE_SIZE: usize = 10;
    /// Default warm-up time before benchmarking starts.
    const WARM_UP_TIME: Duration = Duration::new(1, 0);
    /// Default number of resampling iterations.
    const NRESAMPLES: usize = 1001;
    /// Default time duration for measurement.
    const MEASUREMENT_TIME: Duration = Duration::from_secs(10);

    /// Benchmarks a group of knapsack solvers.
    ///
    /// # Arguments
    ///
    /// * `knapsack_solvers` - A slice of knapsack solvers to benchmark.
    /// * `knapsacks` - A slice of knapsack instances to use for benchmarking.
    /// * `sample_size` - Optional sample size for benchmarking.
    /// * `warm_up_time` - Optional warm-up time duration.
    /// * `nresamples` - Optional number of resampling iterations.
    /// * `measurement_time` - Optional measurement duration.
    pub fn bench_group(
        &self,
        knapsack_solvers: &[Box<dyn KnapsackSolver>],
        knapsacks: &[Knapsack],
        sample_size: Option<usize>,
        warm_up_time: Option<Duration>,
        nresamples: Option<usize>,
        measurement_time: Option<Duration>,
    ) {
        if knapsacks.is_empty() {
            return;
        }

        let group_name = format!("{} items", knapsacks[0].get_items_len());
        let mut criterion = Criterion::default();
        let mut group = criterion.benchmark_group(group_name);

        group
            .sample_size(sample_size.unwrap_or(Self::SAMPLE_SIZE))
            .warm_up_time(warm_up_time.unwrap_or(Self::WARM_UP_TIME))
            .nresamples(nresamples.unwrap_or(Self::NRESAMPLES))
            .measurement_time(measurement_time.unwrap_or(Self::MEASUREMENT_TIME));

        for solver in knapsack_solvers {
            let bench_name = solver.get_name();
            group.bench_with_input(BenchmarkId::new(&bench_name, ""), &knapsacks, |b, ks| {
                b.iter(|| {
                    ks.iter().for_each(|k| {
                        // do not use results - measure just time here
                        let _ = solver.solve(k);
                    })
                });
            });
        }
        group.finish();
    }

    /// Conducts a full experiment, benchmarking solvers and collecting statistics.
    ///
    /// # Arguments
    ///
    /// * `solvers` - A slice of knapsack solvers to test.
    /// * `knapsacks` - A slice of knapsack instances to evaluate.
    pub fn conduct_experiment(
        &self,
        solvers: &[Box<dyn KnapsackSolver>],
        knapsacks: &[Knapsack],
        os_string: &str
    ) {
        // do not bench in the case of empty parameters
        if solvers.is_empty() || knapsacks.is_empty() {
            return;
        }
        let knapsack_num_items_config = knapsacks[0].get_items_len();
        let number_of_samples = knapsacks.len();
        self.bench_group(solvers, knapsacks, None, None, None, None);
        let measurements = Self::get_stats(solvers, knapsacks);
        println!("{:?}", measurements);
        self.report_table(number_of_samples, knapsack_num_items_config, &measurements);

        data_collector::get_mean_plots(os_string.to_string());
        data_collector::delete_criterion_dir();
    }

    /// Computes the correctness rates of solvers by comparing their results.
    ///
    /// # Arguments
    ///
    /// * `solvers` - A slice of knapsack solvers.
    /// * `knapsacks` - A slice of knapsack instances.
    ///
    /// # Returns
    ///
    /// * `HashMap<String, f64>` - A mapping of solver names to their correctness percentage.
    fn calculate_correct_rates(
        solvers: &[Box<dyn KnapsackSolver>],
        knapsacks: &[Knapsack],
    ) -> HashMap<String, f64> {
        let mut correct_rates = HashMap::new();

        for knapsack in knapsacks {
            let results: Vec<u64> = solvers
                .iter()
                .map(|solver| solver.solve(knapsack).unwrap_or(0))
                .collect();

            if let Some(&best_result) = results.iter().max() {
                for (solver, &result) in solvers.iter().zip(results.iter()) {
                    if result == best_result {
                        *correct_rates.entry(solver.get_name()).or_insert(0.0) += 1.0;
                    }
                }
            }
        }

        let number_of_samples = knapsacks.len() as f64;
        correct_rates
            .iter_mut()
            .for_each(|(_, v)| *v = (*v / number_of_samples) * 100.0);

        correct_rates
    }

    /// Retrieves statistical measurements for the given solvers.
    ///
    /// # Arguments
    ///
    /// * `solvers` - A slice of knapsack solvers to evaluate.
    /// * `knapsacks` - A slice of knapsack instances to test.
    ///
    /// # Returns
    ///
    /// * `Vec<Measurement>` - A vector of measurements containing solver statistics.
    fn get_stats(solvers: &[Box<dyn KnapsackSolver>], knapsacks: &[Knapsack]) -> Vec<Measurement> {
        let correct_rates = Self::calculate_correct_rates(solvers, knapsacks);
        let time_stats = data_collector::get_criterion_stats().unwrap_or_default();

        let mut solver_names: Vec<String> = solvers.iter().map(|s| s.get_name()).collect();
        solver_names.sort();

        solver_names
            .into_iter()
            .filter_map(|name| {
                Some(Measurement::from((
                    name.clone(),
                    correct_rates.get(&name)?,
                    time_stats.get(&name)?,
                )))
            })
            .collect()
    }

    /// Reports benchmark results in a formatted markdown table.
    ///
    /// # Arguments
    ///
    /// * `number_of_samples` - The number of knapsack instances tested.
    /// * `knapsack_num_items_config` - The number of items in each knapsack.
    /// * `measurements` - A slice of measurements containing solver statistics.
    pub fn report_table(
        &self,
        number_of_samples: usize,
        knapsack_num_items_config: usize,
        measurements: &[Measurement],
    ) {
        let group_name = format!("{} items", knapsack_num_items_config);
        let table_data: Vec<Vec<String>> = measurements
            .iter()
            .map(|m| {
                let time_stats = m.get_time_stats();
                vec![
                    m.get_solver_name(),
                    format!("{:3.2}%", m.get_correct_rate()),
                    format!(
                        "{:6.3}/{:6.3}/{:6.3}/{:6.3}",
                        time_stats.get_mean_time() / number_of_samples as f64,
                        time_stats.get_std_dev() / number_of_samples as f64,
                        time_stats.get_median_time() / number_of_samples as f64,
                        time_stats.get_median_abs_dev() / number_of_samples as f64,
                    ),
                ]
            })
            .collect();

        let table = Self::format_markdown_table(table_data);
        let output = format!("\n#### {}\n\n{}", group_name, table);

        if let Err(e) = self.reporter.report(&output) {
            eprintln!("Failed to report metrics: {}", e);
        }
    }

    /// Formats data into a markdown table string.
    ///
    /// # Arguments
    ///
    /// * `data` - A vector of vectors containing table data as strings.
    ///
    /// # Returns
    ///
    /// * `String` - The formatted markdown table.
    fn format_markdown_table(data: Vec<Vec<String>>) -> String {
        if data.is_empty() {
            return String::from("Нет доступных данных.");
        }

        let headers = vec![
            "Algorithm",
            "Success Rate",
            "Execution Time (ms) (mean/std_dev/median/median_abs_dev)",
        ];
        let mut widths = headers.iter().map(|h| h.len()).collect::<Vec<usize>>();

        // Calculate maximum widths considering both headers and data
        for row in &data {
            for (i, col) in row.iter().enumerate() {
                widths[i] = widths[i].max(col.len());
            }
        }

        let mut output = String::new();

        // Header row
        output.push_str("| ");
        output.push_str(
            &headers
                .iter()
                .enumerate()
                .map(|(i, h)| format!("{:<width$}", h, width = widths[i]))
                .collect::<Vec<_>>()
                .join(" | "),
        );
        output.push_str(" |\n");

        // Separator row
        output.push_str("|");
        output.push_str(
            &widths
                .iter()
                .map(|&w| format!("-{:-<width$}-", "", width = w))
                .collect::<Vec<_>>()
                .join("|"),
        );
        output.push_str("|\n");

        // Data rows
        for row in data {
            output.push_str("| ");
            output.push_str(
                &row.iter()
                    .enumerate()
                    .map(|(i, col)| format!("{:<width$}", col, width = widths[i]))
                    .collect::<Vec<_>>()
                    .join(" | "),
            );
            output.push_str(" |\n");
        }

        output
    }

    /// Creates a new `Bencher` instance with reporter (write data without truncating).
    ///
    /// # Arguments
    ///
    /// * `file_path` - Optional path to a file where results will be logged.
    ///
    /// # Returns
    ///
    /// * `io::Result<Self>` - A result containing the `Bencher` instance if successful, otherwise an error.
    pub fn new(os_string: Option<&str>, write_to_file_flag: bool) -> io::Result<Self> {
        match write_to_file_flag {
            true => {
                let mut start_dir =
                    env::current_dir().expect("Failed to get current working directory");
                start_dir = start_dir.parent().unwrap().to_path_buf();
                println!("{}", start_dir.display());
                let experiment_results_dir = start_dir
                    .join("docs".to_string())
                    .join("experiments".to_string())
                    .join(os_string.unwrap());
                let _ = fs::create_dir_all(&experiment_results_dir);
                println!("{}", experiment_results_dir.display());
                return Ok(Self {
                    reporter: Reporter::new(
                        experiment_results_dir.join("experiment.md").to_str(),
                        true,
                    )?,
                });
            }
            false => {
                return Ok(Self {
                    reporter: Reporter::new(None, true)?,
                })
            }
        }
    }
}
