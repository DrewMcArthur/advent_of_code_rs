use std::io::Read;

mod equation;

use equation::Equation;

fn main() {
    println!("Hello, world!");
    let data = load_input();
}
fn load_input() -> Vec<Equation> {
    let mut file = std::fs::File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.split("\n").map(Equation::from).collect()
}
