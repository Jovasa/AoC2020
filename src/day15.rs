use std::collections::HashMap;

fn first(nth: usize) {
    let mut last_occurrence = HashMap::new();
    let input = vec![15,12,0,14,3,1];

    for x in input.iter().enumerate() {
        let (a, b) = x;
        last_occurrence.insert(*b as usize, a);
    }
    let mut last_number = 0;
    for i in input.len()..nth - 1 {
        let temp = last_number;
        if last_occurrence.contains_key(&last_number) {
            last_number = i - last_occurrence.get(&last_number).unwrap();
        }
        else {
            last_number = 0;
        }
        last_occurrence.insert(temp, i);
    }
    println!("{}", last_number);
}

fn main() {
    first(2020);
    first(30000000);
}