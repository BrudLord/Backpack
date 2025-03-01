use serde::{Serialize, Deserialize};

/// A structure representing an item in the knapsack.
///
/// This structure contains the weight and value of an item that can be placed in the knapsack.
#[derive(Clone, Debug)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq)]
pub struct Item {
    weight: u64,  // The weight of the item
    value: u64,   // The value of the item
}

impl Item {
    /// Creates a new `Item` with the given weight and value.
    ///
    /// # Arguments
    ///
    /// * `weight` - The weight of the item.
    /// * `value` - The value of the item.
    ///
    /// # Returns
    ///
    /// A new `Item` instance with the specified weight and value.
    pub fn new(weight: u64, value: u64) -> Self {
        Self { weight, value }
    }

    /// Gets the weight of the item.
    ///
    /// # Returns
    ///
    /// The weight of the item as a `u64`.
    pub fn get_weight(&self) -> u64 {
        self.weight
    }

    /// Gets the value of the item.
    ///
    /// # Returns
    ///
    /// The value of the item as a `u64`.
    pub fn get_value(&self) -> u64 {
        self.value
    }
}
