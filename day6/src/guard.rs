use std::collections::HashSet;

use crate::{
    direction::Direction, error::GoError, location::Location, map::Map,
};

const GUARD_CHARS: [char; 5] = ['^', 'v', '<', '>', 'V'];
const CAN_MOVE: [char; 2] = ['.', 'X'];

pub struct Guard<'a> {
    current: Vector,
    history: LocationHistory,
    map: &'a Map,
}

struct LocationHistory(HashSet<Vector>);
impl LocationHistory {
    fn start(loc: Vector) -> LocationHistory {
        let mut v = HashSet::new();
        v.insert(loc);
        LocationHistory(v)
    }

    fn push(&mut self, loc: Vector) -> Result<(), GoError> {
        if !self.0.insert(loc) {
            Err(GoError::StuckInLoop(loc))
        } else {
            Ok(())
        }
    }

    fn num_unique_locations(&self) -> usize {
        self.0
            .iter()
            .map(|l| l.loc)
            .collect::<HashSet<Location>>()
            .len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector {
    loc: Location,
    dir: Direction,
}

impl<'a> From<&'a Map> for Guard<'a> {
    fn from(map: &Map) -> Guard {
        let loc = map.find_guard().expect("no guard found");
        let dir = map.char_at(&loc).into();
        let current = Vector { loc, dir };
        let history = LocationHistory::start(current);
        Guard {
            current,
            history,
            map,
        }
    }
}

impl Guard<'_> {
    pub fn is_guard(c: &char) -> bool {
        GUARD_CHARS.contains(c)
    }

    fn turn(&mut self) -> Result<(), GoError> {
        self.current.dir = match self.current.dir {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
        self.history.push(self.current)?;
        Ok(())
    }

    // takes a map, because we alter the map based on guard movements
    // TODO: swap this around, just have the guard keep track of all previous locations,
    // to be able to correctly throw "stuck in loop" error
    // if location_history.contains(new_loc) { return Err(GoError::StuckInLoop) }
    pub fn step(&mut self) -> Result<(), GoError> {
        let old = self.current.loc;
        let new = old.move_in(self.current.dir);

        match self.map.try_char_at(new)? {
            '#' => {
                self.turn()?;
            }
            c if self.can_move_to(c) => {
                self.current.loc = new.try_into()?;
                self.history.push(self.current)?;
            }
            c => return Err(GoError::UnknownChar(c)),
        }

        Ok(())
    }

    fn can_move_to(&self, c: char) -> bool {
        CAN_MOVE.contains(&c) || Guard::is_guard(&c)
    }

    pub fn num_locations_visited(&self) -> usize {
        self.history.num_unique_locations()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_locations_visited() {
        let start = Vector {
            loc: Location { x: 0, y: 0 },
            dir: Direction::Right,
        };
        let mut guard = Guard {
            current: start,
            history: LocationHistory::start(start),
            map: &mut Map::from("...\n...\n...".to_string()),
        };
        assert_eq!(guard.num_locations_visited(), 1);
        guard.step().unwrap();
        assert_eq!(guard.num_locations_visited(), 2);
    }

    #[test]
    fn out_of_bounds() {
        let start = Vector {
            loc: Location { x: 0, y: 0 },
            dir: Direction::Right,
        };
        let mut guard = Guard {
            current: start,
            history: LocationHistory::start(start),
            map: &mut Map::from("...\n...\n...".to_string()),
        };
        assert_eq!(guard.num_locations_visited(), 1);
        guard.step().unwrap();
        assert_eq!(guard.num_locations_visited(), 2);
        guard.step().unwrap();
        assert_eq!(guard.num_locations_visited(), 3);

        assert_eq!(
            guard.step().unwrap_err(),
            GoError::OutOfBounds(Direction::Right)
        );

        assert_eq!(guard.num_locations_visited(), 3);
        assert_eq!(
            guard.current,
            Vector {
                loc: Location { x: 2, y: 0 },
                dir: Direction::Right
            }
        );

        guard.turn().unwrap();
        assert_eq!(guard.num_locations_visited(), 3);
        assert_eq!(guard.history.0.len(), 4);
        assert_eq!(guard.current.dir, Direction::Down);

        guard.turn().unwrap();
        assert_eq!(guard.num_locations_visited(), 3);
        assert_eq!(guard.history.0.len(), 5);
        assert_eq!(guard.current.dir, Direction::Left);

        guard.turn().unwrap();
        assert_eq!(guard.num_locations_visited(), 3);
        assert_eq!(guard.history.0.len(), 6);
        assert_eq!(guard.current.dir, Direction::Up);

        assert_eq!(
            guard.turn().unwrap_err(),
            GoError::StuckInLoop(Vector {
                loc: Location { x: 2, y: 0 },
                dir: Direction::Right
            })
        );
    }

    #[test]
    fn environment() {
        let map = Map::from(
            r".#..
.>.#
#...
..#."
                .to_string(),
        );
        let mut guard = Guard::from(&map);
        assert_eq!(
            guard.current,
            Vector {
                loc: Location { x: 1, y: 1 },
                dir: Direction::Right
            }
        );
        let mut i = 0;
        let history = [
            Vector {
                loc: Location { x: 1, y: 1 },
                dir: Direction::Right,
            },
            Vector {
                loc: Location { x: 2, y: 1 },
                dir: Direction::Right,
            },
            Vector {
                loc: Location { x: 2, y: 1 },
                dir: Direction::Down,
            },
            Vector {
                loc: Location { x: 2, y: 2 },
                dir: Direction::Down,
            },
            Vector {
                loc: Location { x: 2, y: 2 },
                dir: Direction::Left,
            },
            Vector {
                loc: Location { x: 1, y: 2 },
                dir: Direction::Left,
            },
            Vector {
                loc: Location { x: 1, y: 2 },
                dir: Direction::Up,
            },
            Vector {
                loc: Location { x: 1, y: 1 },
                dir: Direction::Up,
            },
        ];
        loop {
            assert_eq!(guard.current, history[i]);
            i += 1;
            if let Err(e) = guard.step() {
                assert_eq!(guard.history.0.len(), history.len());
                assert_eq!(
                    e,
                    GoError::StuckInLoop(Vector {
                        loc: Location { x: 1, y: 1 },
                        dir: Direction::Right
                    })
                );
                break;
            }
        }
    }

    #[test]
    fn example() {
        let map = Map::from(
            r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
                .to_string(),
        );
        let mut guard = Guard::from(&map);

        loop {
            if let Err(e) = guard.step() {
                assert_eq!(e, GoError::OutOfBounds(Direction::Down));
                break;
            }
        }

        assert_eq!(guard.num_locations_visited(), 41);
    }
}
