use crate::util::read_file_of_numbers;

mod util;


fn first() {
    let mut data = read_file_of_numbers::<usize>("data/day10.txt");
    data.push(0);
    data.sort();

    let mut jolt_difs: [i32; 4] = [0, 0, 0, 1];

    for i in 1..data.len() {
        jolt_difs[(data[i] - data[i - 1])] += 1;
    }
    println!("{}", jolt_difs[1] * jolt_difs[3]);

    let mut paths_to = vec![0 as usize; data.len()];
    paths_to[0] = 1;
    for i in 1..data.len(){
        for j in (0..i).rev() {
            if data[i] - data[j] > 3 {
                break;
            }
            paths_to[i] += paths_to[j];

        }
    }
    println!("{}", paths_to.last().unwrap());
}


fn main(){
    first();
}