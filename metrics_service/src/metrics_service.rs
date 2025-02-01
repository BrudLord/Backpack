use crate::logger::Logger;
use cap::Cap;
use knapsack_library::models::knapsack::Knapsack;
use serde::{Deserialize, Serialize};
use std::alloc;
use std::collections::{HashMap, HashSet};
use std::convert::From;
use std::time::Instant;
use std::num::Wrapping;

#[global_allocator]
static ALLOCATOR: Cap<alloc::System> = Cap::new(alloc::System, usize::max_value());

#[derive(Debug, Serialize, Deserialize)]
pub struct ExperimentUnit {
    pub experiment_name: String,
    pub knapsack: Knapsack,
    pub metrics: HashMap<String, MetricsUnit>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MetricsUnit {
    pub result: Option<u32>, // Результат выполнения алгоритма
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_time: Option<f64>, // Время выполнения в миллисекундах
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_usage: Option<usize>, // Использование памяти в байтах
}

impl MetricsUnit {
    pub fn new() -> Self {
        Self {
            result: None,
            execution_time: None,
            memory_usage: None,
        }
    }
}

impl From<(Option<u32>, Option<f64>, Option<usize>)> for MetricsUnit {
    fn from(t: (Option<u32>, Option<f64>, Option<usize>)) -> MetricsUnit {
        MetricsUnit {
            result: t.0,
            execution_time: t.1,
            memory_usage: t.2,
        }
    }
}

impl ExperimentUnit {
    pub fn new(experiment_name: String, knapsack: &Knapsack) -> Self {
        Self {
            experiment_name,
            knapsack: knapsack.clone(),
            metrics: HashMap::new(),
        }
    }
}

pub struct MetricService {
    logger: Logger,
}

impl MetricService {
    pub fn new(file_path: Option<&str>) -> Self {
        ALLOCATOR.set_limit(30 * 1024 * 1024).unwrap();
        let logger = Logger::new(!file_path.is_none(), file_path).unwrap();
        Self { logger }
    }

    pub fn conduct_experiment_with_no_log<F>(
        &mut self,
        f: F,
        knapsack: &Knapsack,
        algorithm_names: &Vec<String>,
        experiment_name: Option<&str>,
    ) -> ExperimentUnit
    where
        F: Fn(String, &Knapsack) -> Option<u32>,
    {
        let mut experiment_unit: ExperimentUnit = match experiment_name {
            Some(name) => ExperimentUnit::new(name.to_string(), knapsack),
            None => ExperimentUnit::new("Experiment".to_string(), knapsack),
        };
        let maps: HashMap<String, MetricsUnit> = algorithm_names
            .iter()
            .map(|name| {
                let start_time = Instant::now();
                let q1 = ALLOCATOR.allocated();
                let result = f(name.clone(), &knapsack);
                let q2 = ALLOCATOR.allocated();
                let execution_time = start_time.elapsed().as_secs_f64();

                let memory_used = if q2 >= q1 {
                    q2 - q1
                } else {
                    println!("Warning: possible overflow detected.");
                    (Wrapping(q2) - Wrapping(q1)).0
                };
                return (
                    name.clone(),
                    (result, Some(execution_time), Some(memory_used)).into(),
                );
            })
            .collect();
        experiment_unit.metrics = maps;
        experiment_unit
    }

    pub fn conduct_experiment<F>(
        &mut self,
        f: F,
        knapsack: &Knapsack,
        algorithm_names: &Vec<String>,
        experiment_name: Option<&str>,
    ) -> ExperimentUnit
    where
        F: Fn(String, &Knapsack) -> Option<u32>,
    {
        let experiment_result =
            self.conduct_experiment_with_no_log(&f, &knapsack, &algorithm_names, experiment_name);
        self.logger.log_serial(&experiment_result);
        experiment_result
    }

    pub fn conduct_batch_experiment<F>(
        &mut self,
        f: F,
        knapsacks: Vec<&Knapsack>,
        algorithm_names: &Vec<String>,
    ) -> Vec<ExperimentUnit>
    where
        F: Fn(String, &Knapsack) -> Option<u32>,
    {
        let mut experiment_results = Vec::new();
        for knapsack in &knapsacks {
            let experiment_result =
                self.conduct_experiment_with_no_log(&f, &knapsack, &algorithm_names, None);
            experiment_results.push(experiment_result);
        }
        self.logger.log_batch_sequential(&experiment_results);
        experiment_results
    }

    pub fn agreggate(&mut self, experiment_results: Vec<ExperimentUnit>) {
        struct AgreggateMetrics {
            algorithm_name: String,
            correct_rate: f64,
            mean_execution_time: f64,
            mean_memory_usage: f64,
        }

        impl AgreggateMetrics {
            pub fn display(&self) -> String {
                let mut output = String::new();
                output.push_str(&format!("{n:>20}", n = self.algorithm_name));
                output.push_str(&format!("{n:>15}", n = self.correct_rate));
                output.push_str(&format!("{n:>15}", n = self.mean_execution_time));
                output.push_str(&format!("{n:>15}", n = self.mean_memory_usage));
                output.push_str("\n");
                output
            }
        }

        let mut metrics: HashMap<String, AgreggateMetrics> = HashMap::new();
        let mut set = HashSet::new();
        let mut answers = Vec::new();
        for experiment_result in &experiment_results {
            let mut answer = 0;
            for (key, metric) in &experiment_result.metrics {
                set.insert(key);
                answer = std::cmp::max(metric.result.unwrap_or(0), answer);
            }
            answers.push(answer);
        }

        for name in &set {
            metrics.insert(
                name.to_string(),
                AgreggateMetrics {
                    algorithm_name: name.to_string(),
                    correct_rate: 0.0,
                    mean_execution_time: 0.0,
                    mean_memory_usage: 0.0,
                },
            );
        }
        let mut experiment: usize = 0;
        for experiment_result in &experiment_results {
            let answer = answers[experiment];
            experiment += 1;
            for (key, metric) in &experiment_result.metrics {
                let o = metrics.get_mut(key).unwrap();
                (*o).mean_execution_time += metric.execution_time.unwrap_or(0.0);
                (*o).mean_memory_usage += metric.memory_usage.unwrap_or(0) as f64;
                let mut d: f64 = 0.0;
                if metric.result.unwrap_or(0) == answer {
                    d = 1.0;
                }
                (*o).correct_rate += d;
            }
        }
        for (_, metric) in &mut metrics {
            metric.mean_execution_time /= experiment as f64;
            metric.mean_memory_usage /= experiment as f64;
            metric.correct_rate /= experiment as f64;
        }

        let mut out = String::new();
        out.push_str(&format!("{n:>20}", n = "algorithm"));
        out.push_str(&format!("{n:>15}", n = "correct rate"));
        out.push_str(&format!("{n:>15}", n = "mean time"));
        out.push_str(&format!("{n:>15}", n = "mean space"));
        out.push_str("\n");
        for (_, metric) in &metrics {
            out.push_str(&metric.display());
        }
        self.logger.log_display(&out);
    }
}
