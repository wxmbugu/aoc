pub fn largest_amount(data: &str) -> u32 {
    let mut stars: Vec<u32> = data
        .split("\n\n")
        .map(|x| x.lines().flat_map(|y| y.parse::<u32>()).sum())
        .collect();

    stars.sort_unstable_by(|a, b| b.cmp(a));
    // let sum: Option<&u32> = stars.iter().take(1).last();
    // sum.unwrap().to_owned()
    let sum: u32 = stars.iter().take(1).sum();
    sum
}
