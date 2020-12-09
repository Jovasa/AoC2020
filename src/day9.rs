
use crate::util::read_file_of_numbers;

mod util;



fn first() {
    let numbers = read_file_of_numbers::<u64>("data/day9.txt");

    let invalid = find_invalid(&numbers);
    println!("{}", invalid);

    for i in 0..numbers.len() {
        let mut acc = numbers[i];
        let mut j = i + 1;
        while acc < invalid {
            acc += numbers[j];
            j += 1;
        }
        if acc == invalid {
            println!("{}", numbers[i..j+1].iter().min().unwrap() + numbers[i..j+1].iter().max().unwrap());
            break;
        }
    }
}

fn find_invalid(numbers: &Vec<u64>) -> u64{
    let mut ring_buffer: [u64; 25] = [0; 25];

    for i in 0..25 {
        ring_buffer[i] = numbers[i];
    }

    for i in 25..numbers.len() {
        let num = numbers[i];
        if in_ring_buffer(ring_buffer, num) {
            ring_buffer[i % 25] = num;
        } else {
            return num;
        }
    }
    panic!("Didn't find number")
}

fn in_ring_buffer(ring_buffer:[u64; 25], num: u64) -> bool {
    for j in 0..25 {
        for k in j..25 {
            if ring_buffer[j] + ring_buffer[k] == num {
                return true;
            }
        }
    }
    false
}

fn main() {
    first();
}