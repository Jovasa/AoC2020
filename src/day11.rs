use std::fs;
use itertools::Itertools;

fn count_humans(data: &Vec<String>) -> i32 {
    let mut humans = 0;
    for y in data {
        for c in y.chars() {
            if c == '#' {
                humans += 1;
            }
        }
    }
    humans
}

fn print_layout(data: &Vec<String>) {
    for y in data {
        println!("{}", y);
    }
}

fn find_first_in_direction(data: &Vec<String>, x: i32, y: i32, x_shift: i32, y_shift: i32) -> i32 {
    let mut x = x + x_shift;
    let mut y = y + y_shift;
    while y >= 0 && y < data.len() as i32 && x >= 0 && x < data[y as usize].len() as i32 {
        let temp = data[y as usize].chars().nth(x as usize).unwrap();
        if temp == 'L' {
            return 0;
        } else if temp == '#' {
            return 1;
        }
        x += x_shift;
        y += y_shift;
    }
    0
}


fn second() {
    let temp = fs::read_to_string("data/day11.txt")
        .expect("Something went wrong reading the file");
    let mut contents: Vec<String> = temp.split("\n").map(|x| x.to_string()).collect();
    let mut buffer: Vec<String> = Vec::new();

    let height = contents.len();
    let width = contents[0].len();

    let mut changed = true;

    let mut directions: Vec<Vec<i32>> = (-1..=1).permutations(2).collect();
    directions.push(vec!(1, 1));
    directions.push(vec!(-1, -1));

    while changed {
        changed = false;
        for i in 0..height {
            let mut line_buffer = "".to_string();
            for j in 0..width {
                let current = contents[i].chars().nth(j).unwrap();
                if current == '.' {
                    line_buffer += ".";
                    continue;
                }
                let humans: i32 = directions.iter().map(
                    |x| find_first_in_direction(
                        &contents,
                        j as i32,
                        i as i32,
                        x[0],
                        x[1]
                    )
                ).sum();

                let new;
                if current == 'L' {
                    new = if humans == 0 { "#" } else { "L" };
                } else {
                    new = if humans >= 5 { "L" } else { "#" };
                }
                line_buffer += new;
                if new != current.to_string() {
                    changed = true;
                }
            }
            buffer.push(line_buffer);
        }
        contents = buffer.to_vec();
        buffer = Vec::new();
    }
    println!("{}", count_humans(&contents));

}

fn first() {
    let temp = fs::read_to_string("data/day11.txt")
        .expect("Something went wrong reading the file");
    let mut contents: Vec<String> = temp.split("\n").map(|x| x.to_string()).collect();
    let mut buffer: Vec<String> = Vec::new();

    let height = contents.len();
    let width = contents[0].len();

    let mut changed = true;
    while changed {
        changed = false;
        for i in 0..height {
            let mut line_buffer = "".to_string();
            for j in 0..width {
                let current = contents[i].chars().nth(j).unwrap();
                if current == '.' {
                    line_buffer += ".";
                    continue;
                }
                let mut humans = 0;
                for y in (i as i64 - 1).max(0)..=(i as i64 + 1).min(height as i64 - 1) {
                    for x in (j as i64 - 1).max(0)..=(j as i64 + 1).min(width as i64 - 1) {
                        if x == j as i64 && y == i as i64 {
                            continue;
                        }
                        if contents[y as usize].chars().nth(x as usize).unwrap() == '#' {
                            humans += 1;
                        }
                    }
                }

                let new;
                if current == 'L' {
                    new = if humans == 0 { "#" } else { "L" };
                } else {
                    new = if humans >= 4 { "L" } else { "#" };
                }
                line_buffer += new;
                if new != current.to_string() {
                    changed = true;
                }
            }
            buffer.push(line_buffer);
        }
        println!("{}", count_humans(&buffer));
        contents = buffer.to_vec();
        buffer = Vec::new();
    }
}


fn main() {
    second();
}