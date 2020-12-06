use std::fs;

fn first() {
     let temp = fs::read_to_string("data/day6.txt")
        .expect("Something went wrong reading the file");
    let contents: Vec<&str> = temp.split("\n\n").collect();

    let mut questions = 0;
    for item in &contents {
        for c in 'a'..='z' {
            if item.contains(c) {
                questions += 1;
            }
        }
    }
    println!("{}", questions);

    questions = 0;
    for item in contents {
        let temp: Vec<&str> = item.split("\n").collect();
        for c in 'a'..='z' {
            let mut has_all = true;
            for l in &temp {

                if !l.contains(c) {
                    println!("{}", l);
                    has_all = false;
                    break;
                }
            }
            if has_all {
                questions += 1;
            }
        }
    }
    println!("{}", questions);
}

fn main() {
    first();
}