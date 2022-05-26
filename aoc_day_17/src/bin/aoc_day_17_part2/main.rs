mod point;

use point::Point;
use std::{fmt, fmt::Display, fmt::Formatter};

use itertools::Itertools;


fn main() {
    let input = include_str!("../../../input.txt");

    let mut board = Board::new(8 + 20, 8 + 20, 20, 20);

    let offset = Point::new(10, 10, 10, 10);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                board.set(offset + Point::new(x as isize, y as isize, 0, 0));
            }
        }
    }

    //println!("{}", board);

    for _ in 0..6 {
        board = board.time_step();
        //println!("{}", board);
    }
    println!("Part 2: {}", board.calc_alive());
}

struct Board {
    cells: Vec<Vec<Vec<Vec<bool>>>>,
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (w, demention) in self.cells.iter().enumerate() {
            for (z, field) in demention.iter().enumerate() {
                writeln!(f, "Field: (w={}, z{})",w, z)?;
                for line in field.iter() {
                    writeln!(
                        f,
                        "{}",
                        line.iter().map(|v| if *v { '#' } else { '.' }).join("")
                    )?;
                }
            }
        }
        Ok(())
    }
}

impl Board {
    fn new(x: usize, y: usize, z: usize, w: usize) -> Board {
        Board {
            cells: vec![vec![vec![vec![false; x]; y]; z]; w],
        }
    }

    fn get(&self, point: Point) -> Option<bool> {
        self.cells
            .get(point.w as usize)
            .and_then(|demention| demention.get(point.z as usize))
            .and_then(|plane| plane.get(point.y as usize))
            .and_then(|line| line.get(point.x as usize))
            .copied()
    }

    fn set(&mut self, point: Point) {
        self.cells[point.w as usize][point.z as usize][point.y as usize][point.x as usize] = true;
    }

    fn get_alive_adjcent_cells(&self, location: &Point) -> usize {
        let mut count = 0;
        for w in -1 as isize..=1 {
            for z in -1 as isize..=1 {
                for y in -1 as isize..=1 {
                    for x in -1 as isize..=1 {
                        let offset = Point::new(x, y, z, w);
                        if !offset.is_origin() && self.get(offset + *location).unwrap_or(false) {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }

    fn time_step(&self) -> Board {
        let cells = self
            .cells
            .iter()
            .enumerate()
            .map(|(w, demention)| {
                demention
                    .iter()
                    .enumerate()
                    .map(|(z, plane)| {
                        plane
                            .iter()
                            .enumerate()
                            .map(|(y, line)| {
                                line.iter()
                                    .enumerate()
                                    .map(|(x, value)| {
                                        self.update_cel(
                                            *value,
                                            Point::new(
                                                x as isize, y as isize, z as isize, w as isize,
                                            ),
                                        )
                                    })
                                    .collect()
                            })
                            .collect()
                    })
                    .collect()
            })
            .collect();

        Board { cells }
    }

    fn update_cel(&self, value: bool, location: Point) -> bool {
        let nr_alive_neighbors = self.get_alive_adjcent_cells(&location);
        match (value, nr_alive_neighbors) {
            (true, 2) | (true, 3) => true,
            (true, _) => false,
            (false, 3) => true, // If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active.
            (false, _) => false,
        }
    }

    fn calc_alive(&self) -> usize {
        self.cells
            .iter()
            .flat_map(|demention| demention.iter())
            .flat_map(|plane| plane.iter())
            .flat_map(|line| line.iter())
            .filter(|cel| **cel)
            .count()
    }
}
