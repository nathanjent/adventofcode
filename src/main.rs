use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::io::BufReader;
use std::fmt;

#[derive(Hash, Eq, PartialEq, Debug)]
enum Op {
    Store,
    And,
    Or,
    LShift,
    RShift,
    Not,
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Operation {
    l: String,
    r: String,
    op: Op,
}

impl Operation {
    fn new(l: String, r: String, op: Op) -> Operation {
        Operation { l: l, r: r, op: op }
    }
    
    fn execute(&self, ops: &HashMap<String, Operation>) 
            -> u16 {
        let l_val;
        let r_val;
        match self.l.parse::<u16>() {
            Ok(n) => l_val = n,
            Err(_) => {
                l_val = match ops.get(&self.l) {
                    Some(operation) => 
                        operation.execute(&ops),
                    None => 0, // operation might not use l_val
                }
            }
        }
        match self.r.parse::<u16>() {
            Ok(n) => r_val = n,
            Err(_) => {
                r_val = match ops.get(&self.r) {
                    Some(operation) => 
                        operation.execute(&ops),
                    None => 0, // operation might not use r_val
                }
            }
        }
        //println!("{}", &self);
        //println!("left: {}, right: {}", &l_val, &r_val);
        match self.op {
            Op::Store => r_val,
            Op::Not => !r_val,
            Op::And => l_val & r_val,
            Op::Or => l_val | r_val,
            Op::LShift => l_val << r_val,
            Op::RShift => l_val >> r_val,
        }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {:?} {} ->", self.l, self.op, self.r)
    }
}


fn main() {
    let input = File::open("input.txt")
        .ok()
        .expect("File open fail.");
    let reader = BufReader::new(input);

    let mut operations = Box::new(HashMap::new());
    for line in reader.lines() {
        let s = line.unwrap();
        let words: Vec<&str> = s
            .split_whitespace()
            .collect();
        let size = words.len();
        // result is stored to last word
        let to = words[size - 1].to_string();
        let op;
        match size {
            3 => { // store ->
                let l = words[0].to_string();
                op = Operation::new(
                "".to_string(), 
                l.to_string(), 
                Op::Store);
            }
            4 => { // NOT
                let l = words[1].to_string();
                op = Operation::new(
                "".to_string(), 
                l.to_string(), 
                Op::Not);
            }
            5 => { // AND, OR, LSHIFT, RSHIFT
                let l = words[0].to_string();
                let r = words[2].to_string();
                op = match words[1] {
                    "AND" => Operation::new(l, r, Op::And),
                    "OR" => Operation::new(l, r, Op::Or),
                    "LSHIFT" => Operation::new(l, r, Op::LShift),
                    "RSHIFT" => Operation::new(l, r, Op::RShift),
                    _ => continue,
                }
            }
            _ => continue,
        }
        &operations.insert(to, op);
    }
    for (key, op) in operations.iter() {
        println!("{} {}", op, key);
    }

    let key = "a";
    match operations.get(key) {
        Some(var) => println!( 
            "{}: {}", 
            key, 
            var.execute(&operations)),
        None => println!("Value not found."),
    }
}

