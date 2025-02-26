use crate::models::knapsack::Knapsack;
use crate::models::knapsack_solver::KnapsackSolver;
use std::collections::HashMap;

/// Lazy Dynamic Programming implementation of the Knapsack solver.
///
/// This solver uses a memoized recursive approach to solve the 0/1 knapsack problem.
/// Time complexity: O(nW) in the worst case, but can be faster in practice.
/// Space complexity: O(nW) due to memoization.
pub struct LazyDynamicKnapsackSolver;

impl KnapsackSolver for LazyDynamicKnapsackSolver {
    fn get_name(&self) -> String {
        "Lazy Dynamic".to_string()
    }

    fn solve(&self, knapsack: &Knapsack) -> Result<u64, String> {
        let n = knapsack.get_items_len();
        let capacity = knapsack.get_capacity();

        if capacity >= usize::MAX as u64 {
            return Err("Capacity too large to process".to_string());
        }
        let capacity = capacity as usize;

        // No items or capacity == 0 => result is 0
        if n == 0 || capacity == 0 {
            return Ok(0);
        }

        // HashMap for storing previously calculated values
        let mut memo = HashMap::new();

        // Call recursive implementation and return
        Ok(Self::knapsack_recursive(n, capacity, knapsack, &mut memo))
    }
}

impl LazyDynamicKnapsackSolver {
    /// Recursive function to solve the knapsack problem with memoization.
    ///
    /// # Arguments
    /// * `i` - Current item index (1-based).
    /// * `w` - Remaining capacity.
    /// * `knapsack` - Reference to the Knapsack instance.
    /// * `memo` - Mutable reference to a memoization cache.
    ///
    /// # Returns
    /// * Maximum value that can be achieved with the given parameters.
    fn knapsack_recursive(
        i: usize,
        w: usize,
        knapsack: &Knapsack,
        memo: &mut HashMap<(usize, usize), u64>,
    ) -> u64 {
        // If no items or capacity =>
        if i == 0 || w == 0 {
            return 0;
        }

        // Check if value already exists
        // If exists - return it
        if let Some(&cached) = memo.get(&(i, w)) {
            return cached;
        }

        // Zero-based index
        let item = knapsack.get_item(i - 1);

        // Recursive call
        let mut result = Self::knapsack_recursive(i - 1, w, knapsack, memo);

        // Try to put item in current result
        if item.get_weight() as usize <= w {
            let value_with_item =
                Self::knapsack_recursive(i - 1, w - item.get_weight() as usize, knapsack, memo)
                    + item.get_value();
            result = result.max(value_with_item);
        }

        // Store the value and return it
        memo.insert((i, w), result);
        result
    }
}
