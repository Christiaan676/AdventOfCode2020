use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt");

    let mut input: Vec<u32> = input.lines().map(|l| l.parse::<u32>().unwrap()).collect();
    input.push(0); // Start value
    input.sort();

    // Take window of 2 and calulate all the number of 1 and 3 diffs
    let (diff_1, diff_3): (u32, u32) = input.windows(2).fold((0, 0), |(diff_1, diff_3), windo| {
        match windo[1] - windo[0] {
            1 => (diff_1 + 1, diff_3),
            3 => (diff_1, diff_3 + 1),
            _ => (diff_1, diff_3),
        }
    });

    // Add one to the 3 diff for the last adapter
    println!("Part 1: {}", diff_1 * (diff_3 + 1));

    let last_value = input.last().unwrap() + 3;
    input.push(last_value);
    let input = input.into_iter().collect();

    let mut context = Context {
        result_cache: HashMap::new(),
        input,
    };
    let distin_arangements = context.find_distinct_arangement(0, last_value);

    println!("Part 2: {}", distin_arangements);
}

struct Context {
    result_cache: HashMap<u32, usize>,
    input: HashSet<u32>,
}

impl Context {
    fn find_distinct_arangement(&mut self, start: u32, end: u32) -> usize {
        if start == end {
            return 1;
        }
        let next = (1..=3)
            .map(|f| f + start)
            .filter(|i| self.input.contains(i))
            .collect::<Vec<u32>>();

        next.into_iter()
            .map(|v| {
                if let Some(nr) = self.result_cache.get(&v) {
                    *nr
                } else {
                    let result = self.find_distinct_arangement(v, end);
                    self.result_cache.insert(v, result);
                    result
                }
            })
            .sum::<usize>()
    }
}
