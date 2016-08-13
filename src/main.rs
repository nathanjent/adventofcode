use std::fs::File;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::fmt;
use std::str;
use std::str::FromStr;

#[derive(Debug)]
enum Token {
    Op(Operator),
    Mem(Register),
    Set,
    Start,
}

#[derive(Debug)]
enum Operator {
    And,
    Or,
    LShift,
    RShift,
    Not,
    Unknown,
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum Register {
    Value(i32),
    Address(String),
    Unknown,
}

#[derive(Debug)]
struct Operation {
    operator: Operator,
    left: Register,
    right: Register,
    to: Register,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {:?} {:?}", self.left, self.operator, self.right)
    }
}

impl Default for Operation {
    fn default() -> Operation {
        Operation {
            operator: Operator::Unknown,
            left: Register::Unknown,
            right: Register::Unknown,
            to: Register::Unknown,
        }
    }
}

fn main() {
    let input = File::open("input.txt")
        .ok()
        .expect("File open fail.");
    let reader = BufReader::new(input);

    let mut operations = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let tokens: Vec<Token> = line.split_whitespace().map(|s| {
            match s {
                "AND" => Token::Op(Operator::And),
                "OR" => Token::Op(Operator::Or),
                "LSHIFT" => Token::Op(Operator::LShift),
                "RSHIFT" => Token::Op(Operator::RShift),
                "NOT" => Token::Op(Operator::Not),
                "->" => Token::Set,
                _ => {
                    let m = s.to_string();
                    match m.parse::<i32>() {
                        Ok(n) => Token::Mem(Register::Value(n)),
                        Err(_) => Token::Mem(Register::Address(m)),
                    }
                },
            }
        }).collect();

        let mut operation = Operation::default();

        let mut to = Register::Unknown;

        let mut prev = Token::Start;
        for token in tokens {
            match prev {
                Token::Start => {
                    match token {
                        Token::Op(o) => {
                            operation.operator = o;
                            prev = Token::Op(Operator::Unknown);
                        },
                        Token::Mem(m) => {
                            operation.left = m;
                            prev = Token::Mem(Register::Unknown);
                        },
                        _ => {},
                    }
                },
                Token::Mem(_) => {
                    match token {
                        Token::Op(o) => {
                            operation.operator = o;
                            prev = Token::Op(Operator::Unknown);
                        },
                        Token::Set => prev = Token::Set,
                        _ => {},
                    }
                },
                Token::Op(_) => {
                    match token {
                        Token::Mem(m) => {
                            operation.right = m;
                            prev = Token::Mem(Register::Unknown);
                        },
                        _ => {},
                    }
                },
                Token::Set => {
                    match token {
                        Token::Mem(m) => {
                            to = m;
                            prev = Token::Mem(Register::Unknown);
                        },
                        _ => {},
                    }
                },
            }
        }
        operations.insert(to, operation);
    }
    //for (register, operation) in operations {
    //    println!("{} -> {:?}", operation, register);
    //}
    let mut search_key = Register::Address("a".to_string());
    loop {
        let operation = operations.entry(search_key).or_insert(Operation::default());
        break;    
    }
}
