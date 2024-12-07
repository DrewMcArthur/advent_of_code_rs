use crate::{direction::Direction, error::GoError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaybeLocation {
    pub x: i32,
    pub y: i32,
}

impl TryFrom<MaybeLocation> for Location {
    type Error = GoError;
    fn try_from(maybe_loc: MaybeLocation) -> Result<Location, Self::Error> {
        if maybe_loc.x < 0 {
            Err(GoError::OutOfBounds(Direction::Left))
        } else if maybe_loc.y < 0 {
            Err(GoError::OutOfBounds(Direction::Up))
        } else {
            Ok(Location {
                x: maybe_loc.x as usize,
                y: maybe_loc.y as usize,
            })
        }
    }
}

impl Location {
    pub fn move_in(&self, direction: Direction) -> MaybeLocation {
        MaybeLocation {
            x: self.x as i32 + direction.dx(),
            y: self.y as i32 + direction.dy(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_in() {
        let loc = Location { x: 0, y: 0 };
        assert_eq!(loc.move_in(Direction::Right), MaybeLocation { x: 1, y: 0 });
        assert_eq!(loc.move_in(Direction::Down), MaybeLocation { x: 0, y: 1 });
        assert_eq!(loc.move_in(Direction::Left), MaybeLocation { x: -1, y: 0 });
        assert_eq!(loc.move_in(Direction::Up), MaybeLocation { x: 0, y: -1 });
    }
}
