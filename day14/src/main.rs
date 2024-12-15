use day14::{ee, load_input, p1, Location, Robot};

fn main() {
    let input = "input.txt";
    let bounds = Location { x: 101, y: 103 };
    println!("Part 1: {}", p1::solve(input, bounds));

    let mut robots = load_input(input);
    let mut i = 0;
    loop {
        robots.iter_mut().for_each(|r| r.step(1, bounds));
        let locs = robots.iter().map(Robot::loc).collect();
        let map = ee::Map { bounds, locs };
        if map.is_tree() {
            println!("{}", i);
            println!("{}", map);
            // sleep(Duration::from_millis(100));
        }
        i += 1;
    }
}
