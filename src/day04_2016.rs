use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn obsecurity_1(file: &str) -> usize {
    parse_rooms(file)
}

pub fn obsecurity_2(file: &str) -> usize {
    parse_rooms(file)
}

fn parse_rooms(file: &str) -> usize {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let lines: Vec<String> = reader.lines()
        .filter_map(Result::ok)
        .collect();

    for line in lines {
        let words = line.split("-").collect::<Vec<&str>>();
        println!("{:?}", words);
    }
    9
}

struct Room {
    words: Vec<String>,
    id: usize,
    checksum: String,
}
