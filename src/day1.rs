use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut input = File::open("input.txt")
        .ok()
        .expect("File open fail.");
    let mut buffer = String::new();
    input.read_to_string(&mut buffer) 
        .ok()
        .expect("File read fail.");
    let mut floor = 0;
    let mut count = 0;
    for c in buffer.chars() {
        if c == '(' { floor += 1; }
        if c == ')' { floor -= 1; }
        count += 1;
        if floor < 0 { println!("{}", count); break; }
    }
    println!("{}", floor);
}
