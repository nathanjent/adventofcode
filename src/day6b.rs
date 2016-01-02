use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::fmt;
use std::io::BufReader;
use std::str::FromStr;

fn main() {
    let input = File::open("input.txt")
        .ok()
        .expect("File open fail.");
    let reader = BufReader::new(input);

    let mut lights = HashMap::new();

    #[derive(Hash, Eq, PartialEq, Debug)]
    struct Light {
        b: u32,
    }

    #[derive(Hash, Eq, PartialEq, Debug)]
    struct Loc {
        x: isize,
        y: isize,
    }

    impl Light {
        fn new(b: u32) -> Light {
            Light { b: b }
        }

        fn toggle(&mut self) {
            self.inc();
            self.inc();
        }

        fn dec(&mut self) {
            if self.b > 0 { self.b -= 1; }
        }

        fn inc(&mut self) {
            self.b += 1;
        }
    }

    impl Loc {
        fn new(x: isize, y: isize) -> Loc {
            Loc { x: x, y: y }
        }
    }

    impl fmt::Display for Light {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "s: {:?}", self.b)
        }
    }

    impl fmt::Display for Loc {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }


    for line in reader.lines() {
        let s = line.unwrap();
        let words: Vec<&str> = s
            .split_whitespace()
            .collect();
        let toggle;
        let xy0;
        let xy1;
        let loc0: Vec<&str>;
        if words[0] == "turn" { 
            toggle = false;
            loc0 = words[2].split(',').collect();
        } else {
            toggle = true;
            loc0 = words[1].split(',').collect();
        }
        xy0 = (isize::from_str(loc0[0]).unwrap(),
            isize::from_str(loc0[1]).unwrap());
        let loc1: Vec<&str> = words[words.len() - 1]
            .split(',')
            .collect();
        xy1 = (isize::from_str(loc1[0]).unwrap(),
            isize::from_str(loc1[1]).unwrap());
//println!("({},{})-({},{})", xy0.0, xy0.1, xy1.0, xy1.1);
        for x in xy0.0..(xy1.0 + 1) {
            for y in xy0.1..(xy1.1 + 1) {
                let loc = Loc::new(x, y);
                let light = lights
                    .entry(loc)
                    .or_insert(Light::new(0));
                if toggle {
                    light.toggle();
                } else {
                    if words[1] == "on" { 
                        light.inc() 
                    }
                    else {
                        light.dec();
                    }
                }
            }
        }
    }
    let mut brightness = 0;
    for (_, light) in lights.iter() {
        brightness += light.b;
    }
    println!("total brightness of lights: {}", brightness); 
}

