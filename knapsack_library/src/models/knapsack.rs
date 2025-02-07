use crate::models::item::Item;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq)]
// Структура описывающая рюкзак для алгоритмов
pub struct Knapsack {
    capacity: u64,
    items: Vec<Item>,
}

impl Knapsack {
    pub fn new(capacity: u64, items: Vec<Item>) -> Self {
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

    pub fn get_capacity(&self) -> u64 {
        self.capacity
    }
}