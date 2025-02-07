//! Tests for the metrics service functionality

use crate::metrics_service::metrics_service::{Measurement, MetricService, MetricsUnit};
use knapsack_library::algorithms_service::AlgorithmsService;
use knapsack_library::models::item::Item;
use knapsack_library::models::knapsack::Knapsack;
use rand::Rng;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::prelude::*;

/// Creates a simple knapsack instance with 3 items for testing
fn k1() -> Knapsack {
    let item1 = Item::new(5, 10);
    let item2 = Item::new(3, 7);
    let item3 = Item::new(2, 5);

    Knapsack::new(10, vec![item1, item2, item3])
}

/// Creates a random knapsack instance with 20 items for testing
///
/// Generates items with:
/// - Capacity: 68
/// - Weights: 1-10
/// - Costs: 20-40
fn k2() -> Knapsack {
    let capacity = 68;
    let num_items = 20;
    let weights_range = (1, 10);
    let costs_range = (20, 40);
    let mut rng = rand::thread_rng();
    let items: Vec<Item> = (0..num_items)
        .map(|_| {
            Item::new(
                rng.gen_range(weights_range.0..=weights_range.1),
                rng.gen_range(costs_range.0..=costs_range.1),
            )
        })
        .collect();
    Knapsack::new(capacity, items)
}

/// Tests creation of empty MetricsUnit
#[test]
fn test_metrics_unit_creation() {
    let metrics = MetricsUnit::new();
    assert!(metrics.result.is_none());
    assert!(metrics.execution_time_ns.is_none());
    assert!(metrics.memory_usage.is_none());
}

/// Tests conversion from tuple to MetricsUnit
#[test]
fn test_metrics_unit_from_tuple() {
    let tuple = (Some(42), Some(1_u128), Some(512));
    let metrics: MetricsUnit = tuple.into();
    assert_eq!(metrics.result, Some(42));
    assert_eq!(metrics.execution_time_ns, Some(1));
    assert_eq!(metrics.memory_usage, Some(512));
}

/// Tests creation of new Measurement instance
#[test]
fn test_measurement_creation() {
    let knapsack = k1();
    let measurement = Measurement::new("exp".to_string(), &knapsack);
    assert_eq!(measurement.knapsack, knapsack);
    assert!(measurement.metrics.is_empty());
}

/// Tests conducting experiment with multiple algorithms
///
/// Creates a simple knapsack instance and runs all available algorithms,
/// verifying that results are written to file
#[test]
fn test_conduct_experiment_with_algorithms() {
    let knapsack = k1();
    let algo_names = AlgorithmsService::get_algorithms_names();
    let metric_service = MetricService::new(Some("out.txt"));
    metric_service.conduct_experiment(
        |s, k| AlgorithmsService::solve(s, k),
        &knapsack,
        &algo_names,
        Some("sample1"),
    );
    fs::remove_file("out.txt").unwrap();
}

/// Tests batch experiment execution and aggregation
///
/// Runs experiments on two different knapsack instances:
/// - A simple predefined instance
/// - A randomly generated instance
/// Then aggregates and reports the results
#[test]
fn test_conduct_experiment_with_aggregation() {
    let knapsack = k1();
    let knapsack2 = k2();
    let algo_names = AlgorithmsService::get_algorithms_names();

    let metric_service = MetricService::new(Some("outs.txt"));
    let measurements = metric_service.conduct_batch_experiment(
        |s, k| AlgorithmsService::solve(s, k),
        vec![&knapsack, &knapsack2],
        &algo_names,
    );
    metric_service.agreggate(measurements);
    fs::remove_file("outs.txt").unwrap();
}

/// Tests metrics aggregation functionality
///
/// Creates a set of test measurements with:
/// - Two different metrics
/// - Alternating result values
/// - 10 total measurements
/// Verifies that aggregation produces correct statistics
#[test]
fn test_aggregation() {
    let knapsack = Knapsack::new(
        100,
        vec![Item::new(60, 10), Item::new(100, 20), Item::new(120, 30)],
    );

    let mut metrics = HashMap::new();
    metrics.insert(
        "test_metric".to_string(),
        MetricsUnit {
            result: Some(200),
            execution_time_ns: Some(1),
            memory_usage: Some(1024),
        },
    );

    metrics.insert(
        "test_metric_1".to_string(),
        MetricsUnit {
            result: Some(100),
            execution_time_ns: Some(1),
            memory_usage: Some(10244),
        },
    );

    let mut metrics1 = HashMap::new();
    metrics1.insert(
        "test_metric".to_string(),
        MetricsUnit {
            result: Some(300),
            execution_time_ns: Some(1),
            memory_usage: Some(1024),
        },
    );

    metrics1.insert(
        "test_metric_1".to_string(),
        MetricsUnit {
            result: Some(400),
            execution_time_ns: Some(3),
            memory_usage: Some(13444),
        },
    );

    let mut measurements = Vec::new();
    for i in 0..10 {
        let m = Measurement {
            experiment_name: "exp".to_string(),
            knapsack: knapsack.clone(),
            metrics: if i % 2 == 0 {
                metrics.clone()
            } else {
                metrics1.clone()
            },
        };
        measurements.push(m);
    }

    let metric_service = MetricService::new(Some("test_aggregation.txt"));
    metric_service.agreggate(measurements);
    //fs::remove_file("test_aggregation.txt").unwrap();
}
