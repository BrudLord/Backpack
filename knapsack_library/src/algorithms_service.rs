use crate::algorithms_impls::full_iteration_with_recursion::RecursiveKnapsackSolver;
use crate::algorithms_impls::knapsack_solver::KnapsackSolver;
use crate::models::knapsack::Knapsack;

pub struct AlgorithmsService;

impl AlgorithmsService {
    fn new() -> Self {
        Self
    }

    pub fn get_all_algorithms() -> Vec<Box<dyn KnapsackSolver>> {
        vec![
            Box::new(RecursiveKnapsackSolver),
        ]
    }

    pub fn solve(name: String, knapsack: &Knapsack) -> Option<u32> {
        for algorithm in AlgorithmsService::get_all_algorithms() {
            if algorithm.get_name() == name {
                return Some(algorithm.solve(knapsack));
            }
        }
        None
    }
    

    pub fn get_algorithms_names() -> Vec<String> {
        vec!["Recursion".to_string()]
    }
}
