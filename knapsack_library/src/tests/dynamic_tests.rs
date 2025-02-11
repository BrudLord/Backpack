#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms_impls::dynamic::DynamicKnapsackSolver;
    use crate::models::item::Item;
    use crate::models::knapsack::Knapsack;
    use crate::models::knapsack_solver::KnapsackSolver;

    #[test]
    #[should_panic(expected = "Capacity too large to process")]
    fn test_panic_on_large_capacity() {
        let solver = DynamicKnapsackSolver;
        let items = vec![Item::new(1, 1)];
        let knapsack = Knapsack::new(u64::MAX, items);

        solver.solve(&knapsack);
    }


}
