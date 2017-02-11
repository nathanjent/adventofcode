use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use std::ops::Mul;

pub fn balance_bots_1(file: &str) -> usize {
    parse_cmds(file, Some((61, 17)))
}

pub fn balance_bots_2(file: &str) -> usize {
    parse_cmds(file, None)
}

fn parse_cmds(file: &str, chip_pair: Option<(usize, usize)>) -> usize {
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
                                        let input_bin = input_bins.entry(value)
                                            .or_insert(InputBin::new());
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
                                                                put.low
                                                                    = Some(Entity::Bot(low_id));
                                                            }
                                                            "output" => {
                                                                output_bins.entry(low_id)
                                                                    .or_insert(OutputBin::new());
                                                                put.low
                                                                    = Some(Entity::OutputBin(low_id));
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

    //println!("{:?}", bots.len());
    let mut idle = 1;
    'instructions: loop {
        for bot_id in 0..bots.len() {
            let mut put_instruction = None;
            if let Some(mut bot) = bots.get_mut(&bot_id) {
                //println!("{}, {:?}", bot_id, bot);
                if let Some(ref mut get) = bot.get.pop() {
                    //println!("Instruction: bot {} > {:?}", bot_id, get);
                    idle += 1;
                    if let Some(Entity::InputBin(ref from)) = get.from {
                        if let Some(input_bin) = input_bins.remove(&from) {
                            if let Some(chip) = input_bin.chip {
                                bot.chips.push(chip);
                            }
                        }
                    }
                }
                if bot.chips.len() > 1 {
                    if let Some(put) = bot.put.pop() {
                        put_instruction = Some(put);
                    }
                }
            }
            if let Some(ref mut put) = put_instruction {
                idle += 1;
                //println!("Instruction: bot {} > {:?}", bot_id, put);
                if let Some(bot) = bots.get(&bot_id) {
                    if let Some((a, b)) = chip_pair {
                        if bot.chips.iter().any(|chip| chip.0 == a)
                            && bot.chips.iter().any(|chip| chip.0 == b)
                        {
                            return bot_id;
                        }
                    }
                }
                if let Some(ref low) = put.low {
                    match *low {
                        Entity::Bot(other_bot_id) => {
                            let mut chip = None;
                            if let Some(mut bot) = bots.get_mut(&bot_id) {
                                //println!("Bot {}, {:?}", bot_id, bot);
                                chip = bot.low();
                                //println!("Bot {}, {:?}", bot_id, bot);
                            }
                            if let Some(mut other_bot) = bots.get_mut(&other_bot_id) {
                                if let Some(chip) = chip {
                                    //println!("Receiver Bot {}, {:?}", other_bot_id, other_bot);
                                    other_bot.chips.push(chip);
                                    //println!("Receiver Bot {}, {:?}", other_bot_id, other_bot);
                                }
                            }
                        },
                        Entity::OutputBin(out_id) => {
                            if let Some(mut out) = output_bins.get_mut(&out_id) {
                                if let Some(mut bot) = bots.get_mut(&bot_id) {
                                    //println!("Receiver OutputBin {}, {:?}", out_id, out);
                                    //println!("Out {}, {:?}", bot_id, bot);
                                    if let Some(chip) = bot.low() {
                                        out.chips.push(chip);
                                    }
                                    //println!("Out {}, {:?}", bot_id, bot);
                                    //println!("Receiver OutputBin {}, {:?}", out_id, out);
                                }
                            }
                        },
                        Entity::InputBin(_) => {},
                    }
                }
                if let Some(ref high) = put.high {
                    match *high {
                        Entity::Bot(other_bot_id) => {
                            let mut chip = None;
                            if let Some(mut bot) = bots.get_mut(&bot_id) {
                                //println!("Bot {}, {:?}", bot_id, bot);
                                chip = bot.high();
                                //println!("Bot {}, {:?}", bot_id, bot);
                            }
                            if let Some(mut other_bot) = bots.get_mut(&other_bot_id) {
                                if let Some(chip) = chip {
                                    //println!("Receiver Bot {}, {:?}", other_bot_id, other_bot);
                                    other_bot.chips.push(chip);
                                    //println!("Receiver Bot {}, {:?}", other_bot_id, other_bot);
                                }
                            }
                        },
                        Entity::OutputBin(out_id) => {
                            if let Some(mut out) = output_bins.get_mut(&out_id) {
                                if let Some(mut bot) = bots.get_mut(&bot_id) {
                                    //println!("Receiver OutputBin {}, {:?}", out_id, out);
                                    //println!("Out {}, {:?}", bot_id, bot);
                                    if let Some(chip) = bot.high() {
                                        out.chips.push(chip);
                                    }
                                    //println!("Out {}, {:?}", bot_id, bot);
                                    //println!("Receiver OutputBin {}, {:?}", out_id, out);
                                }
                            }
                        },
                        Entity::InputBin(_) => {},
                    }
                }
            }
        }
        if idle == 0 {
            break 'instructions
        }
        idle -= 1;
    }
    //for (k, v) in output_bins.iter() {
    //    println!("{:?}: {:?}", k, v);
    //}
    if let Some(&OutputBin { chips: ref c0 } ) = output_bins.get(&0) {
        if let Some(&OutputBin { chips: ref c1 } ) = output_bins.get(&1) {
            if let Some(&OutputBin { chips: ref c2 } ) = output_bins.get(&2) {
                return (c0[0].clone() * c1[0].clone() * c2[0].clone()).0
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
    chips: Vec<Microchip>,
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
        let mut index = 0;
        match self.chips.iter()
            .enumerate()
            .min_by_key(|&(_, c)| {
                c.0
            })
        {
            Some((i, _)) => index = i,
            None => {},
        }
        Some(self.chips.swap_remove(index))
    }

    fn high(&mut self) -> Option<Microchip> {
        let mut index = 0;
        match self.chips.iter()
            .enumerate()
            .max_by_key(|&(_, c)| {
                c.0
            })
        {
            Some((i, _)) => index = i,
            None => {},
        }
        Some(self.chips.swap_remove(index))
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
            chips: Vec::new(),
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

impl Mul for Microchip {
    type Output = Microchip;

    fn mul(self, other: Microchip) -> Self {
        Microchip(self.0 * other.0)
    }
}
