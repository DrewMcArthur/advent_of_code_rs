#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Location {
    pub x: i32,
    pub y: i32,
}

pub struct Robot {
    pub(crate) p: Location,
    pub(crate) v: Location,
}

impl From<&str> for Location {
    fn from(s: &str) -> Location {
        let (_, loc) = s.split_once("=").unwrap();
        let (x, y) = loc.split_once(",").unwrap();
        Location {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

impl From<&str> for Robot {
    fn from(s: &str) -> Robot {
        let (p, v) = s.split_once(" ").unwrap();
        let p = Location::from(p);
        let v = Location::from(v);
        Robot { p, v }
    }
}

impl Robot {
    // moves the robot n times
    pub fn step(&mut self, n: usize, bounds: Location) {
        let (dx, dy) = (self.v.x * n as i32, self.v.y * n as i32);
        self.p.x += dx;
        self.p.y += dy;
        self.p.x %= bounds.x;
        self.p.y %= bounds.y;
        while self.p.x < 0 {
            self.p.x += bounds.x;
        }
        while self.p.y < 0 {
            self.p.y += bounds.y;
        }
    }

    pub fn loc(&self) -> Location {
        self.p
    }
}

impl From<&(i32, i32)> for Location {
    fn from((x, y): &(i32, i32)) -> Location {
        Location { x: *x, y: *y }
    }
}

impl Location {
    pub fn within(&self, square: &(Location, Location)) -> bool {
        let top = square.0.y;
        let left = square.0.x;
        let right = square.1.x;
        let bottom = square.1.y;

        self.x >= left && self.x < right && self.y >= top && self.y < bottom
    }
}
