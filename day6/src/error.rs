use crate::{direction::Direction, guard::Vector};

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum GoError {
    OutOfBounds(Direction),
    StuckInLoop(Vector),
    UnknownChar(char),
}
