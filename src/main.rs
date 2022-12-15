use aoc::{one::largest_amount, two::rock_paper_scissors};
fn main() {
    // Get the elve with the largest amount of food
    let amount = largest_amount(include_str!("./input/d_1/input.txt"));
    println!("Day one A.O.C, largest = {}", amount);
    // Total scores after playing rock_paper_scissors
    let sum = rock_paper_scissors(include_str!("./input/d_2/input.txt"));
    println!("Day two A.O.C, total score = {}", sum);
}
