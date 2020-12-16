#![feature(iter_advance_by)]

use std::fs;
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn first () {
    let temp = fs::read_to_string("data/day16.txt")
        .expect("Something went wrong reading the file");
    let data: Vec<&str> = temp.split("\n\n").collect();

    let mut ranges = Vec::new();
    let mut fields = HashMap::new();
    let mask_re = Regex::new(r"(.+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();

    for l in data[0].split("\n").enumerate() {
        let (i, l) = l;
        let cap = mask_re.captures(l).unwrap();
        ranges.push(
            (cap[2].parse::<u32>().unwrap(), cap[3].parse::<u32>().unwrap(),)
        );
        ranges.push(
            (cap[4].parse::<u32>().unwrap(), cap[5].parse::<u32>().unwrap(),)
        );
        fields.insert(cap[1].to_string(), (2*i, 2*i + 1));
    }
    let mut k = data[2].split("\n");
    k.advance_by(1);

    let mut valid_tickets = Vec::new();
    let mut result = 0;

    for r in k {
        let mut vec = Vec::new();
        let mut valid = true;
        for n in r.split(",") {
            let n = n.parse::<u32>().unwrap();
            let mut found = false;
            vec.push(n);
            for range in ranges.iter() {
                let (low, high) = range;
                if low <= &n && n <= *high {
                    found = true;
                    break
                }
            }
            if !found {
                valid = false;
                result += n;
            }
        }
        if valid {
            valid_tickets.push(vec);
        }
    }
    println!("{}", result);

    let mut valid_fields: Vec<HashSet<&String>> = vec![fields.keys().collect(); fields.len()];

    for ticket in valid_tickets{
        for t in ticket.iter().enumerate(){
            let (i, value) = t;
            let mut tbr = Vec::new();
            for f in fields.keys() {
                let (a, b) = fields[f];
                if !((&ranges[a].0 <= value && value <= &ranges[a].1) ||
                    (&ranges[b].0 <= value  && value <= &ranges[b].1)) {
                    tbr.push(f);
                }
            }
            for f in tbr {
                valid_fields[i].remove(f);
            }
        }
    }
    let mut order = Vec::new();
    for k in &valid_fields {
        order.push(k.len());
    }
    valid_fields.sort_by(|a, b| a.len().cmp(&b.len()));
    let mut order_of_fields = vec![""; fields.len()];

    for l in &valid_fields {
        for i in l {
            if !order_of_fields.contains(&&***i) {
                let index = order.iter().position(|&x| x == l.len()).unwrap();
                order_of_fields[index] = i;
            }
        }
    }

    let mine: Vec<u32> = data[1]
        .split("\n").last().unwrap()
        .split(",").map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut res2: usize = 1;
    for i in 0..mine.len() {
        if order_of_fields[i].contains("departure") {
            res2 *= mine[i] as usize;
        }
    }
    println!("{}", res2);
}


fn main() {
    first();
}