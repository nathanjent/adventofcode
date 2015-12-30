extern crate md5;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut input = File::open("input.txt")
        .ok()
        .expect("File open fail.");
    let mut buf = String::new();
    input.read_to_string(&mut buf) 
        .ok()
        .expect("File read fail.");

    let input_len = buf.len();
    println!("{}", buf);
    let mut num = 0;
    loop {
        let test: [u8; 16];
        let num_str = num.to_string();
        buf.push_str(&num_str);
        //println!("{}", buf);
        {
            test = md5::compute(buf.as_bytes());
            let mut out = String::new();
            for c in test.iter() {
                if *c < 0xf {
                    out.push('0');
                }
                out.push_str(&format!("{:x}", c));
            }
            //println!("{}", out);
            if out.starts_with("000000") {
                println!("{} found!", buf);
                println!("{} ", out);
                break;
            }
        }
        buf.truncate(input_len);
        num += 1;
    }
}
