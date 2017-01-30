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
                    replacements.insert(element, replacement_elem);
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
    loop {
        let mut other = Vec::new();
        if let Some(c) = med_molecule_iter.next() {
            let mut elem = format!("{}", c);
            if replacements.keys().any(|&k| k == elem) {
                med_molecule_elements.append(&mut other);
                med_molecule_elements.push(elem);
            } else if let Some(b) = med_molecule_iter.next() {
                elem.push(b);
                if replacements.keys().any(|&k| k == elem) {
                    med_molecule_elements.append(&mut other);
                    med_molecule_elements.push(elem);
                } else {
                    other.push(elem);
                }
            } else {
                other.push(elem);
            }
        } else {
            break
        }
    }
    println!("{:?}", med_molecule_elements);

    42
}
