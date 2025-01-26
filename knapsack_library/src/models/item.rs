#[derive(Debug)]
pub struct Item {
    weight: u32,
    value: u32,
}

impl Item {
    pub fn new(weight: u32, value: u32) -> Self {
        Self { weight, value }
    }

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