use crate::models::knapsack_solver::KnapsackSolver;
use crate::models::knapsack::Knapsack;

pub struct GreedyKnapsackSolver;

/// A solver for the knapsack problem using a greedy heuristic approach.
impl KnapsackSolver for GreedyKnapsackSolver {
    /// Returns the name of the algorithm.
    ///
    /// # Returns
    ///
    /// A string representing the algorithm name: "Greedy".
    fn get_name(&self) -> String {
        "Greedy".to_string()
    }

    /// Solves the knapsack problem using a greedy heuristic.
    ///
    /// This method calculates the value-to-weight ratio for each item, sorts the items
    /// in descending order based on that ratio, and then iteratively adds items to the
    /// knapsack as long as the capacity is not exceeded.
    ///
    /// # Arguments
    ///
    /// * `knapsack` - A reference to the `Knapsack` object containing the items and capacity.
    ///
    /// # Returns
    ///
    /// A `u64` value representing the total value of the selected items.
    fn solve(&self, knapsack: &Knapsack) -> Result<u64, String> {
        // Create a vector of item indices.
        let mut indices: Vec<usize> = (0..knapsack.get_items_len()).collect();

        // Sort indices based on the value-to-weight ratio (descending order).
        indices.sort_by(|&i, &j| {
            let item_i = knapsack.get_item(i);
            let item_j = knapsack.get_item(j);
            let ratio_i = item_i.get_value() as f64 / item_i.get_weight() as f64;
            let ratio_j = item_j.get_value() as f64 / item_j.get_weight() as f64;
            // Compare ratios in reverse to get descending order.
            ratio_j.partial_cmp(&ratio_i).unwrap()
        });

        let mut current_weight = 0;
        let mut total_value = 0;

        // Greedily add items while the capacity is not exceeded.
        for i in indices {
            let item = knapsack.get_item(i);
            if current_weight + item.get_weight() <= knapsack.get_capacity() {
                current_weight += item.get_weight();
                total_value += item.get_value();
            }
        }

        Ok(total_value)
    }
}
