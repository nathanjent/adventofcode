use cogset::{Euclid, KmeansBuilder};

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

pub fn aunt_sue_1(file: &str) -> i32 {
    process(file)
}

pub fn aunt_sue_2(file: &str) -> i32 {
    process(file)
}

fn process(file: &str) -> i32 {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let lines = reader.lines()
        .filter_map(Result::ok)
        .collect::<Vec<String>>();

    let mut sues = HashMap::new();
    for line in lines.iter() {
        let mut words = line.split_whitespace()
            .map(|w| w.trim_matches(|c| c == ':' || c == ','));
        if let Some(id) = words.nth(1) {
            if let Ok(id) = id.parse::<u32>() {
                loop {
                    if let Some(name) = words.next() {
                        if let Some(value) = words.next() {
                            if let Ok(value) = value.parse::<f64>() {
                                let sue_info = sues.entry(id).or_insert(HashMap::new());
                                sue_info.insert(name, value);
                            }
                        }
                    } else {
                        break
                    }
                }
            }
        }
    }
    println!("{:?}", sues);

    let mut sue_data = Vec::new();
    for (id, sue) in sues.iter() {
        sue_data.push(Euclid([
                         id.clone() as f64,
            sue.get("children")
                .unwrap_or(&0.).clone(),
            sue.get("cats")
                .unwrap_or(&0.).clone(),
            sue.get("samoyeds")
                .unwrap_or(&0.).clone(),
            sue.get("pomeranians")
                .unwrap_or(&0.).clone(),
            sue.get("akitas")
                .unwrap_or(&0.).clone(),
            sue.get("vizslas")
                .unwrap_or(&0.).clone(),
            sue.get("goldfish")
                .unwrap_or(&0.).clone(),
            sue.get("trees")
                .unwrap_or(&0.).clone(),
            sue.get("cars")
                .unwrap_or(&0.).clone(),
            sue.get("perfumes")
                .unwrap_or(&0.).clone(),
            ]));
    }

    println!("{:?}", sue_data);

    // TODO Probably perform some sort of clustering/classification here
    let k = 3;
    let tolerance = 1e-12;
    let kmeans = KmeansBuilder::new().tolerance(tolerance)
        .kmeans(&sue_data, k);
    println!("{:?}", kmeans.clusters());
    42
}
