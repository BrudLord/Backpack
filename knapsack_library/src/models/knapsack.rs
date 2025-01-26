use crate::models::item::Item;



#[derive(Debug)]
pub struct Knapsack {
    capacity: u32,
    current_value: u32,
    items: Vec<Item>,
    taken_indexes: Vec<usize>,
}

impl Knapsack {
    pub fn new(capacity: u32, items: Vec<Item>) -> Self {
        Self {
            capacity,
            current_value: 0,
            items,
            taken_indexes: Vec::new(),
        }
    }

    pub fn display(&self) {
        println!("Knapsack:");
        println!("  Remaining capacity: {}", self.get_capacity());
        println!("  Current value: {}", self.current_value);
        println!("  Taken items:");
        for &index in &self.taken_indexes {
            let item = &self.items[index];
            println!("    - Item {}: weight = {}, value = {}", index, item.get_weight(), item.get_value());
        }
    }

    pub fn can_add_item(&self, index: usize) -> bool {
        self.items[index].get_weight() <= self.get_capacity()
    }

    pub fn add_item(&mut self, index: usize) {
        if index < self.items.len() && self.items[index].get_weight() <= self.get_capacity() {
            self.capacity -= self.items[index].get_weight();
            self.current_value += self.items[index].get_value();
            self.taken_indexes.push(index);
        } else {
            println!("Expected true: index < self.items.len() (actual {} < {}) and self.items[index].weight <= self.capacity (actual {} <= {})", index, self.items.len(), self.items[index].get_weight(), self.get_capacity());

        }
    }

    fn remove_item(&mut self, index: usize) {
        if let Some(pos) = self.taken_indexes.iter().position(|&i| i == index) {
            self.capacity += self.items[index].get_weight();
            self.current_value -= self.items[index].get_value();
            self.taken_indexes.remove(pos);
        }
    }

    pub fn remove_last_item(&mut self) {
        if let Some(index) = self.taken_indexes.pop() {
            self.capacity += self.items[index].get_weight();
            self.current_value -= self.items[index].get_value();
        }
    }

    pub fn get_items_len(&self) -> usize {
        self.items.len()
    }

    pub fn get_capacity(&self) -> u32 {
        self.capacity
    }

    pub fn get_current_value(&self) -> u32 {
        self.current_value
    }
}