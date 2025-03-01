#[cfg(test)]
mod tests {
    use crate::algorithms_impls::dynamic::DynamicKnapsackSolver;
    use crate::models::item::Item;
    use crate::models::knapsack::Knapsack;
    use crate::models::knapsack_solver::KnapsackSolver;

    #[test]
    fn test_err_on_large_capacity() {
        let solver = DynamicKnapsackSolver;
        let items = vec![Item::new(1, 1)];
        let knapsack = Knapsack::new(u64::MAX, items);

        assert_eq!(solver.solve(&knapsack), Err("Capacity too large to process".to_string()));
    }


}
