use std::{fs::File, time::Instant};

mod direction;
mod error;
mod guard;
mod hypothetical;
mod location;
mod map;

use guard::Guard;
use map::Map;

fn main() {
    let map = load_map();
    let mut guard: Guard = Guard::from(&map);
    loop {
        if let Err(e) = guard.step() {
            println!("Went until error: {:?}", e);
            break;
        }
    }
    println!("Part 1: {}", guard.num_locations_visited());
    let start = Instant::now();
    println!("Part 2: {} in {:?}", hypothetical::solve(), start.elapsed());
}

fn load_map() -> Map {
    let mut file = File::open("input.txt").expect("File not found");
    Map::from(&mut file)
}
