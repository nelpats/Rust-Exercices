use std::{io::{self, prelude::*}};
use std::cmp;

fn describe_moose(l : i32, r : i32) -> String {

    if l == 0 && r == 0 {
        return format!("Not a moose");
    }

    if l == r {
        return format!("Even {}", l + r);
    } else {
        return format!("Odd {}", (cmp::max(l, r) * 2));
    }



}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Lecture de stdin");

    let vec_input = input.split_whitespace().map(|x| x.parse::<i32>().expect("Conversion i32")).collect::<Vec<i32>>();

    let l = vec_input[0];
    let r = vec_input[1];

    let result = describe_moose(l, r);

    println!("{}", result);



}
