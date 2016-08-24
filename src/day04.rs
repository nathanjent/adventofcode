use md5;
use std::fs::File;
use std::io::prelude::*;

pub fn ideal_stocking_stuffer(file: &str, leading_zeros: usize) -> i32 {
    let mut input = File::open(file)
        .expect("File open fail.");
    let mut buf = String::new();
    input.read_to_string(&mut buf) 
        .expect("File read fail.");
    
    let mut leading = String::new();
    for _ in 1..leading_zeros {
        leading.push('0');
    }

    let input_len = buf.len();
    let mut num = 0;
    loop {
        let test: [u8; 16];
        let num_str = num.to_string();
        buf.push_str(&num_str);
        {
            test = md5::compute(buf.as_bytes());
            let mut out = String::new();
            for c in test.iter() {
                if *c < 0xf {
                    out.push('0');
                }
                out.push_str(&format!("{:x}", c));
            }
            if out.starts_with(&leading) {
                //println!("{} found!", buf);
                //println!("{} ", out);
                return out.trim_left_matches(&leading).to_string().parse::<i32>().unwrap()
            }
        }
        buf.truncate(input_len);
        num += 1;
    }
}
