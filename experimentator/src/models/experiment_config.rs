use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ExperimentConfig {
    /// The number of items to generate.
    pub num_items: usize,
    /// The capacity of generated knapsack.
    pub capacity: u32,
    /// The range (inclusive) from which to randomly generate the weight of each item.
    /// The first element of the tuple is the minimum weight, and the second is the maximum.
    pub weights_range: (u32, u32),
    /// The range (inclusive) from which to randomly generate the cost/value of each item.
    /// The first element of the tuple is the minimum cost, and the second is the maximum.
    pub costs_range: (u32, u32),
}