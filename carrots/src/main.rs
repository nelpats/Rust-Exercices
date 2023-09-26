use std::io::{self, prelude::*};


fn main() {
    let mut input = String::new();

    io::stdin()
            .read_line(&mut input)
            .expect("Lecture stdin");
    
    let vec_input = input.split_whitespace().map(|x| x.parse::<i32>().expect("Conversion i32")).collect::<Vec<i32>>();

    println!("{}", format!("{:?}", vec_input));    
}