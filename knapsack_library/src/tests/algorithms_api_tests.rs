use crate::algorithms_api::AlgorithmsAPI;
use crate::models::knapsack::Knapsack;
use crate::models::item::Item;

#[test]
fn test_knapsack_all() {
    let item1 = Item::new(5, 10);
    let item2 = Item::new(3, 7);
    let item3 = Item::new(2, 5);
    
    let mut knapsack = Knapsack::new(10, vec![item1, item2, item3]);

    let mut best_value: u32 = 0;
    assert_eq!(AlgorithmsAPI::new().solve_knapsack(&mut knapsack), 22);
}

#[test]
fn test_knapsack_one_odd() {
    let item1 = Item::new(5, 10);
    let item2 = Item::new(3, 7);
    let item3 = Item::new(3, 5);
    
    let mut knapsack = Knapsack::new(10, vec![item1, item2, item3]);

    let mut best_value: u32 = 0;
    assert_eq!(AlgorithmsAPI::new().solve_knapsack(&mut knapsack), 17);
}

#[test]
fn test_knapsack_empty() {
    let item1 = Item::new(15, 10);
    let item2 = Item::new(33, 7);
    let item3 = Item::new(3666, 5);
    
    let mut knapsack = Knapsack::new(10, vec![item1, item2, item3]);

    let mut best_value: u32 = 0;
    assert_eq!(AlgorithmsAPI::new().solve_knapsack(&mut knapsack), 0);
}