use cogset::{Euclid, KmeansBuilder};
use rand::{thread_rng, Rng};

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use std::str;

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
    let mut sue_ids = Vec::new();
    for line in lines.iter() {
        let mut words = line.split_whitespace()
            .map(|w| w.trim_matches(|c| c == ':' || c == ','));
        if let Some(id) = words.nth(1) {
            if let Ok(id) = id.parse::<usize>() {
                sue_ids.push(id);
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
    //println!("{:?}", sues);

    let mut sue_analysis = HashMap::new();
    sue_analysis.insert("children", 3.);
    sue_analysis.insert("cats", 7.);
    sue_analysis.insert("samoyeds", 2.);
    sue_analysis.insert("pomeranians", 3.);
    sue_analysis.insert("akitas", 0.);
    sue_analysis.insert("vizslas", 0.);
    sue_analysis.insert("goldfish", 5.);
    sue_analysis.insert("trees", 3.);
    sue_analysis.insert("cars", 2.);
    sue_analysis.insert("perfumes", 1.);
    sues.insert(0, sue_analysis);
    sue_ids.push(0);

    //println!("{:?}", sue_data);

    // TODO Probably perform some sort of clustering/classification here
    let mut sue_data = Vec::new();
    let k = 3;
    let tolerance = 1e-12;
    let mut rng = thread_rng();
    for _ in 0..20 {
        sue_data.clear();
        for id in sue_ids.iter() {
            if let Some(sue) = sues.get(&id) {
                println!("Sue {}: {:?} ", id, sue);
                sue_data.push(Euclid([
                                 id.clone() as f64,
                    sue.get("children")
                        .unwrap_or(&(rng.gen_range(0., 10.))).clone(),
                    sue.get("cats")
                        .unwrap_or(&(rng.gen_range(0., 10.))).clone(),
                    sue.get("samoyeds")
                        .unwrap_or(&(rng.gen_range(0., 10.))).clone(),
                    sue.get("pomeranians")
                        .unwrap_or(&(rng.gen_range(0., 10.))).clone(),
                    sue.get("akitas")
                        .unwrap_or(&(rng.gen_range(0., 10.))).clone(),
                    sue.get("vizslas")
                        .unwrap_or(&(rng.gen_range(0., 10.))).clone(),
                    sue.get("goldfish")
                        .unwrap_or(&(rng.gen_range(0., 10.))).clone(),
                    sue.get("trees")
                        .unwrap_or(&(rng.gen_range(0., 10.))).clone(),
                    sue.get("cars")
                        .unwrap_or(&(rng.gen_range(0., 10.))).clone(),
                    sue.get("perfumes")
                        .unwrap_or(&(rng.gen_range(0., 10.))).clone(),
                    ]));
            }
        }
        let kmeans = KmeansBuilder::new().tolerance(tolerance)
            .kmeans(&sue_data, k);
        for (c, &(e, ref v)) in kmeans.clusters().iter().enumerate() {
            println!("");
            println!("Cluster: {}", c);
            println!("{:?}", e);
            if v.contains(&0) {
                println!("Next cluster to process: {}", c);
                sue_ids = v.clone();
            }
        }
        if let Ok(steps) = kmeans.converged() {
            println!("{:?}", steps);
        } else {
            break
        }
        if sue_ids.len() < 4 { break }
    }
    for id in sue_ids.iter() {
        if let Some(sue) = sues.get(&id) {
            println!("Sue {}: {:?} ", id, sue);
        }
    }
    42
}
