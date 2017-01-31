use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

pub fn rudolph_meds_1(file: &str) -> i32 {
    process(file)
}

pub fn rudolph_meds_2(file: &str) -> i32 {
    process(file)
}

fn process(file: &str) -> i32 {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let lines = reader.lines()
        .filter_map(Result::ok)
        .collect::<Vec<String>>();

    let mut replacements = HashMap::new();
    let mut med_molecule = "";
    for line in lines.iter() {
        let mut words = line.split_whitespace();
        loop {
            if let Some(element) = words.next() {
                if let Some(replacement_elem) = words.nth(1) {
                    let replacement_list
                        = replacements.entry(element).or_insert(Vec::new());
                    replacement_list.push(replacement_elem);
                } else {
                    med_molecule = element;
                }
            } else {
                break
            }
        }
    }
    println!("{:?}", replacements);
    println!("{:?}", med_molecule);

    let mut med_molecule_elements = Vec::new();
    let mut med_molecule_iter = med_molecule.chars();
    let mut other = String::new();
    loop {
        if let Some(c) = med_molecule_iter.next() {
            let mut elem = format!("{}", c);
            if replacements.keys().any(|&k| k == elem) {
                if other.len() > 0 {
                    med_molecule_elements.push(other);
                    other = String::new();
                }
                med_molecule_elements.push(elem);
            } else if let Some(b) = med_molecule_iter.next() {
                elem.push(b);
                if replacements.keys().any(|&k| k == elem) {
                    if other.len() > 0 {
                        med_molecule_elements.push(other);
                        other = String::new();
                    }
                    med_molecule_elements.push(elem);
                } else {
                    other.push_str(&*elem);
                }
            } else {
                other.push_str(&*elem);
            }
        } else {
            break
        }
    }
    println!("{:?}", med_molecule_elements);

    let mut count = 1;
    for elem in med_molecule_elements {
        let  mut combinations = 1;
        if let Some(replacement_list) = replacements.get(&*elem) {
            combinations = replacement_list.len();
        }
        count *= combinations;
    }

    count as i32
}
