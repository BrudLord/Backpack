use crate::models::item::Item;
use serde::{Serialize, Deserialize};

/// A structure representing a knapsack for the algorithms.
///
/// This structure contains the capacity of the knapsack and a list of items that can be placed in the knapsack.
#[derive(Clone, Debug)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq)]
pub struct Knapsack {
    capacity: u64,          // The capacity of the knapsack (maximum weight it can hold)
    items: Vec<Item>,      // A list of items that can be placed in the knapsack
}

impl Knapsack {
    /// Creates a new `Knapsack` with the given capacity and list of items.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The capacity of the knapsack, i.e., the maximum weight it can carry.
    /// * `items` - A `Vec<Item>` representing the items that can be placed in the knapsack.
    ///
    /// # Returns
    ///
    /// A new `Knapsack` instance with the specified capacity and items.
    pub fn new(capacity: u64, items: Vec<Item>) -> Self {
        Self {
            capacity,
            items,
        }
    }

    /// Gets the number of items in the knapsack.
    ///
    /// # Returns
    ///
    /// The number of items currently in the knapsack as a `usize`.
    pub fn get_items_len(&self) -> usize {
        self.items.len()
    }

    /// Gets a reference to the item at the specified index in the knapsack.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the item to retrieve.
    ///
    /// # Returns
    ///
    /// A reference to the `Item` at the specified index.
    ///
    /// # Panics
    ///
    /// This method will panic if the `index` is out of bounds.
    pub fn get_item(&self, index: usize) -> &Item {
        &self.items[index]
    }

    /// Gets the capacity of the knapsack.
    ///
    /// # Returns
    ///
    /// The capacity of the knapsack as a `u64`.
    pub fn get_capacity(&self) -> u64 {
        self.capacity
    }
}
