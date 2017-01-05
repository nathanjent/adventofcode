use std::fs::File;
use std::io::prelude::*;

pub fn corporate_policy_1(file: &str) -> usize {
    process(file)
}

pub fn corporate_policy_2(file: &str) -> usize {
    process(file)
}

fn process(file: &str) -> usize {
    let mut file = File::open(file).expect("File open fail.");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("File read fail.");

    println!("{}", input);
    42
}
