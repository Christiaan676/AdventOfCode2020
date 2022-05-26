
use std::collections::{HashMap, hash_map::Entry};

fn main() {
    println!("Part1: {}", calc(2020));
    println!("Part2: {}", calc(30_000_000));
}

fn calc(nth: usize) -> usize {
    let input = [11,18,0,20,1,7,16];
 
    // Inizalize the spoken map
    let mut spoken: HashMap<usize, usize> = HashMap::new();
    for (i, &value) in input[0..input.len()-1].iter().enumerate() {
        spoken.insert(value, i);
    }

    let mut last_value = *input.last().unwrap();
    for i in input.len()..nth {
        let new_value = match spoken.entry(last_value) {
            Entry::Vacant(_) => 0, // Value never spoken before
            Entry::Occupied(entry) => (i-1) - entry.get(),
        };
        spoken.insert(last_value, i-1);
        last_value = new_value;
    }

    last_value
}
