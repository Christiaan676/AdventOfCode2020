mod point;

use point::Point;
use std::convert::From;

#[derive(Debug)]
pub enum Direction {
    North(usize),
    East(usize),
    South(usize),
    West(usize),
    Left(usize),  // Port
    Right(usize), //Starboard
    Forward(usize),
}

impl From<&str> for Direction {
    fn from(line: &str) -> Self {
        let (inst, nr) = line.split_at(1);

        let nr = nr.parse().unwrap();
        match inst.chars().next().unwrap() {
            'N' => Direction::North(nr),
            'E' => Direction::East(nr),
            'S' => Direction::South(nr),
            'W' => Direction::West(nr),
            'L' => Direction::Left(nr),
            'R' => Direction::Right(nr),
            'F' => Direction::Forward(nr),
            _ => unimplemented!(),
        }
    }
}

mod part1 {
    use super::*;
    #[derive(Debug)]
    pub struct Boat {
        pub location: Point,
        pub angle: usize,
    }

    impl Default for Boat {
        fn default() -> Self {
            Boat {
                location: Point::default(),
                angle: 90,
            }
        }
    }

    impl Boat {
        pub fn move_boat(&mut self, direction: &Direction) {
            match direction {
                Direction::North(dist) => self.location += Point::new(0, *dist as isize),
                Direction::East(dist) => self.location += Point::new(*dist as isize, 0),
                Direction::South(dist) => self.location += Point::new(0, -1 * *dist as isize),
                Direction::West(dist) => self.location += Point::new(-1 * *dist as isize, 0),
                Direction::Left(angle) => self.angle = (self.angle + 360 - angle) % 360,
                Direction::Right(angle) => self.angle = (self.angle + angle) % 360,
                Direction::Forward(dist) => match self.angle {
                    0 => self.location += Point::new(0, *dist as isize),
                    90 => self.location += Point::new(*dist as isize, 0),
                    180 => self.location += Point::new(0, -1 * *dist as isize),
                    270 => self.location += Point::new(-1 * *dist as isize, 0),
                    _ => unimplemented!(),
                },
            }
        }
    }
}

mod part2 {
    use super::*;
    #[derive(Debug)]
    pub struct Boat {
        pub location: Point,
        pub way_point: Point, //waypoit relative to boat
    }

    impl Default for Boat {
        fn default() -> Self {
            Boat {
                location: Point::default(),
                way_point: Point::new(10, 1),
            }
        }
    }

    impl Boat {
        pub fn move_boat(&mut self, direction: &Direction) {
            match direction {
                Direction::North(dist) => self.way_point += Point::new(0, *dist as isize),
                Direction::East(dist) => self.way_point += Point::new(*dist as isize, 0),
                Direction::South(dist) => self.way_point += Point::new(0, -1 * *dist as isize),
                Direction::West(dist) => self.way_point += Point::new(-1 * *dist as isize, 0),
                Direction::Left(angle) => {
                    match angle {
                        90 => self.way_point = self.way_point.rotate_90_left(),
                        180 => self.way_point = self.way_point * -1,
                        270 => self.way_point = self.way_point.rotate_90_right(),
                        _ => unimplemented!()
                    }
                },
                Direction::Right(angle) => {
                    match angle {
                        90 => self.way_point = self.way_point.rotate_90_right(),
                        180 => self.way_point = self.way_point * -1,
                        270 => self.way_point = self.way_point.rotate_90_left(),
                        _ => unimplemented!()
                    }
                },
                Direction::Forward(dist) => self.location += self.way_point * *dist as isize,
            }
        }
    }
}
fn main() {
    let input = include_str!("../input.txt");

    {
        use part1::Boat;
        let mut boat = Boat::default();

        for direction in input.lines().map(From::from) {
            boat.move_boat(&direction);
        }

        println!("Part 1: {}", boat.location.x.abs() + boat.location.y.abs());
    }

    {
        use part2::Boat;
        let mut boat = Boat::default();

        for direction in input.lines().map(From::from) {
            boat.move_boat(&direction);
        }

        println!("Part 2: {}", boat.location.x.abs() + boat.location.y.abs());
    }
}
