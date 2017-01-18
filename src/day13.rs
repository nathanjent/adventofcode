use generic_matrix::Matrix;
use abc::{Context, Candidate, HiveBuilder, scaling};
use rand::{thread_rng, Rng};

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::sync::Arc;

pub fn knights_table_1(file: &str) -> i64 {
    process(file, None)
}

pub fn knights_table_2(file: &str) -> i64 {
    process(file, Some("Me"))
}

fn process(file: &str, add: Option<&str>) -> i64 {
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
    if let Some(name) = add {
        guest_names.push(name);
    }
    //println!("{:?}", guest_names);

    let mut guest_table = Matrix::zero(guest_names.len(), guest_names.len());
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
                let num = modifier * n.parse::<i64>().unwrap_or(0);
                if let Some(g) = guest {
                    if let Some(a) = words.last() {
                        //println!("{}, {}", g, a);
                        if let Ok(g_idx) = guest_names.binary_search(&g) {
                            //print!("{}, ", g_idx);
                            if let Ok(a_idx) = guest_names.binary_search(&a.trim_matches('.')) {
                                //println!("{}", a_idx);
                                guest_table[(g_idx, a_idx)] = num;
                            }
                        }
                    }
                }
            }
        }
    }
    //println!("{:?}", guest_table);

    let hive = HiveBuilder::<Ctx<i64>>::new(Ctx { m: guest_table }, 5)
        .set_threads(4)
        .set_scaling(scaling::
                     //power(10_f64) // causes error in hive.rs:273
                     power_rank(10_f64)
                     //proportionate() // causes error in hive.rs:273
                     //rank()
                    );
    let best_after_1000 = hive.build().unwrap().run_for_rounds(1_000);
    let candidate = best_after_1000.expect("Error in hive threading.");
    candidate.solution.iter()
        .filter_map(|&i| guest_names.get(i))
        //.inspect(|s| print!("{:?} ", s))
        .cloned()
        .collect::<Vec<&str>>();
    //println!("{:?}", candidate);
    candidate.fitness as i64
}

// Wrap matrix to allow impl Context
struct Ctx<T> {
    m: Matrix<T>,
}

impl Context for Ctx<i64> {
    type Solution = Arc<Vec<usize>>;

    // Generates random guest seating arrangement
    fn make(&self) -> Self::Solution {
        let mut rng = thread_rng();
        let mut idxs = (0..self.m.row()).collect::<Vec<usize>>();
        rng.shuffle(&mut idxs);
        //print!("{:?} ", idxs);
        Arc::new(idxs)
    }

    // Calculate total change in happiness for the solution
    fn evaluate_fitness(&self, solution: &Self::Solution) -> f64 {
        let mut sum = 0;
        //let mut sums = Vec::new();
        let first = solution[0];
        let last = solution[solution.len() - 1];
        let h_mod_first_last = self.m[(last, first)] + self.m[(first, last)];
        //println!("({}, {}): {}", last, first, self[(last, first)]);
        //println!("({}, {}): {}", first, last, self[(first, last)]);
        //sums.push(h_mod_first_last);
        sum += h_mod_first_last ;
        for (&r, &c) in solution.windows(2)
            //.inspect(|i| println!("{:?}", i))
            .filter_map(|i| {
                if let Some(a) = i.first() {
                    if let Some(b) = i.last() {
                        Some((a, b))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            //.inspect(|&(&i, &j)| {
            //    println!("({}, {}): {}", i, j, self[(i, j)]);
            //    println!("({}, {}): {}", j, i, self[(j, i)]);
            //})
        {
            let happiness_modifier = self.m[(r, c)] + self.m[(c, r)];
            //sums.push(happiness_modifier);
            sum += happiness_modifier;
        }
        //println!("[{}] {:?}", sum, solution);
        //println!("{:?}", sums);
        //println!("");
        sum as f64
    }

    // Swaps two randomly selected guest seat placement generating a "nearby" solution
    fn explore(&self, field: &[Candidate<Self::Solution>], n: usize) -> Self::Solution {
        //println!("{:?}", field);
        let ref solution = *field[n].solution;
        let mut rng = thread_rng();
        let mut a = 0;
        let mut b = 0;
        while a == b {
            a = rng.gen_range(0, solution.len() - 1);
            b = rng.gen_range(0, solution.len() - 1);
        }
        let mut nearby_solution = solution.clone();
        nearby_solution.swap(a, b);
        Arc::new(nearby_solution)
    }
}
