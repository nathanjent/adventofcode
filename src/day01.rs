use std::fs::File;
use std::io::prelude::*;

pub fn not_quite_lisp_1(file: &str) -> i32 {
    let mut input = File::open(file).expect("File open fail.");
    let mut buffer = String::new();
    input.read_to_string(&mut buffer)
         .expect("File read fail.");
    let mut floor = 0;
    for c in buffer.chars() {
        if c == '(' {
            floor += 1;
        }
        if c == ')' {
            floor -= 1;
        }
    }
    floor
}

pub fn not_quite_lisp_2(file: &str) -> i32 {
    let mut input = File::open(file).expect("File open fail.");
    let mut buffer = String::new();
    input.read_to_string(&mut buffer)
         .expect("File read fail.");
    let mut floor = 0;
    let mut count = 0;
    for c in buffer.chars() {
        if c == '(' {
            floor += 1;
        }
        if c == ')' {
            floor -= 1;
        }
        count += 1;
        if floor < 0 {
            break;
        }
    }
    count
}
