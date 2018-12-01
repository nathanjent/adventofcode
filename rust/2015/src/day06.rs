use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::fmt;
use std::io::BufReader;
use std::str::FromStr;

#[derive(Hash, Eq, PartialEq, Debug)]
enum State {
    On,
    Off,
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Light {
    state: State,
    brightness: u32,
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Loc {
    x: isize,
    y: isize,
}

impl Light {
    fn new(s: State, brightness: u32) -> Light {
        Light {
            state: s,
            brightness: brightness,
        }
    }

    fn toggle_brightness(&mut self) {
        self.inc();
        self.inc();
    }

    fn dec(&mut self) {
        if self.brightness > 0 {
            self.brightness -= 1;
        }
    }

    fn inc(&mut self) {
        self.brightness += 1;
    }

    fn toggle(&mut self) {
        if self.state == State::Off {
            self.switch(State::On);
        } else {
            self.switch(State::Off);
        }
    }

    fn switch(&mut self, state: State) {
        self.state = state;
    }
}

impl Loc {
    fn new(x: isize, y: isize) -> Loc {
        Loc { x: x, y: y }
    }
}

impl fmt::Display for Light {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "state: {:?}, brightness: {}",
               self.state,
               self.brightness)
    }
}

impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

pub fn fire_hazard_1(file: &str) -> u32 {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let mut lights = HashMap::new();

    for line in reader.lines() {
        let s = line.unwrap();
        let words: Vec<&str> = s.split_whitespace()
            .collect();
        let toggle = words[0] != "turn";
        let loc0: Vec<&str> = words[if toggle {1} else {2}]
            .split(',')
            .collect();
        let loc1: Vec<&str> = words[words.len() - 1]
            .split(',')
            .collect();

        let top_left = (isize::from_str(loc0[0]).unwrap(), isize::from_str(loc0[1]).unwrap());
        let bottom_right = (isize::from_str(loc1[0]).unwrap(), isize::from_str(loc1[1]).unwrap());

        // println!("({},{})-({},{})", top_left.0, top_left.1, bottom_right.0, bottom_right.1);
        for x in top_left.0..(bottom_right.0 + 1) {
            for y in top_left.1..(bottom_right.1 + 1) {
                let loc = Loc::new(x, y);
                let light = lights.entry(loc)
                    .or_insert(Light::new(State::Off, 0));
                if words[1] == "on" {
                    light.switch(State::On)
                } else if words[1] == "off" {
                    light.switch(State::Off);
                } else {
                    light.toggle();
                }
            }
        }
    }
    let mut count = 0;
    for (_, light) in lights.iter() {
        if light.state == State::On {
            count += 1;
        }
    }
    count
}

pub fn fire_hazard_2(file: &str) -> u32 {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let mut lights = HashMap::new();

    for line in reader.lines() {
        let s = line.unwrap();
        let words: Vec<&str> = s.split_whitespace()
            .collect();
        let toggle;
        let loc0: Vec<&str>;
        if words[0] == "turn" {
            toggle = false;
            loc0 = words[2].split(',').collect();
        } else {
            toggle = true;
            loc0 = words[1].split(',').collect();
        }
        let loc1: Vec<&str> = words[words.len() - 1]
            .split(',')
            .collect();

        let xy0 = (isize::from_str(loc0[0]).unwrap(), isize::from_str(loc0[1]).unwrap());
        let xy1 = (isize::from_str(loc1[0]).unwrap(), isize::from_str(loc1[1]).unwrap());
        // println!("({},{})-({},{})", xy0.0, xy0.1, xy1.0, xy1.1);
        for x in xy0.0..(xy1.0 + 1) {
            for y in xy0.1..(xy1.1 + 1) {
                let loc = Loc::new(x, y);
                let light = lights.entry(loc)
                    .or_insert(Light::new(State::On, 0));
                if toggle {
                    light.toggle_brightness();
                } else {
                    if words[1] == "on" {
                        light.inc()
                    } else {
                        light.dec();
                    }
                }
            }
        }
    }
    let mut brightness = 0;
    for (_, light) in lights.iter() {
        brightness += light.brightness;
    }
    brightness
}

#[cfg(test)]
mod tests {
    #[test]
    fn day06test1() {
        assert_eq!(::day06::fire_hazard_1("input/day06.txt"), 377891);
    }

    #[test]
    fn day06test2() {
        assert_eq!(::day06::fire_hazard_2("input/day06.txt"), 14110788);
    }
}
