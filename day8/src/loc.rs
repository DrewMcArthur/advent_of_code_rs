use crate::Error;
use std::{collections::HashSet, hash::Hash};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Loc {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AntennaPair {
    locs: (Loc, Loc),
}

impl AntennaPair {
    pub fn new(a: Loc, b: Loc) -> AntennaPair {
        if a < b {
            AntennaPair { locs: (a, b) }
        } else {
            AntennaPair { locs: (b, a) }
        }
    }

    pub fn first(&self) -> Loc {
        self.locs.0
    }

    pub fn second(&self) -> Loc {
        self.locs.1
    }
}

/// a vector designated by dx/dy.  
struct DeltaLoc {
    dx: i32,
    dy: i32,
}

impl Loc {
    fn add(&self, delta: &DeltaLoc) -> Result<Loc, Error> {
        let x = self.x as i32 + delta.dx;
        let y = self.y as i32 + delta.dy;
        if x < 0 || y < 0 {
            return Err(Error::OutOfBounds);
        }
        Ok(Loc {
            x: x as usize,
            y: y as usize,
        })
    }

    fn add_in_bounds(
        &self,
        delta: &DeltaLoc,
        bounds: (usize, usize),
    ) -> Result<Loc, Error> {
        let x = self.x as i32 + delta.dx;
        let y = self.y as i32 + delta.dy;
        if x < 0 || y < 0 || x >= bounds.0 as i32 || y >= bounds.1 as i32 {
            return Err(Error::OutOfBounds);
        }
        Ok(Loc {
            x: x as usize,
            y: y as usize,
        })
    }

    fn diff(&self, other: &Loc) -> DeltaLoc {
        let dx = self.x as i32 - other.x as i32;
        let dy = self.y as i32 - other.y as i32;
        DeltaLoc { dx, dy }
    }

    fn maybe_antinode_loc(&self, other: &Loc) -> Result<Loc, Error> {
        let delta = self.diff(other);
        Ok(self.add(&delta)?)
    }
}

pub fn get_first_antinode_locs(a: &AntennaPair) -> [Option<Loc>; 2] {
    let (loc1, loc2) = a.locs;
    [
        loc1.maybe_antinode_loc(&loc2).ok(),
        loc2.maybe_antinode_loc(&loc1).ok(),
    ]
}

pub fn get_all_antinode_locs_in_bounds(
    a: Loc,
    b: Loc,
    bounds: (usize, usize),
) -> HashSet<Loc> {
    let mut res = HashSet::from([a]);
    let delta = a.diff(&b);
    let mut loc = a;
    // in each direction, keep adding deltas until we go out of bounds
    loop {
        loc = match loc.add_in_bounds(&delta, bounds) {
            Ok(loc) => {
                res.insert(loc);
                loc
            }
            Err(Error::OutOfBounds) => break,
        };
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff() {
        let loc1 = Loc { x: 1, y: 2 };
        let loc2 = Loc { x: 3, y: 5 };
        let delta = loc1.diff(&loc2);
        assert_eq!(delta.dx, -2);
        assert_eq!(delta.dy, -3);
        let delta = loc2.diff(&loc1);
        assert_eq!(delta.dx, 2);
        assert_eq!(delta.dy, 3);
    }

    #[test]
    fn test_add() {
        let loc1 = Loc { x: 1, y: 2 };
        let delta = DeltaLoc { dx: 2, dy: 3 };
        let loc2 = loc1.add(&delta).unwrap();
        assert_eq!(loc2.x, 3);
        assert_eq!(loc2.y, 5);

        let e = loc1.add(&DeltaLoc { dx: -2, dy: -3 }).unwrap_err();
        assert_eq!(e, Error::OutOfBounds);
    }

    #[test]
    fn test_add_in_bounds() {
        let loc1 = Loc { x: 1, y: 2 };
        let delta = DeltaLoc { dx: 2, dy: 3 };
        let bounds = (3, 3);
        let e = loc1.add_in_bounds(&delta, bounds).unwrap_err();
        assert_eq!(e, Error::OutOfBounds);
    }

    #[test]
    fn test_get_all_antinode_locs_in_bounds() {
        let a = Loc { x: 1, y: 1 };
        let b = Loc { x: 0, y: 0 };
        let bounds = (5, 5);
        let res = get_all_antinode_locs_in_bounds(a, b, bounds);
        assert_eq!(res.len(), 4);
    }
}
