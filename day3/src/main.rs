use std::{
    fs::File,
    io::{BufReader, Read},
    time::Instant,
};

use regex::Regex;

fn main() {
    let input = get_input();
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let start = Instant::now();
    println!("Total: {}", get_total(&regex, &input));
    println!("Time: {:?}", start.elapsed());

    let start = Instant::now();
    let sections = input
        .split("do()")
        .into_iter()
        .map(|s| s.split("don't()").collect::<Vec<&str>>()[0].to_string())
        .collect::<Vec<String>>();

    let total: i32 = sections
        .into_iter()
        .map(|s| get_total(&regex, s.as_str()))
        .sum();

    println!("Enabled total: {}", total);
    println!("Time: {:?}", start.elapsed());
}

fn get_total(rx: &Regex, input: &str) -> i32 {
    rx.captures_iter(input).into_iter().fold(0, |acc, cap| {
        let a: i32 = cap[1].parse().unwrap();
        let b: i32 = cap[2].parse().unwrap();
        acc + a * b
    })
}

fn get_input() -> String {
    let file = File::open("input.txt").expect("File not found");
    let mut reader = BufReader::new(file);
    let mut input = String::new();
    reader
        .read_to_string(&mut input)
        .expect("Failed to read file");
    input
}
