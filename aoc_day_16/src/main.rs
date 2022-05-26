use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

use regex::Regex;

fn main() {
    let rules = include_str!("../rules.txt");
    let nearby_tickets = include_str!("../nearby_tickets.txt");
    let my_ticket = [
        107, 157, 197, 181, 71, 113, 179, 109, 97, 163, 73, 53, 101, 193, 173, 151, 167, 191, 127,
        103,
    ];

    let nearby_tickets: Vec<Vec<usize>> = nearby_tickets
        .lines()
        .map(|line| line.split(",").map(|v| v.parse().unwrap()).collect())
        .collect();

    let rules = Rules::new(rules.lines().map(|line| parse_rule(line)).collect());

    let total_error_rate: usize = nearby_tickets
        .iter()
        .map(|ticket| rules.error_rate(ticket))
        .sum();
    
    println!("Part1: {}", total_error_rate);

    let valid_tickets: Vec<Vec<usize>> = nearby_tickets
        .into_iter()
        .filter(|ticket| rules.valid_ticket(ticket))
        .collect();

    let mut colum_nrs: HashMap<String, HashSet<usize>>= rules.rules
        .iter()
        .map(|(key, rule)| {
            let index = (0..20)
                .filter(|i|  match_colum(&valid_tickets, *i, rule))
                .collect();
            (key.clone(), index)
        })
        .collect();

    let mut colums: HashMap<String, usize>  = HashMap::new();
    while !colum_nrs.is_empty() {
        let (key, col) = find_single_option(&colum_nrs).unwrap();
        colum_nrs.remove(&key);
        colums.insert(key, col);
        remove_from_col_options(&mut colum_nrs, col);
    }

    let part2: usize = colums.iter()
        .filter(|(k,_)| k.starts_with("departure"))
        .map(|(_,v)| my_ticket[*v])
        .product();

    println!("Part2: {}", part2);
}

fn match_colum(tickets: &Vec<Vec<usize>>, i: usize, rule: &Rule ) -> bool {
    tickets.iter().all(|t| rule.in_range(t[i]))
}

fn remove_from_col_options(colum_nrs: &mut HashMap<String, HashSet<usize>>, i: usize) {
    colum_nrs.values_mut().for_each(|options| {
        options.remove(&i);
    });
}

fn find_single_option(colum_nrs: &HashMap<String, HashSet<usize>>) -> Option<(String, usize)>{
    colum_nrs.iter()
        .find(|(_, v)| v.len() == 1)
        .map(|(k, v)| (k.clone(), *v.iter().next().unwrap()))
}

struct Rules {
    rules: HashMap<String, Rule>,
}

impl Rules {
    fn new(rules: HashMap<String, Rule>) -> Rules {
        Rules { rules }
    }

    fn error_rate(&self, ticket: &[usize]) -> usize {
        ticket.iter().filter(|i| !self.match_any_rule(**i)).sum()
    }

    fn valid_ticket(&self, ticket: &[usize]) -> bool {
        ticket.iter().all(|i| self.match_any_rule(*i))
    }

    fn match_any_rule(&self, i: usize) -> bool {
        self.rules.values().any(|r| r.in_range(i))
    }
}
struct Rule {
    ranges: [RangeInclusive<usize>; 2],
}

impl Rule {
    fn in_range(&self, i: usize) -> bool {
        self.ranges.iter().any(|r| r.contains(&i))
    }
}

fn parse_rule(line: &str) -> (String, Rule) {
    let re = Regex::new(r"^([a-z ]*): ([0-9]*)-([0-9]*) or ([0-9]*)-([0-9]*)$").unwrap();

    let cap = re.captures(line).unwrap();

    let range1 = RangeInclusive::new(cap[2].parse().unwrap(), cap[3].parse().unwrap());
    let range2 = RangeInclusive::new(cap[4].parse().unwrap(), cap[5].parse().unwrap());
    (
        cap[1].to_owned(),
        Rule {
            ranges: [range1, range2],
        },
    )
}
