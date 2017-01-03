use md5;
use std::fs::File;
use std::io::prelude::*;

pub fn chess_1(file: &str) -> String {
    let mut file = File::open(file).expect("File open fail.");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("File read fail.");
    let mut input = String::from(input.trim());
    let file_len = input.len();
    let password_len = 8;

    (0..)
        .filter_map(|num| {
            input.truncate(file_len);
            input.push_str(&num.to_string());
            find_hash_index(&*input)
        })
        .filter_map(|hash| hash.chars().nth(5))
        .take(password_len)
        .collect::<String>()
}

pub fn chess_2(file: &str) -> String {
    let mut file = File::open(file).expect("File open fail.");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("File read fail.");
    let mut input = String::from(input.trim());
    let file_len = input.len();

    let mut password = ['_'; 8];
    let password_len = password.len();
    for indexed_letter in (0..)
        .filter_map(|num| {
            input.truncate(file_len);
            input.push_str(&num.to_string());
            find_hash_index(&*input)
        })
        .filter_map(|hash| {
            if let Some(index) = hash.chars().nth(5) {
                if let Some(letter) = hash.chars().nth(6) {
                    println!("{}, {}", index, letter);
                    let mut i = String::new();
                    i.push(index);
                    // ignore invalid position values
                    if let Ok(idx) = i.parse::<usize>() {
                        if idx < password_len {
                            return Some((idx, letter));
                        }
                    }
                }
            }
            None
        }) {
        let (i, c) = indexed_letter;
        if password[i] == '_' {
            password[i] = c;
        }
        let mut pass_filled = true;
        println!("{:?}", password);
        for l in password.iter().cloned() {
            if l == '_' {
                pass_filled = false;
            }
        }
        if pass_filled {
            break;
        }
    }
    password.iter().cloned().collect::<String>()
}

fn find_hash_index(input: &str) -> Option<String> {
    let test = md5::compute(input.as_bytes());
    let mut hash = String::new();
    for c in test.iter() {
        if *c < 0xf {
            hash.push('0');
        }
        hash.push_str(&format!("{:x}", c));
    }
    if hash.starts_with("00000") {
        println!("input: {}", input);
        println!("hash: {}", hash);
        Some(hash)
    } else {
        None
    }
}
