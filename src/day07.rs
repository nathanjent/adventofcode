use std::fs::File;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::fmt;
use std::str;
use std::str::FromStr;
use std::cell::RefCell;
use std::rc::Rc;

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
    Set,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Operator::And => write!(f, "AND"),
            &Operator::Or => write!(f, "OR"),
            &Operator::LShift => write!(f, "LSHIFT"),
            &Operator::RShift => write!(f, "RSHIFT"),
            &Operator::Not => write!(f, "NOT"),
            &Operator::Set => Ok(()),
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum Register {
    Address(String),
    Value(i32),
    Unknown,
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Register::Address(ref a) => write!(f, "{}", a),
            &Register::Value(v) => write!(f, "{}", v),
            &Register::Unknown => Ok(()),
        }
    }
}

#[derive(Debug)]
struct Operation {
    parent: Register,
    operator: Operator,
    left: Register,
    right: Register,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} -> {}", self.left, self.operator, self.right, self.parent)
    }
}

impl Default for Operation {
    fn default() -> Operation {
        Operation {
            parent: Register::Unknown,
            operator: Operator::Set,
            left: Register::Unknown,
            right: Register::Unknown,
        }
    }
}

pub fn assembly_required(file: &str, override_address: &str, override_value: i32) -> i32 {
    let input = File::open(file)
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

        let mut to = String::new();

        let mut prev = Token::Start;
        for token in tokens {
            match prev {
                Token::Start => {
                    match token {
                        Token::Op(o) => {
                            operation.operator = o;
                            prev = Token::Op(Operator::Set);
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
                            prev = Token::Op(Operator::Set);
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
                            match m {
                                Register::Address(a) => {
                                    operation.parent = Register::Address(a.clone());
                                    to = a;
                                },
                                _ => {},
                            }
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

    let registers: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));
    registers.borrow_mut().insert(String::from(override_address), override_value);
    calculate("a".to_string(), &operations, &registers).unwrap()
}

fn calculate(search_key: String, operations: &HashMap<String, Operation>, registers: &Rc<RefCell<HashMap<String, i32>>>) -> Option<i32> {
    //std::thread::sleep(std::time::Duration::from_millis(70));
    let operation = operations.get(&search_key.clone()).expect("No operation at that location!");
    let register = registers.borrow().get(&search_key.clone()).cloned();
    match register {
        Some(v) => {
//            println!("Already calculated: {} = {}", operation.parent, v);
            return Some(v)
        },
        None => {},
    }
    println!("{}", operation);
    let l = match operation.left {
        Register::Address(ref a) => {
            calculate(a.clone(), &operations, registers)
        },
        Register::Value(v) => {
            Some(v)
        },
        Register::Unknown => None,
    };
    let r = match operation.right {
        Register::Address(ref a) => {
            calculate(a.clone(), &operations, registers)
        },
        Register::Value(v) => {
            Some(v)
        },
        Register::Unknown => None,
    };

    match operation.operator {
        Operator::And => {
            let l = l.unwrap();
            let r = r.unwrap();
            let v = l & r;
//            println!("*{} = {} & {} <- {}", v,  l, r, operation);
            registers.borrow_mut().insert(search_key.clone(), v);
            Some(v)
        },
        Operator::Or => {
            let l = l.unwrap();
            let r = r.unwrap();
            let v = l | r;
//            println!("*{} = {} | {} <- {}", v,  l, r, operation);
            registers.borrow_mut().insert(search_key.clone(), v);
            Some(v)
        },
        Operator::LShift => {
            let l = l.unwrap();
            let r = r.unwrap();
            let v = l << r;
//            println!("*{} = {} << {} <- {}", v,  l, r, operation);
            Some(v)
        },
        Operator::RShift => {
            let l = l.unwrap();
            let r = r.unwrap();
            let v = l >> r;
//            println!("*{} = {} >> {} <- {}", v,  l, r, operation);
            Some(v)
        },
        Operator::Not => {
            let r = r.unwrap();
            let v = !r;
//            println!("*{} = !{} <- {}", v, r, operation);
            Some(v)
        },
        Operator::Set => {
            let l = l.unwrap();
//            println!("*{} <- {}", l, operation);
            Some(l)
        },
    }
}
