use std::ops::{Add, AddAssign, Mul};
use std::{default::Default, fmt, fmt::{Debug, Formatter}};

#[derive(Copy, Clone,PartialEq, Eq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub const fn new(x: isize, y: isize) -> Point {
        Point { x, y}
    }

    pub fn is_origin(&self) -> bool {
        self.x == 0 && self.y == 0
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", &self.x, &self.y)
    }
}

impl Default for Point {
    fn default() -> Self {
        Point { x: 0, y: 0}
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl <'a, 'b> Add<&'a Point> for &'b Point {
    type Output = Point;

    fn add(self, other: &'a Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}


impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul<isize> for Point {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
