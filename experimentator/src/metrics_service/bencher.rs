use crate::metrics_service::models::measurement::Measurement;
use crate::metrics_service::{data_collector, reporter::Reporter};
use criterion::{BenchmarkId, Criterion};
use knapsack_library::models::{knapsack::Knapsack, knapsack_solver::KnapsackSolver};
use std::{collections::HashMap, io, time::Duration};

pub struct Bencher {
    criterion: Criterion,
    reporter: Reporter,
}

impl Bencher {
    const SAMPLE_SIZE: usize = 10;
    const WARM_UP_TIME: Duration = Duration::new(1, 0);
    const NRESAMPLES: usize = 1000;
    const MEASUREMENT_TIME: Duration = Duration::from_secs(10);

    pub fn new(file_path: Option<&str>) -> io::Result<Self> {
        Ok(Self {
            criterion: Criterion::default(),
            reporter: Reporter::new(file_path)?,
        })
    }

    pub fn bench_group(
        &mut self,
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
        let mut group = self.criterion.benchmark_group(group_name);

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
                        let _ = solver.solve(k);
                    })
                });
            });
        }
        group.finish();
    }

    pub fn conduct_experiment(
        &mut self,
        num_items: usize,
        solvers: &[Box<dyn KnapsackSolver>],
        knapsacks: &[Knapsack],
    ) {
        self.bench_group(solvers, knapsacks, None, None, None, None);
        let measurements = Self::get_stats(solvers, knapsacks);
        self.report_table(num_items, &measurements);

        data_collector::get_mean_plots();
        data_collector::delete_criterion_dir();
    }

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

        let knapsack_count = knapsacks.len() as f64;
        correct_rates
            .iter_mut()
            .for_each(|(_, v)| *v = (*v / knapsack_count) * 100.0);

        correct_rates
    }

    pub fn get_stats(
        solvers: &[Box<dyn KnapsackSolver>],
        knapsacks: &[Knapsack],
    ) -> Vec<Measurement> {
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

    pub fn report_table(&self, num_items: usize, measurements: &[Measurement]) {
        // bad style
        let number_of_iters = 100.0;
        let group_name = format!("{} items", num_items);
        let table_data: Vec<Vec<String>> = measurements
            .iter()
            .map(|m| {
                let time_stats = m.get_time_stats();
                vec![
                    m.get_solver_name(),
                    format!("{:3.2}%", m.get_correct_rate()),
                    format!(
                        "{:6.3}/{:6.3}/{:6.3}/{:6.3}",
                        time_stats.get_mean_time() / number_of_iters,
                        time_stats.get_std_dev() / number_of_iters,
                        time_stats.get_median_time() / number_of_iters,
                        time_stats.get_median_abs_dev() / number_of_iters,
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

    fn format_markdown_table(data: Vec<Vec<String>>) -> String {
        let delimiter = "------------------+---------------------+----------------------------------------------+\n";
        let mut output = String::new();

        output.push_str(
            "     Algorithm    |    Success Rate     | Execution Time (ms)                          |\n",
        );
        output.push_str(
            "                  |                     | mean/std_dev/median/median_abs_dev           |\n",
        );
        output.push_str(delimiter);

        for row in data {
            output.push_str(&format!(
                "{:<17} | {:<3}             | {}                  |\n",
                row[0], row[1], row[2]
            ));
            output.push_str(delimiter);
        }
        output
    }
}
