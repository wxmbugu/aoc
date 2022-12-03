use std::{fs::File, io::Read};
#[allow(unused_must_use)]
pub fn largest_amount(file: String) -> i32 {
    let mut f = File::open(file).expect("file");
    let mut s = String::new();
    f.read_to_string(&mut s);
    let stars: Vec<&str> = s.split("\n\n").collect();
    let mut largest = 0;
    for star in stars {
        let dataset: Vec<i32> = star
            .split('\n')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse().unwrap())
            .collect();
        let ok: i32 = dataset.iter().sum();
        if ok > largest {
            largest = ok;
        }
    }
        largest
}
