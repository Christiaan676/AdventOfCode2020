#![feature(str_split_once)]

use std::collections::HashMap;

use regex::Regex;

fn main() {
    let rules = include_str!("../rules.txt");
    let messages = include_str!("../tiles.txt");

    let mut regex = parse_rules(&rules);
    regex.insert(0, '^');
    regex.push('$');

    //println!("Regex: {}", regex);

    let regex = Regex::new(&regex).unwrap();
    let ok_msg = messages.lines().filter(|msg| regex.is_match(msg)).count();

    println!("Part1: {}", ok_msg);

    // Replace
    // 8: 42 | 42 8
    // 11: 42 31 | 42 11 31

}

fn parse_rules(input: &str) -> String {
    let input_map = input
        .lines()
        .map(|line| {
            let (rule_nr, rule) = line.split_once(": ").unwrap();
            (rule_nr.parse().unwrap(), rule)
        })
        .collect();

    RuleBuilder{input_map}.build_rule(0)
}


struct RuleBuilder<'a> {
    input_map: HashMap<u16, &'a str>
}

impl <'a> RuleBuilder<'a> {
    fn build_rule(&self, rule: u16) -> String {
        let rule = *self.input_map.get(&rule).unwrap();

        if rule.contains('|') {
            let (a, b) = rule.split_once(" | ").unwrap();
            self.build_or(a, b)
        } else if rule == r#""a""# {
            "a".into()
        } else if rule == r#""b""# {
            "b".into()
        } else {
            self.build_rules(rule)
        }
    }

    fn build_or(&self, a :&str, b :&str) -> String {
        format!("({}|{})", self.build_rules(a), self.build_rules(b))
    }

    fn build_rules(&self, input: &str) -> String {
        input.split(' ').map(|r| self.build_rule(r.parse().unwrap())).collect()
    } 
}