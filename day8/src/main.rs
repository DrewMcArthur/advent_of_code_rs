use std::time::Instant;

fn main() {
    let start = Instant::now();
    let map = day8::load_input("input.txt");
    let collect = map.find_first_antinode_locs();
    let n = collect.len();
    assert_eq!(n, 285);
    println!("Part 1: {} in {:?}", n, start.elapsed());
    println!(
        "Part 2: {} in {:?}",
        map.find_all_antinode_locs().len(),
        start.elapsed()
    );
}

// todo: add tests to pieces
//       remove char from antennapair
