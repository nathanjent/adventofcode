use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::{VecDeque, HashMap};

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
    let mut non_floor_entity_count = 0;
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
                                tokens.push(Entity::Microchip(compatibility));
                                non_floor_entity_count += 1;
                            }
                        }
                    },
                    "generator" => {
                        if let Some(g) = words.next() {
                            tokens.push(Entity::Generator(g));
                            non_floor_entity_count += 1;
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
        let mut floor_inventory = VecDeque::new();
        for token in tokens {
            match token {
                Entity::Floor(n) => floor_id = Some(n),
                e @ _ => floor_inventory.push_back(e),
            }
        }
        if let Some(f) = floor_id {
            state.insert(f, floor_inventory);
        }
    }

    if let Some(top_floor) = state.iter().map(|s| {
        let (f, _): (&usize, &VecDeque<Entity>) = s;
        *f
    }).max() {
        let mut elevator = Elevator {
            holding: (None, None),
            current_floor: 1,
            top_floor: top_floor,
        };
        println!("{:?}", elevator);

        // implement Frame-Stewart 4+ towers of hanoi algorithm

        loop {
            {
                let inventory = state.entry(elevator.current_floor).or_insert(VecDeque::new());
                println!("Floor {}: {:?}", elevator.current_floor, inventory);
                if let (Some(l), Some(r)) = elevator.unload_all() {
                    inventory.push_back(l);
                    inventory.push_back(r);
                }
                loop {
                    if let Some(item) = inventory.pop_front() {
                        elevator.load_item(item);
                        if elevator.is_full() {
                            break;
                        }
                    } else {
                        panic!("the elevator will only function if it contains at least one RTG or microchip");
                    }
                }

                elevator.inc_floor();
            }
            println!("{:?}", state);
            if state.entry(elevator.top_floor).or_insert(VecDeque::new()).len() == non_floor_entity_count {
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

#[derive(Debug)]
struct Elevator<'e> {
    holding: (Option<Entity<'e>>, Option<Entity<'e>>),
    current_floor: usize,
    top_floor: usize,
}

impl<'e> Elevator<'e> {
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

    fn load_item(&mut self, e: Entity<'e>) {
        if !self.holding.0.is_some() {
            self.holding.0 = Some(e);
        } else if !self.holding.1.is_some() {
            self.holding.1 = Some(e);
        }
    }

    fn is_full(&self) -> bool {
        self.holding.0.is_some() && self.holding.1.is_some()
    }

    fn unload_left(&mut self) -> Option<Entity<'e>> {
        let out = self.holding.0.clone();
        self.holding.0 = None;
        out
    }

    fn unload_right(&mut self) -> Option<Entity<'e>> {
        let out = self.holding.1.clone();
        self.holding.1 = None;
        out
    }

    fn unload_all(&mut self) -> (Option<Entity<'e>>, Option<Entity<'e>>) {
        let out = (self.holding.0.clone(), self.holding.1.clone());
        self.holding.0 = None;
        self.holding.1 = None;
        out
    }
}

// Interesting syntax usage here but I used a method instead
//fn unload_left<'e>(&mut Elevator { holding: (ref mut left, _), .. }: &'e mut Elevator) -> Option<Entity<'e>> {
//    let out = left.clone();
//    *left = None;
//    out
//}
