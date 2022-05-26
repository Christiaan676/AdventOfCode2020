use std::{iter::successors, ops::Add};

#[derive(Debug, Copy, Clone, PartialEq)]
enum Square {
    TREE,
    OPEN,
}

impl Square {
    fn parse(c: char) -> Square {
        match c {
            '.' => Square::OPEN,
            '#' => Square::TREE,
            _ => unimplemented!("Unsupported char!"),
        }
    }
}

struct Field {
    squares: Vec<Vec<Square>>,
    x_size: usize,
}

impl Field {
    fn parse(input: &str) -> Field {
        let squares: Vec<Vec<Square>> = input
            .lines()
            .map(|l| l.chars().map(Square::parse).collect())
            .collect();

        let x_size = squares[0].len();
        Field { squares, x_size }
    }

    fn check_tree(&self, point: &Point) -> bool {
        self.squares[point.y][point.x % self.x_size] == Square::TREE
    }

    fn on_field(&self, point: &Point) -> bool {
        point.y < self.squares.len()
    }

    fn nr_trees(&self, &slope: &Point) -> usize {
        successors(Some(Point{x: 0, y: 0}), |&i| Some(i + slope))
            .take_while(|p| self.on_field(p))
            .filter(|p| self.check_tree(p))
            .count()
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: usize,
    y: usize,
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

fn main() {
    let input = include_str!("../input.txt");
    let field = Field::parse(input);

    let nr_trees = field.nr_trees(&Point { x: 3, y: 1 });
    println!("Part1 - Trees: {}", nr_trees);

    let slopes = [Point { x: 1, y: 1 }, Point { x: 3, y: 1 }, Point { x: 5, y: 1 }, Point { x: 7, y: 1 }, Point { x: 1, y: 2 }];
    let part2 = slopes.iter().map(|s| field.nr_trees(s)).product::<usize>();

    println!("Part2 - Trees: {}", part2);
}
