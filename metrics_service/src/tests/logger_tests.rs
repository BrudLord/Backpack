use crate::logger::Logger;
use crate::metrics_service::{ExperimentUnit, MetricsUnit};
use knapsack_library::models::item::Item;
use knapsack_library::models::knapsack::Knapsack;
use std::collections::HashMap;
use std::fs;

fn sample_experiment_unit() -> ExperimentUnit {
    let knapsack = Knapsack::new(
        100,
        vec![Item::new(60, 10), Item::new(100, 20), Item::new(120, 30)],
    );

    let mut metrics = HashMap::new();
    metrics.insert(
        "test_metric".to_string(),
        MetricsUnit {
            result: Some(200),
            execution_time: Some(1.23),
            memory_usage: Some(1024),
        },
    );

    ExperimentUnit {
        experiment_name: "exp".to_string(),
        knapsack,
        metrics,
    }
}
#[cfg(test)]
#[test]
fn test_log_to_file() {
    let temp_file = "test_log.txt";
    let mut logger = Logger::new(true, Some(temp_file)).unwrap();
    let experiment_unit = sample_experiment_unit();

    logger.log_serial(&experiment_unit);

    let contents = fs::read_to_string(temp_file).unwrap();
    assert!(contents.contains("test_metric"));
    assert!(contents.contains("result"));
}

#[test]
fn test_log_batch_sequential() {
    let mut logger = Logger::new(false, None).unwrap();
    let experiment_units = vec![sample_experiment_unit(), sample_experiment_unit()];
    logger.log_batch_sequential(&experiment_units);
}
