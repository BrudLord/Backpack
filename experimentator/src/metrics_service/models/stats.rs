/// Statistical metrics for numerical data analysis
/// 
/// Contains common statistical measures used to analyze performance data:
/// mean, median, and 95th percentile values.
#[derive(Debug, Clone)]
pub struct Stats {
    pub mean: f64,
    pub median: f64,
    pub percentile95: f64,
}

/// Aggregated metrics for algorithm performance analysis
/// 
/// Combines success rate with execution time and memory usage statistics
/// to provide a comprehensive view of algorithm performance.
#[derive(Debug, Clone)]
pub struct AggregatedMetric {
    pub algorithm_name: String, 
    pub correct_rate: f64,
    pub execution_time: Stats,
    pub memory_usage: Stats,
}

impl Stats {
        /// Creates a new Stats instance with the given statistical values
    ///
    /// # Arguments
    /// * `mean` - Arithmetic mean of the dataset
    /// * `median` - Median value of the dataset
    /// * `percentile95` - 95th percentile value
    ///
    /// # Returns
    /// * `Stats` - New Stats instance with the provided values
    pub fn new(mean: f64, median: f64, percentile95: f64) -> Self {
        Self { mean, median, percentile95 }
    }

    /// Calculates statistical metrics from a slice of numeric values
    ///
    /// # Arguments
    /// * `values` - Slice of numeric values to analyze
    ///
    /// # Returns
    /// * `Stats` - Statistical metrics calculated from the input values
    pub fn calculate_stats<T: Into<f64> + Copy>(values: &[T]) -> Stats {
        if values.is_empty() {
            return Stats::new(0.0, 0.0, 0.0);
        }
    
        let mean = values.iter()
            .map(|&v| v.into())
            .sum::<f64>() / values.len() as f64;
    
        let mid = values.len() / 2;
        let median = if values.len() % 2 == 0 {
            (values[mid - 1].into() + values[mid].into()) / 2.0
        } else {
            values[mid].into()
        };
    
        let p95_idx = ((values.len() as f64 * 0.95) as usize).min(values.len() - 1);
        let percentile95 = values[p95_idx].into();
    
        Stats::new(mean, median, percentile95)
    }
}

impl AggregatedMetric {
    /// Creates a new AggregatedMetric instance
    ///
    /// # Arguments
    /// * `algorithm_name` - Name of the algorithm
    /// * `correct_rate` - Success rate of the algorithm (0.0 to 1.0)
    /// * `execution_time` - Execution time statistics
    /// * `memory_usage` - Memory usage statistics
    ///
    /// # Returns
    /// * `AggregatedMetric` - New instance with the provided metrics
    pub fn new(algorithm_name: String, correct_rate: f64, execution_time: Stats, memory_usage: Stats) -> Self {
        Self { algorithm_name: algorithm_name, correct_rate: correct_rate, execution_time: execution_time, memory_usage: memory_usage }
    }
}
