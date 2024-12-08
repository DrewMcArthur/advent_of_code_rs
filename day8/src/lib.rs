mod loc;
mod map;

pub use loc::Loc;
pub use map::load_input;

const NON_ANTENNA_CHARS: [char; 2] = ['.', '#'];

// then, for each pair figure out if it produces an antinode
// (is in bounds? not overlapping with another of the same kind?
// can overlapp with other antennae or types of antinodes
// but probably not double of its own kind)

#[derive(Debug, PartialEq)]
enum Error {
    OutOfBounds,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_find_antinodes() {
        let map = load_input("test_input.txt");
        let locs = map.find_first_antinode_locs();
        // assert_eq!(locs.len(), 2);
        // let zero_locs = locs.get(&'0').unwrap();
        // assert_eq!(zero_locs.len(), 10);
        // let a_locs = locs.get(&'A').unwrap();
        // assert_eq!(a_locs.len(), 5);

        assert_eq!(14, locs.len());
    }

    #[test]
    fn test_find_all_antinodes() {
        let map = load_input("test_input.txt");
        let locs = map.find_all_antinode_locs();
        assert_eq!(locs.len(), 34);
    }
}
