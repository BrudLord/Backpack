use crate::metrics_service::reporter::{ ReporterType, ConsoleReporter, FileReporter };
use knapsack_library::models::knapsack::Knapsack;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::convert::From;
use std::time::Instant;

#[derive(Debug, Serialize, Deserialize)]
pub struct Measurement {
    /// Name of the experiment
    pub experiment_name: String,
    /// Knapsack unit
    pub knapsack: Knapsack,
    /// Metrics of the experiment
    pub metrics: HashMap<String, MetricsUnit>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MetricsUnit {
    /// Maximum value of the knapsack
    pub result: Option<u32>,
    /// Execution time in nanoseconds of the algorithm
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_time_ns: Option<u128>,
    /// Memory usage of the algorithm
    pub memory_usage: Option<usize>,
}

impl MetricsUnit {
    pub fn new() -> Self {
        Self {
            result: None,
            execution_time_ns: None,
            memory_usage: None,
        }
    }
}

impl From<(Option<u32>, Option<u128>, Option<usize>)> for MetricsUnit {
    /// Convert a tuple of (Option<u32>, Option<u128>, Option<usize>) to a MetricsUnit
    fn from(t: (Option<u32>, Option<u128>, Option<usize>)) -> MetricsUnit {
        MetricsUnit {
            result: t.0,
            execution_time_ns: t.1,
            memory_usage: t.2,
        }
    }
}

impl Measurement {
    pub fn new(experiment_name: String, knapsack: &Knapsack) -> Self {
        Self {
            experiment_name,
            knapsack: knapsack.clone(),
            metrics: HashMap::new(),
        }
    }
}

/// Service for conducting experiments
pub struct MetricService {
    /// Logger
    reporter: ReporterType,
}

impl MetricService {
    pub fn new(file_path: Option<&str>) -> Self {
        let reporter = match file_path {
            Some(path) => ReporterType::File(FileReporter::new(Some(path))),
            None => ReporterType::Console(ConsoleReporter::new()),
        };
        Self { reporter }
    }

    /// Conduct an experiment without logging and return the Measurement
    pub fn conduct_experiment<F>(
        &self,
        f: F,
        knapsack: &Knapsack,
        algorithm_names: &Vec<String>,
        experiment_name: Option<&str>,
    ) -> Measurement
    where
        F: Fn(String, &Knapsack) -> Option<u32>,
    {
        let mut experiment_unit: Measurement = match experiment_name {
            Some(name) => Measurement::new(name.to_string(), knapsack),
            None => Measurement::new("Experiment".to_string(), knapsack),
        };
        let maps: HashMap<String, MetricsUnit> = algorithm_names
            .iter()
            .map(|name| {
                let start_time = Instant::now();
                let result = f(name.clone(), &knapsack);
                let execution_time_ns = start_time.elapsed().as_nanos();
                return (name.clone(), (result, Some(execution_time_ns), None).into());
            })
            .collect();
        experiment_unit.metrics = maps;
        experiment_unit
    }

    /// Conduct a batch experiment without logging and return the Measurement
    pub fn conduct_batch_experiment<F>(
        &self,
        f: F,
        knapsacks: Vec<&Knapsack>,
        algorithm_names: &Vec<String>,
    ) -> Vec<Measurement>
    where
        F: Fn(String, &Knapsack) -> Option<u32>,
    {
        let mut measurements = Vec::new();
        for knapsack in &knapsacks {
            let measurement = self.conduct_experiment(&f, &knapsack, &algorithm_names, None);
            measurements.push(measurement);
        }
        measurements
    }

    /// Write an experiment result to the reporter
    pub fn write_measurement(&self, measurement: &Measurement) {
        self.reporter.report_json(measurement);
    }

    pub fn write_batch_measurement(&self, measurements: &Vec<Measurement>) {
        self.reporter.report_batch(measurements);
    }

