use crate::algorithms_impls::full_iteration_with_bit_mask::BitMaskKnapsackSolver;
use crate::algorithms_impls::full_iteration_with_recursion::RecursiveKnapsackSolver;
use crate::models::knapsack_solver::KnapsackSolver;
use crate::models::knapsack::Knapsack;

/// A service that manages knapsack solving algorithms.
///
/// This service provides methods to retrieve all available knapsack algorithms,
/// solve the knapsack problem using a specific algorithm, and get the names of all algorithms.
pub struct AlgorithmsService;

impl AlgorithmsService {
    /// Returns a list of all available knapsack-solving algorithms.
    ///
    /// This method currently returns a vector with one algorithm: `RecursiveKnapsackSolver`.
    ///
    /// # Returns
    ///
    /// A `Vec<Box<dyn KnapsackSolver>>` containing boxed instances of all available algorithms.
    pub fn get_all_algorithms() -> Vec<Box<dyn KnapsackSolver>> {
        vec![
            Box::new(RecursiveKnapsackSolver),
            Box::new(BitMaskKnapsackSolver),
        ]
    }

    /// Solves the knapsack problem using the specified algorithm.
    ///
    /// This method searches for an algorithm by its name and uses it to solve the given knapsack problem.
    /// If no algorithm with the given name is found, it returns `None`.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the algorithm to use for solving the knapsack problem.
    /// * `knapsack` - A reference to the `Knapsack` instance to solve.
    ///
    /// # Returns
    ///
    /// An `Option<u64>` where:
    /// - `Some(value)` contains the maximum value that can be achieved for the knapsack.
    /// - `None` is returned if the algorithm with the given name is not found.
    pub fn solve(name: String, knapsack: &Knapsack) -> Option<u64> {
        for algorithm in AlgorithmsService::get_all_algorithms() {
            if algorithm.get_name() == name {
                return Some(algorithm.solve(knapsack));
            }
        }
        None
    }

    /// Returns the names of all available algorithms.
    ///
    /// This method retrieves a list of names for all the knapsack algorithms
    /// currently supported by the service.
    ///
    /// # Returns
    ///
    /// A `Vec<String>` containing the names of all available algorithms.
    pub fn get_algorithms_names() -> Vec<String> {
        AlgorithmsService::get_all_algorithms()
            .into_iter()
            .map(|algorithm| algorithm.get_name())
            .collect()
    }
}
