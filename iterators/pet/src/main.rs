use std::io::{self, prelude::*};

struct Contestant {
    i : i32,
    grade_sum : i32,
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Lecture de stdin");
    let mut lines = input.lines();

    let mut winner =  Contestant {
        i: 0,
        grade_sum: 0,
    };

    for (i, line) in lines.enumerate() {
        let grades = line.split_whitespace().map(|x| x.parse::<i32>().expect("Conversion f32"));
        let sum = grades.sum::<i32>();
        if sum > winner.grade_sum {
            winner = Contestant {
                i: i as i32 + 1,
                grade_sum: sum,
            };
        }
    }

    println!("{} {}", winner.i, winner.grade_sum);
}