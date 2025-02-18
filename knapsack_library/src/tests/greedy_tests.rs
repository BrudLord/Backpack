use crate::algorithms_impls::greedy::GreedyKnapsackSolver;
use crate::models::item::Item;
use crate::models::knapsack::Knapsack;
use crate::models::knapsack_solver::KnapsackSolver;

#[test]
// We check that the algorithm takes the greedy option, but not the optimal one.
fn test_err_on_large_capacity() {
    let solver = GreedyKnapsackSolver;
    let items = vec![
        Item::new(3, 4), 
        Item::new(3, 4),
        Item::new(4, 7)
        ];
    let knapsack = Knapsack::new(6, items);

    assert_eq!(solver.solve(&knapsack), Ok(7));
}
