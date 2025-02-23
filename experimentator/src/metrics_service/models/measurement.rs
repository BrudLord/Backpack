use serde::{Deserialize, Serialize};
use crate::metrics_service::models::time_stats::TimeStats;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Measurement {
    solver_name: String,
    correct_rate: f64, 
    time_stats: TimeStats,
}

impl Measurement {
    pub fn get_solver_name(&self) -> String {
        return self.solver_name.clone();
    }

    pub fn get_correct_rate(&self) -> f64 {
        return self.correct_rate;
    }

    pub fn get_time_stats(&self) -> TimeStats {
        return self.time_stats.clone();
    }
}

impl From<(String, &f64, &TimeStats)> for Measurement {
    fn from(tuple: (String, &f64, &TimeStats)) -> Measurement {
        Measurement {
            solver_name: tuple.0,
            correct_rate: tuple.1.clone(),
            time_stats: tuple.2.clone(),
        }
    }
}

