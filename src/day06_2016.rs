use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::BTreeMap;

pub fn noisy_signals_1(file: &str) -> String {
    parse(file)
}

pub fn noisy_signals_2(file: &str) -> usize {
    parse(file);
    42
}

fn parse(file: &str) -> String {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let lines: Vec<String> = reader.lines()
        .filter_map(Result::ok)
        .collect();

    let mut buckets = vec![Vec::new();8];
    for line in lines {
        for (i, letter) in  line.split("")
            .filter_map(|s| {
                match s {
                    "" => None,
                    _ => Some(s.to_string()),
                }
            })
            .enumerate() {
            //print!("{:?}", letter);
            buckets[i].push(letter);
        }
        //println!("");
    }
    //println!("{:?}", buckets);
    let mut message = String::new();
    for bucket in buckets {
        let mut counts = BTreeMap::new();
        for c in bucket.into_iter() {
            *counts.entry(c).or_insert(0) += 1;
        }
        //println!("{:?}", counts);
        let mut stack = counts.into_iter().collect::<Vec<(String, usize)>>();
        // sort high to low
        stack.sort_by(|a, b| b.1.cmp(&a.1));
        //println!("{:?}", stack);
        let (sorted_letters, _): (Vec<String>, Vec<usize>) = stack.iter().cloned().unzip();
        message.push_str(&*sorted_letters[0]);
    }
    //println!("{:?}", message);
    message
}

