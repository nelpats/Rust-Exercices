use std::{io::{self, prelude::*}};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Lecture de stdin");


    let mut lines = input.lines();
    for _line in input.lines() {
        let vec_input = _line.split_whitespace().map(|x| x.parse::<i64>().expect("Conversion i32")).collect::<Vec<i64>>();
        let x1 : i64 = vec_input[0];
        let x2 : i64 = vec_input[1];

        println!("{}", (x1-x2).abs());
    }
}
