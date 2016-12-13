use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn two_factor_1(file: &str) -> usize {
    count_pixels(file)
}

pub fn two_factor_2(file: &str) -> usize {
    count_pixels(file)
}

fn count_pixels(file: &str) -> usize {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let lines: Vec<String> = reader.lines()
        .filter_map(Result::ok)
        .collect();

    let mut count = 0;
    let operations = lines.iter().filter_map(|line| {
        let terms = line.split_whitespace().collect::<Vec<&str>>();
        match terms[0] {
            "rect" => {
                if let Some(i) = terms[1].find('x') {
                    let (a, b) = terms[1].split_at(i);
                    let a = a.parse::<usize>().unwrap_or(0);
                    let b = b.parse::<usize>().unwrap_or(0);
                    Some(Operation::Rect(a, b))
                } else {
                    None
                }
            },
            "rotate" => {
                if let Some(i) = terms[2].find('=') {
                    let axis = match terms[1] {
                        "row" => Axis::Y,
                        "column" => Axis::X,
                        _ => Axis::X,
                    };
                    let (_, a) = terms[2].split_at(i);
                    let a = a.parse::<usize>().unwrap_or(0);
                    let b = terms[4].parse::<usize>().unwrap_or(0);
                    Some(Operation::Rotate(axis, a, b))
                } else {
                    None
                }
            },
            _ => None,
        }
    })
    .collect::<Vec<Operation>>();
    println!("{:?}", operations);
    42
}

#[derive(Debug)]
enum Operation {
    Rect(usize, usize),
    Rotate(Axis, usize, usize),
}

#[derive(Debug)]
enum Axis {
    X,
    Y,
}
