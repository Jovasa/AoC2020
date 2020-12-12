use std::fs::File;
use std::io::{BufReader, BufRead};

fn first() {
    let file = match File::open("data/day12.txt") {
        Ok(s) => s,
        Err(_) => panic!("File doesn't exist")
    };
    let reader = BufReader::new(file);

    let mut north = 0;
    let mut east = 0;
    let mut direction = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let command = line.chars().nth(0).unwrap();
        let value = line[1..].parse::<i32>().unwrap();

        match command {
            'N' => north += value,
            'S' => north -= value,
            'E' => east += value,
            'W' => east -= value,
            'L' => direction = (direction + value) % 360,
            'R' => direction = (((direction - value) % 360) + 360) % 360,
            'F' => {
                match direction {
                    0 => east += value,
                    180 => east -= value,
                    90 => north += value,
                    270 => north -= value,
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }
    println!("{}", north.abs() + east.abs());
}


fn second() {
    let file = match File::open("data/day12.txt") {
        Ok(s) => s,
        Err(_) => panic!("File doesn't exist")
    };
    let reader = BufReader::new(file);

    let mut north = 0;
    let mut east = 0;
    let mut direction = (10, 1, 0, 0);

    for line in reader.lines() {
        let line = line.unwrap();
        let command = line.chars().nth(0).unwrap();
        let value = line[1..].parse::<i32>().unwrap();

        match command {
            'N' => direction.1 += value,
            'S' => direction.3 += value,
            'E' => direction.0 += value,
            'W' => direction.2 += value,
            'L' | 'R' => {
                let temp;
                let v: i32 = if command == 'R' { 360 - value } else { value };
                match v {
                    90 => temp = (direction.3, direction.0, direction.1, direction.2),
                    180 => temp = (direction.2, direction.3, direction.0, direction.1),
                    270 => temp = (direction.1, direction.2, direction.3, direction.0),
                    _ => panic!(),
                }
                direction = temp;
            }
            'F' => {
                east += direction.0 * value;
                north += direction.1 * value;
                east -= direction.2 * value;
                north -= direction.3 * value;
            }
            _ => panic!(),
        }

    }
    println!("{}", north.abs() + east.abs());
}


fn main() {
    first();
    second();
}