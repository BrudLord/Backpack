use crate::algorithms_impls::full_iteration_with_recursion::knapsack_solve;
use crate::models::knapsack::Knapsack;

pub struct AlgorithmsService;

impl AlgorithmsService {
    pub fn new() -> Self {
        Self
    }

    pub fn solve(&self, knapsack: &mut Knapsack) -> u32 {
        let mut best_value: u32 = 0;
        knapsack_solve(knapsack, 0, &mut best_value);
        return best_value;
    }
}
