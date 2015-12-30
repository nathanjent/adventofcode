use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

fn main() {
    let mut input = File::open("input.txt")
        .ok()
        .expect("File open fail.");
    let mut buffer = String::new();
    input.read_to_string(&mut buffer) 
        .ok()
        .expect("File read fail.");
    let mut total = 0;
    let mut ribbon = 0;
    for line in buffer.lines() {
        let v: Vec<&str> = line.split('x').collect();
        let l = i32::from_str(v[0]).unwrap();
        let w = i32::from_str(v[1]).unwrap();
        let h = i32::from_str(v[2]).unwrap();
        let mut dims = vec![l, w, h];
        dims.sort();
        let mut min = 1000;
        let s0 = l * w;
        if s0 < min { min = s0; }
        let s1 = w * h;
        if s1 < min { min = s1; }
        let s2 = h * l;
        if s2 < min { min = s2; }

        let sqft = (2 * (s0+s1+s2)) + min;
        total += sqft;
        //println!("2x({}x{}x{})+{}={}", l, w, h, min, sqft);
        ribbon += (l*w*h) + (2*dims[0] + 2*dims[1]);
    }
    println!("{} sqft of paper", total);
    println!("{} ft of ribbon", ribbon);
}
