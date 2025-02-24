use crate::metrics_service::models::time_stats::TimeStats;
use serde_json::Value;
use std::collections::HashMap;
use std::{env, ffi::OsStr, fs, path::PathBuf};
use walkdir::WalkDir;

fn get_criterion_start_dir() -> PathBuf {
    let criterion_postfix = "target/criterion";
    let cwd = env::current_dir().expect("Failed to get current working directory");
    let base_dir = env::args().nth(2).unwrap_or_else(|| ".".to_string());
    let start_dir = cwd.join(base_dir).join(criterion_postfix);

    start_dir
}

fn get_point_estimate(stat_name: &str, json: &Value) -> Result<f64, String> {
    let mut stat = json[stat_name]["point_estimate"].as_f64().unwrap();
    stat = format!("{:.4e}", stat).parse::<f64>().unwrap();
    Ok(stat)
}

pub fn get_criterion_stats() -> Result<HashMap<String, TimeStats>, String> {
    let filename_stats = "estimates.json";
    let start_dir = get_criterion_start_dir();

    let mut measurements: HashMap<String, TimeStats> = HashMap::new();

    for entry in WalkDir::new(start_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file()
            && path.file_name() == Some(OsStr::new(filename_stats))
            && path
                .ancestors()
                .any(|parent| parent.file_name() == Some(OsStr::new("new")))
        {
            match fs::read_to_string(path) {
                Ok(contents) => {
                    let json: Value = serde_json::from_str(&contents).unwrap();

                    let mean = get_point_estimate("mean", &json).unwrap();
                    let std_dev = get_point_estimate("std_dev", &json).unwrap();
                    let median = get_point_estimate("median", &json).unwrap();
                    let median_abs_dev = get_point_estimate("median_abs_dev", &json).unwrap();

                    let components: Vec<_> = path
                        .components()
                        .map(|c| c.as_os_str().to_str().unwrap_or(""))
                        .collect();
                    println!("components: {:?}", components);

                    let target_index = components
                        .iter()
                        .position(|&component| component == "target")
                        .expect("'target' not found.");
                    let criterion_index = &components[target_index + 1..]
                        .iter()
                        .position(|&component| component == "criterion")
                        .expect("'criterion' not found after 'target'.");

                    let rest_slice = &components[target_index + criterion_index + 2..];
                    let solver_name = rest_slice[1].to_string();

                    println!("{}", rest_slice[1]);
                    // println!("rest_slice: {:?}", rest_slice);
                    let rest_slice = &rest_slice[..rest_slice.len() - 2];
                    // println!("rest_slice2: {:?}", rest_slice);
                    let mut rest = rest_slice.join(",");
                    if rest_slice.len() == 2 {
                        rest = format!("{},", rest);
                    }

                    println!(
                        "{},{},{},{},{}",
                        rest, mean, median, std_dev, median_abs_dev
                    );
                    let ns_to_ms = 1000000.0;
                    if !measurements.contains_key(&solver_name) {
                        measurements.insert(
                            solver_name.clone(),
                            (
                                mean / ns_to_ms,
                                std_dev / ns_to_ms,
                                median / ns_to_ms,
                                median_abs_dev / ns_to_ms,
                            )
                                .into(),
                        );
                    }
                }
                Err(_e) => {
                    return Err(format!(
                        "Error reading file {:?}",
                        path.file_name().unwrap()
                    ))
                }
            }
        }
    }
    return Ok(measurements);
}

pub fn delete_criterion_dir() {
    let path = "target/criterion";
    fs::remove_dir_all(path).unwrap();
}

pub fn get_mean_plots() {
    const MEAN_IMAGE_NAME: &str = "mean.svg";
    let asset_dir = "assets".to_string();
    let start_dir = get_criterion_start_dir();
    let cwd = env::current_dir().expect("Failed to get current working directory");
    let base_dir = env::args().nth(2).unwrap_or_else(|| ".".to_string());
    let end_dir = cwd.join(base_dir).join(asset_dir);

    fs::create_dir_all(&end_dir).expect("Failed to create assets directory");

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
