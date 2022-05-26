use std::cmp::{max, min};

mod point;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TILE {
    Floor,        // .
    EmptySeat,    // L
    OccupiedSeat, // #
}

impl From<char> for TILE {
    fn from(c: char) -> Self {
        match c {
            '.' => TILE::Floor,
            'L' => TILE::EmptySeat,
            '#' => TILE::OccupiedSeat,
            _ => unimplemented!(),
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let board: Vec<Vec<TILE>> = input
        .lines()
        .map(|l| l.chars().map(TILE::from).collect())
        .collect();

    {
        use part1::Board;

        let mut board = Board::new(board.clone());
        let mut next_board = board.step();
        while board != next_board {
            board = next_board;
            next_board = board.step();
        }
        println!("Part 1: {}", board.calc_occupied());
    }

    {
        use part2::Board;

        let mut board = Board::new(board);
        let mut next_board = board.step();
        while board != next_board {
            board = next_board;
            next_board = board.step();
        }
        println!("Part 2: {}", board.calc_occupied());
    }
}

mod part1 {
    use super::*;
    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Board {
        board: Vec<Vec<TILE>>,
    }

    impl Board {
        pub fn new(board: Vec<Vec<TILE>>) -> Board {
            Board { board }
        }

        pub fn step(&self) -> Board {
            let new_board = self
                .board
                .iter()
                .enumerate()
                .map(|(y, l)| {
                    (0..)
                        .zip(l.iter())
                        .map(|(x, t)| {
                            let nr = self.get_occupied_adjacent_seat(x, y);
                            match (t, nr) {
                                (TILE::EmptySeat, 0) => TILE::OccupiedSeat,
                                (TILE::OccupiedSeat, nr) if nr >= 4 => TILE::EmptySeat,
                                _ => *t,
                            }
                        })
                        .collect()
                })
                .collect();

            Board { board: new_board }
        }

        fn get_occupied_adjacent_seat(&self, x: usize, y: usize) -> u8 {
            let y_start = max(0, y as isize - 1) as usize;
            let y_end = min(self.board.len(), y + 2);

            let x_start = max(0, x as isize - 1) as usize;
            let x_end = min(self.board[0].len(), x + 2);

            let correct = if self.board[y][x] == TILE::OccupiedSeat {
                1
            } else {
                0
            };

            self.board[y_start..y_end]
                .iter()
                .flat_map(|r| r[x_start..x_end].iter())
                .filter(|&&t| t == TILE::OccupiedSeat)
                .count() as u8
                - correct
        }

        pub fn calc_occupied(&self) -> usize {
            self.board
                .iter()
                .flat_map(|l| l.iter())
                .filter(|&&t| t == TILE::OccupiedSeat)
                .count()
        }
    }
}

mod part2 {
    use super::*;
    use point::Point;

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Board {
        board: Vec<Vec<TILE>>,
    }

    impl Board {
        pub fn new(board: Vec<Vec<TILE>>) -> Board {
            Board { board }
        }

        pub fn step(&self) -> Board {
            let new_board = self
                .board
                .iter()
                .enumerate()
                .map(|(y, l)| {
                    (0..)
                        .zip(l.iter())
                        .map(|(x, t)| {
                            let nr = self.get_occupied_adjacent_seat(x, y);
                            match (t, nr) {
                                (TILE::EmptySeat, 0) => TILE::OccupiedSeat,
                                (TILE::OccupiedSeat, nr) if nr >= 5 => TILE::EmptySeat,
                                _ => *t,
                            }
                        })
                        .collect()
                })
                .collect();

            Board { board: new_board }
        }

        fn get_occupied_adjacent_seat(&self, x: usize, y: usize) -> u8 {
            let origin = Point::new(x as isize, y as isize);
            (-1 as isize..=1)
                .flat_map(|y| (-1 as isize..=1).map(move |x| Point::new(x,y)))
                .filter(|p| p != &Point::default())
                .filter(|direction| self.is_occupied(origin, direction))
                .count() as u8
        }

        fn is_occupied(&self, origin: Point, direction: &Point) -> bool {
            let mut current = origin;
            loop {
                current += *direction;
                break match self.get_cell(current) {
                    Some(TILE::OccupiedSeat) => true,
                    Some(TILE::EmptySeat) => false,
                    Some(TILE::Floor) => continue,
                    None => false,
                };
            }
        }

        fn get_cell(&self, cell: Point) -> Option<TILE> {
            self.board
                .get(cell.y as usize)
                .and_then(|line| line.get(cell.x as usize))
                .copied()
        }

        pub fn calc_occupied(&self) -> usize {
            self.board
                .iter()
                .flat_map(|l| l.iter())
                .filter(|&&t| t == TILE::OccupiedSeat)
                .count()
        }
    }
}
