use std::time::Instant;

fn main() {
    let start = Instant::now();
    let map = day8::load_input("input.txt");
    let collect = map.find_first_antinode_locs();
    let n = collect.len();
    println!("Part 1: {} in {:?}", n, start.elapsed());
    let p2 = map.find_all_antinode_locs().len();
    println!("Part 2: {} in {:?}", p2, start.elapsed());
}
