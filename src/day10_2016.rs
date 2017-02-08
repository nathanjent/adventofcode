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
    let mut output_bins = HashMap::new();
    let mut input_bins = HashMap::new();
    for line in lines {
        let mut words = line.split_whitespace();
        loop {
            if let Some(word) = words.next() {
                match word {
                    "value" => {
                        if let Some(value) = words.next() {
                            if let Ok(value) = value.parse::<usize>() {
                                if let Some(id) = words.nth(3) {
                                    if let Ok(id) = id.parse::<usize>() {
                                        let input_bin = input_bins.entry(value).or_insert(InputBin::new());
                                        input_bin.chip = Some(Microchip(value));
                                        let bot = bots.entry(id).or_insert(Bot::new());
                                        bot.get.push(Get {
                                            from: Some(Entity::InputBin(value)),
                                        });
                                    }
                                }
                            }
                        }
                    },
                    "bot" => {
                        if let Some(bot_id) = words.next() {
                            if let Ok(bot_id) = bot_id.parse::<usize>() {
                                if let Some(low) = words.nth(3) {
                                    if let Some(low_id) = words.next() {
                                        if let Ok(low_id) = low_id.parse::<usize>() {
                                            if let Some(high) = words.nth(3) {
                                                if let Some(high_id) = words.next() {
                                                    if let Ok(high_id) = high_id.parse::<usize>() {
                                                        let mut put = Put::new();
                                                        match low {
                                                            "bot" => {
                                                                put.low = Some(Entity::Bot(low_id));
                                                            }
                                                            "output" => {
                                                                output_bins.entry(low_id)
                                                                    .or_insert(OutputBin::new());
                                                                put.low = Some(Entity::OutputBin(low_id));
                                                            }
                                                            _ => {},
                                                        };
                                                        match high {
                                                            "bot" => {
                                                                put.high = Some(Entity::Bot(high_id));
                                                            }
                                                            "output" => {
                                                                output_bins.entry(high_id)
                                                                    .or_insert(OutputBin::new());
                                                                put.high = Some(Entity::OutputBin(high_id));
                                                            }
                                                            _ => {},
                                                        };
                                                        let bot = bots.entry(bot_id).or_insert(Bot::new());
                                                        bot.put.push(put);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    },
                    _ => {},
                }
            } else {
                break
            }
        }
    }
    //println!("{:?}", bots);
    //println!("{:?}", output_bins);
    //println!("{:?}", input_bins);

    println!("{:?}", bots.len());
    'instructions: loop {
        for bot_id in 0..bots.len() {
            if let Some(mut bot) = bots.get_mut(&bot_id) {
                //println!("{}, {:?}", bot_id, bot);
                if let Some(ref mut get) = bot.get.pop() {
                    println!("bot {} > {:?}", bot_id, get);
                    if let Some(Entity::InputBin(ref from)) = get.from {
                        if let Some(input_bin) = input_bins.remove(&from) {
                            if let Some(chip) = input_bin.chip {
                                bot.chips.push(chip);
                            }
                        }
                    }
                }
                if bot.chips.len() > 1 {
                    if let Some(ref mut put) = bot.put.pop() {
                        //TODO need to move chip from bot to another bot or output bin & remove reference
                        println!("bot {} > {:?}", bot_id, put);
                    }
                }
                if bot.chips.iter().any(|chip| chip.0 == 61)
                    && bot.chips.iter().any(|chip| chip.0 == 17)
                {
                    break 'instructions
                }
            }
        }
    }

    42
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Debug)]
struct Microchip(usize);

#[derive(Debug)]
struct Bot {
    chips: Vec<Microchip>,
    get: Vec<Get>,
    put: Vec<Put>,
}

#[derive(Debug)]
struct Get {
    from: Option<Entity>,
}

#[derive(Debug)]
struct Put {
    low: Option<Entity>,
    high: Option<Entity>,
}

#[derive(Debug)]
struct InputBin {
    chip: Option<Microchip>,
}

#[derive(Debug)]
struct OutputBin {
    chip: Option<Microchip>,
}

#[derive(Debug)]
enum Entity {
    Bot(usize),
    OutputBin(usize),
    InputBin(usize),
}

impl Bot {
    fn new() -> Self {
        Bot {
            chips: Vec::new(),
            get: Vec::new(),
            put: Vec::new(),
        }
    }

    fn low(&mut self) -> Option<Microchip> {
        self.chips.iter().min().cloned()
    }

    fn high(&mut self) -> Option<Microchip> {
        self.chips.iter().max().cloned()
    }
}

impl InputBin {
    fn new() -> Self {
        InputBin {
            chip: None,
        }
    }
}

impl OutputBin {
    fn new() -> Self {
        OutputBin {
            chip: None,
        }
    }
}

impl Put {
    fn new() -> Self {
        Put {
            low: None,
            high: None,
        }
    }
}
