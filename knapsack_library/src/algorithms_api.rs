use crate::algorithms_service::AlgorithmsService;
use crate::models::knapsack::Knapsack;

pub struct AlgorithmsAPI {
    service: AlgorithmsService,
}

impl AlgorithmsAPI {
    pub fn new() -> Self {
        Self {
            service: AlgorithmsService::new(),
        }
    }

    pub fn solve_knapsack(&self, knapsack: &mut Knapsack) -> u32 {
        self.service.solve(knapsack)
    }
}
