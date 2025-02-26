use crate::metrics_service::models::time_stats::TimeStats;
use serde_json::Value;
use std::{collections::HashMap, env, ffi::OsStr, fs, path::PathBuf};
use walkdir::WalkDir;

/// Returns the base directory for criterion benchmark results
fn get_criterion_start_dir() -> PathBuf {
    let cwd = get_start_dir();
    cwd.join("target/criterion")
}

fn get_start_dir() -> PathBuf {
    let cwd = env::current_dir().expect("Failed to get current working directory");
    let base_dir = env::args().nth(2).unwrap_or_else(|| ".".to_string());
    cwd.join(base_dir)
}

/// Extracts a specific statistical estimate from JSON data
///
/// # Arguments
/// * `stat_name` - Name of the statistic to extract (mean, median, etc.)
/// * `json` - JSON object containing the statistical data
fn get_point_estimate(stat_name: &str, json: &Value) -> Result<f64, String> {
    json[stat_name]["point_estimate"]
        .as_f64()
        .map(|stat| format!("{:.4e}", stat).parse::<f64>().unwrap())
        .ok_or_else(|| format!("Failed to get {} estimate", stat_name))
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

/// Collects and processes benchmark statistics from criterion output
///
/// Returns a HashMap mapping solver names to their performance statistics,
/// converting nanoseconds to milliseconds in the process
pub fn get_criterion_stats() -> Result<HashMap<String, TimeStats>, String> {
    let start_dir = get_criterion_start_dir();
    println!("Looking for criterion data in: {:?}", start_dir);
    let mut measurements = HashMap::new();
    const NS_TO_MS: f64 = 1000000.0;

    // Iterate through criterion output files
    for entry in WalkDir::new(start_dir)
        .into_iter()
        .filter_map(|e| {
            let result = e.ok();
            println!("Found file: {:?}", result.as_ref().map(|e| e.path())); // Debug print
            result
        })
        .filter(|e| {
            let matches = e.path().is_file()
                && e.file_name() == "estimates.json"
                && e.path()
                    .ancestors()
                    .any(|p| p.file_name() == Some(OsStr::new("new")));
            println!("File matches criteria: {:?} -> {}", e.path(), matches); // Debug print
            matches
        })
    {
        let contents = fs::read_to_string(entry.path())
            .map_err(|_| format!("Error reading file {:?}", entry.file_name()))?;
        let json: Value = serde_json::from_str(&contents).unwrap();
        println!("{}", json);

        // Extract all relevant statistics
        let stats = [
            get_point_estimate("mean", &json)?,
            get_point_estimate("std_dev", &json)?,
            get_point_estimate("median", &json)?,
            get_point_estimate("median_abs_dev", &json)?,
        ];

        // Extract solver name from path components
        let components: Vec<_> = entry
            .path()
            .components()
            .map(|c| c.as_os_str().to_str().unwrap_or(""))
            .collect();

        let target_idx = components
            .iter()
            .position(|&c| c == "target")
            .expect("'target' not found");
        let criterion_idx = components[target_idx..]
            .iter()
            .position(|&c| c == "criterion")
            .expect("'criterion' not found");

        let solver_name = capitalize_first(
            components[target_idx + criterion_idx + 2]
                .to_string()
                .as_ref(),
        );

        println!("{:?}", solver_name);

        // Store measurements if not already present
        if !measurements.contains_key(&solver_name) {
            measurements.insert(
                solver_name,
                TimeStats::from((
                    stats[0] / NS_TO_MS,
                    stats[1] / NS_TO_MS,
                    stats[2] / NS_TO_MS,
                    stats[3] / NS_TO_MS,
                )),
            );
        }
    }
    Ok(measurements)
}

/// Cleans up the criterion directory after benchmarking
pub fn delete_criterion_dir() {
    if let Err(e) = fs::remove_dir_all("target/criterion") {
        eprintln!("Failed to delete criterion directory: {}", e);
    }
}

/// Collects and moves mean plot SVGs to a user-specific assets directory
///
/// Moves all mean.svg files from criterion results into assets dir
pub fn get_mean_plots(os_string: String) {
    const MEAN_IMAGE_NAME: &str = "mean.svg";
    let asset_dir = "assets".to_string();
    let start_dir = get_criterion_start_dir();
    let cwd = env::current_dir().expect("Failed to get current working directory");
    let base_dir = env::args().nth(2).unwrap_or_else(|| ".".to_string());
    let end_dir = cwd.join(base_dir).join(asset_dir).join(os_string);

    let _ = fs::create_dir_all(&end_dir);

    for entry in WalkDir::new(&start_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.file_name() == Some(OsStr::new(MEAN_IMAGE_NAME)) {
            match path.strip_prefix(&start_dir) {
                Ok(relative_path) => {
                    let target_path = end_dir.join(relative_path);
                    let target_dir = target_path.parent();
                    if let Some(dir) = target_dir {
                        fs::create_dir_all(dir).expect("Failed to create directory");
                    }
                    match fs::rename(&path, &target_path) {
                        Ok(_) => println! {"Moved {} to {}", path.display(), target_path.display()},
                        Err(e) => {
                            println! {"Error moving {} to {}: {}", path.display(), target_path.display(), e}
                        }
                    }
                }
                Err(e) => println! {"Error getting relative path for {}: {}", path.display(), e},
            }
        }
    }
}
