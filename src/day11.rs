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

    input = input.trim().into();
    println!("{}", input);
    for _ in 0..30 {
        input = inc_str(input);
        //println!("");
        println!("{}", input);
    }
    42
}

fn inc_str(s: String) -> String {
    s.chars()
        .map(|c| {
            if c as u8 >= 'z' as u8 {
                'a' as u8
            } else {
                c as u8 + 1
            }
        })
    //.inspect(|b| print!("{} ", b))
    .map(char::from)
        .collect::<String>()
}
