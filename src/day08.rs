use std::fs::File;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;

pub fn matchsticks(file: &str, ) -> i32 {
    let input = File::open(file)
        .expect("File open fail.");
    let reader = BufReader::new(input);

    for line in reader.lines() {
        let line = line.unwrap();
//        println!("{}", line);
    }
    -1
}
