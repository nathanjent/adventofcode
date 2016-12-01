use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn taxicab(file: &str) -> i64 {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let cur_dir = Direction::North;

    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);

        let words: Vec<&str> = line.split(", ").collect();
        let mut prev_turn = Turn::None;
        for word in words {
            let op: Vec<&str> = word.split(char::is_numeric).collect();
            println!("{} {}", op[0], op[1]);
        }
    }
    64
}


enum Direction {
    North, South, East, West,
}

enum Turn {
    Left, Right, None,
}
