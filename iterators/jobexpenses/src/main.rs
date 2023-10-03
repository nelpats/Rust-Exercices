use std::io::{self, prelude::*};

fn get_expenses(input: Vec<i32>) -> i32 {
    let negative_expenses = input.iter().filter(|x| **x < 0);

    return negative_expenses.sum::<i32>().abs();
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Lecture de stdin");
    let mut lines = input.lines();
    lines.next();
    
    let vec_input: Vec<i32> = input.split_whitespace().map(|x| x.parse::<i32>().expect("Conversion i32")).collect::<Vec<i32>>();
    println!("{}", get_expenses(vec_input));

}
