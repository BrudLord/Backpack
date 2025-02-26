use serde::{Deserialize, Serialize};

/// Statistics about execution time measurements.
///
/// Contains mean, standard deviation, median and median absolute deviation
/// of execution time measurements in milliseconds.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimeStats {
    mean_time_ms: f64,
    std_dev_ms: f64,
    median_time_ms: f64,
    median_abs_dev_ms: f64,
}

impl TimeStats {
    /// Returns the mean execution time in milliseconds.
    pub fn get_mean_time(&self) -> f64 {
        return self.mean_time_ms;
    }

    /// Returns the standard deviation of execution time in milliseconds.
    pub fn get_std_dev(&self) -> f64 {
        return self.std_dev_ms;
    }

    /// Returns the median execution time in milliseconds.
    pub fn get_median_time(&self) -> f64 {
        return self.median_time_ms;
    }

    /// Returns the median absolute deviation of execution time in milliseconds.
    pub fn get_median_abs_dev(&self) -> f64 {
        return self.median_abs_dev_ms;
    }
}

/// Implements conversion from a tuple of statistics to TimeStats.
///
/// # Arguments
///
/// * `t` - Tuple containing (mean, std_dev, median, median_abs_dev) in milliseconds
impl From<(f64, f64, f64, f64)> for TimeStats {
    fn from(t: (f64, f64, f64, f64)) -> TimeStats {
        TimeStats {
            mean_time_ms: t.0,
            std_dev_ms: t.1,
            median_time_ms: t.2,
            median_abs_dev_ms: t.3,
        }
    }
}
