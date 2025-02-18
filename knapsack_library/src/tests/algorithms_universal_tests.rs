use crate::algorithms_service::AlgorithmsService;
use crate::models::knapsack::Knapsack;
use crate::models::item::Item;

#[test]
// We check that the algorithm will take all the items if their total weight is equal to the weight of the backpack.
fn test_knapsack_all() {
    let item1 = Item::new(5, 10);
    let item2 = Item::new(3, 7);
    let item3 = Item::new(2, 5);
    
    let knapsack = Knapsack::new(10, vec![item1, item2, item3]);

    for solver in AlgorithmsService::get_all_algorithms() {
        assert_eq!(solver.solve(&knapsack), Ok(22));
    }
    
}

#[test]
// We check that the algorithm correctly identifies the extra item.
fn test_knapsack_one_odd() {
    let item1 = Item::new(5, 10);
    let item2 = Item::new(3, 7);
    let item3 = Item::new(3, 5);
    
    let knapsack = Knapsack::new(10, vec![item1, item2, item3]);

    for solver in AlgorithmsService::get_all_algorithms() {
        assert_eq!(solver.solve(&knapsack), Ok(17));
    }
}

#[test]
// We check that the algorithm will work correctly when no item can be taken.
fn test_knapsack_elemets_too_big() {
    let item1 = Item::new(15, 10);
    let item2 = Item::new(33, 7);
    let item3 = Item::new(3666, 5);
    
    let knapsack = Knapsack::new(10, vec![item1, item2, item3]);

    for solver in AlgorithmsService::get_all_algorithms() {
        assert_eq!(solver.solve(&knapsack), Ok(0));
    }
}

#[test]
// We check that the algorithm will work correctly when no item can be taken.
fn test_knapsack_empty() {
    let knapsack = Knapsack::new(10, vec![]);

    for solver in AlgorithmsService::get_all_algorithms() {
        assert_eq!(solver.solve(&knapsack), Ok(0));
    }
}

#[test]
// We check that the algorithm focuses on the price of the items, not the quantity.
fn test_knapsack_algo_based_on_value() {
    let item1 = Item::new(1, 2);
    let item2 = Item::new(5, 15);
    let item3 = Item::new(2, 4);
    let item4 = Item::new(5, 15);
    let item5 = Item::new(3, 8);

    let knapsack = Knapsack::new(10, vec![item1, item2, item3, item4, item5]);

    for solver in AlgorithmsService::get_all_algorithms() {
        assert_eq!(solver.solve(&knapsack), Ok(30));
    }
}