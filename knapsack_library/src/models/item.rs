use serde::{Serialize, Deserialize};


#[derive(Clone, Debug)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq)]
// Структура описывающая предмет лежащий в рюкзаке
pub struct Item {
    weight: u64,
    value: u64,
}

impl Item {
    pub fn new(weight: u64, value: u64) -> Self {
        Self { weight, value }
    }
    pub fn get_weight(&self) -> u64 {
        self.weight
    }
    pub fn get_value(&self) -> u64 {
        self.value
    }
}