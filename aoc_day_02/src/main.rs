use regex::Regex;

fn main() {
    let regex = Regex::new(r"^([0-9]*)-([0-9]*) ([a-z]): ([a-z]*)$").unwrap();

    let input = include_str!("../input1.txt");

    let valid_password = input.lines().filter(|s| validate(&regex, s)).count();
    println!("Valid passwords: {}", valid_password);

    let valid_password2 = input.lines().filter(|s| validate2(&regex, s)).count();
    println!("Valid passwords: {}", valid_password2);
}

fn validate(regex: &Regex, line: &str) -> bool {
    let caps = regex.captures(line).unwrap();

    let start = caps[1].parse::<usize>().unwrap();
    let end = caps[2].parse::<usize>().unwrap();
    let char = caps[3].chars().next().unwrap();
    let password = &caps[4];

    let occurances = password.chars().filter(|&c| c == char).count();
    (start..=end).contains(&occurances)
}

fn validate2(regex: &Regex, line: &str) -> bool {
    let caps = regex.captures(line).unwrap();

    let pos1 = caps[1].parse::<usize>().unwrap();
    let pos2 = caps[2].parse::<usize>().unwrap();
    let char = caps[3].chars().next().unwrap();
    let password = &caps[4];

    match (
        password.chars().nth(pos1 - 1),
        password.chars().nth(pos2 - 1),
    ) {
        (Some(a), Some(b)) => (a == char) ^ (b == char),
        _ => false,
    }
}
