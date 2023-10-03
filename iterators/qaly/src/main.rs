use std::io::{self, prelude::*};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Lecture de stdin");
    let mut lines = input.lines();
    lines.next();

    let mut total_sum : f32 = 0.0f32;

    for line in lines {
        let vec_input: Vec<f32> = line.split_whitespace().map(|x| x.parse::<f32>().expect("Conversion f32")).collect::<Vec<f32>>();
        
        total_sum += vec_input[0] * vec_input[1]
        
    }

    println!("{:.3}", total_sum);


}