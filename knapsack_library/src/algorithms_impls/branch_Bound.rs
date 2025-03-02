use crate::models::knapsack::Knapsack;
use crate::models::knapsack_solver::KnapsackSolver;

/// Branch-and-Bound implementation of the Knapsack solver
///
/// This solver uses recursion with pruning based on an upper bound estimate.
/// Time complexity: Depends on the pruning efficiency, but typically much faster than brute force.
pub struct BranchAndBoundKnapsackSolver;

impl KnapsackSolver for BranchAndBoundKnapsackSolver {
    fn get_name(&self) -> String {
        "Branch and Bound".to_string()
    }

    fn solve(&self, knapsack: &Knapsack) -> Result<u64, String> {
        let n = knapsack.get_items_len();
        let capacity = knapsack.get_capacity();

        if n == 0 || capacity == 0 {
            return Ok(0);
        }

        // Sort items by value-to-weight ratio in descending order
        let mut sorted_items = knapsack.items.clone();
        sorted_items.sort_by(|a, b| {
            let ratio_a = a.get_value() as f64 / a.get_weight() as f64;
            let ratio_b = b.get_value() as f64 / b.get_weight() as f64;
            ratio_b.partial_cmp(&ratio_a).unwrap()
        });

        let mut best_value = 0;
        let mut current_weight = 0;
        let mut current_value = 0;

        // Recursive function to explore the tree
        fn branch_and_bound(
            items: &[Item],
            index: usize,
            capacity: u64,
            current_weight: u64,
            current_value: u64,
            best_value: &mut u64,
        ) {
            if index >= items.len() {
                return;
            }

            // Include the current item if it fits
            if current_weight + items[index].get_weight() <= capacity {
                let new_weight = current_weight + items[index].get_weight();
                let new_value = current_value + items[index].get_value();
                if new_value > *best_value {
                    *best_value = new_value;
                }
                branch_and_bound(items, index + 1, capacity, new_weight, new_value, best_value);
            }

            // Exclude the current item
            branch_and_bound(items, index + 1, capacity, current_weight, current_value, best_value);

            // Prune using upper bound
            let mut remaining_capacity = capacity - current_weight;
            let mut upper_bound = current_value;
            for i in index..items.len() {
                if remaining_capacity >= items[i].get_weight() {
                    upper_bound += items[i].get_value();
                    remaining_capacity -= items[i].get_weight();
                } else {
                    upper_bound += (remaining_capacity as f64 * items[i].get_value() as f64 / items[i].get_weight() as f64) as u64;
                    break;
                }
            }
            if upper_bound <= *best_value {
                return;
            }
        }

        branch_and_bound(&sorted_items, 0, capacity, current_weight, current_value, &mut best_value);

        Ok(best_value)
    }
}