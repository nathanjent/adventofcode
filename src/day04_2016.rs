use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::BTreeMap;

pub fn obsecurity_1(file: &str) -> usize {
    parse_rooms(file)
}

pub fn obsecurity_2(file: &str) -> usize {
    parse_rooms(file)
}

fn parse_rooms(file: &str) -> usize {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let lines: Vec<String> = reader.lines()
        .filter_map(Result::ok)
        .collect();

    let mut sum = 0;
    for line in lines {
        let mut words = line.split("-").collect::<Vec<&str>>();

        if let Some(id_checksum) = words.pop() {
            if let Some(i) = id_checksum.find("[") {
                let (id, checksum) = id_checksum.split_at(i);
                let checksum = checksum.trim_matches(|c| c == '[' || c == ']')
                    .split("")
                    .flat_map(str::chars)
                    .collect::<Vec<char>>();
                println!("{} {:?}", id, checksum);
                let letters = words.iter().flat_map(|s| s.chars()).collect::<Vec<char>>();
                println!("{:?}", letters);
                let mut count = BTreeMap::new();
                for c in letters {
                    *count.entry(c).or_insert(0) += 1;
                }
                if let Ok(id) = id.parse::<usize>() {
                    sum += id;
                }
            }
        };
    }
    9
}
