#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::item::Item;
    use crate::models::knapsack::Knapsack;
    use crate::models::knapsack_solver::KnapsackSolver;

    #[test]
    fn test_meet_in_middle_basic_case() {
        let solver = MeetInTheMiddleKnapsackSolver;
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
    fn test_meet_in_middle_zero_capacity() {
        let solver = MeetInTheMiddleKnapsackSolver;
        let items = vec![Item::new(1, 1), Item::new(2, 2)];
        let knapsack = Knapsack::new(0, items);

        assert_eq!(solver.solve(&knapsack), Ok(0));
    }

    #[test]
    fn test_meet_in_middle_no_items() {
        let solver = MeetInTheMiddleKnapsackSolver;
        let items = vec![];
        let knapsack = Knapsack::new(10, items);

        assert_eq!(solver.solve(&knapsack), Ok(0));
    }
}