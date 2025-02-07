use crate::algorithms_impls::knapsack_solver::KnapsackSolver;
use crate::models::knapsack::Knapsack;

pub struct RecursiveKnapsackSolver;

// Алгоритм решения задачи о рюкзаке через рекурсивный перебор
impl KnapsackSolver for RecursiveKnapsackSolver {
    fn get_name(&self) -> String {
        "Recursion".to_string()
    }

    fn solve(&self, knapsack: &Knapsack) -> u32 {
        let mut best_value = 0;

        fn recursive(
            knapsack: &Knapsack,
            index: usize,
            current_weight: u32,
            current_value: u32,
            best_value: &mut u32,
        ) {
            if index > knapsack.get_items_len() {
                return;
            }

            if index == knapsack.get_items_len() {
                if current_value > *best_value {
                    *best_value = current_value;
                }
                return;
            }

            // Не стали брать предмет на позиции index
            recursive(
                knapsack,
                index + 1,
                current_weight,
                current_value,
                best_value,
            );

            // Взяли предмет на позиции index, если тот влез в рюкзак
            if current_weight + knapsack.get_item(index).get_weight() <= knapsack.get_capacity() {
                recursive(
                    knapsack,
                    index + 1,
                    current_weight + knapsack.get_item(index).get_weight(),
                    current_value + knapsack.get_item(index).get_value(),
                    best_value,
                );
            }
        }

        recursive(knapsack, 0, 0, 0, &mut best_value);
        best_value
    }
}
