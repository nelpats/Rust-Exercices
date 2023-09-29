use std::{io::{self, prelude::*}};
fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Lecture de stdin");


    let mut lines = input.lines();
    lines.next();
    for _line in lines {
        let n = _line.trim().len();
        println!("{}", n);

    }

}


