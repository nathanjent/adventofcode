use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn single_night(file: &str) -> i32 {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    for line in reader.lines() {
        let line = line.unwrap();
        let s = String::from(line.trim());
        println!("{}", s);
        //TODO probably need a graph
    }
}
