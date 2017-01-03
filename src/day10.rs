use std::fs::File;
use std::io::prelude::*;

pub fn look_say_1(file: &str) -> usize {
    process(file, 40)
}

pub fn look_say_2(file: &str) -> usize {
    process(file, 1)
}

fn process(file: &str, n: usize) -> usize {
    let mut file = File::open(file).expect("File open fail.");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("File read fail.");

    let mut output = input;
    for _ in 0..n {
        println!("{}", output);
        output = look_say(output.trim());
    }
    output.len()
}

fn look_say(input: &str) -> String {
    let mut cs = input.chars();
    let mut output = String::new();
    let mut prev = '!';
    let mut word = Vec::new();
    loop {
        if let Some(c) = cs.next() {
            println!("curr: {} prev: {}", c, prev);
            if c != prev {
                if word.len() > 0 {
                    if let Some(n) = format!("{}", word.len()).chars().next() {
                        let m = word[0];
                        output.push(n);
                        output.push(m);
                        println!("({}, {})", n, m);
                    }
                    word.clear();
                }
            }
            word.push(c);
            prev = c;
        } else {
            if word.len() > 0 {
                if let Some(n) = format!("{}", word.len()).chars().next() {
                    let m = word[0];
                    output.push(n);
                    output.push(m);
                    println!("({}, {}) eol", n, m);
                }
            }
            break;
        }
    }
    output
}
