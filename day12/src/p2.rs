use crate::{Map, Region};

pub fn solve(input: &str) -> usize {
    Map::from(input)
        .regions()
        .values()
        .flatten()
        .map(price)
        .sum()
}

fn price(r: &Region) -> usize {
    r.area() * r.sides()
}
