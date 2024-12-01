use std::{collections::HashMap, fs};

fn get_input(file_path: &str) -> (Vec<u32>, Vec<u32>) {
    let lines: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut left = vec![];
    let mut right = vec![];
    for line in lines.to_vec() {
        let numbers = line
            .split_ascii_whitespace()
            .map(|s| {s.parse::<u32>().unwrap()})
            .collect::<Vec<u32>>();
        left.push(numbers[0]);
        right.push(numbers[1]);
    }
    return (left, right)
}

fn part_one(left: &mut [u32], right: &mut [u32]) -> u32 {
    left.sort_unstable();
    right.sort_unstable();

    let mut ans = 0;
    for i in 0..left.len() {
        ans += left[i].abs_diff(right[i]);
    }
    ans
}

fn part_two(left: Vec<u32>, right: Vec<u32>) -> u32 {
    println!("{}", right.len());
    let mut occurs: HashMap<u32, u32> = HashMap::new();
    for r in right {
        let val = occurs.entry(r).or_insert(0);
        *val += 1;
    }
    let mut ans = 0;
    for l in left {
        ans += l * occurs.get(&l).unwrap_or(&0);
    }
    ans
}

fn main() {
    let file_path = "src/input.txt";
    let (mut left, mut right) = get_input(file_path);

    println!("{}", part_one(&mut left, &mut right));
    println!("{}", part_two(left, right));
}
