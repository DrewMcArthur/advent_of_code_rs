use std::time::Instant;

use day11::{p1, p2};

fn main() {
    let start = Instant::now();
    let input = std::fs::read_to_string("input.txt").unwrap();
    let res = p1::solve(&input);
    println!("Part 1: {} in {:?}", res, start.elapsed());
    let res = p2::solve(&input);
    println!("Part 2: {} in {:?}", res, start.elapsed());
}
