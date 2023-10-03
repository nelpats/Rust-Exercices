use std::io::{self, prelude::*};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Lecture de stdin");
    let mut lines = input.lines();

    // A1-FBI
    let mut fbi_blimp_list: Vec<i32> = vec![];

    for (i, line) in lines.enumerate() {
        if line.contains("FBI") {
            fbi_blimp_list.push(i as i32 + 1);
        }
    }

    fbi_blimp_list.sort();

    if fbi_blimp_list.len() == 0 {
         println!("HE GOT AWAY!");
    } else {
        let mut solution = fbi_blimp_list.into_iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join(" ");
        println!("{}", solution);   
         
    }

    

}

