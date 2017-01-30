use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use std::fmt;

pub fn yard_gif_1(file: &str, steps: usize) -> i32 {
    process(file, steps, &vec![][..])
}

pub fn yard_gif_2(file: &str, steps: usize) -> i32 {
    process(file, steps, &[(0, 0), (0, 99), (99, 0), (99, 99)][..])
}

fn process<'i>(file: &str, steps: usize, immortals: &'i [(usize, usize)]) -> i32 {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let lines = reader.lines()
        .filter_map(Result::ok)
        .enumerate();

    let mut lights = HashMap::new();
    let mut rows = 0;
    let mut columns = 0;
    for (row, line) in lines {
        for (column, c) in line.chars().enumerate() {
            let state = match c {
                '#' => State::On,
                '.' => State::Off,
                _ => State::Off,
            };
            let mut light = Light::new(state);
            if immortals.contains(&(row, column)) {
                light = Light::immortal();
            }
            lights.insert(Loc::new(row, column), light);

            // inefficient but get the row & column lengths
            rows = row + 1;
            columns = column + 1;
        }
    }
    //println!("Initial state:");
    //for row in 0..rows {
    //    for col in 0..columns {
    //        if let Some(light) = lights.get(&Loc::new(row, col)) {
    //            print!("{}", light);
    //        }
    //    }
    //    println!("");
    //}
    for _step in 0..steps {
        //println!("Step {}", _step);
        let mut light_state = HashMap::new();
        for row in 0..rows {
            for col in 0..columns {
                //println!("({}, {})", row, col);
                /* Loop through list of neighbor positions
                 * counting lights which are on and in bounds
                 */
                let mut neighbors_on = 0;
                for (r, c) in [
                    (-1, -1), (-1, 0), (-1, 1),
                    ( 0, -1),/*(0, 0)*/( 0, 1),
                    ( 1, -1), ( 1, 0), ( 1, 1)
                ]
                .iter()
                .filter_map(|&(r, c)| {
                    let r = row as isize + r;
                    let c = col as isize + c;
                    if r < 0 || c < 0 || r >= 100 || c >= 100 {
                        None
                    } else {
                        Some((r as usize, c as usize))
                    }
                })
                {
                    if let Some(light) = lights.get(&Loc::new(r, c)) {
                        if light.state == State::On {
                            neighbors_on += 1;
                        }
                    }
                }
                if let Some(light) = lights.get(&Loc::new(row, col)) {
                    let new_state = match light.state {
                        State::On => {
                            match neighbors_on {
                                2 | 3 => State::On,
                                _ => State::Off,
                            }
                        },
                        State::Off => {
                            match neighbors_on {
                                3 => State::On,
                                _ => State::Off,
                            }
                        },
                    };
                    let mut new_light = Light::new(new_state);
                    if light.immortal {
                        new_light = Light::immortal();
                    }
                    light_state.insert(Loc::new(row, col), new_light);
                }
            }
        }
        lights = light_state;
        //for row in 0..rows {
        //    for col in 0..columns {
        //        if let Some(light) = lights.get(&Loc::new(row, col)) {
        //            print!("{}", light);
        //        }
        //    }
        //    println!("");
        //}
    }
    let mut on_count = 0;
    for (_loc, light) in lights {
        if light.state == State::On {
            on_count += 1;
            //println!("{:?}", light);
        }
    }
    on_count
}

#[derive(Hash, Eq, PartialEq, Debug)]
enum State {
    On,
    Off,
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Light {
    state: State,
    immortal: bool,
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Loc {
    x: usize,
    y: usize,
}

impl Light {
    fn new(s: State) -> Self {
        Light {
            state: s,
            immortal: false,
        }
    }

    fn immortal() -> Self {
        Light {
            state: State::On,
            immortal: true,
        }
    }
}

impl Loc {
    fn new(x: usize, y: usize) -> Loc {
        Loc { x: x, y: y }
    }
}

impl fmt::Display for Light {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self.state {
            State::On => '#',
            State::Off => '.',
        })
    }
}