    /// Agreggate the metrics of the experiment results and display by reporter
    pub fn agreggate(&self, measurements: Vec<Measurement>) {
        struct AgreggateMetrics {
            algorithm_name: String,
            correct_rate: f64,
            mean_execution_time_ns: f64,
            percentile_95_execution_time_ns: f64,
            mean_memory_usage: f64,
            percentile_95_memory_usage: f64,
        }

        #[derive(Debug)]
        enum DisplayValue {
            Text(String),
            Number(f64),
        }

        impl std::fmt::Display for DisplayValue {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    DisplayValue::Text(s) => write!(f, "{:>15}", s),
                    DisplayValue::Number(n) => write!(f, "{:>15.3}", n),
                }
            }
        }

        fn display_line(
            algorithm_name: DisplayValue,
            correct_rate: DisplayValue,
            mean_execution_time_ns: DisplayValue,
            percentile_95_execution_time_ns: DisplayValue,
            mean_memory_usage: DisplayValue,
            percentile_95_memory_usage: DisplayValue,
        ) -> String {
            let mut output_string = String::new();
            output_string += &format!("{n:>20}", n = algorithm_name);
            output_string += &format!("{}", correct_rate);
            output_string += &format!("{}", mean_execution_time_ns);
            output_string += &format!("{}", percentile_95_execution_time_ns);
            output_string += &format!("{}", mean_memory_usage);
            output_string += &format!("{}", percentile_95_memory_usage);
            output_string += "\n";
            output_string
        }

        impl AgreggateMetrics {
            pub fn display(&self) -> String {
                display_line(
                    DisplayValue::Text(self.algorithm_name.to_string()),
                    DisplayValue::Number(self.correct_rate),
                    DisplayValue::Number(self.mean_execution_time_ns),
                    DisplayValue::Number(self.percentile_95_execution_time_ns),
                    DisplayValue::Number(self.mean_memory_usage),
                    DisplayValue::Number(self.percentile_95_memory_usage),
                )
            }
        }

        let mut metrics: HashMap<String, AgreggateMetrics> = HashMap::new();
        let mut set = HashSet::new();

        // Get the maximum result for each algorithm
        let mut answers = Vec::new();
        for measurement in &measurements {
            let mut answer = 0;
            for (key, metric) in &measurement.metrics {
                set.insert(key);
                answer = std::cmp::max(metric.result.unwrap_or(0), answer);
            }
            answers.push(answer);
        }

        // Initialize metrics for each algorithm
        for name in &set {
            metrics.insert(
                name.to_string(),
                AgreggateMetrics {
                    algorithm_name: name.to_string(),
                    correct_rate: 0.0,
                    mean_execution_time_ns: 0.0,
                    percentile_95_execution_time_ns: 0.0,
                    mean_memory_usage: 0.0,
                    percentile_95_memory_usage: 0.0,
                },
            );
        }

        // Calculate metrics for each algorithm
        let mut experiment_number: usize = 0;
        for measurement in &measurements {
            let answer = answers[experiment_number];
            experiment_number += 1;
            for (key, metric) in &measurement.metrics {
                let o = metrics.get_mut(key).unwrap();
                (*o).mean_execution_time_ns += metric.execution_time_ns.unwrap_or(0) as f64;
                (*o).mean_memory_usage += metric.memory_usage.unwrap_or(0) as f64;
                let mut is_correct_answer: f64 = 0.0;
                if metric.result.unwrap_or(0) == answer {
                    is_correct_answer = 1.0;
                }
                (*o).correct_rate += is_correct_answer;
            }
        }
        for (_, metric) in &mut metrics {
            metric.mean_execution_time_ns /= experiment_number as f64;
            metric.mean_memory_usage /= experiment_number as f64;
            metric.correct_rate /= experiment_number as f64;
        }

        // Display the metrics
        let mut out = display_line(
            DisplayValue::Text("algorithm".to_string()),
            DisplayValue::Text("correct_rate".to_string()),
            DisplayValue::Text("mean_time".to_string()),
            DisplayValue::Text("95%_time".to_string()),
            DisplayValue::Text("mean_memory".to_string()),
            DisplayValue::Text("95%_memory".to_string()),
        );
        for (_, metric) in &metrics {
            out.push_str(&metric.display());
        }
        self.reporter.report(&out);
    }
}
