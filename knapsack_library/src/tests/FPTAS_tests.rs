#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::item::Item;
    use crate::models::knapsack::Knapsack;
    use crate::models::knapsack_solver::KnapsackSolver;

    #[test]
    fn test_fptas_basic_case() {
        let solver = FptasKnapsackSolver::new(0.1);
        let items = vec![
            Item::new(2, 3),
            Item::new(3, 4),
            Item::new(4, 5),
            Item::new(5, 6),
        ];
        let knapsack = Knapsack::new(5, items);

        let result = solver.solve(&knapsack).unwrap();
        assert!(result >= 6 && result <= 7); // Approximation within ε
    }

    #[test]
    fn test_fptas_zero_capacity() {
        let solver = FptasKnapsackSolver::new(0.1);
        let items = vec![Item::new(1, 1), Item::new(2, 2)];
        let knapsack = Knapsack::new(0, items);

        assert_eq!(solver.solve(&knapsack), Ok(0));
    }

    #[test]
    fn test_fptas_no_items() {
        let solver = FptasKnapsackSolver::new(0.1);
        let items = vec![];
        let knapsack = Knapsack::new(10, items);

        assert_eq!(solver.solve(&knapsack), Ok(0));
    }

    #[test]
    fn test_fptas_small_epsilon() {
        let solver = FptasKnapsackSolver::new(0.01);
        let items = vec![
            Item::new(2, 3),
            Item::new(3, 4),
            Item::new(4, 5),
            Item::new(5, 6),
        ];
        let knapsack = Knapsack::new(5, items);

        let result = solver.solve(&knapsack).unwrap();
        assert!(result >= 7 && result <= 7); // More precise approximation
    }
}