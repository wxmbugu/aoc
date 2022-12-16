use aoc::{
    one::largest_amount,
    three::{rucksack, rucksack_p2},
    two::rock_paper_scissors,
};
fn main() {
    // Get the elve with the largest amount of food
    let amount = largest_amount(include_str!("./input/d_1/input.txt"));
    println!("Day one A.O.C, largest = {}", amount);
    // Total scores after playing rock_paper_scissors
    let sum = rock_paper_scissors(include_str!("./input/d_2/input.txt"));
    println!("Day two A.O.C, total score = {}", sum);
    let output = rucksack(include_str!("./input/d_3/input.txt"));
    println!("Day three sum of rucksack is = {}", output);
    println!(
        "part two  of day three rucksack = {}",
        rucksack_p2(include_str!("./input/d_3/input.txt"))
    );
}
