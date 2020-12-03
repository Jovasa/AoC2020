use std::fs::File;
use std::io::{BufReader, BufRead};

fn first(slope: usize, nth_row: i32) -> usize {
    let file = match File::open("data/day3.txt") {
        Ok(s) => s,
        Err(_) => panic!("File doesn't exist")
    };
    let reader = BufReader::new(file);

    let mut trees = 0;
    let mut offset = 0;

    let mut row = -1;

    for line in reader.lines() {
        row += 1;
        if row % nth_row != 0 {
            continue;
        }

        match line {
            Ok(s) => match s.chars().nth(offset % s.len()) {
                Some(c) => if c == '#' {
                    trees += 1;
                },
                None => ()
            },
            Err(_) => panic!()
        }
        offset += slope;
    }
    println!("{}", trees);
    trees
}

fn main() {

    let mut result = 1;

    for i in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter() {
        let (slope, row) = i;
        result *= first(*slope, *row);
    }
    println!("{}", result);
}