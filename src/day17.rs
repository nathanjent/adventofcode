use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn much_such_1(file: &str, expected: usize) -> i32 {
    let (count, _) = process(file, expected);
    count
}

pub fn much_such_2(file: &str, expected: usize) -> i32 {
    let (_, count) = process(file, expected);
    count
}

fn process(file: &str, expected: usize) -> (i32, i32) {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let mut lines = reader.lines()
        .filter_map(Result::ok);

    let mut containers = Vec::new();
    loop {
        if let Some(value) = lines.next() {
            if let Ok(value) = value.parse::<usize>() {
                containers.push(value);
            }
        } else {
            break
        }
    }
    let mut n = 1;
    let mut count = 0;
    let mut min = i32::max_value();
    let mut min_count = 0;
    loop {
        let mut sum = 0;
        let mut digits = to_digits(n, 2);
        if digits.len() > containers.len() {
            break
        }
        // pad with zeros
        while digits.len() < containers.len() {
            digits.insert(0, 0);
        }
        let mut bin_count = 0;
        for (i, &d) in digits.iter().enumerate() {
            if d == 1 {
                sum += containers[i];
                bin_count += 1;
            }
        }
        if sum == expected {
            count += 1;
            if bin_count < min {
                //println!("{:?}", containers);
                //println!("{:?}", digits);
                //println!("{:?}", sum);
                min = bin_count;
                min_count = 0;
            }
            if bin_count == min {
                min_count += 1;
            }
        }
        n += 1;
    }
    (count, min_count)
}

fn to_digits(mut n: usize, base: usize) -> Vec<usize> {
    let mut digits = Vec::new();
    while n > 0 {
        digits.insert(0, n % base);
        n = n / base;
    }
    digits
}

#[test]
fn to_d() {
    let digits = to_digits(42, 2);
    assert_eq!(vec![1, 0, 1, 0, 1, 0], digits);
}

#[test]
fn from_d() {
    fn from_digits(digits: Vec<usize>, base: usize) -> usize {
        digits.iter().fold(0, |acc, d| base * acc + d)
    }

    let n = from_digits(vec![1, 0, 1, 0, 1, 0], 2);
    assert_eq!(42, n);
}
