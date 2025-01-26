use crate::models::knapsack::Knapsack;

pub fn knapsack_solve(knapsack: &mut Knapsack, index: usize, best_value: &mut u32) {
    if index == knapsack.get_items_len() {
        if knapsack.get_current_value() > *best_value {
            *best_value = knapsack.get_current_value();
        }
        return;
    }

    knapsack_solve(knapsack, index + 1, best_value);

    if knapsack.can_add_item(index) {
        knapsack.add_item(index);
        knapsack_solve(knapsack, index + 1, best_value);
        knapsack.remove_last_item();
    }
}