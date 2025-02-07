//! Tests for the reporter functionality in the metrics service.
use crate::metrics_service::metrics_service::{Measurement, MetricsUnit};
use crate::metrics_service::reporter::{ConsoleReporter, FileReporter, ReporterType};
use knapsack_library::models::item::Item;
use knapsack_library::models::knapsack::Knapsack;
use std::collections::HashMap;
use std::fs;

/// Creates a sample measurement for testing purposes
///
/// # Returns
/// A `Measurement` instance with:
/// - Knapsack of capacity 100 and 3 items
/// - Single metric named "test_metric"
/// - Predefined execution time and memory usage values
fn sample_measurement() -> Measurement {
    let knapsack = Knapsack::new(
        100,
        vec![Item::new(60, 10), Item::new(100, 20), Item::new(120, 30)],
    );

    let mut metrics = HashMap::new();
    metrics.insert(
        "test_metric".to_string(),
        MetricsUnit {
            result: Some(200),
            execution_time_ns: Some(123456789),
            memory_usage: Some(1024),
        },
    );

    Measurement {
        experiment_name: "exp".to_string(),
        knapsack,
        metrics,
    }
}

/// Tests file reporting functionality
///
/// # Test steps
/// 1. Creates a temporary file reporter
/// 2. Generates and reports a sample measurement
/// 3. Verifies that the output contains expected content
///
/// # Expected results
/// - File should contain "test_metric"
/// - File should contain "result"
#[test]
fn test_report_to_file() {
    let temp_file = "test_report.txt";
    let reporter = ReporterType::File(FileReporter::new(Some(temp_file)));
    let measurement = sample_measurement();

    reporter.report_json(&measurement);

    let contents = fs::read_to_string(temp_file).unwrap();
    assert!(contents.contains("test_metric"));
    assert!(contents.contains("result"));
}

/// Tests batch reporting to console
///
/// # Test steps
/// 1. Creates a console reporter
/// 2. Generates multiple sample measurements
/// 3. Reports them in batch
///
/// # Expected results
/// - Should print multiple measurements to console
/// - No assertions as this is a visual test
#[test]
fn test_report_batch() {
    let reporter = ReporterType::Console(ConsoleReporter::new());
    let measurements = vec![sample_measurement(), sample_measurement()];
    reporter.report_batch(&measurements);
}
