use std::fs::File;
use std::io::{BufReader, BufRead};

enum Operation {
    Multi,
    Add,
}

fn parse_paranthesis(a: &mut dyn Iterator<Item = char>) -> i64 {
    let mut previous = 0;
    let mut op: Option<Operation> = None;
    while let Some(c) = a.next() {
        let current  = match c {
            '(' => parse_paranthesis(a),
            ')' => return previous,
            '*' => {
                op = Some(Operation::Multi);
                continue
            },
            '+' => {
                op = Some(Operation::Add);
                continue
            },
            '0'..='9' => c as i64 - '0' as i64,
            _ => unreachable!()
        };
        match op {
            Some(Operation::Multi) => previous *= current,
            Some(Operation::Add) => previous += current,
            None => previous = current,
        }
    }
    previous
}
fn parse_paranthesis_str(a: &mut dyn Iterator<Item = String>) -> i64 {
    let mut previous = 0;
    let mut op: Option<Operation> = None;
    while let Some(c) = a.next() {
        let temp = c.as_ref();
        let current  = match temp {
            "(" => parse_paranthesis_str(&mut parse_additions(a).into_iter()),
            ")" => return previous,
            "*" => {
                op = Some(Operation::Multi);
                continue
            },
            "+" => {
                op = Some(Operation::Add);
                continue
            },
            _ => c.parse::<i64>().unwrap(),
        };
        match op {
            Some(Operation::Multi) => previous *= current,
            Some(Operation::Add) => previous += current,
            None => previous = current,
        }
    }
    previous
}

fn parse_additions(a: &mut dyn Iterator<Item = String>) -> Vec<String> {
    let mut previous = 0;
    let mut op: Option<Operation> = None;
    let mut out = Vec::new();
    while let Some(c) = a.next() {
        let temp = c.as_ref();
        let current  = match temp {
            "(" => parse_paranthesis_str(&mut parse_additions(a).into_iter()),
            ")" => {
                out.push(previous.to_string());
                return out;
            },
            "*" => {
                op = Some(Operation::Multi);
                continue
            },
            "+" => {
                op = Some(Operation::Add);
                continue
            },
            _ => c.parse::<i64>().unwrap(),
        };
        match op {
            Some(Operation::Multi) => {
                out.push(previous.to_string());
                out.push("*".to_string());
                previous = current;
            },
            Some(Operation::Add) => previous += current,
            None => previous = current,
        }
    }
    out.push(previous.to_string());
    out
}

fn main() {
    first();
    second();
}

fn first() {
    let file = match File::open("data/day18.txt") {
        Ok(s) => s,
        Err(_) => panic!("File doesn't exist")
    };
    let reader = BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut a = {
            let mut data = Vec::new();
            for c in line.chars() {
                if c == ' ' {
                    continue;
                }
                data.push(c);
            }
            data.into_iter()
        };
        sum += parse_paranthesis(&mut a);
    }
    println!("{}", sum);
}

fn second () {
    let file = match File::open("data/day18.txt") {
        Ok(s) => s,
        Err(_) => panic!("File doesn't exist")
    };
    let reader = BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut a = {
            let mut data = Vec::new();
            for c in line.chars() {
                if c == ' ' {
                    continue;
                }
                data.push(c.to_string());
            }
            data.into_iter()
        };
        sum += parse_paranthesis_str(&mut parse_additions(&mut a).into_iter());
    }
    println!("{}", sum);
}