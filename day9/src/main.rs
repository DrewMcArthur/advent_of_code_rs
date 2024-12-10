use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = load_input("input.txt");
    let res = day9::p1::solve(&input);
    println!("Part 1: {} in {:?}", &res, start.elapsed());
    let res = day9::p2::solve(&input);
    println!("Part 2: {} in {:?}", &res, start.elapsed());
}

fn load_input(filename: &str) -> String {
    std::fs::read_to_string(filename).unwrap()
}
