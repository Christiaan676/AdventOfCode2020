#![feature(iterator_fold_self)]

use std::char;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");

    let awnsers = input
        .split("\n\n")
        .map(|p| {
            p.chars()
                .filter(char::is_ascii_alphabetic)
                .collect::<HashSet<char>>()
        })
        .map(|p| p.len())
        .sum::<usize>();

    println!("Part 1; Nr: {}", awnsers);

    let awnsers = input
        .split("\n\n")
        .map(|p| {
            p.lines()
                .map(|l| l.chars().collect::<HashSet<char>>())
                .fold_first(|mut a, b| {
                    a.retain(|e| b.contains(e));
                    a
                })
                .unwrap()
        })
        .map(|p| p.len())
        .sum::<usize>();

    println!("Part 2; Nr: {}", awnsers);
}
