// for part 2, they ask where to put an obstacle such that the guard gets stuck in a loop

use std::fs::File;

use crate::{error::GoError, guard::Guard, location::Location, map::Map};

pub fn solve() -> usize {
    let mut file = File::open("input.txt").expect("File not found");
    let mut map = Map::from(&mut file);
    println!(
        "Solving map with dimensions {}x{}",
        map.width(),
        map.height()
    );
    let solutions = find_places_that_create_loops(&mut map);
    solutions.len()
}

fn find_places_that_create_loops(map: &mut Map) -> Vec<Location> {
    (0..map.height())
        .flat_map(|y| {
            (0..map.width())
                .map(|x| Location { x, y })
                .filter(|loc| loc_causes_loop(loc, map))
                .collect::<Vec<Location>>()
        })
        .collect()
}

fn loc_causes_loop(loc: &Location, map: &mut Map) -> bool {
    let c = map.char_at(loc);
    if Guard::is_guard(&c) || c == '#' {
        return false;
    }
    map.set_char_at(loc.clone(), '#');
    let res;
    let mut guard = Guard::from(&*map);
    loop {
        match guard.step() {
            Ok(()) => (),
            Err(GoError::StuckInLoop(_)) => {
                res = true;
                break;
            }
            Err(_) => {
                res = false;
                break;
            }
        }
    }
    map.set_char_at(loc.clone(), c);
    res
}
