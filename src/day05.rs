use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use regex::Regex;

pub fn intern_elves_1(file: &str) -> i32 {
    let input = File::open(file)
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
    count
}

pub fn intern_elves_2(file: &str) -> i32 {
    let input = File::open(file)
        .expect("File open fail.");
    let reader = BufReader::new(input);
    let mut count = 0;

    for line in reader.lines() {
        let mut s: String = line.unwrap();
        let mut s_cln = s.clone();

        println!("{}", s);
        let mut matched_pairs = false;
        while let Some(last) = s.pop() {
            if let Some(next) = s.pop() {
                let mut pair = String::new();
                pair.push(next);
                pair.push(last);
                matched_pairs = s.contains(pair.as_str());
                if matched_pairs {
                    println!("{}", pair);
                    break;
                }
                s.push(next);
            }
        }

        let mut single_partition = false;
        while let Some(last) = s_cln.pop() {
            if let Some(mid) = s_cln.pop() {
                if let Some(first) = s_cln.pop() {
                    single_partition = last == first;
                    if single_partition {
                        println!("{}{}{}", first, mid, last);
                        break;
                    }
                    s_cln.push(first);
                    s_cln.push(mid);
                }
            }
        }

        if matched_pairs && single_partition {
            count += 1;
            //println!("{}", s);
        }
    }
    count
}
