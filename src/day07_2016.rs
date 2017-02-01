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
        let mut state = State::Out;
        for c in line.chars() {
            match c {
                '[' => {
                    outer.push(word.iter().cloned().collect::<String>());
                    word.clear();
                    state = State::In;
                },
                ']' => {
                    inner.push(word.iter().cloned().collect::<String>());
                    word.clear();
                    state = State::Out;
                },
                _ => {
                    word.push(c);
                },
            }
        }
        match state {
            State::In => inner.push(word.iter().cloned().collect::<String>()),
            State::Out => outer.push(word.iter().cloned().collect::<String>()),
        }
        println!("inners: {:?}", inner);
        println!("outers: {:?}", outer);

        let mut valid = true;

        'inner: for mut s in inner {
            println!("inner: [{}]", s);
            while let Some(d) = s.pop() {
                if let Some(c) = s.pop() {
                    if let Some(b) = s.pop() {
                        if let Some(a) = s.pop() {
                            let abba_found = a != b && a == d && b == c;
                            if abba_found {
                                // the IP also must not have an ABBA within any hypernet sequences
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
        //let mut s = outer.iter().flat_map(|s| s.chars()).collect::<String>();
        for mut s in outer {
        println!("outer: {}", s);
        while let Some(d) = s.pop() {
            if let Some(c) = s.pop() {
                if let Some(b) = s.pop() {
                    if let Some(a) = s.pop() {
                        let abba_found = a != b && a == d && b == c;
                        if abba_found {
                            // supports TLS if it has an Autonomous Bridge Bypass Annotation
                            println!("{}{}{}{} {}", a, b, c, d, abba_found);
                            abba_count += 1;
                        }
                        s.push(a);
                    }
                    s.push(b);
                }
                s.push(c);
            }
        }
        }

        if valid && abba_count > 0 {
            println!("TLS Supported!");
            count += 1;
        } else {
            println!("Unsupported!");
        }
    }
    count
}

enum State {
    In,
    Out,
}
