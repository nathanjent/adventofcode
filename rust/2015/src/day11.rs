use std::fs::File;
use std::io::prelude::*;

pub fn corporate_policy_1(file: &str) -> String {
    process(file, 1)
}

pub fn corporate_policy_2(file: &str) -> String {
    process(file, 2)
}

fn process(file: &str, n: usize) -> String {
    let mut file = File::open(file).expect("File open fail.");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("File read fail.");

    input = input.trim().into();
    println!("{}", input);

    for _ in 0..n {
        for _ in 0.. {
            input = inc_str(input);
            //println!("");
            //println!("{}", input);
            if check_straight(&*input) && check_disimilar_pairs(&*input) {
                break
            }
        }
    }
    input
}

/// increasing straight of at least three letters, like abc, bcd, cde, and so on, up to xyz.
fn check_straight(s: &str) -> bool {
    let mut run = 0;
    let mut prev = 'a' as u8 - 1;
    for c in s.chars() {
        if prev + 1 == c as u8 {
            run += 1;
        } else {
            run = 0;
        }
        //println!("prev: {}", prev as char);
        //println!("curr: {}", c);
        //println!("run: {}", run);
        if run == 2 {
            return true
        }
        prev = c as u8;
    }
    false
}

/// at least two different, non-overlapping pairs of letters, like aa, bb, or zz.
fn check_disimilar_pairs(s: &str) -> bool {
    let mut s = s.to_string();
    let mut pairs: Vec<String> = Vec::new();
    loop {
        if let Some(a) = s.pop() {
            if let Some(b) = s.pop() {
                if a == b {
                    //print!("pair: {}{} ", a, b);
                    let mut pair = String::new();
                    pair.push(a);
                    pair.push(b);
                    if pairs.iter().any(|ref prev_pair| {
                        //println!("pairs: {}:{}", prev_pair, pair);
                        **prev_pair != pair
                    }) {
                        return true
                    }
                    pairs.push(pair);
                } else {
                    s.push(b);
                }
            } else {
                break
            }
        } else {
            break
        }
    }
    false
}

/// Increment letter by one: xx, xy, xz, ya, yb, and so on.
fn inc_str(s: String) -> String {
    let mut iter = s.chars().rev();
    let mut updated = String::new();
    loop {
        if let Some(c) = iter.next() {
            //println!("pop: {}", c);
            let byte = c as u8 + 1;
            if byte <= 'z' as u8 {
                match byte as char {
                    ch @ 'i' | ch @ 'o' | ch @ 'l' => {
                        updated.push((ch as u8 + 1) as char);
                        //println!("skipping: {}", ch);
                        //println!("push: {}", (ch as u8 + 1) as char);
                    },
                    ch @ _ => {
                        updated.push(ch);
                        //println!("push: {}", ch);
                    },
                }
                break
            }
            updated.push('a' as char);
            //println!("push an a");
        }
    }
    let mut remaining = iter.rev().collect::<String>();
    //println!("remaining: {}", remaining);
    remaining.push_str(&*updated.chars().rev().collect::<String>());
    remaining
}

#[cfg(test)]
mod tests {
    #[test]
    fn day11test1base() {
        assert_eq!(::day11::corporate_policy_1("input/day11_base.txt"), "abcdffaa");
    }

    #[test]
    fn day11test1() {
        assert_eq!(::day11::corporate_policy_1("input/day11.txt"), "hxbxxyzz");
    }

    #[test]
    fn day11test2() {
        assert_eq!(::day11::corporate_policy_2("input/day11.txt"), "hxcaabcc");
    }
}
