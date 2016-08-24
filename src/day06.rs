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
    s: State,
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Loc {
    x: isize,
    y: isize,
}

impl Light {
    fn new(s: State) -> Light {
        Light { s: s }
    }

    fn toggle(&mut self) {
        if self.s == State::Off { 
            self.switch(State::On);
        } else {
            self.switch(State::Off);
        }
    }

    fn switch(&mut self, s: State) {
        self.s = s;
    }
}

impl Loc {
    fn new(x: isize, y: isize) -> Loc {
        Loc { x: x, y: y }
    }
}

impl fmt::Display for Light {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "s: {:?}", self.s)
    }
}

impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

pub fn fire_hazard(file: &str) -> i32 {
    let input = File::open(file)
        .expect("File open fail.");
    let reader = BufReader::new(input);

    let mut lights = HashMap::new();

    for line in reader.lines() {
        let s = line.unwrap();
        let words: Vec<&str> = s
            .split_whitespace()
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
        
        let top_left = (isize::from_str(loc0[0]).unwrap(),
            isize::from_str(loc0[1]).unwrap());
        let bottom_right = (isize::from_str(loc1[0]).unwrap(),
            isize::from_str(loc1[1]).unwrap());

//println!("({},{})-({},{})", top_left.0, top_left.1, bottom_right.0, bottom_right.1);
        for x in top_left.0..(bottom_right.0 + 1) {
            for y in top_left.1..(bottom_right.1 + 1) {
                let loc = Loc::new(x, y);
                let light = lights
                    .entry(loc)
                    .or_insert(Light::new(State::Off));
                if toggle {
                    light.toggle();
                } else {
                    if words[1] == "on" { 
                        light.switch(State::On) 
                    }
                    else {
                        light.switch(State::Off);
                    }
                }
            }
        }
    }
    let mut count = 0;
    for (_, light) in lights.iter() {
        if light.s == State::On { count += 1; }
    }
    count
}

