    use std::collections::HashMap;
    use std::io::{self, prelude::*};

    fn main() {
        let mut input = String::new();
        io::stdin()
            .read_to_string(&mut input)
            .expect("Error reading from stdin");

        let mut lines = input.lines();

        let mut votes: HashMap<&str, i32> = HashMap::new();

        for line in lines {
            if line != "***" {
                *votes.entry(line.trim()).or_insert(0) += 1;
            }
        }

        let max_votes = votes.values().cloned().max().unwrap_or(0);
        let winners: Vec<&str> = votes
            .iter()
            .filter_map(|(&candidate, &votes)| if votes == max_votes { Some(candidate) } else { None })
            .collect();

        if winners.len() == 1 {
            println!("{}", winners[0]);
        } else {
            println!("Runoff!"  );
        }
    }
