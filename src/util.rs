use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn read_file_of_numbers(filename: &str) -> Vec<i32> {
    let file = match File::open(filename) {
        Ok(s) => s,
        Err(_) => panic!("File doesn't exist")
    };
    let reader = BufReader::new(file);

    let mut numbers: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let a = match line {
            Ok(s) => match s.parse::<i32>(){
                Ok(i) => i,
                Err(_) => panic!("Failed to convert {} to int.", s)
            },
            Err(_) => 0
        };

        numbers.push(a);
    }
    numbers
}