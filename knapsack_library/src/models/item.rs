use serde::{Serialize, Deserialize};


#[derive(Clone, Debug)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq)]
// Структура описывающая предмет лежащий в рюкзаке
pub struct Item {
    weight: u32,
    value: u32,
}

impl Item {
    pub fn new(weight: u32, value: u32) -> Self {
        Self { weight, value }
    }
    
    #[allow(dead_code)]
    fn display(&self) {
        println!("Item: weight = {}, value = {}", self.weight, self.value);
    }

    pub fn get_weight(&self) -> u32 {
        self.weight
    }

    pub fn get_value(&self) -> u32 {
        self.value
    }
}