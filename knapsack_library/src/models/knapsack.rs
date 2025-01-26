use crate::models::item::Item;



#[derive(Debug)]
pub struct Knapsack {
    capacity: u32,
    items: Vec<Item>,
}

impl Knapsack {
    pub fn new(capacity: u32, items: Vec<Item>) -> Self {
        Self {
            capacity,
            items,
        }
    }

    pub fn get_items_len(&self) -> usize {
        self.items.len()
    }

    pub fn get_item(&self, index: usize) -> &Item {
        &self.items[index]
    }

    pub fn get_capacity(&self) -> u32 {
        self.capacity
    }
}