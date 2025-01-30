use crate::data::config_manager::read_rand_config;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
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
fn test_read_rand_config_invalid() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("invalid.json");
    let mut file = File::create(&file_path).unwrap();

    let invalid_json = "{ not valid json }";
    file.write_all(invalid_json.as_bytes()).unwrap();

    let result = read_rand_config(&file_path);
    assert!(result.is_err());
}
