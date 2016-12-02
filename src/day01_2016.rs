use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn taxicab(file: &str) -> i64 {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let cur_dir = Direction::North;

    let mut result = None;
    let lines: Vec<String> = reader.lines()
        .filter_map(Result::ok)
        .collect();
    for line in lines {
        let r: Vec<(Turn, i64)> = line.split(", ")
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
                    _ => Turn::None,
                };
                let distance = distance.parse::<i64>().unwrap_or(0);
                (turn, distance)
            })
            .inspect(|op| {
                let (ref turn, ref distance) = *op;
                println!("Turn {:?} then walk {:?} blocks.", turn, distance);
            })
            .collect();
    };
    result = Some(64);
    result.unwrap()
}


enum Direction {
    North, South, East, West,
}

#[derive(Debug)]
enum Turn {
    Left, Right, None,
}
