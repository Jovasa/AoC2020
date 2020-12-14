use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;
use std::collections::HashMap;
use itertools::Itertools;

fn first() {
    let file = match File::open("data/day14.txt") {
        Ok(s) => s,
        Err(_) => panic!("File doesn't exist")
    };
    let reader = BufReader::new(file);

    let mask_re = Regex::new(r"mask = (\w{36})").unwrap();
    let mem_re = Regex::new(r"mem\[(\d+)] = (\d+)").unwrap();

    let mut memory: HashMap<usize, usize> = HashMap::new();

    let mut mask: [Option<bool>; 36] = [None; 36];

    for line in reader.lines() {
        let line = line.unwrap();

        if mask_re.is_match(&line) {
            for t in mask_re.captures(&line).unwrap()[1].chars().rev().enumerate() {
                let (i, c) = t;
                mask[i] = match c {
                    'X' => None,
                    '1' => Some(true),
                    '0' => Some(false),
                    _ => panic!()
                }
            }
        }
        else if mem_re.is_match(&line) {
            let cap = mem_re.captures(&line).unwrap();
            let address = cap[1].parse::<usize>().unwrap();
            let mut value = cap[2].parse::<usize>().unwrap();

            for m in mask.iter().enumerate() {
                let (i, m) = m;
                match m {
                    None => continue,
                    Some(true) => value |= 1 << i,
                    Some(false) => value &= !(1 << i),
                }
            }
            memory.insert(address, value);
        }
    }
    let mut total = 0;
    for item in memory.values() {
        total += item;
    }
    println!("{}", total);
}


fn second() {
    let file = match File::open("data/day14.txt") {
        Ok(s) => s,
        Err(_) => panic!("File doesn't exist")
    };
    let reader = BufReader::new(file);

    let mask_re = Regex::new(r"mask = (\w{36})").unwrap();
    let mem_re = Regex::new(r"mem\[(\d+)] = (\d+)").unwrap();

    let mut memory: HashMap<u64, u64> = HashMap::new();

    let mut mask: u64 = 0;
    let mut neg_mask: u64 = 0;
    let mut flucs = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        if mask_re.is_match(&line) {
            mask = 0;
            neg_mask = 0;
            flucs = Vec::new();
            for t in mask_re.captures(&line).unwrap()[1].chars().rev().enumerate() {
                let (i, c) = t;
                match c {
                    'X' => { flucs.push(1u64 << i); neg_mask |= 1 << i },
                    '1' => mask |= 1 << i,
                    '0' => continue,
                    _ => panic!()
                }
            }
        }
        else if mem_re.is_match(&line) {
            let cap = mem_re.captures(&line).unwrap();
            let address = (cap[1].parse::<u64>().unwrap() | mask) & !neg_mask;
            let value = cap[2].parse::<u64>().unwrap();

            for i in 0..=flucs.len() {
                for t in flucs.iter().combinations(i) {
                    let temp: u64 = {
                        let mut a = 0;
                        for i in t.iter() {
                            a |= *i;
                        }
                        a
                    };
                    memory.insert(address | temp, value);
                }
            }
        }
    }
    let mut total = 0;
    for item in memory.values() {
        total += item;
    }
    println!("{}", total);
}


fn main() {
    second();
}