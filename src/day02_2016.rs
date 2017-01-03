use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn bathroom_security_1(file: &str) -> String {
    let mut keypad = Keypad::new(vec![0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9], 3, 1, 1);
    let keys = parse_keypass(file, &mut keypad)
        .iter()
        .map(|n| format!("{:X}", n))
        .collect::<String>();
    println!("{}", keys);
    keys
}

pub fn bathroom_security_2(file: &str) -> String {
    let mut keypad = Keypad::new(vec![0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x2, 0x3, 0x4, 0x0, 0x5, 0x6,
                                      0x7, 0x8, 0x9, 0x0, 0xA, 0xB, 0xC, 0x0, 0x0, 0x0, 0xD, 0x0,
                                      0x0],
                                 5,
                                 0,
                                 2);
    let keys = parse_keypass(file, &mut keypad)
        .iter()
        .map(|n| format!("{:X}", n))
        .collect::<String>();
    println!("{}", keys);
    keys
}

fn parse_keypass(file: &str, keypad: &mut Keypad) -> Vec<i32> {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let lines: Vec<String> = reader.lines()
        .filter_map(Result::ok)
        .collect();

    let mut passkey: Vec<i32> = Vec::new();
    println!("{:?}", keypad);

    for line in lines {
        let instructions = line.chars()
            //.inspect(|op| println!("{:?}", op))
            .collect::<Vec<char>>();
        for instruction in instructions {
            // println!("IN: {:?}", keypad);
            // println!("{:?}", instruction);
            match instruction {
                'U' => keypad.up(),
                'D' => keypad.down(),
                'L' => keypad.left(),
                'R' => keypad.right(),
                _ => {}
            }
            // println!("OUT: {:?}", keypad);
        }
        passkey.push(keypad.get_current());
    }
    passkey
}

#[derive(Debug)]
struct Keypad {
    row: isize,
    col: isize,
    width: usize,
    keys: Vec<i32>,
}

impl Keypad {
    fn new(keys: Vec<i32>, width: usize, row: isize, col: isize) -> Self {
        Keypad {
            keys: keys,
            width: width,
            row: row,
            col: col,
        }
    }

    fn up(&mut self) {
        let row = self.row - 1;
        if row >= 0 {
            let i = row * self.width as isize + self.col;
            if self.keys[i as usize] != 0x0 {
                self.row -= 1;
                //                println!("{:?}", self.get_current());
            }
        }
    }

    fn down(&mut self) {
        let height = self.keys.len() / self.width;
        let row = self.row + 1;
        if row < height as isize {
            let i = row * self.width as isize + self.col;
            if self.keys[i as usize] != 0x0 {
                self.row += 1;
                //                println!("{:?}", self.get_current());
            }
        }
    }

    fn left(&mut self) {
        let col = self.col - 1;
        if col >= 0 {
            let i = self.row * self.width as isize + col;
            if self.keys[i as usize] != 0x0 {
                self.col -= 1;
                //                println!("{:?}", self.get_current());
            }
        }
    }

    fn right(&mut self) {
        let col = self.col + 1;
        if col < self.width as isize {
            let i = self.row * self.width as isize + col;
            if self.keys[i as usize] != 0x0 {
                self.col += 1;
                //                println!("{:?}", self.get_current());
            }
        }
    }

    fn get_current(&self) -> i32 {
        self.keys[self.row as usize * self.width + self.col as usize]
    }
}
