use crate::logger::Logger;
use crate::metrics_service::{ExperimentUnit, MetricService, MetricsUnit};
use knapsack_library::algorithms_service::AlgorithmsService;
use knapsack_library::models::item::Item;
use knapsack_library::models::knapsack::Knapsack;
use rand::Rng;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::prelude::*;

fn k1() -> Knapsack {
    let item1 = Item::new(5, 10);
    let item2 = Item::new(3, 7);
    let item3 = Item::new(2, 5);

    Knapsack::new(10, vec![item1, item2, item3])
}

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

#[cfg(test)]
#[test]
fn test_metrics_unit_creation() {
    let metrics = MetricsUnit::new();
    assert!(metrics.result.is_none());
    assert!(metrics.execution_time.is_none());
    assert!(metrics.memory_usage.is_none());
}

#[test]
fn test_metrics_unit_from_tuple() {
    let tuple = (Some(42), Some(1.23), Some(512));
    let metrics: MetricsUnit = tuple.into();
    assert_eq!(metrics.result, Some(42));
    assert_eq!(metrics.execution_time, Some(1.23));
    assert_eq!(metrics.memory_usage, Some(512));
}

#[test]
fn test_experiment_unit_creation() {
    let knapsack = k1();
    let experiment_unit = ExperimentUnit::new("exp".to_string(), &knapsack);
    assert_eq!(experiment_unit.knapsack, knapsack);
    assert!(experiment_unit.metrics.is_empty());
}

#[test]
fn test_conduct_experiment_with_algorithms() {
    let knapsack = k1();
    let mut service: AlgorithmsService = AlgorithmsService::new();
    let algo_names = AlgorithmsService::get_algorithms_names();
    let mut metricService = MetricService::new(Some("out.txt"));
    metricService.conduct_experiment(
        |s, k| service.solve(s, k),
        &knapsack,
        &algo_names,
        Some("sample1"),
    );
    fs::remove_file("out.txt").unwrap();
}

fn test_conduct_experiment_with_aggregation() {
    let knapsack = k1();
    let knapsack2 = k2();
    let mut service: AlgorithmsService = AlgorithmsService::new();

    let algo_names = AlgorithmsService::get_algorithms_names();

    let mut metricService = MetricService::new(Some("outs.txt"));
    let out = metricService.conduct_batch_experiment(
        |s, k| service.solve(s, k),
        vec![&knapsack, &knapsack2],
        &algo_names,
    );
    metricService.agreggate(out);
    fs::remove_file("outs.txt").unwrap();
}

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
            execution_time: Some(1.23),
            memory_usage: Some(1024),
        },
    );

    metrics.insert(
        "test_metric_1".to_string(),
        MetricsUnit {
            result: Some(100),
            execution_time: Some(1.0),
            memory_usage: Some(10244),
        },
    );

    let mut metrics1 = HashMap::new();
    metrics1.insert(
        "test_metric".to_string(),
        MetricsUnit {
            result: Some(300),
            execution_time: Some(1.0),
            memory_usage: Some(1024),
        },
    );

    metrics1.insert(
        "test_metric_1".to_string(),
        MetricsUnit {
            result: Some(400),
            execution_time: Some(3.0),
            memory_usage: Some(13444),
        },
    );

    let mut rr = Vec::new();
    for i in 0..10 {
        let mut e = ExperimentUnit::new("exp".to_string(), &knapsack.clone());
        if i as u32 % 2 == 0 {
            e = ExperimentUnit {
                experiment_name: "exp".to_string(),
                knapsack: knapsack.clone(),
                metrics: metrics.clone(),
            };
        } else {
            e = ExperimentUnit {
                experiment_name: "exp".to_string(),
                knapsack: knapsack.clone(),
                metrics: metrics1.clone(),
            };
        }
        rr.push(e)
    }

    let mut metric_service = MetricService::new(Some("test_aggregation.txt"));
    metric_service.agreggate(rr);

    let mut file = File::open("test_aggregation.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut iter = contents.split_whitespace().rev();
    // there is no guaranteed order
    assert!(vec![Some("1024"), Some("11844")].contains(&iter.next()));
    assert!(vec![Some("1.115"), Some("2")].contains(&iter.next()));
    assert!(vec![Some("0.5"), Some("0.5")].contains(&iter.next()));
    assert!(vec![Some("test_metric"), Some("test_metric_1"),].contains(&iter.next()));

    fs::remove_file("test_aggregation.txt").unwrap();
}
