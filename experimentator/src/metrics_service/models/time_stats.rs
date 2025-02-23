use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimeStats {
    mean_time_ms: f64,
    std_dev_ms: f64, 
    median_time_ms: f64,
    median_abs_dev_ms: f64,
}

impl TimeStats {
    pub fn get_mean_time(&self) -> f64 {
        return self.mean_time_ms;
    }

    pub fn get_std_dev(&self) -> f64 {
        return self.std_dev_ms;
    }

    pub fn get_median_time(&self) -> f64 {
        return self.median_time_ms;
    }

    pub fn get_median_abs_dev(&self) -> f64 {
        return self.median_abs_dev_ms;
    }
}


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
