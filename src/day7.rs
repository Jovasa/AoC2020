use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufRead};


// Garbage language. Please tell me how to do this

#[derive(Hash, Eq, PartialEq, Debug)]
struct Bag<'a> {
    can_contain_this: Vec<&'a Bag<'a>>,
    name: String,
}


impl Bag<'_> {
    fn new(name: String) -> Bag<'static> {
        return Bag{
            can_contain_this: Vec::new(),
            name
        };
    }

    fn add_upper(&mut self, other: Bag<'_>){
        self.can_contain_this.push(&other);
    }
}

fn first(){
    let file = match File::open("data/day7.txt") {
        Ok(s) => s,
        Err(_) => panic!("File doesn't exist")
    };
    let reader = BufReader::new(file);

    let mut bags: HashMap<String, Bag> = HashMap::new();

    for line in reader.lines() {
        let string = line.unwrap();
        let asd: Vec<&str> = string.split(" contain ").collect();

        if asd[1] == "no other bags."{
            continue;
        }

        let first = asd[0][0..&asd[0].len() - 1].to_string();

        if !bags.contains_key(&first) {
            bags.insert(first.to_string(), Bag::new(first));
        }


        for bag in asd[1].split(", ") {
            let a = bag[2..].to_string();
            if !bags.contains_key(&a) {
                bags.insert(a.to_string(), Bag::new(a));
            }
            let mut temp = bags.get_mut(&a).unwrap();
            let t = bags.get(&first).unwrap();
            temp.add_upper(t);
        }
    }

    let mut seen = HashSet::new();

    for i in &bags["shiny gold bag"].can_contain_this {
        println!("{}", i.name);
    }
}


fn main() {
    first();
}