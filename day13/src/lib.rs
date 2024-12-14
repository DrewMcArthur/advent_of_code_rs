pub fn p1(input: &[Machine]) -> usize {
    input.iter().filter_map(solve_p1).map(cost).sum::<i64>() as usize
}

pub fn p2(input: &[Machine]) -> usize {
    input.iter().filter_map(solve).map(cost).sum::<i64>() as usize
}

pub fn load_input(filename: &str) -> Vec<Machine> {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut res = Vec::new();
    let mut a = None;
    let mut b = None;
    let mut p = None;
    input.lines().enumerate().for_each(|(i, line)| match i % 4 {
        0 => a = Some(Location::from(line)),
        1 => b = Some(Location::from(line)),
        2 => {
            p = Some(Location::from(line));
            res.push(Machine {
                a: a.clone().unwrap(),
                b: b.clone().unwrap(),
                p: p.clone().unwrap(),
            })
        }
        3 => {}
        _ => unreachable!(),
    });

    res
}

// for p2, all solutions are incremented by 10000000000000
pub fn modify_input(input: &mut [Machine]) {
    input.iter_mut().for_each(|m| {
        m.p.x += 10000000000000;
        m.p.y += 10000000000000;
    });
}

#[derive(Debug, Clone, PartialEq)]
struct Location {
    x: i64,
    y: i64,
}

// prize is at a location (p), and the buttons a and b change the current location by dx and dy (stored as location)
#[derive(Debug, PartialEq)]
pub struct Machine {
    a: Location,
    b: Location,
    p: Location,
}

impl From<&str> for Location {
    fn from(value: &str) -> Self {
        let x_loc = value.find('X').unwrap();
        let comma_loc = value.find(',').unwrap();
        let x = value[x_loc + 2..comma_loc].parse().unwrap();
        let y_loc = value.find('Y').unwrap();
        let y = value[y_loc + 2..].parse().unwrap();
        Location { x, y }
    }
}

// represents the solution to an equation
struct Solution {
    a: i64,
    b: i64,
}

fn cost(s: Solution) -> i64 {
    s.a * 3 + s.b
}

fn solve(m: &Machine) -> Option<Solution> {
    // i did ask chatgpt for help with this function,
    // but only for "how to solve two linear equations",
    // since i only know how to do them on paper

    let a1 = m.a.x;
    let b1 = m.b.x;
    let c1 = m.p.x;

    let a2 = m.a.y;
    let b2 = m.b.y;
    let c2 = m.p.y;
    // Calculate the determinant of the coefficient matrix
    let det = a1 * b2 - a2 * b1;
    if det == 0 {
        println!("No solution for machine {:?}", m);
        return if a1 * c2 == a2 * c1 && b1 * c2 == b2 * c1 {
            Some(Solution { a: 0, b: c2 / b2 })
        } else {
            None
        };
    }

    if (c1 * b2 - c2 * b1) % det != 0 || (a1 * c2 - a2 * c1) % det != 0 {
        return None; // Skip invalid cases
    }

    // Cramer's Rule
    let a = (c1 * b2 - c2 * b1) / det;
    let b = (a1 * c2 - a2 * c1) / det;

    if a < 0 || b < 0 {
        return None;
    }

    Some(Solution { a, b })
}

fn solve_p1(m: &Machine) -> Option<Solution> {
    solve(m).filter(|s| s.a < 100 && s.b < 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_input() {
        let input = load_input("test_input.txt");
        assert_eq!(input.len(), 4);

        let expected = vec![
            Machine {
                a: Location { x: 94, y: 34 },
                b: Location { x: 22, y: 67 },
                p: Location { x: 8400, y: 5400 },
            },
            Machine {
                a: Location { x: 26, y: 66 },
                b: Location { x: 67, y: 21 },
                p: Location { x: 12748, y: 12176 },
            },
            Machine {
                a: Location { x: 17, y: 86 },
                b: Location { x: 84, y: 37 },
                p: Location { x: 7870, y: 6450 },
            },
            Machine {
                a: Location { x: 69, y: 23 },
                b: Location { x: 27, y: 71 },
                p: Location { x: 18641, y: 10279 },
            },
        ];
        assert_eq!(input, expected);
    }

    #[test]
    fn test_solve() {
        let machine = Machine {
            a: Location { x: 94, y: 34 },
            b: Location { x: 22, y: 67 },
            p: Location { x: 8400, y: 5400 },
        };
        let sol = solve(&machine);
        assert!(sol.is_some());
        let sol = sol.unwrap();
        assert_eq!(sol.a, 80);
        assert_eq!(sol.b, 40);
        let cost = cost(sol);
        assert_eq!(cost, 280);
    }

    #[test]
    fn test_full_test_input() {
        let input = load_input("test_input.txt");
        assert_eq!(p1(&input), 480);
    }
}
