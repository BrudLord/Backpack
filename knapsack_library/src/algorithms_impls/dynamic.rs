use crate::models::knapsack::Knapsack;
use crate::models::knapsack_solver::KnapsackSolver;

/// Dynamic Programming implementation of the Knapsack solver
///
/// This solver uses a bottom-up dynamic programming approach to solve the 0/1 knapsack problem.
/// Time complexity: O(nW) where n is the number of items and W is the capacity
/// Space complexity: O(nW)
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
        return "Dynamic".to_string();
    }

    fn solve(&self, knapsack: &Knapsack) -> Result<u64, String> {
        let n = knapsack.get_items_len();
        let capacity = knapsack.get_capacity();
        if capacity >= usize::MAX as u64 {
            return Err("Capacity too large to process".to_string());
        }
        let capacity = capacity as usize;

        let mut dp = vec![vec![0; capacity + 1]; n + 1];

        // Build table dp[][] in bottom-up manner
        for i in 1..=n {
            for w in 0..=capacity {
                let item = knapsack.get_item(i - 1);
                if item.get_weight() as usize <= w {
                    dp[i][w] = dp[i - 1][w]
                        .max(dp[i - 1][w - item.get_weight() as usize] + item.get_value());
                } else {
                    dp[i][w] = dp[i - 1][w];
                }
            }
        }

        Ok(dp[n][capacity])
    }
}
