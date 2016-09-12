use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::fmt;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Location {
    x: isize,
    y: isize,
}

impl Location {
    fn new(x: isize, y: isize) -> Location {
        Location { x: x, y: y }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Santa {
    row: isize,
    col: isize,
}

impl Santa {
    fn new(row: isize, col: isize) -> Santa {
        Santa {
            row: row,
            col: col,
        }
    }
}

pub fn sperical_houses_1(file: &str) -> usize {
    let mut input = File::open(file).expect("File open fail.");
    let mut buffer = String::new();
    input.read_to_string(&mut buffer)
        .expect("File read fail.");
    let mut houses = HashMap::new();
    let mut santa = Santa::new(0, 0);
    houses.insert(Location::new(santa.row, santa.col), 1);
    houses.insert(Location::new(santa.row, santa.col), 1);

    for c in buffer.chars() {
        if c == '<' {
            santa.row -= 1;
        }
        if c == '>' {
            santa.row += 1;
        }
        if c == '^' {
            santa.col += 1;
        }
        if c == 'v' {
            santa.col -= 1;
        }
        let counter = houses.entry(Location::new(santa.row, santa.col))
            .or_insert(0);
        *counter += 1;
        //        println!("dir: {}, row: {}, col: {}, visits: {}",
        //                c, row, col, counter);
    }
    //    for (loc, count) in houses.iter() {
    //        println!("location: {}, visits: {}", loc, count);
    //    }
    //    println!("count of houses: {}", houses.len());
    houses.len()
}


pub fn sperical_houses_2(file: &str) -> usize {
    let mut input = File::open(file).expect("File open fail.");
    let mut buffer = String::new();
    input.read_to_string(&mut buffer)
        .expect("File read fail.");
    let mut houses = HashMap::new();

    let santa = Santa::new(0, 0);
    let robo = Santa::new(0, 0);
    let mut santa_queue = Vec::new();
    santa_queue.push(santa);
    santa_queue.push(robo);
    let mut current = 0;
    houses.insert(Location::new(santa_queue[0].row, santa_queue[0].col), 1);
    houses.insert(Location::new(santa_queue[1].row, santa_queue[1].col), 1);

    for c in buffer.chars() {
        if c == '<' {
            santa_queue[current].row -= 1;
        }
        if c == '>' {
            santa_queue[current].row += 1;
        }
        if c == '^' {
            santa_queue[current].col += 1;
        }
        if c == 'v' {
            santa_queue[current].col -= 1;
        }
        let counter =
            houses.entry(Location::new(santa_queue[current].row, santa_queue[current].col))
                .or_insert(0);
        *counter += 1;
        current += 1;
        if current == 2 {
            current = 0;
        }
        //        println!("dir: {}, row: {}, col: {}, visits: {}",
        //                c, row, col, counter);
    }
    //    for (loc, count) in houses.iter() {
    //        println!("location: {}, visits: {}", loc, count);
    //    }
    //    println!("count of houses: {}", houses.len());
    houses.len()
}
