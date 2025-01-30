use crate::models::knapsack::Knapsack;

pub trait KnapsackSolver {
    fn get_name(&self) -> String;

    fn solve(&self, knapsack: &Knapsack) -> u32;
}
