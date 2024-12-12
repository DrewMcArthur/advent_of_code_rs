use std::{collections::HashMap, time::Instant};

pub fn solve(input: &str) -> usize {
    let mut res = Field::from(input);
    let start = Instant::now();
    for _ in 0..25 {
        res = blink(res);
    }
    println!("did 25 in {:?}", start.elapsed());
    for _ in 0..50 {
        res = blink(res);
    }
    res.0.values().sum()
}

struct Field(HashMap<u128, usize>);

impl From<&str> for Field {
    fn from(value: &str) -> Self {
        let mut field = HashMap::new();
        for stone in value.split(" ") {
            let stone: u128 = stone.parse().unwrap();
            // increment field at stone
            let count = *field.entry(stone).or_insert_with(|| 0);
            field.insert(stone, count + 1);
        }
        Field(field)
    }
}

impl Field {
    fn size(&self) -> usize {
        self.0.values().len()
    }
}

fn blink(field: Field) -> Field {
    let mut new = HashMap::with_capacity(2 * field.size());
    for stone in field.0.keys() {
        let count = field.0.get(stone).unwrap();
        match apply(stone) {
            OutputType::Single(new_stone) => {
                let count = *new.entry(new_stone).or_insert_with(|| 0) + count;
                new.insert(new_stone, count);
            }
            OutputType::Double(l, r) => {
                let left_count = *new.entry(l).or_insert_with(|| 0) + count;
                new.insert(l, left_count);
                let right_count = *new.entry(r).or_insert_with(|| 0) + count;
                new.insert(r, right_count);
            }
        };
    }
    Field(new)
}

enum OutputType {
    Single(u128),
    Double(u128, u128),
}

fn apply(stone: &u128) -> OutputType {
    if *stone == 0 {
        OutputType::Single(1)
    } else if let Some(log) = stone.checked_ilog10() {
        if log % 2 == 0 {
            OutputType::Single(stone * 2024)
        } else {
            split(*stone, log)
        }
    } else {
        panic!("couldn't get log of stone");
    }
}

// given a stone (number) and the result of it log base 10, split it in two by digits
fn split(stone: u128, log: u32) -> OutputType {
    let factor = 10_u128.checked_pow((log + 1) / 2).unwrap();
    let left = stone / factor;
    let right = stone - (left * factor);
    OutputType::Double(left, right)
}

#[cfg(test)]
mod tests {
    use crate::p1;

    use super::*;

    #[test]
    fn compare_blinks() {
        let input = "1 2 3 4";
        let p1 = p1::solve(input);
        let mut f = Field::from(input);
        for _ in 0..25 {
            f = blink(f);
        }
        let p2: usize = f.0.values().sum();
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_blink() {
        let mut f = Field::from("0");
        f = blink(f);
        assert_eq!(f.0, HashMap::from([(1, 1)]));
        f = blink(f);
        assert_eq!(f.0, HashMap::from([(2024, 1)]));
        f = blink(f);
        assert_eq!(f.0, HashMap::from([(20, 1), (24, 1)]));
        f = Field::from("0 0");
        f = blink(f);
        assert_eq!(f.0, HashMap::from([(1, 2)]));
        f = blink(f);
        assert_eq!(f.0, HashMap::from([(2024, 2)]));
        f = blink(f);
        assert_eq!(f.0, HashMap::from([(20, 2), (24, 2)]));
        for _ in 3..7 {
            f = blink(f)
        }
    }

    #[test]
    fn test_idempotency() {
        let mut a = Field::from("0");
        let mut b = Field::from("0");
        let mut i = 0;
        while a.0 == b.0 && i < 100 {
            a = blink(a);
            b = blink(b);
            i += 1;
        }
        assert_eq!(i, 100);
        assert_eq!(a.0, b.0);
    }
}
