use crate::metrics_service::models::time_stats::TimeStats;
use serde::{Deserialize, Serialize};

/// Represents a measurement of a solver's performance metrics.
///
/// Contains information about the solver's name, success rate, and timing statistics.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Measurement {
    solver_name: String,
    /// Percentage of correct solutions (0-100)
    correct_rate: f64,
    time_stats: TimeStats,
}

impl Measurement {
    /// Returns the name of the solver.
    ///
    /// # Returns
    ///
    /// * `String` - The solver's name
    pub fn get_solver_name(&self) -> String {
        self.solver_name.clone()
    }

    /// Returns the correct solution rate of the solver.
    ///
    /// # Returns
    ///
    /// * `f64` - The percentage of correct solutions (0-100)
    pub fn get_correct_rate(&self) -> f64 {
        self.correct_rate
    }

    /// Returns the timing statistics for the solver.
    ///
    /// # Returns
    ///
    /// * `TimeStats` - Statistics about the solver's execution time
    pub fn get_time_stats(&self) -> TimeStats {
        self.time_stats.clone()
    }
}

/// Implements conversion from a tuple of solver data into a Measurement.
///
/// # Arguments
///
/// * `tuple` - A tuple containing (solver_name, correct_rate, time_stats)
impl From<(String, &f64, &TimeStats)> for Measurement {
    fn from(tuple: (String, &f64, &TimeStats)) -> Measurement {
        Measurement {
            solver_name: tuple.0,
            correct_rate: tuple.1.clone(),
            time_stats: tuple.2.clone(),
        }
    }
}
