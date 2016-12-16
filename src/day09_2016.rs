use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn explosives_1(file: &str) -> usize {
    decompress(file)
}

pub fn explosives_2(file: &str) -> usize {
    decompress(file)
}

fn decompress(file: &str) -> usize {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let lines: Vec<String> = reader.lines()
        .filter_map(Result::ok)
        .collect();

    let mut out = String::new();
    for line in lines {
        let mut cs = line.chars().peekable();
        loop {
            if let Some(c) = cs.next() {
                match c {
                    '(' => {
                        let mut marker = Vec::new();
                        let mut num = String::new();
                        // collect digits from both sides of x then parse
                        loop {
                            if let Some(l) = cs.next() {
                                if l == ')' {
                                    if let Ok(d) = num.parse::<usize>() {
                                        marker.push(d);
                                    }
                                    break;
                                }
                                if l.is_digit(10) {
                                    num.push(l);
                                }
                                if l == 'x' {
                                    if let Ok(d) = num.parse::<usize>() {
                                        marker.push(d);
                                        num = String::new();
                                    }
                                }
                            }
                        }
                        if marker.len() == 2 {
                            let mut sequence = String::new();
                            for _ in 0..(marker[0]) {
                                if let Some(l) = cs.next() {
                                    sequence.push(l);
                                }
                            }
                            for _ in 0..(marker[1]) {
                                out.push_str(&*sequence);
                            }
                            //println!("({}x{}){}", marker[0], marker[1], sequence);
                        }
                        //println!("{}", out);
                        //println!("");
                    },
                    _ => {
                        out.push(c);
                    },
                }
            } else {
                break;
            }
        }
    }

    //println!("{}", out);
    out.chars().count()
}
