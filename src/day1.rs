use crate::util::read_file_of_numbers;

mod util;

fn main() {
    first();
    second();
}

fn second() {
    let mut numbers = read_file_of_numbers("data/day1.txt");
    numbers.sort();

    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            for k in j + 1..numbers.len() {
                if numbers[i] + numbers[j] + numbers[k] == 2020 {
                    println!("{}", numbers[i] * numbers[j] * numbers[k]);
                    return;
                }
                if numbers[i] + numbers[j] + numbers[k] > 2020 {
                    break;
                }
            }
            if numbers[i] + numbers[j] > 2020 {
                break;
            }
        }
    }
}

fn first() {
    let numbers = read_file_of_numbers("data/day1.txt");

    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            if numbers[i] + numbers[j] == 2020 {
                println!("{}", numbers[i] * numbers[j]);
                return;
            }
        }
    }
}