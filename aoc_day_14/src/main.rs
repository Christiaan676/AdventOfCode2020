use regex::Regex;
use std::collections::HashMap;

struct Memory {
    mem: HashMap<u32, u64>,
    set_mask: u64,
    clear_mask: u64,
}

impl Memory {
    // XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X -> 36 bits
    fn set_mask(&mut self, mask: &str) {
        // 00010000, 1 where bit needs to be set, using | operation
        self.set_mask = u64::from_str_radix(&mask.replace("X", "0"), 2).unwrap();
        // 11111011, 0 where bit needs to be cleared, using & operation
        self.clear_mask = u64::from_str_radix(&mask.replace("X", "1"), 2).unwrap();
    }

    fn write(&mut self, addres: u32, value: u64) {
        let value = value & self.clear_mask;
        let value = value | self.set_mask;
        self.mem.insert(addres, value);
    }
}
struct Memory2 {
    mem: HashMap<u64, u64>,
    index: Vec<usize>,
    set_mask: u64,
}

impl Memory2 {
    fn set_mask(&mut self, mask: &str) {
        // 00010000, 1 where bit needs to be set, using | operation
        self.set_mask = u64::from_str_radix(&mask.replace("X", "0"), 2).unwrap();

        self.index = mask
            .chars()
            .rev()
            .enumerate()
            .filter(|(_, c)| *c == 'X')
            .map(|(i, _)| i)
            .collect();
    }

    fn write(&mut self, addres: u64, value: u64) {
        let addres = addres | self.set_mask;
        for addres in calc_addres(addres, &self.index) {
            self.mem.insert(addres, value);
        }
    }
}

fn calc_addres(addres: u64, index: &[usize]) -> Vec<u64> {
    if index.is_empty() {
        return vec![addres];
    }

    let addres_set = addres | 1 << index[0];
    let addres_clear = addres & !(1 << index[0]);

    let mut result = calc_addres(addres_set, &index[1..]);
    result.append(&mut calc_addres(addres_clear, &index[1..]));
    result
}

fn main() {
    let input = include_str!("../input.txt");
    let regex = Regex::new(r"^([a-z]*)(\[([0-9]*)\])? = (.*)$").unwrap();

    let mut mem = Memory {
        mem: HashMap::new(),
        set_mask: 0,
        clear_mask: 0,
    };
    for line in input.lines() {
        let cap = regex.captures(line).unwrap();
        match &cap[1] {
            "mask" => mem.set_mask(&cap[4]),
            "mem" => mem.write(cap[3].parse().unwrap(), cap[4].parse().unwrap()),
            _ => unimplemented!(),
        }
    }

    println!("Part 1: {}", mem.mem.values().sum::<u64>());

    let mut mem = Memory2 {
        mem: HashMap::new(),
        set_mask: 0,
        index: vec![],
    };
    for line in input.lines() {
        let cap = regex.captures(line).unwrap();
        match &cap[1] {
            "mask" => mem.set_mask(&cap[4]),
            "mem" => mem.write(cap[3].parse().unwrap(), cap[4].parse().unwrap()),
            _ => unimplemented!(),
        }
    }

    println!("Part 2: {}", mem.mem.values().sum::<u64>());
}
