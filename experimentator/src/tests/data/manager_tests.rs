use crate::data::manager::generate_rnd_knapsacks;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_generate_rnd_knapsacks() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("experiments.json");
    let mut file = File::create(&file_path).unwrap();

    let json_data = r#"[
        {
            "num_items": 5,
            "capacity": 50,
            "weights_range": [1, 10],
            "costs_range": [1, 10]
        }
    ]"#;

    file.write_all(json_data.as_bytes()).unwrap();

    let result = generate_rnd_knapsacks(&file_path);
    assert!(result.is_ok());
    let knapsacks = result.unwrap();
    assert_eq!(knapsacks.len(), 1);

    let knapsack = &knapsacks[0];
    assert_eq!(knapsack.get_items_len(), 5);
    assert_eq!(knapsack.get_capacity(), 50);

    for i in 0..knapsack.get_items_len() {
        let item = knapsack.get_item(i);

        assert!(item.get_weight() >= 1 && item.get_weight() <= 10);
        assert!(item.get_value() >= 1 && item.get_value() <= 10);
    }
}
