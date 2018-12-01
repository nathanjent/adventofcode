use abc::{Context, Candidate, HiveBuilder, scaling};
use rand::{thread_rng, Rng};

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use std::str;

pub fn rtg_1(file: &str) -> usize {
    process(file)
}

pub fn rtg_2(file: &str) -> usize {
    process(file)
}

fn process(file: &str) -> usize {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let lines = reader.lines()
        .filter_map(Result::ok)
        .collect::<Vec<String>>();

    let mut floors = HashMap::new();
    for line in &lines {
        // parse line in reverse to gather entities first
        let mut words = line.split_whitespace().rev();
        let mut floor_num = 0;
        let mut entities = Vec::new();
        loop {
            if let Some(w) = words.next() {
                let w = w.trim_matches(|c: char| !c.is_alphabetic());
                match w {
                    "microchip" => {
                        if let Some(m) = words.next() {
                            if let Some(i) = m.find('-') {
                                let (compatibility, _) = m.split_at(i);
                                entities.push(Entity::Microchip(compatibility.to_string()));
                            }
                        }
                    }
                    "generator" => {
                        if let Some(g) = words.next() {
                            entities.push(Entity::Generator(g.to_string()));
                        }
                    }
                    "floor" => {
                        if let Some(f) = words.next() {
                            floor_num = match f {
                                "first" => 1,
                                "second" => 2,
                                "third" => 3,
                                "fourth" => 4,
                                _ => 0,
                            }
                        }
                    }
                    _ => {}
                }
            } else {
                floors.insert(floor_num, entities);
                break;
            }
        }
    }
    for (floor, inventory) in floors.iter() {
        println!("{:?}: {:?}", floor, inventory);
    }
    let mut elevator = Elevator {
        holding: Vec::new(),
        current_floor: 1,
        limit: 2,
    };

    let ctx = Ctx {
        s: floors,
        t: elevator,
    };
    let hive = HiveBuilder::<Ctx<HashMap<usize, Vec<Entity>>, Elevator>>::new(ctx, 5)
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
    candidate.solution as usize
}

#[derive(Debug, Clone)]
enum Entity {
    Generator(String),
    Microchip(String),
}

#[derive(Debug)]
struct Elevator {
    holding: Vec<Entity>,
    current_floor: usize,
    limit: usize,
}

impl Elevator {
    fn up(&mut self) {
        if self.current_floor < 4 {
            self.current_floor += 1;
        }
    }

    fn down(&mut self) {
        if self.current_floor > 0 {
            self.current_floor -= 1;
        }
    }

    fn load_item(&mut self, e: Entity) {
        if self.holding.len() < self.limit {
            self.holding.push(e);
        }
    }

    fn unload(&mut self) -> Option<Entity> {
        self.holding.pop()
    }
}

struct Ctx<S, T> {
    s: S,
    t: T,
}

impl Context for Ctx<HashMap<usize, Vec<Entity>>, Elevator> {
    type Solution = usize;

    // Generates random index
    fn make(&self) -> Self::Solution {
        thread_rng().gen_range(1, self.s.len())
    }

    // Calculate total change
    fn evaluate_fitness(&self, solution: &Self::Solution) -> f64 {
        let mut fitness = 100.0;
        fitness /= *solution as f64;
        // load elevator
    //    if let Some(ref mut floor_inventory) = floors.get_mut(&elevator.current_floor) {
    //        if let Some(entity) = floor_inventory.pop() {
    //            elevator.holding.push(entity);
    //        }
    //    }
    //    // Test move
    //    if let Some(floor_inventory) = floors.get(&(elevator.current_floor + 1)) {
    //        for floor_entity in floor_inventory.iter() {
    //            for entity in elevator.holding.iter() {
    //                match *floor_entity {
    //                    Entity::Generator(fg) => {
    //                        match *entity {
    //                            Entity::Generator(g) => {
    //                            },
    //                            Entity::Microchip(m) => {
    //                            },
    //                        }
    //                    },
    //                    Entity::Microchip(fm) => {
    //                        match *entity {
    //                            Entity::Generator(g) => {
    //                            },
    //                            Entity::Microchip(m) => {
    //                            },
    //                        }
    //                    },
    //                }
    //            }
    //        }
    //    }
        fitness
    }

    // Generate a "nearby" solution
    fn explore(&self, field: &[Candidate<Self::Solution>], n: usize) -> Self::Solution {
        let mut rng = thread_rng();
        let m: isize = rng.gen_range(-2, 2);
        //println!("{} + {} = {}", field[n].solution, m, field[n].solution as isize + m);
        let sol = field[n].solution as isize + m;
        if sol >= 0 {
            sol as usize
        } else {
            field[n].solution + ((-m) as usize)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        assert_eq!(::day11::rtg_1("input/day11.txt"), 0);
    }

    #[test]
    fn test2() {
        assert_eq!(::day11::rtg_2("input/day11.txt"), 0);
    }
}
