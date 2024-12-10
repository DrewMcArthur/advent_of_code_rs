use std::{collections::HashSet, time::Instant};

fn main() {
    let start = Instant::now();
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {} in {:?}", solve(&input), start.elapsed());
    println!("Part 2: {} in {:?}", solve_p2(&input), start.elapsed());
}

fn find_trailheads(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '0')
                .map(move |(x, _)| (x as i32, y as i32))
        })
        .collect()
}

fn char_at((x, y): (i32, i32), input: &str) -> char {
    let line = input.lines().nth(y as usize).unwrap();
    line.chars().nth(x as usize).unwrap()
}

fn in_bounds((x, y): (i32, i32), input: &str) -> bool {
    x >= 0
        && x < input.lines().next().unwrap().len() as i32
        && y >= 0
        && y < input.lines().count() as i32
}

fn total_score(trailhead: (i32, i32), input: &str) -> usize {
    all_trails(trailhead, input).len()
}

fn unique_score(trailhead: (i32, i32), input: &str) -> usize {
    unique_trails(trailhead, input).len()
}

fn all_trails(trailhead: (i32, i32), input: &str) -> Vec<(i32, i32)> {
    let (x, y) = trailhead;
    let curr = char_at(trailhead, input);
    if curr == '9' {
        return Vec::from([trailhead]);
    }

    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    directions
        .iter()
        .filter_map(|(dx, dy)| {
            let next = (x + dx, y + dy);
            if !in_bounds(next, input) {
                return None;
            }
            if char_at(next, input) as u8 == curr as u8 + 1 {
                Some(all_trails(next, input))
            } else {
                None
            }
        })
        .flatten()
        .collect()
}

fn unique_trails(trailhead: (i32, i32), input: &str) -> HashSet<(i32, i32)> {
    all_trails(trailhead, input)
        .iter()
        .copied()
        .collect::<HashSet<(i32, i32)>>()
}

fn solve(input: &str) -> usize {
    find_trailheads(input)
        .iter()
        .map(|t| unique_score(*t, input))
        .sum()
}

fn solve_p2(input: &str) -> usize {
    find_trailheads(input)
        .iter()
        .map(|t| total_score(*t, input))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_score() {
        assert_eq!(unique_score((1, 0), INPUT), 1);
        assert_eq!(unique_score((0, 0), INPUT), 1);
        assert_eq!(unique_score((1, 2), INPUT), 2);
        assert_eq!(unique_score((4, 4), INPUT), 3);
        assert_eq!(unique_score((3, 4), INPUT), 3);
        assert_eq!(unique_score((2, 4), INPUT), 3);
        assert_eq!(unique_score((7, 7), INPUT), 0);
        assert_eq!(unique_score((5, 3), INPUT), 3);
        assert_eq!(unique_score((6, 3), INPUT), 3);
        assert_eq!(unique_score((6, 2), INPUT), 3);
        assert_eq!(unique_score((2, 0), INPUT), 5);
    }

    #[test]
    fn solve() {
        let trailheads = find_trailheads(INPUT);
        assert_eq!(trailheads.len(), 9);
        let expected_scores = [5, 6, 5, 3, 1, 3, 5, 3, 5];
        for (i, th) in trailheads.iter().enumerate() {
            assert_eq!(char_at(*th, INPUT), '0');
            assert_eq!(unique_score(*th, INPUT), expected_scores[i]);
        }
        assert_eq!(super::solve(INPUT), 36);
    }

    #[test]
    fn solve_p2() {
        let trailheads = find_trailheads(INPUT);
        let expected_scores = [20, 24, 10, 4, 1, 4, 5, 8, 5];
        for (i, th) in trailheads.iter().enumerate() {
            assert_eq!(total_score(*th, INPUT), expected_scores[i]);
        }
        assert_eq!(super::solve_p2(INPUT), 81);
    }
}
