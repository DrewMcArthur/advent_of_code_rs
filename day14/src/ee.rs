// easter egg
// sometimes the list of robots appear like a christmas tree?
// define a map, get the string repr, check if it matches the pic

use std::fmt::Display;

use itertools::Itertools;

use crate::Location;

pub struct Map {
    pub locs: Vec<Location>,
    pub bounds: Location,
}

impl Map {
    fn as_str(&self) -> Vec<Vec<char>> {
        let mut v = Vec::new();
        for y in 0..self.bounds.y {
            let mut s = Vec::new();
            for x in 0..self.bounds.x {
                if self.locs.contains(&Location { x, y }) {
                    s.push('#');
                } else {
                    s.push(' ');
                }
            }
            v.push(s);
        }
        v
    }

    pub fn is_tree(&self) -> bool {
        self.locs
            .iter()
            .chunk_by(|l| l.y)
            .into_iter()
            .any(|(_, g)| g.count() > 5)
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for s in &self.as_str() {
            for c in s {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_tree() {
        let map = Map {
            locs: vec![
                Location { x: 0, y: 0 },
                Location { x: 1, y: 0 },
                Location { x: 2, y: 0 },
                Location { x: 3, y: 0 },
                Location { x: 4, y: 0 },
                Location { x: 4, y: 0 },
            ],
            bounds: Location { x: 5, y: 1 },
        };
        assert!(map.is_tree());
    }
}
