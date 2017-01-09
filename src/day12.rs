extern crate simple_json;

use self::simple_json::{Json, Number};

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

pub fn abacus_framework_1(file: &str) -> i64 {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let lines = reader.lines()
        .filter_map(Result::ok);
    let mut sum = 0;
    for line in lines {
        sum += line.split(|c| !(c as char).is_digit(10) && c != '-')
            //.inspect(|n| println!("numbers: {:?}", n))
            .filter_map(|word| word.parse::<i64>().ok())
            //.inspect(|n| print!("{:?}, ", n))
            .sum();
    }
    sum
}

pub fn abacus_framework_2(file: &str) -> i64 {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let lines = reader.lines()
        .filter_map(Result::ok);
    let mut sum = 0;
    for line in lines {
        if let Ok(json) = Json::parse(&*line) {
            //println!("{:?}", json.to_string());
            sum += process_json(&json).unwrap_or(0);
        }
    }
    sum
}

fn process_json(json: &Json) -> Result<i64, &str> {
    match json {
        &Json::Null => {
            //println!("null, ");
            return Ok(0)
        },
        &Json::Boolean(value) => {
            //println!("{:?}, ", value);
            return Ok(0)
        },
        &Json::Number(ref value) => {
            match value {
                 &Number::Unsigned(value) => {
                     //println!("{:?}, ", value);
                     return Ok(value as i64)
                 },
                 &Number::Integer(value) => {
                     //println!("{:?}, ", value);
                     return Ok(value)
                 },
                 &Number::Float(value) => {
                     //println!("{:?}, ", value);
                     return Ok(value as i64)
                 },
            }
        },
        &Json::String(ref value) => {
            if value == "red" {
                //println!("RED string, ");
                return Err("red")
            } else {
                //println!("{:?}, ", value);
                return Ok(0)
            }
        },
        &Json::Array(ref value) => {
            let mut sum = 0;
            for elem in value {
                sum += process_json(elem).unwrap_or(0);
            }
            Ok(sum)
        },
        &Json::Object(ref value) => {
            let mut sum = 0;
            for (k, v) in value {
                if k == "red" {
                    //print!("RED key : ");
                    sum = 0;
                    break
                } else {
                    //print!("{:?} : ", k);
                }
                if let Ok(value) = process_json(v) {
                    sum += value;
                } else {
                    sum = 0;
                    break
                }
            }
            Ok(sum)
        },
    }
}
