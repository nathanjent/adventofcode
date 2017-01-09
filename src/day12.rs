extern crate simple_json;

use self::simple_json::Json;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

pub fn abacus_framework_1(file: &str) -> i64 {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let lines = reader.lines()
        .filter_map(Result::ok);
    let mut sum = 0;
    for line in lines {
        sum += line.split(|c| !(c as char).is_digit(10) && c != '-')
            //.inspect(|n| println!("numbers: {:?}", n))
            .filter_map(|word| word.parse::<i64>().ok())
            .inspect(|n| print!("{:?}, ", n))
            .sum();
    }
    sum
}

pub fn abacus_framework_2(file: &str) -> i64 {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let lines = reader.lines()
        .filter_map(Result::ok);
    let mut sum = 0;
    for line in lines {
        if let Ok(json) = Json::parse(&*line) {
            println!("{:?}", json.to_string());
        }
    }
    sum
}
