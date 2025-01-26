use knapsack_library::algorithms_impls::full_iteration_with_recursion::knapsack_solve;
use knapsack_library::models::knapsack::Knapsack;
use knapsack_library::models::item::Item;

fn main() {
    let item1 = Item::new(5, 10);
    let item2 = Item::new(3, 7);
    let item3 = Item::new(2, 5);
    
    let mut knapsack = Knapsack::new(10, vec![item1, item2, item3]);


    let mut best_value: u32 = 0;
    knapsack_solve(&mut knapsack, 0, &mut best_value);
    println!("Maximum value in knapsack: {}", best_value);
}
