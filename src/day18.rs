use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

pub fn yard_gif_1(file: &str) -> i32 {
    process(file)
}

pub fn yard_gif_2(file: &str) -> i32 {
    process(file)
}

fn process(file: &str) -> i32 {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let lines = reader.lines()
        .filter_map(Result::ok)
        .enumerate();

    let mut lights = HashMap::new();
    for (row, line) in lines {
        for (column, c) in line.chars().enumerate() {
            let state = match c {
                '#' => State::On,
                '.' => State::Off,
                _ => State::Off,
            };
            lights.insert(Loc::new(row, column), Light::new(state));
        }
    }
    //println!("{:?}", lights);
    for _ in 0..100 {
        let mut light_state = HashMap::new();
        for row in 0..100 {
            for col in 0..100 {
                //println!("({}, {})", row, col);
                /* Loop through list of neighbor positions
                 * counting lights which are on and in bounds
                 */
                let mut neighbors_on = 0;
                for (r, c) in [
                    (-1, -1), (-1, 0), (1, 1),
                    ( 0, -1),/*(0, 0)*/(0, 1),
                    ( 1, -1), ( 1, 0), (1, 1)
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
                let mut new_state = State::Off;
                if let Some(light) = lights.get(&Loc::new(row, col)) {
                    //println!("{:?}", light);
                    match light.state {
                        State::On => {
                            new_state = match neighbors_on {
                                2 | 3 => State::On,
                                _ => State::Off,
                            };
                        },
                        State::Off => {
                            new_state = match neighbors_on {
                                3 => State::On,
                                _ => State::Off,
                            };
                        },
                    }
                }
                light_state.insert(Loc::new(row, col), Light::new(new_state));
            }
        }
        lights = light_state;
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
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Loc {
    x: usize,
    y: usize,
}

impl Light {
    fn new(s: State) -> Light {
        Light {
            state: s,
        }
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
    fn new(x: usize, y: usize) -> Loc {
        Loc { x: x, y: y }
    }
}
