use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn square_triangles_1(file: &str) -> usize {
    parse_count_triangles(file)
}

pub fn square_triangles_2(file: &str) -> usize {
    parse_count_triangles(file)
}

fn parse_count_triangles(file: &str) -> usize {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    reader.lines()
        .filter_map(Result::ok)
        .map(|line| {
            line.split_whitespace()
            .filter_map(|side| {
                side.parse::<i32>().ok()
            })
            .collect::<Vec<i32>>()
        })
        .inspect(|v| {
            for n in v {
                print!("{} ", n);
            }
            println!("");
        })
        .filter_map(|mut t| {
            t.sort();
            let a = t[0];
            let b = t[1];
            let c = t[2];
            if a + b > c {
                Some(c)
            } else {
                None
            }
        })
        //.inspect(|n| println!("{} ", n))
        .count()
}
