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
                //println!("{}", id);
                //println!("{:?}", checksum);

                let letters = words.iter().flat_map(|s| s.chars()).collect::<Vec<char>>();
                //println!("{:?}", letters);

                let mut counts = BTreeMap::new();
                for c in letters.iter() {
                    *counts.entry(*c).or_insert(0) += 1;
                }
                //println!("{:?}", counts);

                let mut stack = counts.iter().collect::<Vec<(&char, &usize)>>();
                // sort high to low
                stack.sort_by(|a, b| b.1.cmp(a.1));
                //println!("{:?}", stack);

                let (checkedsum_full, _): (Vec<&char>, Vec<&usize>) = stack.iter().cloned().unzip();
                let checkedsum = checkedsum_full[..5].iter()
                    .cloned()
                    .cloned()
                    .collect::<Vec<char>>();
                //println!("{:?}", checkedsum);

                if let Ok(id) = id.parse::<usize>() {
                    if checksum == checkedsum {
                        sum += id;
                    }
                }
            }
        };
        //println!("");
    }
    sum
}
