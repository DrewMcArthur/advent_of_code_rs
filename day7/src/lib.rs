use std::io::Read;

mod equation;

use equation::compute;
pub use equation::concat;
pub use equation::perms;
pub use equation::Equation;
use equation::Op;

pub fn load_input() -> Vec<Equation> {
    let mut file = std::fs::File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.split("\n").map(Equation::from).collect()
}

pub fn p1(data: &[Equation]) -> i64 {
    compute(data, &[Op::Add, Op::Mul])
}

pub fn p2(data: &[Equation]) -> i64 {
    compute(data, &[Op::Add, Op::Mul, Op::Cat])
}
