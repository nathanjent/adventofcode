use md5;
use std::fs::File;
use std::io::prelude::*;

pub fn ideal_stocking_stuffer(file: &str, leading_zeros: usize) -> i32 {
    let mut file = File::open(file).expect("File open fail.");
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("File read fail.");
    let mut input = String::from(input.trim());

    let mut leading = String::new();
    for _ in 1..(leading_zeros + 1) {
        leading.push('0');
    }

    let file_len = input.len();
    let mut num = 0;
    loop {
        let test: [u8; 16];
        let num_str = num.to_string();
        input.push_str(&num_str);
        // println!("{}", input);
        {
            test = md5::compute(input.as_bytes());
            let mut out = String::new();
            for c in test.iter() {
                if *c < 0xf {
                    out.push('0');
                }
                out.push_str(&format!("{:x}", c));
            }
            if out.starts_with(leading.as_str()) {
                println!("{} answer!", input);
                println!("{} ", out);
                return num;
            }
            // println!("{}", out);
        }
        // Reset input
        input.truncate(file_len);
        num += 1;
    }
}
