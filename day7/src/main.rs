use std::io::Read;

mod equation;

use equation::Equation;

fn main() {
    let data = load_input();
    let p1: i64 = data
        .iter()
        .filter(|e| e.has_solution())
        .map(|e| e.res)
        .sum();
    println!("Part 1: {}", p1);
}
fn load_input() -> Vec<Equation> {
    let mut file = std::fs::File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.split("\n").map(Equation::from).collect()
}
