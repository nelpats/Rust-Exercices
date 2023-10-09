use std::collections::{HashSet};
use std::io::{self, prelude::*};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Lecture de stdin");

    let mut lines = input.lines();
    lines.next();

    let mut free_food: HashSet<i32> = HashSet::new();

    for line in lines {
        let event_dates = line.split_whitespace().map(|x| x.parse::<i32>().expect("Conversion i32")).collect::<Vec<i32>>();
        
        for i in event_dates[0]..event_dates[1]+1 {
            free_food.insert(i);
        }

    }
    println!("{}", free_food.len());
}