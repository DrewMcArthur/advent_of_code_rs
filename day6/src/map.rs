use std::collections::HashMap;
use std::{fs::File, io::Read};

use crate::error::GoError;
use crate::guard::Guard;
use crate::location::{Location, MaybeLocation};

#[derive(Clone)]
pub struct Map {
    rows: Vec<Row>,
}

#[derive(Clone)]
struct Row(HashMap<usize, char>); // maybe just a string?

impl From<&mut File> for Map {
    fn from(file: &mut File) -> Map {
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        Map::from(contents)
    }
}

impl From<String> for Map {
    fn from(s: String) -> Map {
        let rows = s.split("\n").map(Row::from).collect();
        Map { rows }
    }
}

impl From<&str> for Row {
    fn from(row: &str) -> Row {
        Row(row.chars().enumerate().collect())
    }
}

impl Map {
    pub fn find_guard(&self) -> Option<Location> {
        match self
            .rows
            .iter()
            .enumerate()
            .find(|(_, r)| r.has_guard())
            .map(|(y, r)| (y, r.find_guard()))
        {
            None => None,
            Some((_, None)) => panic!("unreachable, returned a row with no guard"),
            Some((y, Some(x))) => Some(Location { x, y }),
        }
    }

    pub fn check_in_bounds(&self, loc: &MaybeLocation) -> Result<(), GoError> {
        let (x, y) = (loc.x, loc.y);
        if x < 0 {
            Err(GoError::OutOfBounds(crate::direction::Direction::Left))
        } else if y < 0 {
            Err(GoError::OutOfBounds(crate::direction::Direction::Up))
        } else if y >= self.rows.len() as i32 {
            Err(GoError::OutOfBounds(crate::direction::Direction::Down))
        } else if x >= self.rows[0].0.len() as i32 {
            Err(GoError::OutOfBounds(crate::direction::Direction::Right))
        } else {
            Ok(())
        }
    }

    pub fn try_char_at(&self, loc: MaybeLocation) -> Result<char, GoError> {
        self.check_in_bounds(&loc)?;
        Ok(self.char_at(&loc.try_into()?))
    }

    pub fn set_char_at(&mut self, loc: &Location, c: char) {
        self.rows[loc.y].set_char_at(loc.x, c);
    }

    pub fn char_at(&self, loc: &Location) -> char {
        self.rows[loc.y].char_at(loc.x)
    }

    pub fn height(&self) -> usize {
        self.rows.len()
    }

    pub fn width(&self) -> usize {
        self.rows[0].0.len()
    }
}

impl Row {
    fn set_char_at(&mut self, x: usize, c: char) {
        self.0.insert(x, c);
    }

    fn char_at(&self, x: usize) -> char {
        *self.0.get(&x).unwrap()
    }

    fn has_guard(&self) -> bool {
        self.0.values().any(Guard::is_guard)
    }

    fn find_guard(&self) -> Option<usize> {
        self.0
            .iter()
            .filter(|(_, c)| Guard::is_guard(c))
            .next()
            .map(|(x, _)| *x)
    }
}

#[cfg(test)]
mod tests {
    use crate::direction::Direction;

    use super::*;

    #[test]
    fn row() {
        let row = Row::from("abc");
        assert_eq!(row.char_at(0), 'a');
        assert_eq!(row.char_at(1), 'b');
        assert_eq!(row.char_at(2), 'c');
    }

    #[test]
    fn row_find_guard() {
        let row = Row::from("abc");
        assert!(!row.has_guard());
        assert_eq!(row.find_guard(), None);

        let rows = vec![
            Row::from("avc"),
            Row::from("aVc"),
            Row::from("a<c"),
            Row::from("a>c"),
            Row::from("a^c"),
        ];
        for row in rows {
            assert!(row.has_guard());
            assert_eq!(row.find_guard(), Some(1));
        }
    }

    #[test]
    fn in_bounds() {
        let map = Map::from("abc\ndef".to_string());
        assert!(map.check_in_bounds(&MaybeLocation { x: 0, y: 0 }).is_ok());
        assert!(map.check_in_bounds(&MaybeLocation { x: 2, y: 0 }).is_ok());
        assert!(map.check_in_bounds(&MaybeLocation { x: 0, y: 1 }).is_ok());
        assert!(map.check_in_bounds(&MaybeLocation { x: 2, y: 1 }).is_ok());
    }

    #[test]
    fn out_of_bounds() {
        let map = Map::from("abc\ndef".to_string());
        let cases = [
            (MaybeLocation { x: -1, y: 0 }, Direction::Left),
            (MaybeLocation { x: 0, y: -1 }, Direction::Up),
            (MaybeLocation { x: 3, y: 0 }, Direction::Right),
            (MaybeLocation { x: 0, y: 3 }, Direction::Down),
        ];
        for (loc, dir) in cases {
            assert_eq!(map.check_in_bounds(&loc), Err(GoError::OutOfBounds(dir)));
        }
    }

    #[test]
    fn find_guard() {
        let map = Map::from("abc\ndef\nghi".to_string());
        assert_eq!(map.find_guard(), None);

        let map = Map::from("abc\ndef\n.>i".to_string());
        assert_eq!(map.find_guard(), Some(Location { x: 1, y: 2 }));

        let map = Map::from("^bc\ndef\n<gi".to_string());
        assert_eq!(map.find_guard(), Some(Location { x: 0, y: 0 }));
    }

    #[test]
    fn char_at() {
        let map = Map::from("abc\ndef\nghi".to_string());
        let cases = [
            (Location { x: 0, y: 0 }, 'a'),
            (Location { x: 1, y: 0 }, 'b'),
            (Location { x: 2, y: 0 }, 'c'),
            (Location { x: 0, y: 1 }, 'd'),
            (Location { x: 1, y: 1 }, 'e'),
            (Location { x: 2, y: 1 }, 'f'),
            (Location { x: 0, y: 2 }, 'g'),
            (Location { x: 1, y: 2 }, 'h'),
            (Location { x: 2, y: 2 }, 'i'),
        ];
        for (loc, c) in cases {
            assert_eq!(map.char_at(&loc), c);
        }
    }
}
