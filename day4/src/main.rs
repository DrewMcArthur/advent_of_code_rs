pub fn main() {
    let rows = day4::load("input.txt");
    let matches = day4::p1(rows.clone());
    println!("Part 1: {}", matches);
    day4::p2(rows.clone());
}
