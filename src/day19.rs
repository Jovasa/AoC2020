use std::fs;
use std::collections::HashMap;

struct Rule {
    first: Option<u32>,
    second: Option<u32>,
    char: Option<char>,
}

fn check(current: u32, data: &HashMap<u32, Vec<Rule>>, string: &str, start: usize) -> (bool, usize) {
    for rule in &data[&current] {
        if rule.char != None {
            return (rule.char == string.chars().nth(start), 1);
        }
        let (first, length) = check(rule.first.unwrap(), data, string, start);
        if !first {
            continue;
        }
        let (second, length2) = match rule.second {
            None => (first, 0),
            Some(i) => check(i, data, string, start + length)
        };
        if first && second {
            return (true, length + length2);
        }

    }
    (false, 0)
}

fn first() {
    let temp = fs::read_to_string("data/day19.txt")
        .expect("Something went wrong reading the file");
    let data: Vec<&str> = temp.split("\n\n").collect();

    let mut rules = HashMap::new();

    for rule in data[0].split("\n") {
        let d: Vec<&str> = rule.split(":").collect();

        let mut temp = Vec::new();
        let index = d[0].parse::<u32>().unwrap();
        for item in d[1].trim().split("|") {
            let t: Vec<&str> = item.trim().split(" ").collect();
            let (first, char) = match t[0].parse::<u32>() {
                Ok(i) => (Some(i), None),
                Err(_) => (None, Some(t[0].chars().nth(1).unwrap()))
            };
            let second = match t.len() {
                1 => None,
                2 => Some(t[1].parse::<u32>().unwrap()),
                _ => panic!()
            };
            temp.push(Rule { first, second, char });
        }

        rules.insert(index, temp);
    }

    let mut count = 0;
    for line in data[1].trim().split("\n") {
        let (found, length) = check(0, &rules, line, 0);
        println!("{} {} {}", line, length, found);
        if found && length == line.len() {
            count += 1;
        }
    }
    println!("{}", count);
}


fn main() {
    first();
}