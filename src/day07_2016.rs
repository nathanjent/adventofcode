use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn ip_7_1(file: &str) -> usize {
    count_ips(file)
}

pub fn ip_7_2(file: &str) -> usize {
    count_ips(file)
}

fn count_ips(file: &str) -> usize {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let lines: Vec<String> = reader.lines()
        .filter_map(Result::ok)
        .collect();

    let mut count = 0;
    for line in lines {
        println!("{}", line);
        let mut word = Vec::new();
        let mut inner = Vec::new();
        let mut outer = Vec::new();
        for c in line.chars() {
            match c {
                '[' => {
                    outer.push(word.iter().cloned().collect::<String>());
                    word.clear();
                },
                ']' => {
                    inner.push(word.iter().cloned().collect::<String>());
                    word.clear();
                },
                _ => {
                    word.push(c);
                },
            }
        }

        let mut valid = true;

        'inner: for mut s in inner {
            println!("inner: [{}]", s);
            while let Some(d) = s.pop() {
                if let Some(c) = s.pop() {
                    if let Some(b) = s.pop() {
                        if let Some(a) = s.pop() {
                            let abba_found = a == d && b == c && a != b;
                            if abba_found {
                                println!("[{}{}{}{}] {}", a, b, c, d, abba_found);
                                valid = false;
                                break 'inner;
                            }
                            s.push(a);
                        }
                        s.push(b);
                    }
                    s.push(c);
                }
            }
        }

        let mut abba_count = 0;
        let mut s = outer.iter().flat_map(|s| s.chars()).collect::<String>();
        //for mut s in outer {
        println!("outer: {}", s);
        while let Some(d) = s.pop() {
            if let Some(c) = s.pop() {
                if let Some(b) = s.pop() {
                    if let Some(a) = s.pop() {
                        let abba_found = a == d && b == c && a != b;
                        println!("{}{}{}{} {}", a, b, c, d, abba_found);
                        if abba_found {
                            abba_count += 1;
                        }
                        s.push(a);
                    }
                    s.push(b);
                }
                s.push(c);
            }
        }
        //}

        if valid && abba_count > 0 {
            println!("TLS Supported!");
            count += 1;
        } else {
            println!("Unsupported!");
        }
    }
    count
}
