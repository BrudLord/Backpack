#[cfg(test)]
mod tests {
    use super::*;
    use crate::metrics_service::metrics_service::MetricService;
    use crate::metrics_service::models::measurement::Measurement;
    use crate::metrics_service::models::metrics_data::MetricsData;
    use crate::metrics_service::models::stats::{AggregatedMetric, Stats};
    use knapsack_library::models::item::Item;
    use knapsack_library::models::knapsack::Knapsack;
    use std::collections::HashMap;
    use std::io;
    use tempfile::NamedTempFile;

    fn create_test_knapsack() -> Knapsack {
        let items = vec![Item::new(5, 10), Item::new(3, 7), Item::new(2, 5)];
        Knapsack::new(10, items)
    }

    fn create_test_measurement(name: &str, result: Option<u64>) -> Measurement {
        let mut metrics = HashMap::new();
        metrics.insert(
            "test_algo".to_string(),
            MetricsData {
                result,
                execution_time_ns: Some(1_000_000),
                memory_usage: Some(1024),
            },
        );

        Measurement {
            experiment_name: name.to_string(),
            knapsack: create_test_knapsack(),
            metrics,
        }
    }

    fn create_test_measurements() -> Vec<Measurement> {
        vec![
            create_test_measurement("test1", Some(42)),
            create_test_measurement("test2", Some(24)),
        ]
    }

    #[test]
    fn test_service_creation() -> io::Result<()> {
        let service = MetricService::new(None);
        assert!(service.is_ok());
        Ok(())
    }

    #[test]
    fn test_conduct_single_experiment() -> io::Result<()> {
        let service = MetricService::new(None)?;
        let knapsack = create_test_knapsack();
        let algo_names = vec!["test_algo".to_string()];

        let measurement =
            service.conduct_experiment(|_, _| Some(42), &knapsack, &algo_names, Some("test"));

        assert_eq!(measurement.experiment_name, "test");
        assert_eq!(measurement.metrics.len(), 1);
        assert!(measurement
            .metrics
            .get("test_algo")
            .unwrap()
            .result
            .is_some());
        Ok(())
    }

    #[test]
    fn test_conduct_batch_experiment() -> io::Result<()> {
        let service = MetricService::new(None)?;
        let knapsacks = vec![create_test_knapsack(), create_test_knapsack()];
        let algo_names = vec!["test_algo".to_string()];

        let measurements = service.conduct_batch_experiment(
            |_, _| Some(42),
            knapsacks.iter().collect(),
            &algo_names,
        );

        assert_eq!(measurements.len(), 2);
        for measurement in measurements {
            assert_eq!(measurement.metrics.len(), 1);
            assert!(measurement
                .metrics
                .get("test_algo")
                .unwrap()
                .result
                .is_some());
        }
        Ok(())
    }

    #[test]
    fn test_get_bests_results() -> io::Result<()> {
        let measurements = create_test_measurements();

        let max_results = Measurement::get_bests_results(&measurements);
        assert_eq!(max_results, vec![42, 24]);
        Ok(())
    }

    #[test]
    fn test_group_metrics_by_algorithm() -> io::Result<()> {
        let service = MetricService::new(None)?;
        let measurements = create_test_measurements();

        let grouped = Measurement::group_metrics_by_algorithm(&measurements);
        assert_eq!(grouped.len(), 1); // One algorithm
        assert_eq!(grouped.get("test_algo").unwrap().len(), 2); // Two measurements
        Ok(())
    }

    #[test]
    fn test_aggregate_metrics() -> io::Result<()> {
        let temp_file = NamedTempFile::new()?;
        let service = MetricService::new(Some(temp_file.path().to_str().unwrap()))?;
        let measurements = create_test_measurements();

        let metrics = service.aggregate(measurements);

        assert_eq!(metrics.len(), 1); // One algorithm
        let metric = &metrics[0];
        assert_eq!(metric.algorithm_name, "test_algo");
        assert_eq!(metric.correct_rate, 1.0); // All results present
        assert!(metric.execution_time.mean > 0.0);
        assert!(metric.memory_usage.mean > 0.0);
        Ok(())
    }

    #[test]
    fn test_write_measurement() -> io::Result<()> {
        let temp_file = NamedTempFile::new()?;
        let service = MetricService::new(Some(temp_file.path().to_str().unwrap()))?;
        let measurement = create_test_measurement("write_test", Some(42));

        // This should not panic
        service.write_measurement(&measurement);
        Ok(())
    }

    #[test]
    fn test_write_batch_measurement() -> io::Result<()> {
        let temp_file = NamedTempFile::new()?;
        let service = MetricService::new(Some(temp_file.path().to_str().unwrap()))?;
        let measurements = create_test_measurements();

        // This should not panic
        service.write_batch_measurement(&measurements);
        Ok(())
    }

    #[test]
    fn test_aggregate_with_incorrect_results() -> io::Result<()> {
        let temp_file = NamedTempFile::new()?;
        let service = MetricService::new(Some(temp_file.path().to_str().unwrap()))?;

        // Create measurements with some incorrect results
        let mut measurements = vec![
            create_test_measurement("test1", Some(42)), // Correct
            create_test_measurement("test2", Some(40)), // Incorrect
            create_test_measurement("test3", None),     // Failed
        ];

        let metrics = service.aggregate(measurements);

        assert_eq!(metrics.len(), 1); // One algorithm
        let metric = &metrics[0];
        assert_eq!(metric.algorithm_name, "test_algo");
        assert!(metric.correct_rate < 1.0); // Not all results are correct
        assert!(metric.execution_time.mean > 0.0);
        Ok(())
    }

    #[test]
    fn test_aggregate_with_missing_metrics() -> io::Result<()> {
        let temp_file = NamedTempFile::new()?;
        let service = MetricService::new(Some(temp_file.path().to_str().unwrap()))?;

        // Create a measurement with missing execution time and memory usage
        let mut measurement = create_test_measurement("test1", Some(42));
        measurement
            .metrics
            .get_mut("test_algo")
            .unwrap()
            .execution_time_ns = None;
        measurement
            .metrics
            .get_mut("test_algo")
            .unwrap()
            .memory_usage = None;

        let metrics = service.aggregate(vec![measurement]);

        assert_eq!(metrics.len(), 1);
        let metric = &metrics[0];
        assert_eq!(metric.execution_time.mean, 0.0);
        assert_eq!(metric.memory_usage.mean, 0.0);
        Ok(())
    }

    #[test]
    fn test_aggregate_with_multiple_algorithms() -> io::Result<()> {
        let temp_file = NamedTempFile::new()?;
        let service = MetricService::new(Some(temp_file.path().to_str().unwrap()))?;

        let mut measurement = create_test_measurement("multi_algo", Some(42));
        measurement.metrics.insert(
            "another_algo".to_string(),
            MetricsData {
                result: Some(42),
                execution_time_ns: Some(2_000_000),
                memory_usage: Some(2048),
            },
        );

        let metrics = service.aggregate(vec![measurement]);

        assert_eq!(metrics.len(), 2); // Two algorithms
        assert!(metrics.iter().any(|m| m.algorithm_name == "test_algo"));
        assert!(metrics.iter().any(|m| m.algorithm_name == "another_algo"));
        Ok(())
    }
}
