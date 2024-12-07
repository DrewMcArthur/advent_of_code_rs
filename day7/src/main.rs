use std::{io::Read, time::Instant};

mod equation;

use equation::{Equation, Op};

fn main() {
    let data = load_input();
    let start = Instant::now();
    println!(
        "Part 1: {} in {:?}",
        compute(&data, &[Op::Add, Op::Mul]),
        start.elapsed()
    );
    println!(
        "Part 2: {} in {:?}",
        compute(&data, &[Op::Add, Op::Mul, Op::Cat]),
        start.elapsed()
    );
}

fn load_input() -> Vec<Equation> {
    let mut file = std::fs::File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.split("\n").map(Equation::from).collect()
}

fn compute(data: &Vec<Equation>, ops: &[Op]) -> i64 {
    data.iter()
        .filter(|e| e.has_solution(ops))
        .map(|e| e.res)
        .sum()
}
