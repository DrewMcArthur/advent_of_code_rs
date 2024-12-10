pub mod p1 {
    use super::*;

    pub fn solve(input: &str) -> usize {
        let mut drive = Drive::from(input);
        let mapping = drive.reorder();
        checksum(&mapping)
    }
}

pub mod p2 {
    use super::*;

    pub fn solve(input: &str) -> usize {
        let mut drive = Drive::from(input);
        drive.reorder_no_split_files();
        checksum(&drive.mapping())
    }
}

struct Drive {
    files: Vec<File>,
}

#[derive(Debug, Clone)]
struct File {
    id: Option<usize>,
    location: usize,
    size: usize,
}

impl From<&str> for Drive {
    fn from(value: &str) -> Self {
        let file_sizes: Vec<usize> = value
            .chars()
            .filter_map(|c| c.to_digit(10).map(|x| x as usize))
            .collect();
        let size: usize = file_sizes.iter().map(|x| *x as usize).sum();
        let mut mapping = Vec::with_capacity(size);
        let mut files = Vec::with_capacity(file_sizes.len() / 2);
        for i in 0..file_sizes.len() {
            let file_id = match i % 2 {
                0 => Some(i / 2),
                1 => None,
                _ => unreachable!(),
            };
            let file_size = file_sizes[i];
            files.push(File {
                id: file_id,
                location: mapping.len(),
                size: file_size,
            });
            for _ in 0..file_size {
                mapping.push(file_id);
            }
        }
        Drive { files }
    }
}

impl Drive {
    fn reorder(&mut self) -> Vec<Option<usize>> {
        let mut mapping = self.mapping();
        let mut last_index = mapping.len() - 1;
        for i in 0..mapping.len() {
            while mapping[last_index].is_none() {
                last_index -= 1;
            }
            if i >= last_index {
                break;
            }
            if mapping[i].is_none() {
                mapping[i] = mapping[last_index];
                mapping[last_index] = None;
                last_index -= 1;
            }
        }
        mapping
    }

    fn mapping(&self) -> Vec<Option<usize>> {
        let file_sizes: Vec<usize> =
            self.files.iter().map(|f| f.size).collect();
        let size: usize = file_sizes.iter().sum();
        let mut mapping = Vec::with_capacity(size);
        for i in 0..file_sizes.len() {
            let file_id = match i % 2 {
                0 => Some(i / 2),
                1 => None,
                _ => unreachable!(),
            };
            let file_size = file_sizes[i];
            for _ in 0..file_size {
                mapping.push(file_id);
            }
        }
        mapping
    }

    // returns the index of the first empty space
    fn first_empty_space(&self, min_size: usize) -> Option<usize> {
        let mut sorted = self.files.clone();
        sorted.sort_by(|a, b| a.location.cmp(&b.location));
        sorted
            .iter()
            .find(|f| f.id.is_none() && f.size >= min_size)
            .map(|f| f.location)
    }

    fn reorder_no_split_files(&mut self) {
        let mut reversed = self.files.clone();
        reversed.sort_by(|a, b| b.location.cmp(&a.location));
        for f in &mut reversed {
            if f.id.is_some() {
                if let Some(loc) = self.first_empty_space(f.size) {
                    if loc < f.location {
                        self.move_file(f, loc);
                    }
                }
            }
        }
    }

    fn move_file(&mut self, file: &mut File, from_loc: usize) {
        let to_loc = file.id.unwrap() * 2;
        self.files[from_loc] = File {
            id: None,
            location: to_loc,
            size: file.size,
        };
        let to_space = &mut self.files[to_loc];
        if to_space.size > file.size {
            to_space.size -= file.size;
            to_space.location += file.size;
        }
        file.location = to_loc;
        self.files.insert(from_loc, file.to_owned());
    }
}

fn checksum(mapping: &[Option<usize>]) -> usize {
    mapping
        .iter()
        .filter_map(|v| *v)
        .enumerate()
        .fold(0, |acc, (i, v)| v * i + acc)
}

#[cfg(test)]
mod tests {
    use std::fmt::Display;

    use super::*;

    fn to_string(mapping: &[Option<usize>]) -> String {
        let mut s = String::new();
        for i in 0..mapping.len() {
            match mapping[i] {
                Some(file) => s.push_str(&format!("{}", file)),
                None => s.push('.'),
            }
        }
        s
    }

    impl Display for Drive {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mapping = self.mapping();
            let s = to_string(&mapping);
            write!(f, "{}", s)?;
            Ok(())
        }
    }

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_map_drive() {
        let d = Drive::from(INPUT);
        assert_eq!(d.to_string(), "00...111...2...333.44.5555.6666.777.888899");
    }

    #[test]
    fn test_reorder() {
        let mut d = Drive::from(INPUT);
        let mapping = d.reorder();
        let s = to_string(&mapping);
        assert_eq!(s, "0099811188827773336446555566..............");
    }

    #[test]
    fn test_solve() {
        assert_eq!(p1::solve(INPUT), 1928);
    }

    #[test]
    fn test_reorder_2() {
        let mut d = Drive::from(INPUT);
        d.reorder_no_split_files();
        assert_eq!(d.to_string(), "00992111777.44.333....5555.6666.....8888..");
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(p2::solve(INPUT), 2858);
    }
}
