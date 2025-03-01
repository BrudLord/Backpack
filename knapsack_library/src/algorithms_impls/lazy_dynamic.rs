use crate::models::knapsack::Knapsack;
use crate::models::knapsack_solver::KnapsackSolver;
use std::collections::HashMap;

/// Lazy Dynamic Programming implementation of the Knapsack solver
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
        let mut memo = HashMap::new();
        
        fn knapsack_recursive(
            i: usize, w: usize, knapsack: &Knapsack, memo: &mut HashMap<(usize, usize), u64>
        ) -> u64 {
            if i == 0 || w == 0 {
                return 0;
            }
            if let Some(&cached) = memo.get(&(i, w)) {
                return cached;
            }
            
            let item = knapsack.get_item(i - 1);
            let mut result = knapsack_recursive(i - 1, w, knapsack, memo);
            if item.get_weight() as usize <= w {
                result = result.max(
                    knapsack_recursive(i - 1, w - item.get_weight() as usize, knapsack, memo)
                        + item.get_value()
                );
            }
            memo.insert((i, w), result);
            result
        }
        
        Ok(knapsack_recursive(n, capacity, knapsack, &mut memo))
    }
}
