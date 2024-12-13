use std::collections::HashMap;

pub mod p1;
pub mod p2;

pub fn read_input(filename: &str) -> String {
    println!("Reading input from {}", filename);
    std::fs::read_to_string(filename).expect("Failed to read input file")
}

struct Map {
    data: Vec<Vec<char>>,
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        Self {
            data: input.lines().map(|line| line.chars().collect()).collect(),
        }
    }
}

impl Map {
    fn regions(&self) -> HashMap<char, Vec<Region>> {
        let mut locations_by_char = HashMap::new();
        for (y, row) in self.data.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell != ' ' {
                    locations_by_char
                        .entry(cell)
                        .or_insert_with(Vec::new)
                        .push((x, y));
                }
            }
        }

        locations_by_char
            .into_iter()
            .map(|(c, locations)| {
                let regions = regions(&locations);
                (c, regions)
            })
            .collect()
    }
}

fn regions(locations: &Vec<(usize, usize)>) -> Vec<Region> {
    let mut regions: Vec<Region> = vec![];
    let mut found_region = false;
    for location in locations {
        for region in &mut regions {
            if region
                .locations
                .iter()
                .any(|l| is_adjacent(&&(*l, *location)))
            {
                region.locations.push(*location);
                found_region = true;
                break;
            }
        }
        if !found_region {
            regions.push(Region {
                locations: vec![*location],
            });
        }
        found_region = false;
    }

    while regions
        .iter()
        .any(|r| regions.iter().any(|s| r.should_merge(s)))
    {
        let mut merged_regions = vec![];
        for region in &regions {
            let mut merged = false;
            for merged_region in &mut merged_regions {
                if region.should_merge(merged_region) {
                    merged_region.merge(region);
                    merged = true;
                    break;
                }
            }
            if !merged {
                merged_regions.push(region.clone());
            }
        }
        regions = merged_regions;
    }

    regions
}

#[derive(Clone)]
struct Region {
    locations: Vec<(usize, usize)>,
}

impl PartialEq for Region {
    fn eq(&self, other: &Self) -> bool {
        self.locations.iter().all(|l| other.locations.contains(l))
    }
}

impl Region {
    fn area(&self) -> usize {
        self.locations.len()
    }

    fn num_adjacent_locations(&self) -> usize {
        perm(&self.locations).iter().filter(is_adjacent).count() / 2
    }

    fn perimeter(&self) -> usize {
        self.area() * 4 - 2 * self.num_adjacent_locations()
    }

    fn sides(&self) -> usize {
        0
    }

    fn should_merge(&self, other: &Region) -> bool {
        self != other
            && self.locations.iter().any(|l| {
                other.locations.iter().any(|m| is_adjacent(&&(*l, *m)))
            })
    }

    fn merge(&mut self, other: &Region) {
        self.locations.extend(other.locations.iter().copied());
    }
}

fn perm<T: Copy>(v: &[T]) -> Vec<(T, T)> {
    v.iter()
        .flat_map(|x| v.iter().map(move |y| (*x, *y)))
        .collect()
}

#[allow(clippy::nonminimal_bool)]
fn is_adjacent(pair: &&((usize, usize), (usize, usize))) -> bool {
    let (a, b) = pair;
    let (x1, y1) = a;
    let (x2, y2) = b;
    (*x1 == *x2 && *y1 + 1 == *y2)
        || (*x1 == *x2 && *y1 == *y2 + 1)
        || (*x1 + 1 == *x2 && *y1 == *y2)
        || (*x1 == *x2 + 1 && *y1 == *y2)
}

#[cfg(test)]
mod tests {
    use crate::{p1, read_input, regions, Map};

    #[test]
    fn test_region() {
        let a = (0, 0);
        let b = (0, 1);
        let c = (1, 0);
        let d = (1, 1);
        assert!(super::is_adjacent(&&(a, b)));
        assert!(super::is_adjacent(&&(a, c)));
        assert!(!super::is_adjacent(&&(a, d)));
        assert!(!super::is_adjacent(&&(b, c)));
        assert!(super::is_adjacent(&&(b, d)));
        assert!(super::is_adjacent(&&(c, d)));
        assert!(!super::is_adjacent(&&(a, a)));
        assert!(!super::is_adjacent(&&(d, d)));
        let region = super::Region {
            locations: vec![a, b, c, d],
        };

        assert_eq!(region.area(), 4);
        assert_eq!(region.num_adjacent_locations(), 4);
        assert_eq!(region.perimeter(), 8);
        assert_eq!(p1::price(&region), 32);
    }

    #[test]
    fn test_internal() {
        let input = r"AAA
ABA
AAA";
        let map = Map::from(input);
        let regions = map.regions();
        assert_eq!(regions.len(), 2);

        let a = regions.get(&'A').unwrap().first().unwrap();
        assert_eq!(a.area(), 8);
        assert_eq!(a.num_adjacent_locations(), 8);
        assert_eq!(a.perimeter(), 16);

        let b = regions.get(&'B').unwrap().first().unwrap();
        assert_eq!(b.area(), 1);
        assert_eq!(b.perimeter(), 4);
    }

    #[test]
    fn test_disparate_regions() {
        let a = (0, 0);
        let b = (2, 0);
        let regions = regions(&vec![a, b]);
        assert_eq!(regions.len(), 2);

        let input = r"ABA";
        let map = Map::from(input);
        let regions = map.regions().values().flatten().count();
        assert_eq!(regions, 3);
    }

    #[test]
    fn test_input() {
        let input = read_input("test_input.txt");
        let map = Map::from(input.as_str());
        let regions = map.regions();
        let total_price: usize =
            regions.values().flatten().map(p1::price).sum();
        let _output = r"
A region of R plants with price 12 * 18 = 216.
A region of I plants with price 4 * 8 = 32.
A region of C plants with price 14 * 28 = 392.
A region of F plants with price 10 * 18 = 180.
A region of V plants with price 13 * 20 = 260.
A region of J plants with price 11 * 20 = 220.
A region of C plants with price 1 * 4 = 4.
A region of E plants with price 13 * 18 = 234.
A region of I plants with price 14 * 22 = 308.
A region of M plants with price 5 * 12 = 60.
A region of S plants with price 3 * 8 = 24.";
        assert_eq!(p1::price(regions.get(&'R').unwrap().first().unwrap()), 216);
        // assert_eq!(regions.get(&'I').unwrap().price(), 32);
        // assert_eq!(regions.get(&'I').unwrap().price(), 308);
        assert_eq!(regions.get(&'I').unwrap().len(), 2);
        assert_eq!(p1::price(&regions.get(&'I').unwrap()[0]), 32);
        assert_eq!(p1::price(&regions.get(&'I').unwrap()[1]), 308);
        // assert_eq!(regions.get(&'C').unwrap().price(), 392);
        // assert_eq!(regions.get(&'C').unwrap().price(), 4);
        // assert_eq!(regions.get(&'C').unwrap().price(), 397);
        // assert_eq!(regions.get(&'F').unwrap().price(), 181);
        // assert_eq!(regions.get(&'V').unwrap().price(), 261);
        // assert_eq!(regions.get(&'J').unwrap().price(), 221);
        // assert_eq!(regions.get(&'E').unwrap().price(), 235);
        // assert_eq!(regions.get(&'M').unwrap().price(), 61);
        // assert_eq!(regions.get(&'S').unwrap().price(), 25);
        assert_eq!(total_price, 1930);
    }
}
