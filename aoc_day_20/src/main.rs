
#![feature(min_const_generics)]

mod point;
mod matrix;

use std::{
    collections::{HashMap, HashSet},
    str,
    {
        fmt,
        fmt::{Display, Formatter},
    },
};

use std::rc::Rc;

use matrix::Matrix;
use point::Point;

fn main() {
    let input = include_str!("../input.txt");
    let tiles: Vec<Tile> = input.split("\n\n").map(Tile::from).collect();

    let mut board = Board::new();
    board.insert_tiles(tiles);

    println!("Part1: {}", board.get_corners_product())
}

static BORDER_OFFSETS: [Point; 4] = [
    Point::new(0, -1),
    Point::new(1, 0),
    Point::new(0, 1),
    Point::new(-1, 0),
];

struct Board {
    cells: HashMap<Point, Rc<Tile>>,
    open_cells: HashSet<Point>, // Open cells that need evaluation
}

struct OpenSpot {
    neigbours: Vec<Option<Rc<Tile>>>,
}

impl Board {
    fn new() -> Board {
        Board {
            cells: HashMap::new(),
            open_cells: HashSet::new(),
        }
    }

    fn insert_tiles(&mut self, mut tiles: Vec<Tile>) {
        let tiles = &mut tiles;
        // place the starting tile
        self.insert(
            Point::default(),
            tiles.pop().unwrap(),
        );

        while !tiles.is_empty() {
            let (point, open_spot) = self.get_open_spot();

            let result = tiles.iter().position(|tile| {
                if let Some(tile) = open_spot.fits_tile(tile) {
                    println!("Found tile: {:?} {:?}", point, tile.id);
                    self.insert(point, tile);
                    true
                } else {
                    false
                }
            });
            if let Some(index) = result {
                tiles.remove(index);
            }
        }
    }

    fn insert(&mut self, point: Point, tile: Tile) {
        let new_open_cells: Vec<Point> = BORDER_OFFSETS
            .iter()
            .map(|offset| offset + &point)
            .filter(|point| !self.cells.contains_key(point))
            .collect();
        self.open_cells.extend(new_open_cells.into_iter());
        self.cells.insert(point, Rc::new(tile));
    }

    fn get_open_spot(&mut self) -> (Point, OpenSpot) {
        let open_spot = self.open_cells.iter().next().cloned().unwrap();
        self.open_cells.remove(&open_spot);

        let neigbours = BORDER_OFFSETS
            .iter()
            .map(|offset| self.cells.get(&(open_spot + *offset)).cloned())
            .collect();
        (open_spot, OpenSpot { neigbours })
    }

    fn get_corners_product(&self) -> usize {
        let min_x = self.cells.keys().map(|p| p.x).min().unwrap();
        let max_x = self.cells.keys().map(|p| p.x).max().unwrap();
        
        let min_y = self.cells.keys().map(|p| p.y).min().unwrap();
        let max_y = self.cells.keys().map(|p| p.y).max().unwrap();

        self.cells.get(&Point::new(min_x, min_y)).unwrap().id
        * self.cells.get(&Point::new(min_x, max_y)).unwrap().id
        * self.cells.get(&Point::new(max_x, min_y)).unwrap().id
        * self.cells.get(&Point::new(max_x, max_y)).unwrap().id
    }

}

impl OpenSpot {
    fn fits_tile(&self, tile: &Tile) -> Option<Tile> {
        let mut flipped = tile.clone();
        flipped.tile_data.flip_horizontal();
        self.fits(tile).or_else(|| self.fits(&flipped))
    }

    fn fits(&self, tile: &Tile) -> Option<Tile> {
        let mut tile = tile.clone();
        for _ in 0..4 {
            let fits = self.neigbours.iter().enumerate().all(|(i, n)| {
                n.as_ref().map(|adjecent_tile| {
                    let a = get_side(&tile.tile_data, i);
                    let mut b = get_side(&adjecent_tile.tile_data, i+2 % 4);
                    b.reverse();
                    a == b
                })
                .unwrap_or(true)
            });
            if fits {
                return Some(tile);
            }
            tile.tile_data.rotate_l90();
        }
        None
    }
}

fn get_side(data: &Matrix<u8, 10, 10>, side: usize) -> Vec<u8> {
    //let side = (4 - self.rotation + side as u8) % 4;
    match side {
        0 => data.row(0).iter().cloned().collect(),
        1 => data.colum(9).iter().cloned().collect(),
        2 => data.row(9).iter().rev().cloned().collect(),
        3 => data.colum(0).iter().cloned().rev().collect(),
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone)]
struct Tile {
    id: usize,
    tile_data: Matrix<u8, 10, 10>,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Tile: {}", self.id)?;
        self.tile_data.fmt(f)
    }
}

impl From<&str> for Tile {
    fn from(input: &str) -> Self {
        // The is always of the form:
        // "Tile 3391:"
        //  0123456789
        let id = input.lines().next().unwrap()[5..9].parse().unwrap();
        let tile_data: [[u8; 10]; 10];

        for (line, in_line) in tile_data.iter_mut().zip(input.lines().skip(1)) {
            for (cell, in_char) in line.iter_mut().zip(in_line.bytes()) {
                *cell = in_char;
            }
        }

        Tile {id, tile_data: Matrix::from(tile_data)}
    }
}
