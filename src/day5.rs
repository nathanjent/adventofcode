extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use regex::Regex;

fn main() {
    let input = File::open("input.txt")
        .ok()
        .expect("File open fail.");
    let reader = BufReader::new(input);
    let mut count = 0;
    let v_re = Regex::new(r"[aeiou].*[aeiou].*[aeiou]").unwrap();
    //let dub_re = Regex::new(r"([A-Za-z])").unwrap();
    let not_re = Regex::new(r"ab|cd|pq|xy").unwrap();
    for line in reader.lines() {
        let s = line.unwrap();
        let vowels = v_re.is_match(&s);
        let mut dubs = false;
        let mut p = '0';
        for c in s.chars() {
            if c == p { 
                dubs = true;
                break;
            }
            p = c;
        }
        let ugly = not_re.is_match(&s);
        //println!("{}", s);
        if vowels && dubs && !ugly {
            count += 1;
            //println!("{}", s);
        }
    }
    println!("{}", count);
}
