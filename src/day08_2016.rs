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
                let cs = terms[1].split('x')
                    .collect::<Vec<&str>>();
                let a = cs[0].parse::<usize>().unwrap_or(0);
                let b = cs[1].parse::<usize>().unwrap_or(0);
                Some(Operation::Rect(a, b))
            },
            "rotate" => {
                let axis = match terms[1] {
                    "row" => Axis::Row,
                    "column" => Axis::Column,
                    _ => Axis::Row,
                };
                let a = terms[2].split('=')
                    .filter_map(|s| {
                        s.parse::<usize>().ok()
                    })
                .last()
                    .unwrap_or(0);
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
                            //FIXME not correct range syntax
                            for x in screen.width..0 {
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
                                        println!("Pixel {}", p);
                                        screen_map.insert(curr_location, p);
                                    },
                                    None => {
                                        screen_map.insert(curr_location, 0);
                                    },
                                }
                                print_screen(&screen, &screen_map);
                                println!("");
                            }
                        }
                    },
                    Axis::Column => {
                        for _ in 0..(b % screen.height) {
                            //FIXME not correct range syntax
                            for y in screen.height..0 {
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
                                print_screen(&screen, &screen_map);
                                println!("");
                            }
                        }
                    },
                }
            },
        }
        print_screen(&screen, &screen_map);
        println!("");
    }
    //println!("{:?}", screen_map);

    fn print_screen(screen: &Screen, screen_map: &HashMap<Location, usize>) {
        let mut output = vec![vec!['.';screen.width];screen.height];
        for (loc, status) in screen_map.iter() {
            if *status == 1 {
                output[loc.y % screen.height][loc.x % screen.width] = '#';
            }
        }
        for row in 0..screen.height {
            for col in 0..screen.width {
                print!("{}", output[row][col]);
            }
            println!("");
        }
    }

    print_screen(&screen, &screen_map);
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
