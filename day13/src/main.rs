use std::time::Instant;

use day13::{load_input, modify_input, p1, p2};

fn main() {
    let mut input = load_input("input.txt");
    let start = Instant::now();
    println!("Part 1: {} in {:?}", p1(&input), start.elapsed());
    modify_input(&mut input);
    println!("Part 2: {} in {:?}", p2(&input), start.elapsed());
}
