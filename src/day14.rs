use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

pub fn reindeer_olympics_1(file: &str, seconds: usize) -> i64 {
    process(file, seconds)
}

pub fn reindeer_olympics_2(file: &str, seconds: usize) -> i64 {
    process(file, seconds)
}

fn process(file: &str, seconds: usize) -> i64 {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let lines = reader.lines()
        .filter_map(Result::ok)
        .collect::<Vec<String>>();

    let mut reindeer = Vec::new();
    for line in lines.iter() {
        let mut words = line.split_whitespace();
        if let Some(name) = words.nth(0) {
            if let Some(velocity) = words.nth(2) {
                if let Ok(velocity) = velocity.parse::<usize>() {
                    if let Some(active_time) = words.nth(2) {
                        if let Ok(active_time) = active_time.parse::<usize>() {
                            if let Some(rest_time) = words.nth(6) {
                                if let Ok(rest_time) = rest_time.parse::<usize>() {
                                    reindeer.push(Reindeer::new(name,
                                                                velocity,
                                                                active_time,
                                                                rest_time));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    //reindeer.sort();
    //reindeer.dedup();
    println!("{:?}", reindeer);

    let mut racers = HashMap::new();
    for time in 0..seconds {
        println!("{:?}", racers);
        println!("{}", time + 1);
        for rd in reindeer.iter() {
            let racer = racers.entry(&rd.name)
                .or_insert(Racer::default());
            racer.state = match racer.state {
                State::Active { time: t } => {
                    if t == rd.active_time {
                        State::Resting { time: 1 }
                    } else {
                        racer.distance += rd.velocity;
                        State::Active { time: t + 1 }
                    }
                },
                State::Resting { time: t } => {
                    if t == rd.rest_time {
                        State::Active { time: 0 }
                    } else {
                        State::Resting { time: t + 1 }
                    }
                },
            };
            print!("{} {}, ", rd.name, racer.distance);
        }
        println!("");
    }
    println!("{:?}", racers);

    let (_, winner) = racers.into_iter()
        .max_by_key(|&(_, ref v)| {
            v.distance
        }).expect("Race cancelled");
    winner.distance as i64
}

#[derive(Debug, PartialEq)]
enum State {
    Active { time: usize },
    Resting { time: usize },
}

#[derive(Debug)]
struct Racer {
    distance: usize,
    state: State,
}

impl Racer {
    fn default() -> Self {
        Racer {
            distance: 0,
            state: State::Active { time: 0 },
        }
    }
}

#[derive(Debug)]
struct Reindeer<'d> {
    name: &'d str,
    velocity: usize,
    active_time: usize,
    rest_time: usize,
}

impl<'d> Reindeer<'d> {
    fn new(name: &'d str, velocity: usize, active_time: usize, rest_time: usize,) -> Self {
        Reindeer {
            name: name,
            velocity: velocity,
            active_time: active_time,
            rest_time: rest_time,
        }
    }
}

