use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;
use std::io::Error;


fn first() -> Option<i32> {
    let file = match File::open("data/day2.txt") {
    Ok(s) => s,
        Err(_) => panic!("File doesn't exist")
    };
    let reader = BufReader::new(file);

    let re = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)").unwrap();

    let mut first_valids = 0;
    let mut second_valids = 0;

    for line in reader.lines() {
        let (cap, lower, upper, character) = parse_row(&re, &line);

        let count = cap.matches(character).count();

        if lower <= count && count <= upper {
            first_valids += 1;
        }

        let first = cap.chars().nth(lower - 1)?;
        let second = cap.chars().nth(upper - 1)?;
        if (first == character) ^ (second == character) {
            second_valids += 1;
        }

    }
    println!("{}", first_valids);
    println!("{}", second_valids);
    Some(first_valids)
}

fn parse_row(re: &Regex, line: &Result<String, Error>) -> (String, usize, usize, char) {
    let x = match line.as_ref().ok(){
        Some(s) => s,
        None => panic!()
    };
    let cap = match re.captures(x) {
        Some(s) => s,
        None => panic!()
    };

    let lower: usize = match cap[1].parse() {
        Ok(o) => o,
        Err(_) => 0
    };
    let upper: usize = match cap[2].parse() {
        Ok(o) => o,
        Err(_) => 0
    };

    let character: char = match cap[3].chars().nth(0) {
        Some(c) => c,
        None => ' '
    };
    (cap[4].parse().unwrap(), lower, upper, character)
}

fn main() {
    first();
}