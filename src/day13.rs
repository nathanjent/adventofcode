use optimization::{Minimizer, GradientDescent, NumericalDifferentiation, Func};
use abc::{Context, Candidate, HiveBuilder, scaling};
use rand::{thread_rng, Rng};
use num::{Zero, One};

use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use std::ops::{Add, Sub, Mul, Div, Rem};

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
    println!("{:?}", guest_names);

    let mut guest_data = Matrix10::new(0f64);
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
                                guest_data[(g_idx, a_idx)] = num as f64;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{:?}", guest_data);

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

impl<T> Context for Matrix10<T> where T: Copy + Send + Sync + One {
    type Solution = (usize, usize);

    fn make(&self) -> Self::Solution {
        (thread_rng().gen_range(0, 10), thread_rng().gen_range(0, 10))
    }

    fn evaluate_fitness(&self, solution: &Self::Solution) -> f64 {
        // TODO convert T to f64 somehow
        //self[*solution]
        42.0
    }

    fn explore(&self, field: &[Candidate<Self::Solution>], n: usize) -> Self::Solution {
        field[n].solution
    }
}

struct Matrix10<T> {
    d: [T; 100],
}

impl<T> Matrix10<T> {
    fn new(init: T) -> Matrix10<T> where T: Copy + Send + Sync + One {
        Matrix10 { d: [init; 100] }
    }
}

impl<T> Index<(usize, usize)> for Matrix10<T> {
    type Output = T;

    #[inline]
    fn index(&self, (i, j): (usize, usize)) -> &T {
        assert!(i < 10 && j < 10);
        &self.d[i * 10 + j]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix10<T> {
    #[inline]
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut T {
        assert!(i < 10 && j < 10);
        &mut self.d[i * 10 + j]
    }
}

impl<T> fmt::Debug for Matrix10<T> where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.d.iter()).finish()
    }
}
