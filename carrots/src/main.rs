use std::{io::{self, prelude::*}, vec, fmt::format};


fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Lecture de stdin");

    let mut lines = input.lines();
    let vec_input = lines.next().expect("ligne attendue").split_whitespace().map(|x| x.parse::<i32>().expect("Conversion i32")).collect::<Vec<i32>>();
    
    let result = format!("{}", vec_input[1]);

    println!("{}", result);


        
}