#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::item::Item;
    use crate::models::knapsack::Knapsack;
    use crate::models::knapsack_solver::KnapsackSolver;

    #[test]
    fn test_branch_and_bound_basic_case() {
        let solver = BranchAndBoundKnapsackSolver;
        let items = vec![
            Item::new(2, 3),
            Item::new(3, 4),
            Item::new(4, 5),
            Item::new(5, 6),
        ];
        let knapsack = Knapsack::new(5, items);

        assert_eq!(solver.solve(&knapsack), Ok(7));
    }

    #[test]
    fn test_branch_and_bound_zero_capacity() {
        let solver = BranchAndBoundKnapsackSolver;
        let items = vec![Item::new(1, 1), Item::new(2, 2)];
        let knapsack = Knapsack::new(0, items);

        assert_eq!(solver.solve(&knapsack), Ok(0));
    }

    #[test]
    fn test_branch_and_bound_no_items() {
        let solver = BranchAndBoundKnapsackSolver;
        let items = vec![];
        let knapsack = Knapsack::new(10, items);

        assert_eq!(solver.solve(&knapsack), Ok(0));
    }

    #[test]
    fn test_branch_and_bound_pruning_effectiveness() {
        let solver = BranchAndBoundKnapsackSolver;
        let items = vec![
            Item::new(10, 60),
            Item::new(20, 100),
            Item::new(30, 120),
        ];
        let knapsack = Knapsack::new(50, items);

        assert_eq!(solver.solve(&knapsack), Ok(220));
    }
}