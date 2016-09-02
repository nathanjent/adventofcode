use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collection::{HashMap, HashSet};

pub fn single_night(file: &str) -> i32 {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    //TODO probably need a graph
    let locs = HashSet::new();
    let dists = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let s = String::from(line.trim());
        println!("{}", s);

        while let Some(w) = s.split_whitespace() {
            if w != "to" && w != "=" {
                let loc_dist = w.parse::<i32>();
                match loc_dist {
                    Ok(n) => {
                        dists.insert(from_to, n);
                    },
                    Err(w) => {
                        locs.insert(w);
                    },
                }
            }
        }
    }
    -1
}
