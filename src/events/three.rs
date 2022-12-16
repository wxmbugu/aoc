pub fn rucksack(data: &str) -> i32 {
    let datav: Vec<&str> = data.split('\n').filter(|x| !x.is_empty()).collect();
    let mut sum = 0;
    for ruck in datav {
        let (sack1, sack2): (&str, &str) = ruck.split_at(ruck.len() / 2);
        let occur = occurence_1(sack1, sack2);
        let value = priority(occur);
        sum += value;
    }
    sum
}

fn priority(data: char) -> i32 {
    match data {
        'a'..='z' => return data as i32 - 'a' as i32 + 1,
        'A'..='Z' => return data as i32 - 'A' as i32 + 27,
        _ => eprint!("Not applicable"),
    }
    0
}

pub fn rucksack_p2(data: &str) -> i32 {
    let datav: Vec<&str> = data.split('\n').filter(|x| !x.is_empty()).collect();
    let v_slices: Vec<&[&str]> = datav.chunks(3).collect();
    let mut sum = 0;
    for slice in v_slices {
        let result = occurence_2(slice[0], slice[1], slice[2]);
        sum += priority(result);
    }
    sum
}

fn occurence_1(data1: &str, data2: &str) -> char {
    let mut result: Vec<char> = data1.chars().filter(|c| data2.contains(*c)).collect();
    result.pop().unwrap_or(' ')
}

fn occurence_2(data1: &str, data2: &str, data3: &str) -> char {
    let mut result: Vec<char> = data1
        .chars()
        .filter(|c| data2.contains(*c) && data3.contains(*c))
        .collect();
    result.pop().unwrap_or(' ')
}
