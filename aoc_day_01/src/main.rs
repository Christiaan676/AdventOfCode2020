use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

fn main() {
    match find_2020(2) {
        Some(r) => println!("Result 1: {:?}", r),
        None => print!("Result 1: no result found?"),
    }

    match find_2020(3) {
        Some(r) => println!("Result 2: {:?}", r),
        None => print!("Result 2: no result found?"),
    }
}

fn find_2020(pair_size: usize) -> Option<u32> {
    read_lines("./input1.txt")
        .unwrap()
        .map(|s| s.unwrap().parse::<u32>().unwrap())
        .permutations(pair_size)
        .find(|a| a.iter().sum::<u32>() == 2020)
        .map(|a| a.iter().product::<u32>())
}

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
