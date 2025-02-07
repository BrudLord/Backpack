use crate::models::knapsack::Knapsack;

// Интерфейс решения задачи о рюкзаке. Все алгоритмы должны от него наследоваться
pub trait KnapsackSolver {
    fn get_name(&self) -> String;

    fn solve(&self, knapsack: &Knapsack) -> u64;
}
