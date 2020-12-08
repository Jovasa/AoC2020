use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn first(){
    let file = match File::open("data/day8.txt") {
        Ok(s) => s,
        Err(_) => panic!("File doesn't exist")
    };
    let reader = BufReader::new(file);

    let mut ops: Vec<(String, i32)> = Vec::new();

    for line in reader.lines() {
        let temp = line.unwrap();
        let data: Vec<&str> = temp.split(" ").collect();

        let value: i32 = match data[1].chars().nth(0).unwrap() {
            '+' => data[1][1..].parse::<i32>().unwrap(),
            '-' => -data[1][1..].parse::<i32>().unwrap(),
            _ => panic!()
        };
        ops.push((data[0].to_string(), value));
    }

    let (value, _, used_codes) = run_code(&mut ops);
    println!("{}", value);
    for index in used_codes{
        let index = index as usize;
        let (value, code) = match ops[index].0.as_ref() {
            "nop" => {
                let mut ops_ = ops.to_vec();
                ops_[index].0 = "jmp".to_string();
                let (a, b, _) = run_code(& mut ops_);
                (a, b)
            },
            "jmp" => {
                let mut ops_ = ops.to_vec();
                ops_[index].0 = "nop".to_string();
                let (a, b, _) = run_code(& mut ops_);
                (a, b)
            },
            _ => continue,
        };
        if code as usize >= ops.len() {
            println!("{}", value);
            break;
        }
    }
}

fn run_code(ops: &mut Vec<(String, i32)>) -> (i32, i32, HashSet<i32>) {
    let mut used_codes = HashSet::new();
    let mut proc_counter: i32 = 0;
    let mut value = 0;
    while !used_codes.contains(&proc_counter) && (proc_counter as usize) < ops.len() {
        used_codes.insert(proc_counter);
        let (op, param) = &ops[proc_counter as usize];
        match op.as_ref() {
            "nop" => proc_counter += 1,
            "jmp" => proc_counter += param,
            "acc" => {
                value += param;
                proc_counter += 1;
            },
            _ => panic!()
        }
        if proc_counter < 0 {
            panic!("Negative program counter");
        }
    }
    (value, proc_counter, used_codes)
}

fn main() {
    first();
}