use std::io::{self, prelude::*};


fn spell(n:i32) {
    for i in 1..n+1 {
        let result = format!("{} Abracadabra", i);
        println!("{}", result);
    }
}

fn main() {
    let mut input = String::new();

    io::stdin()
            .read_line(&mut input)
            .expect("Lecture stdin");

    let input_int = input.trim().parse::<i32>().expect("Conversion i32");
    spell(input_int);
    
}
