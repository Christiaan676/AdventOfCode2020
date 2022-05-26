use std::default::Default;
use std::ops::{Add, AddAssign, Mul};
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }

    pub fn rotate_90_left(&self) -> Point {
        Point {
            x: -self.y,
            y: self.x,
        }
    }

    pub fn rotate_90_right(&self) -> Point {
        Point {
            x: self.y,
            y: -self.x,
        }
    }
}

impl Default for Point {
    fn default() -> Self {
        Point { x: 0, y: 0 }
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
