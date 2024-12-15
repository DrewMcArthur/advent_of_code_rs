use std::collections::HashMap;

pub use robot::{Location, Robot};

pub mod ee;
mod robot;

pub mod p1 {
    use crate::{robot::Location, safety_factor};

    pub fn solve(filename: &str, bounds: Location) -> usize {
        let mut robots = super::load_input(filename);
        robots.iter_mut().for_each(|r| r.step(100, bounds));
        safety_factor(&robots, bounds)
    }
}

pub fn load_input(filename: &str) -> Vec<Robot> {
    let input = std::fs::read_to_string(filename).unwrap();
    input.lines().map(|l| l.trim().into()).collect()
}

fn safety_factor(robots: &[Robot], bounds: Location) -> usize {
    robots
        .iter()
        .map(Robot::loc)
        .map(|l| (quadrant(bounds, l), l))
        .filter(|(q, _)| q.is_some())
        .map(|(q, l)| (q.unwrap(), l))
        .fold(HashMap::new(), |mut map: HashMap<usize, usize>, (q, _)| {
            *map.entry(q).or_insert(0) += 1;
            map
        })
        .values()
        .product()
}

// returns which quadrant the location belongs in,
// 0 = top left, 1=top right, 2=bottom left, 3=bottom right,
// None=on center lines
fn quadrant(bounds: Location, loc: Location) -> Option<usize> {
    if loc.x == bounds.x / 2 || loc.y == bounds.y / 2 {
        return None;
    }
    let quadrant_boundaries: Vec<(Location, Location)> = [
        ((0, 0), (bounds.x / 2, bounds.y / 2)),
        ((bounds.x / 2, 0), (bounds.x, bounds.y / 2)),
        ((0, bounds.y / 2), (bounds.x / 2, bounds.y)),
        ((bounds.x / 2, bounds.y / 2), (bounds.x, bounds.y)),
    ]
    .iter()
    .map(|(a, b)| (Location::from(a), Location::from(b)))
    .collect();

    for (i, q) in quadrant_boundaries.iter().enumerate() {
        if loc.within(q) {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "test_input.txt";
        let bounds = Location { x: 11, y: 7 };
        assert_eq!(p1::solve(input, bounds), 12);
    }

    #[test]
    fn test_quadrant() {
        let bounds = Location { x: 11, y: 7 };
        assert_eq!(quadrant(bounds, Location { x: 0, y: 0 }), Some(0));
        assert_eq!(quadrant(bounds, Location { x: 0, y: 2 }), Some(0));
        assert_eq!(quadrant(bounds, Location { x: 0, y: 4 }), Some(2));
        assert_eq!(quadrant(bounds, Location { x: 5, y: 2 }), None);
        assert_eq!(quadrant(bounds, Location { x: 10, y: 2 }), Some(1));
        assert_eq!(quadrant(bounds, Location { x: 11, y: 2 }), None);

        let robots = load_input("test_input.txt");
        for r in &robots {
            let q = quadrant(bounds, r.loc());
            assert_eq!(q.is_some(), r.loc().x != 5 && r.loc().y != 3);
        }

        let num_in_quads = robots
            .iter()
            .map(|r| quadrant(bounds, r.loc()))
            .filter(|q| q.is_some())
            .count();

        assert_eq!(num_in_quads, 8);
    }

    #[test]
    fn test_num_in_quadrants() {
        let bounds = Location { x: 11, y: 7 };
        let mut robots = load_input("test_input.txt");
        let quads = robots
            .iter()
            .map(Robot::loc)
            .map(|l| (quadrant(bounds, l), l))
            .filter(|(q, _)| q.is_some())
            .map(|(q, l)| (q.unwrap(), l));

        assert_eq!(quads.clone().count(), 8);
        let mut map: HashMap<usize, Vec<Location>> = HashMap::new();
        for (q, l) in quads {
            map.entry(q).or_default().push(l);
        }

        println!("{:?}", map);
        assert_eq!(map.get(&0).unwrap().len(), 4);
        assert_eq!(map.get(&1), None);
        assert_eq!(map.get(&2).unwrap().len(), 2);
        assert_eq!(map.get(&3).unwrap().len(), 2);

        robots.iter_mut().for_each(|r| r.step(100, bounds));
        let map: Vec<(Option<usize>, Location)> = robots
            .iter()
            .map(Robot::loc)
            .map(|l| (quadrant(bounds, l), l))
            .collect();
        println!("{:?}", map);

        let map = map
            .iter()
            .filter(|(q, _)| q.is_some())
            .map(|(q, l)| (q.unwrap(), l))
            .fold(
                HashMap::new(),
                |mut map: HashMap<usize, Vec<Location>>, (q, l)| {
                    map.entry(q).or_default().push(*l);
                    map
                },
            );
        println!("{:?}", map);
        assert_eq!(map.get(&0).unwrap().len(), 1);
        assert_eq!(map.get(&1).unwrap().len(), 3);
        assert_eq!(map.get(&2).unwrap().len(), 4);
        assert_eq!(map.get(&3).unwrap().len(), 1);
    }

    #[test]
    fn test_load_input() {
        let robots = load_input("test_input.txt");
        assert_eq!(robots.len(), 12);
        let locs = [
            (0, 4),
            (6, 3),
            (10, 3),
            (2, 0),
            (0, 0),
            (3, 0),
            (7, 6),
            (3, 0),
            (9, 3),
            (7, 3),
            (2, 4),
            (9, 5),
        ];
        for (i, r) in robots.iter().enumerate() {
            assert_eq!(r.loc(), Location::from(&locs[i]));
        }
    }

    #[test]
    fn test_step() {
        let bounds = Location { x: 11, y: 13 };
        let mut r = Robot {
            p: Location { x: 0, y: 0 },
            v: Location { x: 1, y: 0 },
        };

        assert_eq!(r.loc(), Location { x: 0, y: 0 });
        assert_eq!(quadrant(bounds, r.loc()), Some(0));

        r.step(1, bounds);
        assert_eq!(r.loc(), Location { x: 1, y: 0 });
        assert_eq!(quadrant(bounds, r.loc()), Some(0));

        r.step(1, bounds);
        assert_eq!(r.loc(), Location { x: 2, y: 0 });
        assert_eq!(quadrant(bounds, r.loc()), Some(0));

        r.step(1, bounds);
        assert_eq!(r.loc(), Location { x: 3, y: 0 });
        assert_eq!(quadrant(bounds, r.loc()), Some(0));

        r.step(1, bounds);
        assert_eq!(r.loc(), Location { x: 4, y: 0 });
        assert_eq!(quadrant(bounds, r.loc()), Some(0));

        r.step(1, bounds);
        assert_eq!(r.loc(), Location { x: 5, y: 0 });
        assert_eq!(quadrant(bounds, r.loc()), None);

        r.step(1, bounds);
        assert_eq!(r.loc(), Location { x: 6, y: 0 });
        assert_eq!(quadrant(bounds, r.loc()), Some(1));

        r.step(5, bounds);
        assert_eq!(r.loc(), Location { x: 0, y: 0 });
        assert_eq!(quadrant(bounds, r.loc()), Some(0));
    }
}
