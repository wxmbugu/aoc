use std::{fs::File, io::Read};
pub fn readfile(file: String) {
    let mut f = File::open(file).expect("file");
    let mut s = String::new();
    f.read_to_string(&mut s);
    let newstr = s.replace("\n\n", "\ngroup\n");
    let binding = newstr.replace('\n', ",");
    let stars: Vec<&str> = binding.split(",group,").collect();
    let mut largest = 0;
    for star in stars {
        let dataset: Vec<i32> = star
            .trim()
            .split(',')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse().unwrap())
            .collect();
        let ok: i32 = dataset.iter().sum();
        if ok > largest {
            largest = ok;
        }
    }
    println!("largest = {}", largest)
}
