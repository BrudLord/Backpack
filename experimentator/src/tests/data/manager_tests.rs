use crate::data::manager::generate_rnd_knapsacks;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
/// Tests the `generate_rnd_knapsacks` function.
///
/// This test creates a temporary file with valid JSON configuration data, then calls
/// `generate_rnd_knapsacks` to create knapsacks based on that data.
///
/// # Expected behavior
///
/// The function should return `Ok` containing a vector of `Knapsack` instances. The number
/// of knapsacks, the number of items within each knapsack, the capacity of each
/// knapsack, and the weight and value ranges of the items should all match the
/// configuration data in the JSON file.
///
/// # Error behavior
///
/// The test will panic if any of the file operations fail.  These panics indicate a
/// problem with the test setup itself, not necessarily with the
/// `generate_rnd_knapsacks` function. The function itself should return an `Err` if
/// the config file is unreadable or contains invalid json.
fn test_generate_rnd_knapsacks() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("experiments.json");
    let mut file = File::create(&file_path).unwrap();

    let json_data = r#"{
        "generations": 10,
        "algorithms": ["Dynamic", "Greedy"],
        "num_items": 5,
        "capacity": 50,
        "weights_range": [1, 10],
        "costs_range": [1, 10]
    }"#;

    file.write_all(json_data.as_bytes()).unwrap();

    let result = generate_rnd_knapsacks(&file_path);
    assert!(result.is_ok());
    let (knapsacks, _) = result.unwrap();
    assert_eq!(knapsacks.len(), 10);

    let knapsack = &knapsacks[0];
    assert_eq!(knapsack.get_items_len(), 5);
    assert_eq!(knapsack.get_capacity(), 50);

    for i in 0..knapsack.get_items_len() {
        let item = knapsack.get_item(i);

        assert!(item.get_weight() >= 1 && item.get_weight() <= 10);
        assert!(item.get_value() >= 1 && item.get_value() <= 10);
    }
}
