extern crate optimization;

use self::optimization::{Minimizer, GradientDescent, NumericalDifferentiation, Func};

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn knights_table_1(file: &str) -> i64 {
    process(file)
}

pub fn knights_table_2(file: &str) -> i64 {
    process(file)
}

fn process(file: &str) -> i64 {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let lines = reader.lines()
        .filter_map(Result::ok)
        .collect::<Vec<String>>();

    let mut guests = Vec::new();
    for line in lines {
        let mut words = line.split_whitespace();
        let guest = words.next();
        words.next();
        if let Some(word) = words.next() {
            match word {
                "gain" => {
                    if let Some(w) = words.next() {
                        let num = w.parse::<i32>().unwrap_or(0);
                        if let Some(g) = guest {
                            if let Some(a) = words.last() {
                                guests.push(GuestRule {
                                        affected_guest: g.to_string(),
                                        affecting_guest: a.to_string(),
                                        modifier: num,
                                });
                            }
                        }
                    }
                },
                "lose" => {
                    if let Some(w) = words.next() {
                        let num = -w.parse::<i32>().unwrap_or(0);
                        if let Some(g) = guest {
                            if let Some(a) = words.last() {
                                guests.push(GuestRule {
                                        affected_guest: g.to_string(),
                                        affecting_guest: a.to_string(),
                                        modifier: num,
                                });
                            }
                        }
                    }
                },
                _ => println!("{:?}", word),
            };
        }
    }
    println!("{:?}", guests);

    // numeric version of the Rosenbrock function
    let function =
        NumericalDifferentiation::new(Func(|g_rule: &[GuestRule]| {
            300
    }));

    let minimizer = GradientDescent::new();

    let solution = minimizer.minimize(&function, guests);
    42
}

struct GuestRule {
    affected_guest: String,
    affecting_guest: String,
    modifier: isize,
}
