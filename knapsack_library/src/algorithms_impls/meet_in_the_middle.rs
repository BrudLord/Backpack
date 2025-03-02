use crate::models::knapsack::Knapsack;
use crate::models::knapsack_solver::KnapsackSolver;
use std::collections::BTreeSet;

/// Meet-in-the-Middle implementation of the Knapsack solver
///
/// This solver splits the items into two halves, generates all possible subsets for each half,
/// and then combines the results using binary search to find the optimal solution.
/// Time complexity: O(2^(n/2) * n)
pub struct MeetInTheMiddleKnapsackSolver;

impl KnapsackSolver for MeetInTheMiddleKnapsackSolver {
    fn get_name(&self) -> String {
        "Meet in the Middle".to_string()
    }

    fn solve(&self, knapsack: &Knapsack) -> Result<u64, String> {
        let n = knapsack.get_items_len();
        let capacity = knapsack.get_capacity();

        if n == 0 || capacity == 0 {
            return Ok(0);
        }

        // Split items into two halves
        let mid = n / 2;
        let (first_half, second_half) = knapsack.items.split_at(mid);

        // Generate all subsets for the first half
        let first_subsets = generate_subsets(first_half);

        // Generate all subsets for the second half and sort by weight
        let mut second_subsets = generate_subsets(second_half);
        second_subsets.sort_by_key(|&(weight, _)| weight);

        // Find the maximum value using binary search
        let mut max_value = 0;
        for &(weight1, value1) in &first_subsets {
            if weight1 > capacity {
                continue;
            }
            let remaining_capacity = capacity - weight1;

            // Binary search for the best subset in the second half
            let (_, value2) = match second_subsets.binary_search_by_key(&remaining_capacity, |&(w, _)| w) {
                Ok(idx) => second_subsets[idx],
                Err(idx) => if idx > 0 { second_subsets[idx - 1] } else { (0, 0) },
            };

            max_value = max_value.max(value1 + value2);
        }

        Ok(max_value)
    }
}

/// Generates all possible subsets of items with their total weight and value
fn generate_subsets(items: &[Item]) -> Vec<(u64, u64)> {
    let mut subsets = vec![(0, 0)];
    for item in items {
        let mut new_subsets = Vec::new();
        for &(weight, value) in &subsets {
            let new_weight = weight + item.get_weight();
            let new_value = value + item.get_value();
            new_subsets.push((new_weight, new_value));
        }
        subsets.extend(new_subsets);
    }
    subsets
}