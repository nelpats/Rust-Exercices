use std::{io::{self, prelude::*}};
use std::ops;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Lecture de stdin");

    let n : i32 = input.trim().parse::<i32>().expect("Integer");
    let ticks = n as f32 / 4.0;
    let result = format!("{}",  ticks);

    println!("{}", result);
}
