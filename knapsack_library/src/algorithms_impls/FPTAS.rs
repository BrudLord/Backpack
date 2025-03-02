use crate::models::knapsack::Knapsack;
use crate::models::knapsack_solver::KnapsackSolver;

/// FPTAS implementation of the Knapsack solver
///
/// This solver scales down the values of items to reduce precision and applies dynamic programming.
/// The approximation factor is controlled by ε.
/// Time complexity: O(nW/ε)
pub struct FptasKnapsackSolver {
    epsilon: f64,
}

impl FptasKnapsackSolver {
    /// Creates a new FPTAS solver with the given ε value
    ///
    /// # Arguments
    ///
    /// * `epsilon` - The approximation factor (0 < ε < 1)
    pub fn new(epsilon: f64) -> Self {
        assert!(epsilon > 0.0 && epsilon < 1.0, "Epsilon must be between 0 and 1");
        Self { epsilon }
    }
}

impl KnapsackSolver for FptasKnapsackSolver {
    fn get_name(&self) -> String {
        format!("FPTAS (ε = {:.3})", self.epsilon)
    }

    fn solve(&self, knapsack: &Knapsack) -> Result<u64, String> {
        let n = knapsack.get_items_len();
        let capacity = knapsack.get_capacity();

        if n == 0 || capacity == 0 {
            return Ok(0);
        }

        // Find the maximum value among all items
        let max_value = knapsack
            .items
            .iter()
            .map(|item| item.get_value())
            .max()
            .unwrap_or(0);

        // Scale down the values
        let k = (self.epsilon * max_value as f64).ceil() as u64;
        if k == 0 {
            return Ok(0);
        }

        let scaled_items: Vec<Item> = knapsack
            .items
            .iter()
            .map(|item| Item::new(item.get_weight(), item.get_value() / k))
            .collect();

        let mut dp = vec![0; (capacity + 1) as usize];

        // Dynamic programming
        for item in scaled_items {
            for w in (item.get_weight()..=capacity).rev() {
                dp[w as usize] = dp[w as usize].max(dp[(w - item.get_weight()) as usize] + item.get_value());
            }
        }

        // Scale up the result
        Ok(dp[capacity as usize] * k)
    }
}