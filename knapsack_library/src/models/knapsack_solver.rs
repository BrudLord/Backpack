use crate::models::knapsack::Knapsack;

/// A trait for solving the knapsack problem.
///
/// All knapsack-solving algorithms should implement this trait. It provides methods
/// to get the name of the algorithm and to solve the knapsack problem for a given
/// `Knapsack` instance.
pub trait KnapsackSolver {
    /// Returns the name of the algorithm.
    ///
    /// # Returns
    ///
    /// A string representing the name of the algorithm (e.g., "Recursion", "Dynamic Programming").
    fn get_name(&self) -> String;

    /// Solves the knapsack problem for the given `Knapsack`.
    ///
    /// This method is responsible for finding the optimal combination of items
    /// that maximizes the value without exceeding the capacity of the knapsack.
    ///
    /// # Arguments
    ///
    /// * `knapsack` - A reference to the `Knapsack` instance to solve the problem for.
    ///
    /// # Returns
    ///
    /// The maximum value that can be achieved by selecting items from the knapsack.
    fn solve(&self, knapsack: &Knapsack) -> u64;
}
