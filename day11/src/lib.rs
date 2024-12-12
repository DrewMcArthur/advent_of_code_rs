pub mod p1 {
    use super::*;
    pub fn solve(input: &str) -> usize {
        let mut res = input.split(" ").map(|c| c.parse().unwrap()).collect();
        for _ in 0..25 {
            res = blink(res);
        }
        res.len()
    }
}

pub mod p2;

fn blink(field: Vec<u128>) -> Vec<u128> {
    field.iter().flat_map(apply).flatten().collect()
}

fn apply(stone: &u128) -> [Option<u128>; 2] {
    if *stone == 0 {
        [Some(1), None]
    } else if let Some(log) = stone.checked_ilog10() {
        if log % 2 == 0 {
            [Some(stone * 2024), None]
        } else {
            split(*stone, log)
        }
    } else {
        panic!("couldn't get log of stone");
    }
}

// given a stone (number) and the result of it log base 10, split it in two by digits
fn split(stone: u128, log: u32) -> [Option<u128>; 2] {
    let factor = 10_u128.checked_pow((log + 1) / 2).unwrap();
    let left = stone / factor;
    let right = stone - (left * factor);
    [Some(left), Some(right)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        let r = split(1, 0);
        assert_eq!(r, [Some(1), Some(0)]);
        let r = split(10, 1);
        assert_eq!(r, [Some(1), Some(0)]);
        let r = split(1000, 3);
        assert_eq!(r, [Some(10), Some(0)]);
        let r = split(1234, 3);
        assert_eq!(r, [Some(12), Some(34)]);
        let r = split(123456, 5);
        assert_eq!(r, [Some(123), Some(456)]);
    }

    #[test]
    fn test_blink() {
        let input = vec![0, 1, 10, 99, 999];
        let res = blink(input);
        assert_eq!(res, vec![1, 2024, 1, 0, 9, 9, 2021976]);
    }

    #[test]
    fn test_blinks() {
        let mut v = vec![125, 17];
        v = blink(v);
        assert_eq!(v, vec![253000, 1, 7]);
        v = blink(v);
        assert_eq!(v, vec![253, 0, 2024, 14168]);
        v = blink(v);
        assert_eq!(v, vec![512072, 1, 20, 24, 28676032]);
        v = blink(v);
        assert_eq!(v, vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032]);
        v = blink(v);
        assert_eq!(
            v,
            vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]
        );
        v = blink(v);
        assert_eq!(
            v,
            vec![
                2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80,
                96, 2, 8, 6, 7, 6, 0, 3, 2
            ]
        );
        for _ in 6..25 {
            v = blink(v);
        }
        assert_eq!(v.len(), 55312);
    }
}
