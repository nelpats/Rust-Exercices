use std::collections::{HashSet};
use std::io::{self, prelude::*};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Lecture de stdin");
    let mut lines = input.lines();
    let _ = lines.next();
    lines.next();

    let mut cities : HashSet<String> = HashSet::new();

    for (i, line) in lines.into_iter().enumerate() {
        
        if line.trim().chars().into_iter().all(| x | x.is_numeric()) {
            println!("{:?}", cities.len());
            
            cities = HashSet::new();
        } else {
            cities.insert(line.trim().to_string());
        }

    }

    println!("{:?}", cities.len());




}
