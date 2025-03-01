use crate::algorithms_impls::full_iteration_with_bit_mask::BitMaskKnapsackSolver;
use crate::models::item::Item;
use crate::models::knapsack::Knapsack;
use crate::models::knapsack_solver::KnapsackSolver;

#[test]
fn test_err_on_more_than_64_items() {
    let solver = BitMaskKnapsackSolver;
    let mut items: Vec<Item> = vec!();
    for i in 0..66 {
        items.push(Item::new(i, i));
    }
    let knapsack = Knapsack::new(32, items);

    assert_eq!(solver.solve(&knapsack), Err("The number of items exceeds the maximum allowed (64).".to_string()));
}