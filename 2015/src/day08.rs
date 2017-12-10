use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn matchsticks_1(file: &str) -> i32 {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let mut lit_count = 0;
    let mut mem_count = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let s = String::from(line.trim());
        let lit_c = s.len();
        //       println!("{} length: {}", s, lit_c);
        let mut mem_c = 0;
        let mut sr: String = s.chars().rev().collect();
        while let Some(c) = sr.pop() {
            if c != '"' {
                if c == '\\' {
                    if let Some(x) = sr.pop() {
                        match x {
                            '\\' => {
                                mem_c += 1;
                                // println!("{}{} +1", c, x);
                            }
                            '"' => {
                                mem_c += 1;
                                // println!("{}{} +1", c, x);
                            }
                            'x' => {
                                if let Some(a) = sr.pop() {
                                    if let Some(b) = sr.pop() {
                                        if a.is_digit(16) && b.is_digit(16) {
                                            mem_c += 1;
                                            // println!("{}{}{}{} +1", c, x, a, b);
                                        }
                                    }
                                }
                            }
                            _ => {
                                sr.push(x);
                            }
                        }
                    }
                }
                if c.is_alphabetic() {
                    // println!("{} +1", c);
                    mem_c += 1;
                }
            }
        }
        lit_count += lit_c;
        mem_count += mem_c;
        // println!("{} - {} = {}", lit_c, mem_c, lit_c as i32 - mem_c);
        // println!("lit: {}, mem: {}, Total: {}",
        // lit_count, mem_count, lit_count as i32 - mem_count);
    }
    lit_count as i32 - mem_count
}

pub fn matchsticks_2(file: &str) -> i32 {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let mut enc_count = 0;
    let mut lit_count = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut s = String::from(line.trim());
        let lit_c = s.len();
        //        println!("{} length: {}", s, lit_c);
        let mut enc_c = 0;
        while let Some(c) = s.pop() {
            match c {
                '"' => {
                    enc_c += 2;
                }
                '\\' => {
                    enc_c += 2;
                }
                _ => {
                    enc_c += 1;
                }
            }
        }
        enc_c += 2; // outer quotes
        enc_count += enc_c;
        lit_count += lit_c;
        // println!("{} - {} = {}", enc_c, lit_c, enc_c - lit_c as i32);
        // println!("enc: {}, lit: {}, Total: {}",
        // enc_count, lit_count, enc_count - lit_count as i32);
    }
    enc_count - lit_count as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn day08test1() {
        assert_eq!(::day08::matchsticks_1("input/day08.txt"), 1371);
    }

    #[test]
    fn day08test2() {
        assert_eq!(::day08::matchsticks_2("input/day08.txt"), 2117);
    }
}
