use abc::{Context, Candidate, HiveBuilder, scaling};
use rand::{thread_rng, Rng};

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use std::sync::Arc;

pub fn aunt_sue_1(file: &str) -> i32 {
    process(file)
}

pub fn aunt_sue_2(file: &str) -> i32 {
    process(file)
}

fn process(file: &str) -> i32 {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let lines = reader.lines()
        .filter_map(Result::ok)
        .collect::<Vec<String>>();

    let mut sues = HashMap::new();
    for line in lines.iter() {
        let mut words = line.split_whitespace()
            .map(|w| w.trim_matches(|c| c == ':' || c == ','));
        if let Some(sue) = words.nth(1) {
            loop {
                if let Some(name) = words.next() {
                    if let Some(value) = words.next() {
                        if let Ok(value) = value.parse::<i32>() {
                            let sue_info = sues.entry(sue).or_insert(HashMap::new());
                            sue_info.insert(name, value);
                        }
                    }
                } else {
                    break
                }
            }
        }
    }
    println!("{:?}", sues);
    42
}
