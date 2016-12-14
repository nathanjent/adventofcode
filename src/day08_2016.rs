use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

pub fn two_factor_1(file: &str) -> usize {
    count_pixels(file)
}

pub fn two_factor_2(file: &str) -> usize {
    count_pixels(file)
}

fn count_pixels(file: &str) -> usize {
    let ininsert = File::open(file).expect("File open fail.");
    let reader = BufReader::new(ininsert);

    let lines: Vec<String> = reader.lines()
        .filter_map(Result::ok)
        .collect();

    let operations = lines.iter().filter_map(|line| {
        let terms = line.split_whitespace().collect::<Vec<&str>>();
        match terms[0] {
            "rect" => {
                //FIXME the terms need split correctly
                let cs = terms[1].split(|c| !c.is_digit())
                    .filter(char::is_digit)
                    .collect::<Vec<char>>();
                let a = cs[0].parse::<usize>().unwrap_or(0);
                let b = cs[2].parse::<usize>().unwrap_or(0);
                Some(Operation::Rect(a, b))
            },
            "rotate" => {
                //FIXME the terms need split correctly
                let axis = match terms[1] {
                    "row" => Axis::Row,
                    "column" => Axis::Column,
                    _ => Axis::Row,
                };
                let a = terms[2].split(|c| !c.is_digit())
                    .filter(char::is_digit)
                    .collect::<String>()
                    .parse::<usize>().unwrap_or(0);
                let b = terms[4].parse::<usize>().unwrap_or(0);
                Some(Operation::Rotate(axis, a, b))
            },
            _ => None,
        }
    })
    .collect::<Vec<Operation>>();

    //println!("{:?}", operations);

    let screen = Screen { width: 50, height: 6 };
    let mut screen_map = HashMap::new();
    for operation in operations {
        match operation {
            Operation::Rect(a, b) => {
                println!("Rect({}, {})", a, b);
                for x in 0..a {
                    for y in 0..b {
                        let loc = Location {
                                x: x % screen.width,
                                y: y % screen.height,
                            };
                        println!("{}, {}", loc.x, loc.y);
                        screen_map.insert(loc, 1);
                    }
                }
            },
            Operation::Rotate(axis, a, b) => {
                println!("Rotate({:?}, {}, {})", axis, a, b);
                match axis {
                    Axis::Row => {
                        for _ in 0..(b % screen.width) {
                            for x in 0..screen.width {
                                let curr_location = Location {
                                    x: x,
                                    y: a,
                                };
                                let prev_location;
                                if x == 0 {
                                    prev_location = Location {
                                        x: screen.width - 1,
                                        y: a,
                                    };
                                } else {
                                    prev_location = Location {
                                        x: x - 1,
                                        y: a,
                                    };
                                }
                                let prev_pixel = screen_map.get(&prev_location)
                                    .cloned();
                                match prev_pixel {
                                    Some(p) => {
                                        screen_map.insert(curr_location, p);
                                    },
                                    None => {
                                        screen_map.insert(curr_location, 0);
                                    },
                                }
                            }
                        }
                    },
                    Axis::Column => {
                        for _ in 0..(b % screen.height) {
                            for y in 0..screen.height {
                                let curr_location = Location {
                                    x: a,
                                    y: y,
                                };
                                let prev_location;
                                if y == 0 {
                                    prev_location = Location {
                                        x: a,
                                        y: screen.width - 1,
                                    };
                                } else {
                                    prev_location = Location {
                                        x: a,
                                        y: y - 1,
                                    };
                                }
                                let prev_pixel = screen_map.get(&prev_location)
                                    .cloned();
                                match prev_pixel {
                                    Some(p) => {
                                        screen_map.insert(curr_location, p);
                                    },
                                    None => {
                                        screen_map.insert(curr_location, 0);
                                    },
                                }
                            }
                        }
                    },
                }
            },
        }
    }
    //println!("{:?}", screen_map);
    let mut screen_array = vec![vec![0; screen.width]; screen.height];
    for (key, val) in screen_map.iter() {
        screen_array[key.x][key.y] = *val;
    }
    for i in 0..screen.height {
        for j in 0..screen.width {
            print!("{}", screen_array[i % screen.height][j % screen.width]);
        }
    }
    screen_map.iter()
        .filter(|e| {
            let (_, state) = *e;
            *state == 1
        })
    .count()
}

#[derive(Debug)]
enum Operation {
    Rect(usize, usize),
    Rotate(Axis, usize, usize),
}

#[derive(Debug)]
enum Axis {
    Row,
    Column,
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Location {
    x: usize,
    y: usize,
}

struct Screen {
    width: usize,
    height: usize,
}
