use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn ip_7_1(file: &str) -> usize {
    let (tls_count, _) = count_ips(file);
    tls_count
}

pub fn ip_7_2(file: &str) -> usize {
    let (_, ssl_count) = count_ips(file);
    ssl_count
}

fn count_ips(file: &str) -> (usize, usize) {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let lines: Vec<String> = reader.lines()
        .filter_map(Result::ok)
        .collect();

    let mut tls_count = 0;
    let mut ssl_count = 0;
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
        //println!("inners: {:?}", inner);
        //println!("outers: {:?}", outer);

        let mut valid = true;

        'inner: for s in inner.iter() {
            let mut s = s.clone();
            //println!("inner: [{}]", s);
            while let Some(d) = s.pop() {
                if let Some(c) = s.pop() {
                    if let Some(b) = s.pop() {
                        if let Some(a) = s.pop() {
                            let abba_found = a != b && a == d && b == c;
                            if abba_found {
                                // the IP also must not have an ABBA within any hypernet sequences
                                //println!("[{}{}{}{}] {}", a, b, c, d, abba_found);
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
        for s in outer.iter() {
            let mut s = s.clone();
            //println!("outer: {}", s);
            while let Some(d) = s.pop() {
                if let Some(c) = s.pop() {
                    if let Some(b) = s.pop() {
                        if let Some(a) = s.pop() {
                            let abba_found = a != b && a == d && b == c;
                            if abba_found {
                                // supports TLS if it has an Autonomous Bridge Bypass Annotation
                                //println!("{}{}{}{} {}", a, b, c, d, abba_found);
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

        let mut aba_list = Vec::new();
        for mut s in outer {
            //println!("outer: {}", s);
            while let Some(c) = s.pop() {
                if let Some(b) = s.pop() {
                    if let Some(a) = s.pop() {
                        let aba_found = a == c && a != b;
                        if aba_found {
                            // supports SSL if it has an Area-Broadcast Accessor
                            println!("{}{}{} {}", a, b, c, aba_found);
                            aba_list.push((a, b, c));
                        }
                        s.push(a);
                    }
                    s.push(b);
                }
            }
        }
        let mut bab_list = Vec::new();
        for mut s in inner {
            //println!("inner: {}", s);
            while let Some(c) = s.pop() {
                if let Some(b) = s.pop() {
                    if let Some(a) = s.pop() {
                        let bab_found = a == c && a != b;
                        if bab_found {
                            // supports SSL if it has an Area-Broadcast Accessor
                            println!("{}{}{} {}", a, b, c, bab_found);
                            bab_list.push((a, b, c));
                        }
                        s.push(a);
                    }
                    s.push(b);
                }
            }
        }
        println!("aba list: {:?}", aba_list);
        println!("bab list: {:?}", bab_list);

        'ssl: for &(aba0, aba1, _aba2) in aba_list.iter() {
            for &(bab0, bab1, _bab2) in bab_list.iter() {
                if aba0 == bab1 && aba1 == bab0 {
                    ssl_count += 1;
                    break 'ssl
                }
            }
        }

        if valid && abba_count > 0 {
            println!("TLS Supported!");
            tls_count += 1;
        } else {
            println!("Unsupported!");
        }
    }
    (tls_count, ssl_count)
}

enum State {
    In,
    Out,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1base() {
        assert_eq!(::day07::ip_7_1("input/day07_base.txt"), 2);
    }

    #[test]
    fn test2base() {
        assert_eq!(::day07::ip_7_2("input/day07_base2.txt"), 3);
    }

    #[test]
    fn test1() {
        assert_eq!(::day07::ip_7_1("input/day07.txt"), 118);
    }

    #[test]
    fn test2() {
        assert_eq!(::day07::ip_7_2("input/day07.txt"), 260);
    }
}
