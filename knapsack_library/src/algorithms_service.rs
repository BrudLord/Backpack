use crate::algorithms_impls::full_iteration_with_recursion::RecursiveKnapsackSolver;
use crate::algorithms_impls::knapsack_solver::KnapsackSolver;
use crate::models::knapsack::Knapsack;

pub struct AlgorithmsService;

impl AlgorithmsService {
    pub fn new() -> Self {
        Self
    }

    pub fn get_all_algorithms(&self) -> Vec<Box<dyn KnapsackSolver>> {
        vec![
            Box::new(RecursiveKnapsackSolver),
        ]
    }

    pub fn solve(&self, knapsack: &mut Knapsack, name: String) -> Option<u32> {
        for algorithm in self.get_all_algorithms() {
            if algorithm.get_name() == name {
                return Some(algorithm.solve(knapsack));
            }
        }
        None
    }
    

    pub fn get_algorithms_names() -> Vec<String> {
        vec!["Full iteration with recursion".to_string()]
    }
}
