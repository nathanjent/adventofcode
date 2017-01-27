use abc::{Context, Candidate, HiveBuilder, scaling};
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
    for line in lines.iter() {
        let mut words = line.split_whitespace()
            .map(|w| w.trim_matches(|c| c == ':' || c == ','));
        if let Some(id) = words.nth(1) {
            if let Ok(id) = id.parse::<usize>() {
                loop {
                    if let Some(name) = words.next() {
                        if let Some(value) = words.next() {
                            if let Ok(value) = value.parse::<i32>() {
                                let sue = sues.entry(id).or_insert(Sue::new(id));
                                match name {
                                    "children" => sue.children = Some(value),
                                    "cats" => sue.cats = Some(value),
                                    "samoyeds" => sue.samoyeds = Some(value),
                                    "pomeranians" => sue.pomeranians = Some(value),
                                    "akitas" => sue.akitas = Some(value),
                                    "vizslas" => sue.vizslas = Some(value),
                                    "goldfish" => sue.goldfish = Some(value),
                                    "trees" => sue.trees = Some(value),
                                    "cars" => sue.cars = Some(value),
                                    "perfumes" => sue.perfumes = Some(value),
                                    _ => {},
                                }
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

    let test_sue = Sue {
        id: 0,
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
    };

    let ctx = Ctx {
        m: sues,
        t: test_sue,
    };
    let hive = HiveBuilder::<Ctx<usize, Sue>>::new(ctx, 5)
        .set_threads(8)
        .set_scaling(scaling::
                     //power(10_f64) // causes error in hive.rs:273
                     power_rank(10_f64)
                     //proportionate() // causes error in hive.rs:273
                     //rank()
                    );
    let best = hive.build().unwrap().run_for_rounds(50_000);
    let candidate = best.expect("Error in hive threading.");
    println!("{:?}", candidate);
    candidate.solution as i32
}

#[derive(Debug)]
struct Sue {
    id: usize,
    children: Option<i32>,
    cats: Option<i32>,
    samoyeds: Option<i32>,
    pomeranians: Option<i32>,
    akitas: Option<i32>,
    vizslas: Option<i32>,
    goldfish: Option<i32>,
    trees: Option<i32>,
    cars: Option<i32>,
    perfumes: Option<i32>,
}

impl Sue {
    fn new(id: usize) -> Self {
        Sue {
            id: id,
            children: None,
            cats: None,
            samoyeds: None,
            pomeranians: None,
            akitas: None,
            vizslas: None,
            goldfish: None,
            trees: None,
            cars: None,
            perfumes: None,
        }
    }
}

struct Ctx<S, T> {
    m: HashMap<S, T>,
    t: T,
}

impl Context for Ctx<usize, Sue> {
    type Solution = usize;

    // Generates random index
    fn make(&self) -> Self::Solution {
        thread_rng().gen_range(1, self.m.len())
    }

    // Calculate total change
    fn evaluate_fitness(&self, solution: &Self::Solution) -> f64 {
        let mut fitness = 100.0;
        if let Some(sue) = self.m.get(solution) {
            if let Some(children) = sue.children {
                fitness -= (children - self.t.children.unwrap()).abs() as f64 / 100.0;
            }
            if let Some(cats) = sue.cats {
                fitness -= (cats - self.t.cats.unwrap()).abs() as f64 / 100.0;
            }
            if let Some(samoyeds) = sue.samoyeds {
                fitness -= (samoyeds - self.t.samoyeds.unwrap()).abs() as f64 / 100.0;
            }
            if let Some(pomeranians) = sue.pomeranians {
                fitness -= (pomeranians - self.t.pomeranians.unwrap()).abs() as f64 / 100.0;
            }
            if let Some(akitas) = sue.akitas {
                fitness -= (akitas - self.t.akitas.unwrap()).abs() as f64 / 100.0;
            }
            if let Some(vizslas) = sue.vizslas {
                fitness -= (vizslas - self.t.vizslas.unwrap()).abs() as f64 / 100.0;
            }
            if let Some(goldfish) = sue.goldfish {
                fitness -= (goldfish - self.t.goldfish.unwrap()).abs() as f64 / 100.0;
            }
            if let Some(trees) = sue.trees {
                fitness -= (trees - self.t.trees.unwrap()).abs() as f64 / 100.0;
            }
            if let Some(cars) = sue.cars {
                fitness -= (cars - self.t.cars.unwrap()).abs() as f64 / 100.0;
            }
            if let Some(perfumes) = sue.perfumes {
                fitness -= (perfumes - self.t.perfumes.unwrap()).abs() as f64 / 100.0;
            }
        }
        fitness
    }

    // Generate a "nearby" solution
    fn explore(&self, field: &[Candidate<Self::Solution>], n: usize) -> Self::Solution {
        let mut rng = thread_rng();
        let m: isize = rng.gen_range(-5, 5);
        //println!("{} + {} = {}", field[n].solution, m, field[n].solution as isize + m);
        let sol = field[n].solution as isize + m;
        if sol >= 0 {
            sol as usize
        } else {
            field[n].solution + ((-m) as usize)
        }
    }
}
