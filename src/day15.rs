use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

pub fn hungry_science_1(file: &str) -> i64 {
    process(file)
}

pub fn hungry_science_2(file: &str) -> i64 {
    process(file)
}

fn process(file: &str) -> i64 {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let lines = reader.lines()
        .filter_map(Result::ok)
        .collect::<Vec<String>>();

    let mut ingredients = HashMap::new();
    for line in lines.iter() {
        let mut words = line.split_whitespace()
            .map(|w| w.trim_matches(|c| c == ':' || c == ','));
        if let Some(name) = words.next() {
            if let Some(cap) = words.nth(1) {
                if let Ok(cap) = cap.parse::<i32>() {
                    if let Some(dur) = words.nth(1) {
                        if let Ok(dur) = dur.parse::<i32>() {
                            if let Some(flav) = words.nth(1) {
                                if let Ok(flav) = flav.parse::<i32>() {
                                    if let Some(tex) = words.nth(1) {
                                        if let Ok(tex) = tex.parse::<i32>() {
                                            if let Some(cal) = words.nth(1) {
                                                if let Ok(cal) = cal.parse::<i32>() {
                                                    ingredients.insert(name, Ingredient {
                                                        capacity: cap,
                                                        durability: dur,
                                                        flavor: flav,
                                                        texture: tex,
                                                        calories: cal,
                                                    });
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{:?}", ingredients);
    let mut total = Ingredient {
        capacity: 0,
        durability: 0,
        flavor: 0,
        texture: 0,
        calories: 0,
    };
    for (i, (_, props)) in ingredients.iter()
        .enumerate()
    {
        let part = i as i32;
        total.capacity += props.capacity * part;
        total.durability += props.capacity * part;
        total.flavor += props.capacity * part;
        total.texture += props.capacity * part;
    }
    println!("{:?}", total);
    42
}

#[derive(Debug)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}
