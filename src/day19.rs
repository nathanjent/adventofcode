use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::{HashMap, HashSet};

pub fn rudolph_meds_1(file: &str) -> usize {
    process(file)
}

pub fn rudolph_meds_2(file: &str) -> usize {
    process(file)
}

fn process(file: &str) -> usize {
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
    println!("{:?}", med_molecule);
    for (k, replacement_list) in replacements.iter() {
        println!("{:?} {:?}", k, replacement_list);
    }

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

    let mut molecules = HashSet::new();
    for (elem, replacement_list) in replacements.iter() {
        for replacement in replacement_list {
            let elements = med_molecule_elements.clone();
            let mut element_iter = med_molecule_elements.iter().enumerate();
            loop {
                let mut index = None;
                if let Some((i, e)) = element_iter.next() {
                    if e == elem {
                        index = Some(i);
                    }
                } else {
                    break
                }
                if let Some(index) = index {
                    let s: String = elements.iter()
                        .enumerate()
                        .map(|(i, e)| {
                            if i == index {
                                replacement.clone()
                            } else {
                                e
                            }
                        })
                    .collect();
                    println!("{} {:?} {:?}", index, replacement, s);
                    molecules.insert(s);
                }
            }
        }
    }
    for molecule in molecules.iter() {
        println!("{:?}", molecule);
    }

    molecules.len()
}
