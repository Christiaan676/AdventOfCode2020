use itertools::*;

fn main() {
    let input = include_str!("../input.txt");

    let input: Vec<u64> = input.lines().map(|l| l.parse().unwrap()).collect();

    let index = (25..)
        .find(|&i| !pair_match(&input[i - 25..i], input[i]))
        .unwrap();

    let part1_result = input[index];
    println!("Part 1: {}", part1_result);


    for start in 0.. {
        if let Some(section) = check_range(&input[start..], part1_result) {
            let min = section.iter().min().unwrap();
            let max = section.iter().max().unwrap();
            println!("Part 2: {} + {} = {}", min, max, min + max);
            break;
        }
    };
}

fn check_range(section: &[u64], value: u64) -> Option<&[u64]> {
    let mut sum = section[0];
    for i in 1.. {
        sum += section[i];
        if sum == value { // Found the solution!
            return Some(&section[0..=i]);
        } else if sum > value { // Sum to large, stop
            return None;
        }
    }
    unreachable!();
}

fn pair_match(section: &[u64], value: u64) -> bool {
    section
        .iter()
        .permutations(2)
        .any(|p| p.into_iter().sum::<u64>() == value)
}
