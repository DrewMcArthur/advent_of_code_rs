use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

use day7::perms;

use crate::{
    loc::{
        get_all_antinode_locs_in_bounds, get_first_antinode_locs, AntennaPair,
        Loc,
    },
    NON_ANTENNA_CHARS,
};

pub struct Map {
    // data: Vec<Vec<char>>,
    antennae: HashMap<char, Vec<Loc>>,
    width: usize,
    height: usize,
}

impl From<String> for Map {
    fn from(s: String) -> Map {
        let data: Vec<Vec<char>> =
            s.split("\n").map(|line| line.chars().collect()).collect();
        let antennae = data
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(x, c)| (*c, Loc { x, y }))
                    .filter(|(c, _)| !NON_ANTENNA_CHARS.contains(c))
            })
            .fold(
                HashMap::new(),
                |mut map: HashMap<char, Vec<Loc>>, (c, loc)| {
                    map.entry(c).or_default().push(loc);
                    map
                },
            );
        let width = data[0].len();
        let height = data.len();

        Map {
            // data,
            antennae,
            width,
            height,
        }
    }
}

pub fn load_input(filename: &str) -> Map {
    let mut file = File::open(filename).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    Map::from(contents)
}

impl Map {
    pub fn find_first_antinode_locs(&self) -> HashSet<Loc> {
        self.find_pairs()
            .iter()
            .flat_map(get_first_antinode_locs)
            .flatten()
            .filter(|l| self.is_in_bounds(l))
            .collect()
    }

    pub fn find_all_antinode_locs(&self) -> HashSet<Loc> {
        self.find_pairs()
            .iter()
            .flat_map(|p| {
                get_all_antinode_locs_in_bounds(
                    p.first(),
                    p.second(),
                    (self.width, self.height),
                )
                .iter()
                .chain(
                    get_all_antinode_locs_in_bounds(
                        p.second(),
                        p.first(),
                        (self.width, self.height),
                    )
                    .iter(),
                )
                .copied()
                .collect::<Vec<Loc>>()
            })
            .collect()
    }

    /// returns a vector of pairs of locations of matching types
    fn find_pairs(&self) -> HashSet<AntennaPair> {
        fn find_pairs(antennae: &[Loc]) -> Vec<AntennaPair> {
            perms(antennae, 2)
                .iter()
                .filter(|locs| locs[0] != locs[1])
                .map(move |locs| AntennaPair::new(locs[0], locs[1]))
                .collect::<Vec<AntennaPair>>()
        }

        // we only want to find pairs for antennae of the same type
        // so we find pairs within each list, rather than
        // doing the above permutation on all antennae locations
        self.antennae
            .values()
            .flat_map(|antenna_list| find_pairs(antenna_list))
            .collect()
    }

    fn is_in_bounds(&self, loc: &Loc) -> bool {
        loc.x < self.width && loc.y < self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_input() {
        let map = load_input("test_input.txt");
        assert_eq!(map.width, 12);
        assert_eq!(map.height, 12);
        let expected = HashMap::from([
            (
                '0',
                vec![
                    Loc { x: 8, y: 1 },
                    Loc { x: 5, y: 2 },
                    Loc { x: 7, y: 3 },
                    Loc { x: 4, y: 4 },
                ],
            ),
            (
                'A',
                vec![
                    Loc { x: 6, y: 5 },
                    Loc { x: 8, y: 8 },
                    Loc { x: 9, y: 9 },
                ],
            ),
        ]);
        assert_eq!(map.antennae, expected);
    }

    #[test]
    fn test_pair_equality() {
        let a = AntennaPair::new(Loc { x: 1, y: 2 }, Loc { x: 3, y: 4 });
        let b = AntennaPair::new(Loc { x: 1, y: 2 }, Loc { x: 3, y: 4 });
        assert_eq!(a, b);
        let b = AntennaPair::new(Loc { x: 3, y: 4 }, Loc { x: 1, y: 2 });
        assert_eq!(a, b);

        let c = vec![a, b].into_iter().collect::<HashSet<AntennaPair>>();
        assert_eq!(c.len(), 1);
    }

    #[test]
    fn test_find_pairs() {
        let map = load_input("test_input.txt");
        let pairs = map.find_pairs();
        assert_eq!(pairs.len(), 9);
    }

    #[test]
    fn test_is_antinode_loc_valid() {}
}
