pub mod p1 {
    use super::*;

    pub fn solve(input: &str) -> usize {
        let mut drive = Drive::from(input);
        let mapping = drive.reorder_chunks();
        checksum(&mapping)
    }
}

pub mod p2 {
    use super::*;

    pub fn solve(input: &str) -> usize {
        let mut drive = Drive::from(input);
        drive.reorder_files();
        checksum(&drive.mapping())
    }
}

struct Drive {
    files: Vec<File>,
    size: usize,
}

#[derive(Debug, Clone)]
struct File {
    id: usize,
    location: usize,
    size: usize,
}

impl File {
    fn end_index(&self) -> usize {
        self.location + self.size - 1
    }
}

impl From<&str> for Drive {
    fn from(value: &str) -> Self {
        let file_sizes: Vec<usize> = value
            .chars()
            .filter_map(|c| c.to_digit(10).map(|x| x as usize))
            .collect();
        let size = file_sizes.iter().sum();
        let mut current_index = 0;
        let mut files = Vec::with_capacity(file_sizes.len() / 2);
        for (i, file_size) in file_sizes.iter().enumerate() {
            let file_id = match i % 2 {
                0 => Some(i / 2),
                1 => None,
                _ => unreachable!(),
            };
            if let Some(file_id) = file_id {
                files.push(File {
                    id: file_id,
                    location: current_index,
                    size: *file_size,
                });
            }
            current_index += file_size;
        }
        Drive { files, size }
    }
}

impl Drive {
    fn reorder_chunks(&mut self) -> Vec<Option<usize>> {
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
        let mut files = self.files.clone();
        files.sort_by(|a, b| a.location.cmp(&b.location));
        let size: usize = files.iter().map(|f| f.size).sum();
        let mut mapping = Vec::with_capacity(size);
        let mut maybe_last_file: Option<&File> = None;
        for file in &files {
            if let Some(last_file) = maybe_last_file {
                let space_size = file.location - last_file.end_index() - 1;
                for _ in 0..space_size {
                    mapping.push(None);
                }
            }
            for _ in 0..file.size {
                mapping.push(Some(file.id))
            }
            maybe_last_file = Some(file);
        }
        while mapping.len() < self.size {
            mapping.push(None);
        }
        mapping
    }

    // returns the index of the first empty space
    fn first_empty_space(&self, min_size: usize) -> Option<usize> {
        let mut files = self.files.clone();
        files.sort_by(|a, b| a.location.cmp(&b.location));

        let mut maybe_last_file: Option<File> = None;
        // todo could rewrite with indices
        for file in files {
            match maybe_last_file {
                None => {
                    if file.location > 0 {
                        let space_size = file.location;
                        if space_size >= min_size {
                            return Some(0);
                        }
                    }
                }
                Some(last_file) => {
                    let space_size = file.location - last_file.end_index() - 1;
                    if space_size >= min_size {
                        return Some(last_file.end_index() + 1);
                    }
                }
            }
            maybe_last_file = Some(file);
        }
        None
    }

    fn reorder_one_file(&mut self, index: usize) {
        let mut f = self.files[index].clone();
        if let Some(loc) = self.first_empty_space(f.size) {
            if loc < f.location {
                f.location = loc;
                self.files[index] = f;
            }
        }
    }

    fn reorder_files(&mut self) {
        let n_files = self.files.len();
        for i in (0..n_files).rev() {
            self.reorder_one_file(i);
        }
    }
}

fn checksum(mapping: &[Option<usize>]) -> usize {
    mapping
        .iter()
        .map(|v| v.unwrap_or(0))
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
        let mapping = d.reorder_chunks();
        let s = to_string(&mapping);
        assert_eq!(s, "0099811188827773336446555566..............");
    }

    #[test]
    fn test_solve() {
        assert_eq!(p1::solve(INPUT), 1928);
    }

    #[test]
    fn test_first_empty_space() {
        let d = Drive::from(INPUT);
        assert_eq!(d.first_empty_space(3), Some(2));
        assert_eq!(d.first_empty_space(2), Some(2));
        assert_eq!(d.first_empty_space(4), None);
    }

    #[test]
    fn test_reorder_2() {
        let mut d = Drive::from(INPUT);
        d.reorder_files();
        assert_eq!(d.to_string(), "00992111777.44.333....5555.6666.....8888..");
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(p2::solve(INPUT), 2858);
    }
}
