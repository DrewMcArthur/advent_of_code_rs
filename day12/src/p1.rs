use crate::{Map, Region};

pub fn solve(input: &str) -> usize {
    Map::from(input)
        .regions()
        .values()
        .flatten()
        .map(price)
        .sum()
}

pub(super) fn price(region: &Region) -> usize {
    region.area() * region.perimeter()
}
