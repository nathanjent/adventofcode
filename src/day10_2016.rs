use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

pub fn balance_bots_1(file: &str) -> usize {
    parse_cmds(file)
}

pub fn balance_bots_2(file: &str) -> usize {
    parse_cmds(file)
}

fn parse_cmds(file: &str) -> usize {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let lines = reader.lines()
        .filter_map(Result::ok);

    let mut bots = HashMap::new();
    let mut out_bins = HashMap::new();

    let (transfer_tokens, input_tokens): (Vec<Vec<Token>>, Vec<Vec<Token>>) = lines.map(|line| {
            let mut words = line.split_whitespace();
            let mut tokens = Vec::new();
            loop {
                if let Some(w) = words.next() {
                    match w {
                        "value" => {
                            if let Some(v) = words.next() {
                                if let Ok(n) = v.parse::<usize>() {
                                    tokens.push(Token::Value(n))
                                }
                            }
                        }
                        "bot" => {
                            if let Some(v) = words.next() {
                                if let Ok(n) = v.parse::<usize>() {
                                    tokens.push(Token::Bot(n))
                                }
                            }
                        }
                        "output" => {
                            if let Some(v) = words.next() {
                                if let Ok(n) = v.parse::<usize>() {
                                    tokens.push(Token::Output(n))
                                }
                            }
                        }
                        _ => {}
                    };
                } else {
                    return tokens;
                }
            }
        })
        .partition(|ref t| t.len() > 2);

    // Load MicroChips from input bins
    input_tokens.iter().map(|tokens| {
        let mut val;
        match tokens[0] {
            Token::Value(n) => val = MicroChip(n),
            Token::Bot(_) => unreachable!(),
            Token::Output(_) => unreachable!(),
        }
        match tokens[1] {
            Token::Bot(n) => {
                bots.insert(n, Bot { chips: vec![val] });
            }
            Token::Output(_) => unreachable!(),
            Token::Value(_) => unreachable!(),
        }
    });

    // Process transfers
    transfer_tokens.iter().map(|tokens| {
        let low;
        let high;
        match tokens[0] {
            Token::Bot(n) => {
                let bot = bots.entry(n).or_insert(Bot { chips: Vec::new() });
                low = bot.low();
                high = bot.high();
            }
            Token::Value(n) => unreachable!(),
            Token::Output(n) => unreachable!(),
        }
        if let Some(low) = low {
            match tokens[1] {
                Token::Bot(n) => {
                    let mut to = bots.entry(n).or_insert(Bot { chips: Vec::new() });
                    if to.chips.len() > 0 {
                        to.chips.push(low);
                    }
                }
                Token::Output(n) => {
                    let mut to = out_bins.entry(n).or_insert(OutputBin { chip: None });
                    to.chip = Some(low);
                }
                Token::Value(n) => unreachable!(),
            }
        }
        if let Some(high) = high {
            match tokens[2] {
                Token::Bot(n) => {
                    let mut to = bots.entry(n).or_insert(Bot { chips: Vec::new() });
                    if to.chips.len() > 0 {
                        to.chips.push(high);
                    }
                }
                Token::Output(n) => {
                    let mut to = out_bins.entry(n).or_insert(OutputBin { chip: None });
                    to.chip = Some(high);
                }
                Token::Value(n) => unreachable!(),
            }
        }
    });

    42
}

#[derive(Debug)]
enum Token {
    Value(usize),
    Bot(usize),
    Output(usize),
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone)]
struct MicroChip(usize);

struct Bot {
    chips: Vec<MicroChip>,
}

impl Bot {
    fn low(&mut self) -> Option<MicroChip> {
        self.chips.iter().min().cloned()
    }

    fn high(&mut self) -> Option<MicroChip> {
        self.chips.iter().max().cloned()
    }
}

struct OutputBin {
    chip: Option<MicroChip>,
}

trait Entity {}

impl Entity for Bot {}
impl Entity for OutputBin {}
