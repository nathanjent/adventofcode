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

#[cfg(test)]
mod tests {

    #[test]
    fn test1() {
        assert_eq!(::day01::not_quite_lisp_1("input/day01.txt"), 232);
    }

    #[test]
    fn test2() {
        assert_eq!(::day01::not_quite_lisp_2("input/day01.txt"), 1783);
    }
}
