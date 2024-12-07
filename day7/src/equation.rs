pub struct Equation {
    res: i32,
    rhs: Vec<i32>,
}

impl From<&str> for Equation {
    fn from(s: &str) -> Self {
        let (lhs, rhs) = s.split_once(": ").unwrap();
        let res = lhs.parse().unwrap();
        let rhs = rhs.split(" ").map(|s| s.parse().unwrap()).collect();
        Equation { res, rhs }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let s = "123: 1 2 3";
        let eq = Equation::from(s);
        assert_eq!(eq.res, 123);
        assert_eq!(eq.rhs, vec![1, 2, 3]);
    }
}
