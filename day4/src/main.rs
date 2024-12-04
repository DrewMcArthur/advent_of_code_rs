use regex::Regex;

fn main() {
    p1();
    p2();
}

fn p1() {
    let rx = Regex::new(r"XMAS").unwrap();
    let rows = load();
    let rotations = get_rotations(rows);
    let matches: usize = rotations
        .iter()
        .map(|rows| {
            rows.iter()
                .map(|r| rx.captures_iter(r).count())
                .sum::<usize>()
        })
        .sum();
    println!("Part 1: {}", matches);
}

fn p2() {
    let grid = load();
    let chunks = get_chunks(grid);
    let matches: usize = chunks.iter().filter(|chunk| has_xmas(chunk)).count();
    println!("Part 2: {}", matches);
}

fn has_xmas(chunk: &Vec<Vec<char>>) -> bool {
    let a = vec![chunk[0][0], chunk[1][1], chunk[2][2]];
    let b = vec![chunk[0][2], chunk[1][1], chunk[2][0]];
    let c: Vec<char> = "MAS".chars().collect();
    let d: Vec<char> = "SAM".chars().collect();
    return (a == c || a == d) && (b == c || b == d);
}

fn get_chunks(grid: Vec<String>) -> Vec<Vec<Vec<char>>> {
    let grid: Vec<Vec<char>> = grid.iter().map(|r| r.chars().collect()).collect();
    let mut chunks: Vec<Vec<Vec<char>>> = Vec::new();
    for i in 0..grid.len() - 2 {
        for j in 0..grid[i].len() - 2 {
            let chunk = vec![
                vec![grid[i][j], grid[i][j + 1], grid[i][j + 2]],
                vec![grid[i + 1][j], grid[i + 1][j + 1], grid[i + 1][j + 2]],
                vec![grid[i + 2][j], grid[i + 2][j + 1], grid[i + 2][j + 2]],
            ];
            chunks.push(chunk);
        }
    }
    chunks
}

fn get_rotations(rows: Vec<String>) -> Vec<Vec<String>> {
    let mut nineties: Vec<Vec<String>> = (0..4).map(|i| rotate_n(rows.clone(), i)).collect();
    let diag = rotate_diag(rows.clone());
    // let diags: Vec<Vec<String>> = (0..4).map(|i| rotate_n(diag.clone(), i)).collect();
    let rev_diag = rotate_diag(rotate_n(rows.clone(), 3));
    // let rev_diags: Vec<Vec<String>> = (0..4).map(|i| rotate_n(rev_diag.clone(), i)).collect();

    nineties.push(diag.clone());
    nineties.push(rotate_n(diag, 2));
    nineties.push(rev_diag.clone());
    nineties.push(rotate_n(rev_diag, 2));
    nineties
}

fn rotate_n(rows: Vec<String>, i: i32) -> Vec<String> {
    if i == 0 {
        rows
    } else {
        rotate_n(rotate(rows), i - 1)
    }
}

fn rotate(rows: Vec<String>) -> Vec<String> {
    let n_rows = rows.len();
    let n_cols = rows[0].len();
    let mut rotated = Vec::with_capacity(n_cols);
    let rows: Vec<Vec<char>> = rows.iter().map(|r| r.chars().collect()).collect();
    for r in 0..n_cols {
        rotated.push(Vec::with_capacity(n_rows));
        for c in 0..n_rows {
            rotated[r].push(rows[rows.len() - c - 1][r]);
        }
    }
    rotated.iter().map(|r| r.iter().collect()).collect()
}

/// given a square matrix, rotate it 45 degrees clockwise
fn rotate_diag(rows: Vec<String>) -> Vec<String> {
    let rows: Vec<Vec<char>> = rows.iter().map(|r| r.chars().collect()).collect();
    let side_length = 2 * rows[0].len() - 1;
    let mut rotated: Vec<Vec<char>> = Vec::with_capacity(rows.len());
    for r in 0..rows.len() {
        rotated.push(Vec::with_capacity(side_length));
        for _ in 0..r {
            rotated[r].push('.');
        }
        for c in 0..rows[r].len() {
            rotated[r].push(rows[r][c]);
        }
        for _ in r..side_length - rows[r].len() {
            rotated[r].push('.');
        }
    }

    rotate(rotated.iter().map(|row| row.iter().collect()).collect())
}

fn load() -> Vec<String> {
    std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let rows = vec!["abc", "def", "ghi"];
        let rows: Vec<String> = rows.iter().map(|s| s.to_string()).collect();
        print_grid(&rows);
        print_grid(&rotate_n(rows.clone(), 1));
        print_grid(&rotate_n(rows.clone(), 2));
        print_grid(&rotate_n(rows.clone(), 3));
        assert_eq!(rotate_n(rows.clone(), 0), rows);
        assert_eq!(rotate_n(rows.clone(), 4), rows);
        assert_eq!(rotate_n(rows.clone(), 1), vec!["gda", "heb", "ifc"]);
        assert_eq!(rotate_n(rows.clone(), 2), vec!["ihg", "fed", "cba"]);
        assert_eq!(rotate_n(rows.clone(), 3), vec!["cfi", "beh", "adg"]);
    }

    #[test]
    fn test_rotate_diag() {
        let rows = vec!["abc", "def", "ghi"];
        let rows: Vec<String> = rows.iter().map(|s| s.to_string()).collect();
        let rotated = vec!["..a", ".db", "gec", "hf.", "i.."];
        print_grid(&rows);
        print_grid(&rotated.iter().map(|s| s.to_string()).collect());
        print_grid(&rotate_diag(rows.clone()));
        print_grid(&rotate_diag(rotate_diag(rows.clone())));
        assert_eq!(rotate_diag(rows.clone()), rotated);
    }

    #[test]
    fn test_reverse_diag() {
        let rows = vec!["abc", "def", "ghi"];
        let rows: Vec<String> = rows.iter().map(|s| s.to_string()).collect();
        let rotated = vec!["..c", ".bf", "aei", "dh.", "g.."];
        print_grid(&rows);
        print_grid(&rotated.iter().map(|s| s.to_string()).collect());
        print_grid(&rotate_diag(rows.clone()));
        assert_eq!(rotate_diag(rotate_n(rows.clone(), 3)), rotated);
    }

    #[test]
    fn test_all() {
        let rows = vec!["abc", "def", "ghi"];
        let rows = rows.iter().map(|s| s.to_string()).collect();
        let grids = get_rotations(rows);
        let rows: Vec<&String> = grids.iter().flat_map(|g| g.iter()).collect();
        let expected_strs = vec!["abc", "cba", "gec", "aei", "gda", "adg"];
        for s in expected_strs {
            assert!(rows.contains(&&s.to_string()), "s: {}", s);
            assert!(
                rows.contains(&&s.chars().rev().collect::<String>()),
                "s: {}",
                s
            );
        }
    }

    fn print_grid(rows: &Vec<String>) {
        for r in rows {
            println!("{}", r);
        }
        println!();
    }
}
