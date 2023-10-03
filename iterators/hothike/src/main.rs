use std::{io::{self, prelude::*}, collections::btree_map::VacantEntry};
use std::cmp::max;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Lecture de stdin");
    let mut lines = input.lines();
    lines.next();

    let mut weather = lines.next().expect("ligne").split_whitespace().map(|x| x.parse::<i32>().expect("Conversion f32")).collect::<Vec<i32>>();

    let fixed_len = weather.len() as i32 - 2;

    let mut hike_temperatures = vec![];
    for i in 0..fixed_len {
        let mut vaction_weather = weather.iter().skip(i as usize).take(3);

        let a = vaction_weather.next().expect("i32");
        vaction_weather.next(); // skip the rest day
        let b = vaction_weather.next().expect("i32");
        hike_temperatures.push((i as i32 + 1,  std::cmp::max(*a, *b)));

    }

    hike_temperatures.sort_by_key(|(_, y)| *y);
    let best_day = hike_temperatures[0];

    println!("{} {}", best_day.0, best_day.1);



}