use std::fs::File;
use std::io::{BufReader, BufRead};

fn first() {
    let file = match File::open("data/day5.txt") {
        Ok(s) => s,
        Err(_) => panic!("File doesn't exist")
    };
    let reader = BufReader::new(file);

    let mut seat_ids: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let l = line.unwrap();
        let mut seat = 0;
        for i in 0..7 {
            match l.chars().nth(i) {
                Some('B') => seat += (64 >> i) * 8,
                _ => ()
            }
        }

        for i in 0..3 {
            match l.chars().nth(7 + i) {
                Some('R') => seat += 4 >> i,
                _ => ()
            }
        }
        seat_ids.push(seat);
    }
    seat_ids.sort();
    println!("{}", seat_ids.last().unwrap());

    for i in 1..seat_ids.len() -1 {
        if seat_ids[i - 1] == seat_ids[i] - 2 && seat_ids[i] + 1 == seat_ids[i + 1]{
            println!("{}", seat_ids[i] - 1);
            break;
        }
    }
}


fn main() {
    first();
}