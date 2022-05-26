use std::{convert::From, write};
use std::ops::{Add, AddAssign, Mul};
use std::{default::Default, fmt, fmt::{Debug, Formatter}};

#[derive(Copy, Clone, PartialEq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
    pub z: isize,
    pub w: isize,
}

impl Point {
    pub fn new(x: isize, y: isize, z: isize, w: isize) -> Point {
        Point { x, y, z , w}
    }

    pub fn is_origin(&self) -> bool {
        self.x == 0 && self.y == 0 && self.z == 0 && self.w == 0
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({},{},{}, {})", &self.x, &self.y, &self.z,  &self.w)
    }
}

impl Default for Point {
    fn default() -> Self {
        Point { x: 0, y: 0, z: 0 , w:0}
    }
}

impl From<&[isize]> for Point {
    fn from(a: &[isize]) -> Self {
        Point {
            x: a[0],
            y: a[1],
            z: a[2],
            w: a[3],
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl Mul<isize> for Point {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}
