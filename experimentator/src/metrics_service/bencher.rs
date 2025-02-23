use crate::metrics_service::reporter::Reporter;
use criterion::{BenchmarkId, Criterion};
use std::collections::HashMap;
use std::io;

use crate::metrics_service::data_collector;
use crate::metrics_service::models::measurement::Measurement;
use crate::metrics_service::models::time_stats::TimeStats;
use knapsack_library::models::knapsack::Knapsack;
use knapsack_library::models::knapsack_solver::KnapsackSolver;


// Mutable criterion is part of struct Bencher
pub struct Bencher {
    criterion: Criterion,
    reporter: Reporter,
}

impl Bencher {
    const SAMPLE_SIZE: usize = 10;
    const WARM_UP_TIME: std::time::Duration = std::time::Duration::new(1, 0);
    const NRESAMPLES: usize = 1000;
    const MEASUREMENT_TIME: std::time::Duration = std::time::Duration::from_secs(10);
    pub fn new(file_path: Option<&str>) -> io::Result<Self> {
        Ok(Self {
            criterion: Criterion::default(),
            reporter: Reporter::new(file_path)?,
        })
    }

    pub fn bench_group(
        &mut self,
        knapsack_solvers: &Vec<Box<dyn KnapsackSolver>>,
        knapsacks: &Vec<Knapsack>,
        sample_size: Option<usize>,
        warm_up_time: Option<std::time::Duration>,
        nresamples: Option<usize>,
        measurement_time: Option<std::time::Duration>,
    ) {
        if knapsacks.len() == 0 {
            return;
        }
        let num_items = knapsacks[0].get_items_len();
        let group_name = format!("{} items", num_items);
        let mut group = self.criterion.benchmark_group(group_name);
        group.sample_size(sample_size.unwrap_or(Bencher::SAMPLE_SIZE));
        group.warm_up_time(warm_up_time.unwrap_or(Bencher::WARM_UP_TIME));
        group.nresamples(nresamples.unwrap_or(Bencher::NRESAMPLES));
        group.measurement_time(measurement_time.unwrap_or(Bencher::MEASUREMENT_TIME));

        for knapsack_solver in knapsack_solvers {
            let bench_name = format!("{}", knapsack_solver.get_name());
            group.bench_with_input(
                BenchmarkId::new(bench_name.as_str(), ""),
                &knapsacks,
                |b, knapsacks| {
                    b.iter(|| {
                        for knapsack in *knapsacks {
                            // do not use results of knapsacks due to only time measurements here
                            let _ = knapsack_solver.solve(knapsack);
                        }
                    })
                },
            );
        }
        group.finish();
    }


    pub fn conduct_experiment(&mut self,
        num_items: usize,
        knapsack_solvers: &Vec<Box<dyn KnapsackSolver>>,
        knapsacks: &Vec<Knapsack>) {
        self.bench_group( &knapsack_solvers, &knapsacks, None, None, None, None);
        let measurements = Bencher::get_stats(&knapsack_solvers, &knapsacks);
        self.report_table(num_items, &measurements);
        data_collector::get_mean_plots();
        data_collector::delete_criterion_dir();
    }



    fn calculate_correct_rates(
        knapsack_solvers: &Vec<Box<dyn KnapsackSolver>>,
        knapsacks: &Vec<Knapsack>,
    ) -> HashMap<String, f64> {
        let mut accumulated_correct_results: HashMap<String, f64> = HashMap::new();

        for knapsack in knapsacks {
            let results = knapsack_solvers
                .into_iter()
                .map(|knapsack_solver| knapsack_solver.solve(knapsack).unwrap_or(0))
                .collect::<Vec<u64>>();
            let best_result = *results.iter().max().unwrap();
            for (ind, knapsack_solver) in knapsack_solvers.iter().enumerate() {
                if (results[ind] == best_result) {
                    *accumulated_correct_results
                        .entry(knapsack_solver.get_name())
                        .or_insert(0.0) += 1.0;
                };
            }
        }
        for (_, value) in &mut accumulated_correct_results {
            *value /= knapsacks.len() as f64;
            *value *= 100.0;
        }
        accumulated_correct_results
    }

    pub fn get_stats(
        knapsack_solvers: &Vec<Box<dyn KnapsackSolver>>,
        knapsacks: &Vec<Knapsack>,
    ) -> Vec<Measurement> {
        let map_correct_rates = Bencher::calculate_correct_rates(knapsack_solvers, knapsacks);
        let map_time_stats = data_collector::get_criterion_stats().unwrap();

        let mut knapsacks_names: Vec<String> = knapsack_solvers
            .into_iter()
            .map(|knapsack_solver| knapsack_solver.get_name())
            .collect::<Vec<String>>();
        knapsacks_names.sort();

        let measurements: Vec<Measurement> = knapsacks_names
            .into_iter()
            .map(|name| {
                (
                    name.clone(),
                    map_correct_rates.get(&name).unwrap(),
                    map_time_stats.get(&name).unwrap(),
                )
                    .into()
            })
            .collect::<Vec<Measurement>>();
        measurements
    }

    fn format_markdown_table(data: Vec<Vec<String>>) -> String {
        let delimeter = 
            "------------------+---------------------+----------------------------------------------+\n";
        let mut output = String::new();
        output.push_str(
            "     Algorithm    |    Success Rate     |      Execution Time (ms)                     |\n",
        );
        output.push_str(
            "                  |                     |      mean/std_dev/median/median_abs_dev      |\n",
        );
        output.push_str(&delimeter);

        for row in data {
            output.push_str(&format!(
                "{:<17} | {:<3}%                | {}                 |\n",
                row[0], row[1], row[2]
            ));
            output.push_str(&delimeter);
        }
        output
    }

    pub fn report_table(&self, num_items: usize, measurements: &Vec<Measurement>) {
        let mut table_data = Vec::new();

        let group_name = format!("{} items", num_items);

        for measurement in measurements {
            let ns_to_ms = 1000000.0;
            let solver_name = measurement.get_solver_name();
            let correct_rate = measurement.get_correct_rate();
            println!("{}", correct_rate);
            let time_stats = measurement.get_time_stats();
            let mean_time = time_stats.get_mean_time();
            let std_dev = time_stats.get_std_dev();
            let median_time = time_stats.get_median_time();
            let median_abs_dev = time_stats.get_median_abs_dev();
            let mut table_item: Vec<String> = vec![solver_name, correct_rate.to_string()];
            table_item.push(format!(
                "{:>6.3}/{:>6.3}/{:>6.3}/{:>6.3}",
                mean_time, std_dev, median_time, median_abs_dev
            ));
            table_data.push(table_item);
        }

        let table = Bencher::format_markdown_table(table_data);

        let mut output = String::new();
        output.push_str(&format!("\n#### {}\n\n{}", group_name, table));
        self.reporter.report(&output).unwrap_or_else(|e| {
            eprintln!("Failed to report metrics: {}", e);
        });
    }
}
