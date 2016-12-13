use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn two_factor_1(file: &str) -> usize {
    count_pixels(file)
}

pub fn two_factor_1(file: &str) -> usize {
    count_pixels(file)
}

fn count_pixels(file: &str) -> usize {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let lines: Vec<String> = reader.lines()
        .filter_map(Result::ok)
        .collect();

    let mut count = 0;
    for line in lines {
    }
    42
}
