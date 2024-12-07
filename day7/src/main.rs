use std::time::Instant;

use day7::{load_input, p1, p2};

fn main() {
    let data = load_input();
    let start = Instant::now();
    println!("Part 1: {} in {:?}", p1(&data), start.elapsed());
    println!("Part 2: {} in {:?}", p2(&data), start.elapsed());
}
