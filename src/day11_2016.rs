use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

pub fn rtg_1(file: &str) -> usize {
    process(file)
}

pub fn rtg_2(file: &str) -> usize {
    process(file)
}

fn process(file: &str) -> usize {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let lines: Vec<String> = reader.lines()
        .filter_map(Result::ok)
        .collect();

    let mut state = HashMap::new();
    for tokens in lines.iter().map(|line| {
        let mut words = line.split_whitespace().rev();
        let mut tokens = Vec::new();
        loop {
            if let Some(w) = words.next() {
                let w = w.trim_matches(|c: char| !c.is_alphabetic());
                match w {
                    "floor" => {
                        if let Some(f) = words.next() {
                            match f {
                                "first" => tokens.push(Entity::Floor(1)),
                                "second" => tokens.push(Entity::Floor(2)),
                                "third" => tokens.push(Entity::Floor(3)),
                                "fourth" => tokens.push(Entity::Floor(4)),
                                _ => {},
                            }
                        }
                    },
                    "microchip" => {
                        if let Some(m) = words.next() {
                            if let Some(i) = m.find('-') {
                                let (compatibility, _) = m.split_at(i);
                                tokens.push(Entity::Microchip(compatibility))
                            }
                        }
                    },
                    "generator" => {
                        if let Some(g) = words.next() {
                            tokens.push(Entity::Generator(g))
                        }
                    },
                    _ => {},
                }
            } else {
                break;
            }
        }
        tokens
    })
    {
        let mut floor_id = None;
        let mut floor_inventory = Vec::new();
        for token in tokens {
            match token {
                Entity::Floor(n) => floor_id = Some(n),
                e @ _ => floor_inventory.push(e),
            }
        }
        if let Some(f) = floor_id {
            state.insert(f, floor_inventory);
        }
    }
    println!("{:?}", state);

    if let Some(top_floor) = state.iter().map(|s| {
        let (f, _): (&usize, &Vec<Entity>) = s;
        *f
    }).max() {
        let elevator = Elevator {
            a: None,
            b: None,
            current_floor: 1,
            top_floor: top_floor,
        };

        loop {
            if true {
                break;
            }
        }
    }
    42
}

#[derive(Debug, Clone)]
enum Entity<'e> {
    Floor(usize),
    Generator(&'e str),
    Microchip(&'e str),
}

struct Elevator {
    a: Option<usize>,
    b: Option<usize>,
    current_floor: usize,
    top_floor: usize,
}

impl Elevator {
    fn inc_floor(&mut self) {
        if self.current_floor < self.top_floor {
            self.current_floor += 1;
        }
    }

    fn dec_floor(&mut self) {
        if self.current_floor > 0 {
            self.current_floor -= 1;
        }
    }
}
