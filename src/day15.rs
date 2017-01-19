use abc::{Context, Candidate, HiveBuilder, scaling};
use rand::{thread_rng, Rng};

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use std::sync::Arc;

pub fn hungry_science_1(file: &str) -> i32 {
    process(file, None)
}

pub fn hungry_science_2(file: &str) -> i32 {
    process(file, Some(500))
}

fn process(file: &str, consider_calories: Option<i32>) -> i32 {
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
                                                    ingredients.insert(name.to_string(), Ingredient {
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

    let ctx = Ctx {
        m: ingredients,
        consider_calories: consider_calories,
    };
    let hive = HiveBuilder::<Ctx<String, Ingredient>>::new(ctx, 5)
        .set_threads(5)
        .set_scaling(scaling::
                     //power(10_f64) // causes error in hive.rs:273
                     //power_rank(10_f64)
                     //proportionate() // causes error in hive.rs:273
                     rank()
                    );
    let best_after_1000 = hive.build().unwrap().run_for_rounds(10_000);
    let candidate = best_after_1000.expect("Error in hive threading.");
    //candidate.solution.iter()
    //    .filter_map(|&i| guest_names.get(i))
    //    .inspect(|s| print!("{:?} ", s))
    //    .cloned()
    //    .collect::<Vec<&str>>();
    //println!("{:?}", candidate);
    candidate.fitness as i32
}

struct Ctx<S, T> {
    m: HashMap<S, T>,
    consider_calories: Option<i32>,
}

impl Context for Ctx<String, Ingredient> {
    type Solution = Arc<Vec<i32>>;

    // Generates random guest seating arrangement
    fn make(&self) -> Self::Solution {
        let mut distribution = vec![0; self.m.len()];
        //println!("{:?}", distribution);
        distribution_gen(&mut distribution[..]);
        //println!("{:?}", distribution);
        Arc::new(distribution)
    }

    // Calculate total change in happiness for the solution
    fn evaluate_fitness(&self, solution: &Self::Solution) -> f64 {
        let mut total = Ingredient {
            capacity: 0,
            durability: 0,
            flavor: 0,
            texture: 0,
            calories: 0,
        };

        //println!("Solution {:?}", solution);
        for (part, props) in solution.iter()
            .zip(self.m.values())
        {
            //println!("props: {:?}", props);
            //println!("part: {}", part);
            total.capacity += props.capacity * part;
            total.durability += props.durability * part;
            total.flavor += props.flavor * part;
            total.texture += props.texture * part;
            total.calories += props.calories * part;
        }
        //println!("Total {:?}", total);

        if total.capacity < 0 || total.durability < 0 || total.flavor < 0 || total.texture < 0 {
            return 0.0
        }
        let score = total.capacity * total.durability * total.flavor * total.texture;
        match self.consider_calories {
            Some(expected) => {
                if total.calories == expected {
                    score as f64
                } else {
                    0.0
                }
            },
            None => score as f64,
        }
    }

    // Swaps two randomly selected solution points generating a "nearby" solution
    fn explore(&self, field: &[Candidate<Self::Solution>], n: usize) -> Self::Solution {
        //println!("{:?}", field[n].solution);
        let ref solution = *field[n].solution;
        let mut rng = thread_rng();
        let mut a = 0;
        let mut b = 0;
        let mut nearby_solution = solution.clone();
        while a == b {
            a = rng.gen_range(0, solution.len() - 1);
            b = rng.gen_range(0, solution.len() - 1);
        }
        let adjustment = rng.gen_range(1, 2);
        nearby_solution[a] += adjustment;
        nearby_solution[b] -= adjustment;
        //println!("{:?}", nearby_solution);
        Arc::new(nearby_solution)
    }
}

fn distribution_gen<'d>(d: &'d mut [i32]) {
    let mut rng = thread_rng();
    let mut idx_iter = (0..d.len()).cycle();
    let mut sum = 0;
    if let Some(idx) = idx_iter.nth(rng.gen_range(3, 7)) {
        d[idx] = rng.gen_range(0, 100);
        sum += d[idx];
        for _ in 1..(d.len() - 1) {
            //println!("d: {:?}", d);
            if let Some(i) = idx_iter.next() {
                //println!("sum: {}", sum);
                let high = 100 - sum;
                d[i] = if high > 0 { rng.gen_range(0, high) } else { 0 };
                sum += d[i];
            }
        }
    }
    if let Some(i) = idx_iter.next() {
        d[i] = 100 - sum;
    }
    //println!("d: {:?}", d);
    assert!(d.iter().inspect(|&&n| assert!(n >= 0)).fold(0, |sum, n| sum + n) == 100);
}

#[derive(Debug)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}
