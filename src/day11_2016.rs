use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn rtg_1(file: &str) -> usize {
    process(file)
}

pub fn rtg_2(file: &str) -> usize {
    process(file)
}

fn process(file: &str) -> usize {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let lines: Vec<String> = reader.lines()
        .filter_map(Result::ok)
        .collect();

    for token in lines.iter().map(|line| {
        line.split_whitespace()
            .filter_map(|w| {
                match w {
                    "first" => Some(Token::Floor(1)),
                    "second" => Some(Token::Floor(2)),
                    "third" => Some(Token::Floor(3)),
                    "fourth" => Some(Token::Floor(4)),
                    "promethium" => Some(Token::PromethiumGenerator),
                    "cobalt" => Some(Token::CobaltGenerator),
                    "curium" => Some(Token::CuriumGenerator),
                    "ruthenium" => Some(Token::RutheniumGenerator),
                    "promethium-compatible" => Some(Token::PromethiumMicrochip),
                    "cobalt-compatible" => Some(Token::CobaltMicrochip),
                    "curium-compatible" => Some(Token::CuriumMicrochip),
                    "ruthenium-compatible" => Some(Token::RutheniumMicrochip),
                    _ => None,
                }
            })
        .collect::<Vec<Token>>()
    })
    .flat_map(|v| {
        v.into_iter()
    })
    {
        match token {
            Token::Floor(n) => {},
            Token::PromethiumGenerator => {},
            Token::CobaltGenerator => {},
            Token::CuriumGenerator => {},
            Token::RutheniumGenerator => {},
            Token::PromethiumMicrochip => {},
            Token::CobaltMicrochip => {},
            Token::CuriumMicrochip => {},
            Token::RutheniumMicrochip => {},
        }
    }
    42
}

#[derive(Debug, Clone)]
enum Token {
    Floor(usize),
    PromethiumGenerator,
    CobaltGenerator,
    CuriumGenerator,
    RutheniumGenerator,
    PromethiumMicrochip,
    CobaltMicrochip,
    CuriumMicrochip,
    RutheniumMicrochip,
}
