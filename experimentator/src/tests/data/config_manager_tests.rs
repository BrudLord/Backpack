use crate::data::config_manager::read_rand_config;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
/// Tests `read_rand_config` with a valid JSON configuration file.
///
/// This test creates a temporary directory and file, writes valid JSON data representing
/// an experiment configuration to the file, and then calls `read_rand_config`.
///
/// # Expected behavior
///
/// The function should return `Ok` containing a vector with one `ExperimentConfig` struct.
/// The fields of the config struct should match the values in the JSON data.
///
/// # Error behavior
///
/// The test will panic if any of the file operations fail or if the assertions fail.  These
/// panics indicate a problem with the test setup itself, not necessarily with the
/// `read_rand_config` function.
fn test_read_rand_config_valid() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("experiments.json");
    let mut file = File::create(&file_path).unwrap();

    let json_data = r#"[
        {
            "num_items": 10,
            "capacity": 10,
            "weights_range": [1, 1000],
            "costs_range": [1, 1000]
        }
    ]"#;

    file.write_all(json_data.as_bytes()).unwrap();

    let result = read_rand_config(&file_path);
    assert!(result.is_ok());
    let configs = result.unwrap();
    assert_eq!(configs.len(), 1);
    assert_eq!(configs[0].num_items, 10);
    assert_eq!(configs[0].capacity, 10);
}

#[test]
/// Tests `read_rand_config` with an invalid JSON configuration file.
///
/// This test creates a temporary directory and file, writes invalid JSON data to the file,
/// and then calls `read_rand_config`.
///
/// # Expected behavior
///
/// The function should return `Err`.
///
/// # Error behavior
///
/// The test will panic if any of the file operations fail.  These panics indicate a
/// problem with the test setup itself, not necessarily with the `read_rand_config`
/// function. The function itself should return an `Err` due to the invalid json.
fn test_read_rand_config_invalid() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("invalid.json");
    let mut file = File::create(&file_path).unwrap();

    let invalid_json = "{ not valid json }";
    file.write_all(invalid_json.as_bytes()).unwrap();

    let result = read_rand_config(&file_path);
    assert!(result.is_err());
}