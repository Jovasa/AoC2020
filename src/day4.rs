use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;

fn first() -> Option<i32> {
    let file = match File::open("data/day4.txt") {
        Ok(s) => s,
        Err(_) => panic!("File doesn't exist")
    };
    let reader = BufReader::new(file);
    let re = Regex::new(r"(\w+):([\w#]\w*)").unwrap();
    let hex_re = Regex::new(r"#[0-9a-f]{6}").unwrap();
    let pid_re = Regex::new(r"[0-9]{9}$").unwrap();

    let mut valids = 0;

    let mut fields = 0;


    for line in reader.lines() {
        let lin = &line.ok()?;
        if lin.is_empty() {
            if fields == 7 {
                valids += 1;
            }

            fields = 0;
            continue;
        }
        for cap in re.captures_iter(lin) {
            match &cap[1] {
                "cid" => continue,
                "byr" => match cap[2].parse::<i32>() {
                    Ok(t) => if 1920 <= t && t <= 2002 {
                        fields += 1;
                    },
                    Err(_) => panic!()
                },
                "iyr" => match cap[2].parse::<i32>() {
                    Ok(t) => if 2010 <= t && t <= 2020 {
                        fields += 1;
                    },
                    Err(_) => panic!()
                },
                "eyr" => match cap[2].parse::<i32>() {
                    Ok(t) => if 2020 <= t && t <= 2030 {
                        fields += 1;
                    },
                    Err(_) => panic!()
                },
                "hgt" => if cap[2].ends_with("in") {
                    match cap[2][..cap[2].len() - 2].parse::<i32>() {
                        Ok(t) => if 59 <= t && t <= 76 {
                            fields += 1;
                        },
                        Err(_) => panic!()
                    }
                } else if cap[2].ends_with("cm") {
                    match cap[2][..cap[2].len() - 2].parse::<i32>() {
                        Ok(t) => if 150 <= t && t <= 193 {
                            fields += 1;
                        },
                        Err(_) => panic!()
                    }
                },
                "hcl" => if hex_re.is_match(&cap[2]) {
                    fields += 1;
                },
                "ecl" => if ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&&cap[2]) {
                    fields += 1;
                },
                "pid" => {
                    if pid_re.is_match(&cap[2]) {
                        fields += 1;
                    }
                }
                _ => (),
            }
        }
    }
    println!("{}", valids);
    Some(0)
}

fn main() {
    first();
}