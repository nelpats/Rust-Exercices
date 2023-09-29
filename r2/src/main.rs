use std::{io::{self, prelude::*}};

fn find_mean(r1 : i32, s : i32) -> String {
    if r1 < s {
        return format!("{}", s + (s-r1));
    } else if r1 > s {
        return format!("{}", s - (r1-s))
    }

    return format!("{}", r1);
}


fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Lecture de stdin");

    let vec_input = input.split_whitespace().map(|x| x.parse::<i32>().expect("Conversion i32")).collect::<Vec<i32>>();
    
    let r1 = vec_input[0];
    let s = vec_input[1];

    let result = find_mean(r1, s);

    println!("{}", result);
}
