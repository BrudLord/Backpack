use crate::models::knapsack_solver::KnapsackSolver;
use crate::models::knapsack::Knapsack;

pub struct RecursiveKnapsackSolver;

/// A solver for the knapsack problem using a recursive brute-force approach.
impl KnapsackSolver for RecursiveKnapsackSolver {
    /// Returns the name of the algorithm.
    ///
    /// # Returns
    ///
    /// A string representing the algorithm name: "Recursion".
    fn get_name(&self) -> String {
        "Recursion".to_string()
    }

    /// Solves the knapsack problem using recursion.
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
    fn solve(&self, knapsack: &Knapsack) -> u64 {
        let mut best_value = 0; // Keeps track of the best (maximum) value found

        /// A recursive helper function to explore all possible item combinations.
        ///
        /// # Arguments
        ///
        /// * `knapsack` - A reference to the `Knapsack` object.
        /// * `index` - The index of the current item being considered.
        /// * `current_weight` - The total weight of the selected items so far.
        /// * `current_value` - The total value of the selected items so far.
        /// * `best_value` - A mutable reference to store the best value found.
        fn recursive(
            knapsack: &Knapsack,
            index: usize,
            current_weight: u64,
            current_value: u64,
            best_value: &mut u64,
        ) {
            // Base case: if we've gone through all the items, stop recursion
            if index > knapsack.get_items_len() {
                return;
            }

            // If we've considered all items, check if the current value is the best so far
            if index == knapsack.get_items_len() {
                if current_value > *best_value {
                    *best_value = current_value; // Update the best value
                }
                return;
            }

            // Case 1: Don't take the item at the current index
            recursive(
                knapsack,
                index + 1,
                current_weight,
                current_value,
                best_value,
            );

            // Case 2: Take the item at the current index (if it fits in the knapsack)
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

        // Start the recursion with initial values (starting from the first item)
        recursive(knapsack, 0, 0, 0, &mut best_value);
        best_value
    }
}
