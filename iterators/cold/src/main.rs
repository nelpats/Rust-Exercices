use std::io::{self, prelude::*};


fn find_tempratures_bellow_zero(temps: Vec<i32>) -> i32 {
    let bellow_zero = temps.iter().filter(|x| **x < 0);
    return bellow_zero.count() as i32;
}


fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Lecture de stdin");
    let mut lines = input.lines();
    lines.next();
    
    let vec_input: Vec<i32> = input.split_whitespace().map(|x| x.parse::<i32>().expect("Conversion i32")).collect::<Vec<i32>>();
    println!("{}", find_tempratures_bellow_zero(vec_input));
}