use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn taxicab(file: &str) -> i64 {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let cur_dir = Direction::North;

    reader.lines()
        .filter_map(Result::ok)
        .flat_map(|line| {
            line.split(", ")
        })
        .filter_map(|word| {
            if let Some(i) = word.find(char::is_numeric) {
                Some(word.split_at(i))
            } else {
                None
            }
        })
        .inspect(|op| println!("{:?}", op));
    64
}


enum Direction {
    North, South, East, West,
}

enum Turn {
    Left, Right, None,
}
