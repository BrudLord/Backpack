use crate::models::knapsack::Knapsack;
use crate::models::knapsack_solver::KnapsackSolver;

/// Dynamic Programming implementation of the Knapsack solver
///
/// This solver uses a bottom-up dynamic programming approach to solve the 0/1 knapsack problem.
/// Time complexity: O(nW) where n is the number of items and W is the capacity
/// Space complexity: O(W)
pub struct DynamicKnapsackSolver;

/// Solves the knapsack problem using dynamic programming
///
/// # Arguments
/// * `knapsack` - The knapsack instance to solve
///
/// # Returns
/// * `u64` - The maximum value that can be achieved
///
/// # Panics
/// * If the capacity is too large to process (exceeds or equals usize::MAX)
impl KnapsackSolver for DynamicKnapsackSolver {
    fn get_name(&self) -> String {
        "Dynamic".to_string()
    }

    fn solve(&self, knapsack: &Knapsack) -> Result<u64, String> {
        let n = knapsack.get_items_len();
        let capacity = knapsack.get_capacity();

        if capacity >= usize::MAX as u64 {
            return Err("Capacity too large to process".to_string());
        }

        let capacity = capacity as usize;

        // Optimize space: use only two rows instead of n×W matrix
        let mut prev = vec![0; capacity + 1];
        let mut curr = vec![0; capacity + 1];

        // Build table in bottom-up manner
        for i in 0..n {
            let item = knapsack.get_item(i);
            let weight = item.get_weight() as usize;
            let value = item.get_value();

            for w in 0..=capacity {
                curr[w] = if weight <= w {
                    prev[w].max(prev[w - weight] + value)
                } else {
                    prev[w]
                };
            }
            std::mem::swap(&mut prev, &mut curr);
        }

        Ok(prev[capacity])
    }
}
