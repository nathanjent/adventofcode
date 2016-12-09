use md5;
use std::fs::File;
use std::io::prelude::*;

pub fn chess_1(file: &str, take_count: usize) -> String {
    let mut word = String::new();
    for hash in find_hash_index(file, take_count) {
        if let Some(letter) = hash.chars().nth(5) {
            word.push(letter);
        }
    }
    word
}

pub fn chess_2(file: &str, take_count: usize) -> String {
    let mut word = [' ';8];
    for hash in find_hash_index(file, take_count) {
        if let Some(index) = hash.chars().nth(5) {
            if let Some(letter) = hash.chars().nth(6) {
                println!("{}, {}", index, letter);
                let mut i = String::new();
                i.push(index);
                // ignore invalid position values
                if let Ok(idx) = i.parse::<usize>() {
                    word[idx] = letter;
                }
            }
        }
    }
    word.iter().cloned().collect::<String>()
}

fn find_hash_index(file: &str, take_count: usize) -> Vec<String> {
    let mut file = File::open(file).expect("File open fail.");
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("File read fail.");
    let mut input = String::from(input.trim());

    let leading = "00000";
    let file_len = input.len();

    (2231250..).filter_map(|num| {
        input.truncate(file_len);
        let test: [u8; 16];
        let num_str = num.to_string();
        input.push_str(&num_str);
        // println!("{}", input);
        {
            test = md5::compute(input.as_bytes());
            let mut hash = String::new();
            for c in test.iter() {
                if *c < 0xf {
                    hash.push('0');
                }
                hash.push_str(&format!("{:x}", c));
            }
            if hash.starts_with(leading) {
                println!("input: {}", input);
                println!("hash: {}", hash);
                Some(hash)
            } else {
                None
            }
        }
    })
    .take(take_count)
    .collect::<Vec<String>>()
}
