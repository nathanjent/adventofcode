use optimization::{Minimizer, GradientDescent, NumericalDifferentiation, Func};
use abc::{Context, Candidate, HiveBuilder, scaling};
use rand::{thread_rng, Rng};

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

pub fn knights_table_1(file: &str) -> i64 {
    process(file)
}

pub fn knights_table_2(file: &str) -> i64 {
    process(file)
}

fn process(file: &str) -> i64 {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let lines = reader.lines()
        .filter_map(Result::ok)
        .collect::<Vec<String>>();

    let mut guest_names = Vec::new();
    for line in lines.iter() {
        let mut words = line.split_whitespace();
        if let Some(guest) = words.next() {
            guest_names.push(guest);
        }
    }
    guest_names.sort();
    guest_names.dedup();
    //println!("{:?}", guest_names);

    let mut guests = HashMap::new();
    for line in lines.iter() {
        let mut words = line.split_whitespace();
        let guest = words.next();
        words.next();
        if let Some(word) = words.next() {
            let modifier = match word {
                "gain" => 1,
                "lose" => -1,
                _ => 0,
            };
            if let Some(n) = words.next() {
                let num = modifier * n.parse::<isize>().unwrap_or(0);
                if let Some(g) = guest {
                    if let Some(a) = words.last() {
                        //println!("{}, {}", g, a);
                        if let Ok(g_idx) = guest_names.binary_search(&g) {
                            //print!("{}, ", g_idx);
                            if let Ok(a_idx) = guest_names.binary_search(&a.trim_matches('.')) {
                                //println!("{}", a_idx);
                                let guest = guests.entry(g_idx).or_insert(Guest {
                                        name_idx: g_idx,
                                        affectors: HashMap::new(),
                                    });
                                guest.affectors.insert(a_idx, num);
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{:?}", guests);

//    // numeric version of the Rosenbrock function
//    let function =
//        NumericalDifferentiation::new(Func(|g_rule: &[f64]| {
//            30.0
//    }));
//
//    let minimizer = GradientDescent::new();
//
//    let solution = minimizer.minimize(&function, guests);


//    let hive = HiveBuilder::<HashMap<Guest>>::new(guests, 5)
//                .set_threads(5)
//                        .set_scaling(scaling::power_rank(10_f64));
//    println!("{:?}", hive.build().unwrap().run_for_rounds(1_000));
    42
}

#[derive(Clone, Debug)]
struct Guest {
    name_idx: usize,
    affectors: HashMap<usize, isize>,
}

impl Context for Guest {
    type Solution = usize;

    fn make(&self) -> Self::Solution {
        thread_rng().gen_range(0, self.affectors.len())
    }

    fn evaluate_fitness(&self, solution: &Self::Solution) -> f64 {
        let mut x = 0;
        for _ in 0..1_000 {
            x += 1;
        }
        (x - x) as f64 + *solution as f64
    }

    fn explore(&self, field: &[Candidate<Self::Solution>], n: usize) -> Self::Solution {
        field[n].solution + thread_rng().gen_range(0, 10)
    }
}
