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
    let matching_pairs_re = Regex::new(r"([a-z][a-z])").unwrap();
    let single_partition_re = Regex::new(r"([a-z]).{1}([a-z])").unwrap();
    for line in reader.lines() {
        let s = line.unwrap();

        let mut matched_pairs = false;
//        let matches = matching_pairs_re.captures_iter(&s);
//        for m1 in matches.cloned() {
//            for m2 in matches {
//                matched_pairs = m1 == m2;
//            }
//        }


        let mut single_partition = false;
//        for cap in single_partition_re.captures_iter(&s) {
//            for right in single_partition_re.captures_iter(&s) {
//                single_partition = left == right;
//            }
//        }

        //println!("{}", s);
        if matched_pairs && single_partition {
            count += 1;
            //println!("{}", s);
        }
    }
    count
}
