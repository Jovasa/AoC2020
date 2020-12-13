use std::fs;

fn first() {

    let temp = fs::read_to_string("data/day13.txt")
        .expect("Something went wrong reading the file");
    let contents: Vec<&str> = temp.split("\n").collect();
    let timestamp = contents[0].parse::<usize>().unwrap();

    let mut min_delay = usize::max_value();
    let mut first_bus: usize = 0;

    for bus in contents[1].split(",") {
        if bus == "x"{
            continue;
        }
        let bus = bus.parse::<usize>().unwrap();

        let temp = bus - (timestamp % bus);
        if temp < min_delay {
            min_delay = temp;
            first_bus = bus;
        }
    }
    println!("{}", first_bus * min_delay);

    let mut offset = 0;
    let mut delay = 0;
    let mut multi = 0;

    for bus in contents[1].split(",") {
        if bus == "x"{
            delay += 1;
            continue;
        }
        let bus = bus.parse::<usize>().unwrap();
        if multi == 0 {
            multi = bus;
            delay += 1;
            continue;
        }

        let mut t = bus - (offset % bus);
        while t != (delay % bus){
            offset += multi;
            t = bus - (offset % bus);
        }
        multi *= bus;
        delay += 1;
    }
    println!("{} {} {}", offset, delay, multi);
}

fn main() {
    first();
}