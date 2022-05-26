#![feature(str_split_once)]

use std::collections::{hash_map::Entry, HashMap, HashSet, BTreeMap};
use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");

    let mut allergen_ingredient_map: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut all_ingredients: HashSet<&str> = HashSet::new();

    for line in input.lines() {
        // mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
        let (ingredients, allergen) = line.split_once(" (contains ").unwrap();

        let ingredients: HashSet<_> = ingredients.split(' ').collect();
        let allergens: Vec<_> = allergen.strip_suffix(')').unwrap().split(", ").collect();
        for i in &ingredients {
            all_ingredients.insert(*i);
        }

        for allergen in allergens {
            match allergen_ingredient_map.entry(allergen) {
                Entry::Vacant(entry) => {
                    entry.insert(ingredients.clone());
                }
                Entry::Occupied(mut entry) => entry.get_mut().retain(|a| ingredients.contains(a)),
            };
        }
    }

    let unsave: HashSet<&str> = allergen_ingredient_map.values().flat_map(|i| i.iter()).cloned().collect();
    let save: HashSet<&str> = all_ingredients.iter().cloned().filter(|i| !unsave.contains(i)).collect();

    let mut count = 0;
    for line in input.lines() {
        count += save.iter().filter(|i| line.contains(**i)).count();
    }

    println!("Part1: {}", count);

    let mut danger_list = BTreeMap::new(); // Output needs to be sorted by allergen
    while !allergen_ingredient_map.is_empty() {
        let (&allergen, ingredients) = allergen_ingredient_map.iter().find(|(_, i)| i.len() == 1).unwrap();
        let ingredient = *ingredients.iter().next().unwrap();
        allergen_ingredient_map.remove(allergen);

        for ingredents in allergen_ingredient_map.values_mut() {
            ingredents.remove(ingredient);
        }
        danger_list.insert(allergen, ingredient);
    }
    
    println!("Part 2: {}", danger_list.values().join(","));
}