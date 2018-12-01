use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashSet;

pub fn taxicab_1(file: &str) -> i64 {
    let mut s: State = State::new();
    for instruction in parse_instructions(file) {
        s.walk(instruction);
    }
    println!("{:?}", s.location);
    (s.location.x + s.location.y).abs() as i64
}

pub fn taxicab_2(file: &str) -> i64 {
    let mut s: State = State::new();
    let mut l: Option<Location> = None;
    for instruction in parse_instructions(file) {
        l = s.walk(instruction);
        if l.is_some() {
            break;
        }
    }
    println!("{:?}", s.location);
    let l = l.unwrap();
    println!("Revisited: {:?}", l);
    (l.x + l.y).abs() as i64
}

fn parse_instructions(file: &str) -> Vec<(Turn, isize)> {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let lines: Vec<String> = reader.lines()
        .filter_map(Result::ok)
        .collect();

    let mut instructions: Vec<(Turn, isize)> = Vec::new();

    for line in lines {
        let mut line_instructions = line.split(", ")
            .filter_map(|word| {
                if let Some(i) = word.find(char::is_numeric) {
                    Some(word.split_at(i))
                } else {
                    None
                }
            })
            //.inspect(|op| println!("{:?}", op))
            .map(|op| {
                let (turn, distance) = op;
                let turn = match turn {
                    "L" => Turn::Left,
                    "R" => Turn::Right,
                    _ => Turn::Right, // Assume right because hand preference ;)
                };
                let distance = distance.parse::<isize>().unwrap_or(0);
                (turn, distance)
            })
            //.inspect(|op| {
            //    let (ref turn, ref distance) = *op;
            //    println!("Turn {:?} then walk {:?} blocks.", turn, distance);
            //})
            .collect::<Vec<(Turn, isize)>>();
        instructions.append(&mut line_instructions);
    }
    instructions
}

#[derive(Debug)]
struct State {
    direction: Direction,
    location: Location,
    visited: HashSet<Location>,
}

impl State {
    fn new() -> Self {
        State {
            direction: Direction::North,
            location: Location { x: 0, y: 0 },
            visited: HashSet::new(),
        }
    }

    /// Returns the first location found that had been visited before.
    fn walk(&mut self, instruction: (Turn, isize)) -> Option<Location> {
        let mut revisited = None;
        let (turn, distance) = instruction;
        self.direction = match turn {
            Turn::Left => self.direction.left(),
            Turn::Right => self.direction.right(),
        };
        match self.direction {
            Direction::North => {
                for _ in 0..distance {
                    self.location.y += 1;
                    if !self.visited.insert(self.location.clone()) && revisited.is_none() {
                        revisited = Some(self.location.clone());
                    }
                }
            }
            Direction::South => {
                for _ in 0..distance {
                    self.location.y -= 1;
                    if !self.visited.insert(self.location.clone()) && revisited.is_none() {
                        revisited = Some(self.location.clone());
                    }
                }
            }
            Direction::East => {
                for _ in 0..distance {
                    self.location.x += 1;
                    if !self.visited.insert(self.location.clone()) && revisited.is_none() {
                        revisited = Some(self.location.clone());
                    }
                }
            }
            Direction::West => {
                for _ in 0..distance {
                    self.location.x -= 1;
                    if !self.visited.insert(self.location.clone()) && revisited.is_none() {
                        revisited = Some(self.location.clone());
                    }
                }
            }
        }
        revisited
    }
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn left(&self) -> Direction {
        match *self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    fn right(&self) -> Direction {
        match *self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Debug)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Location {
    x: isize,
    y: isize,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        assert_eq!(::day01::taxicab_1("input/day01.txt"), 161);
    }

    #[test]
    fn test2() {
        assert_eq!(::day01::taxicab_2("input/day01.txt"), 110);
    }
}
