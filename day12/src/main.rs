use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = day12::read_input("input.txt");
    println!(
        "Part 1: {} in {:?}",
        day12::p1::solve(&input),
        start.elapsed()
    );
    println!(
        "Part 2: {} in {:?}",
        day12::p2::solve(&input),
        start.elapsed()
    );
}
