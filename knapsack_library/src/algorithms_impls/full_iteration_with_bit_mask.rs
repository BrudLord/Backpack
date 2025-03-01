use crate::models::knapsack_solver::KnapsackSolver;
use crate::models::knapsack::Knapsack;

pub struct BitMaskKnapsackSolver;

/// A solver for the knapsack problem using bit masks.
impl KnapsackSolver for BitMaskKnapsackSolver {
    /// Returns the name of the algorithm.
    ///
    /// # Returns
    ///
    /// A string representing the algorithm name: "Bit mask".
    fn get_name(&self) -> String {
        "Bit mask".to_string()
    }

    /// Solves the knapsack problem using bit masks.
    ///
    /// This method attempts all possible combinations of items to find the maximum value
    /// that can be carried within the given weight limit of the knapsack.
    ///
    /// # Arguments
    ///
    /// * `knapsack` - A reference to the `Knapsack` object containing the items and capacity.
    ///
    /// # Returns
    ///
    /// A `u64` value representing the maximum value that can be achieved by fitting items
    /// in the knapsack without exceeding the capacity.
    fn solve(&self, knapsack: &Knapsack) -> Result<u64, String> {
        let item_count = knapsack.get_items_len();
        let mut best_value = 0;

        if knapsack.get_items_len() > 64 {
            return Err("The number of items exceeds the maximum allowed (64).".to_string());
        }

        // Iterate over all possible combinations of items represented by bit masks
        for mask in 0..(1u64 << item_count) {
            let mut current_weight = 0;
            let mut current_value = 0;

            // Iterate over each bit in the mask to determine whether to include the corresponding item
            for i in 0..item_count {
                if (mask & (1 << i)) != 0 {
                    let item = knapsack.get_item(i);
                    current_weight += item.get_weight();
                    current_value += item.get_value();

                    // If the current weight exceeds the capacity, skip this combination
                    if current_weight > knapsack.get_capacity() {
                        break;
                    }
                }
            }

            // Update the best value if the current combination is valid and better
            if current_weight <= knapsack.get_capacity() {
                best_value = best_value.max(current_value);
            }
        }

        Ok(best_value)
    }
}
