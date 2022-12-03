use aoc::{one::largest_amount, two::rock_paper_scissors};
fn main() {
    let amount = largest_amount("src/input/d_1/input.txt".to_string());
    println!("Day one A.O.C, largest = {}", amount);
    let sum = rock_paper_scissors("src/input/d_2/input.txt".to_owned());
    println!("Day two A.O.C, total score = {}", sum);
}
