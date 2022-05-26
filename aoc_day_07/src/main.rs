use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");

    let input = input
        .lines()
        .map(|l| parse_line(l))
        .collect::<HashMap<String, Vec<Contains>>>();

    let count = input
        .keys()
        .filter(|color| can_contain_gold(&input, color))
        .count();
    println!("Part 1: {}", count);

    println!("Part 2: {}", nr_bags_inside(&input, "shiny gold"));
}

fn nr_bags_inside(input: &HashMap<String, Vec<Contains>>, color: &str) -> usize {
    let contains = input.get(color).unwrap();
    contains
        .iter()
        .map(|c| c.nr + c.nr * nr_bags_inside(input, &c.color))
        .sum()
}

fn can_contain_gold(input: &HashMap<String, Vec<Contains>>, color: &str) -> bool {
    let contains = input.get(color).unwrap();
    contains
        .iter()
        .any(|contain| contain.color == "shiny gold" || can_contain_gold(input, &contain.color))
}

struct Contains {
    color: String,
    nr: usize,
}

fn parse_line(input: &str) -> (String, Vec<Contains>) {
    let mut split = input.splitn(2, " bags contain ");

    let outer_bag = split.next().unwrap();
    let inner_bags = split.next().unwrap();

    let inner_bags = inner_bags.trim_end_matches('.');

    let contains = if inner_bags == "no other bags" {
        vec![]
    } else {
        inner_bags
            .split(", ")
            .map(|bag| {
                let mut split = bag.splitn(2, ' ');

                let nr = split.next().unwrap().parse().unwrap();
                let color = split
                    .next()
                    .unwrap()
                    .trim_end_matches(" bags")
                    .trim_end_matches(" bag")
                    .to_string();
                Contains { color, nr }
            })
            .collect()
    };

    (outer_bag.to_string(), contains)
}
