use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[allow(dead_code)]
fn intcode_parse(file: &str) -> Vec<u32> {
    let input = File::open(file).expect("File open fail.");
    let mut buf = String::new();
    let _ = BufReader::new(input).read_to_string(&mut buf);
    buf.trim().split(',')
        .filter_map(|s| {
            str::parse::<u32>(s).ok()
        }).collect()
}

#[allow(dead_code)]
fn intcode_runner(mut prog: Vec<u32>) -> u32 {
    let mut pc = 0;
    loop {
        //println!("{:?}", prog);
        let op = prog[pc];
        //println!("pc: {} op: {}", pc, op);
        match op {
            1 => {
                let left = prog[pc + 1];
                let right = prog[pc + 2];
                let store = prog[pc + 3];
                //println!("left: {} right: {} store {}", left, right, store);
                prog[store as usize] = prog[left as usize] + prog[right as usize];
                //println!("{1} + {2} = {0}", prog[store as usize], prog[left as usize], prog[right as usize]);
                pc = pc + 4;
            },
            2 => {
                let left = prog[pc + 1];
                let right = prog[pc + 2];
                let store = prog[pc + 3];
                //println!("left: {} right: {} store {}", left, right, store);
                prog[store as usize] = prog[left as usize] * prog[right as usize];
                //println!("{1} + {2} = {0}", prog[store as usize], prog[left as usize], prog[right as usize]);
                pc = pc + 4;
            },
            99 => {
                break;
            },
            _ => panic!("Intcode program error"),
        }
    }
    prog[0]
}

#[allow(dead_code)]
fn concatenated_inputs_from_output_compute(output: u32, prog: Vec<u32>) -> Option<u32> {
    if let Some((m, n)) = inputs_from_output_compute(output, prog) {
        Some(100 * m + n)
    } else {
        None
    }
}

#[allow(dead_code)]
fn inputs_from_output_compute(output: u32, prog: Vec<u32>) -> Option<(u32, u32)> {
    for i in 0..100 {
        for j in 0..100 {
            let mut mod_prog = prog.clone();
            mod_prog[1] = i;
            mod_prog[2] = j;
            if intcode_runner(mod_prog) == output {
                return Some((i, j));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_base0() {
        let prog = crate::day02::intcode_parse("../../input/2019/day02_base.txt");
        assert_eq!(crate::day02::intcode_runner(prog), 3500);
    }

    #[test]
    fn test_base1() {
        let prog = vec![
            1,0,0,0,99
        ];
        assert_eq!(crate::day02::intcode_runner(prog), 2);
    }

    #[test]
    fn test_base2() {
        let prog = vec![
            2,3,0,3,99
        ];
        assert_eq!(crate::day02::intcode_runner(prog), 2);
    }

    #[test]
    fn test_base3() {
        let prog = vec![
            2,4,4,5,99,0
        ];
        assert_eq!(crate::day02::intcode_runner(prog), 2);
    }

    #[test]
    fn test_base4() {
        let prog = vec![
            1,1,1,4,99,5,6,0,99
        ];
        assert_eq!(crate::day02::intcode_runner(prog), 30);
    }

    #[test]
    fn test1() {
        let mut prog = crate::day02::intcode_parse("../../input/2019/day02.txt");
        assert_eq!(crate::day02::intcode_runner(prog.clone()), 29848);
        prog[1] = 12;
        prog[2] = 2;
        assert_eq!(crate::day02::intcode_runner(prog), 3716250);
    }

    #[test]
    fn test2() {
        let prog = crate::day02::intcode_parse("../../input/2019/day02.txt");
        assert_eq!(crate::day02::concatenated_inputs_from_output_compute(19690720, prog), Some(6472));
    }
}
