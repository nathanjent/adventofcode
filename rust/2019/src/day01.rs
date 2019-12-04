use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[allow(dead_code)]
fn calculate_fuel_requirement_1(file: &str) -> u32 {
    let input = File::open(file).expect("File open fail.");
    BufReader::new(input).lines()
        .filter_map(Result::ok)
        .filter_map(|line| {
            str::parse::<u32>(&line).ok()
        })
    .map(|n| {
        ((n as f32 / 3.0).floor() - 2.0) as u32
    })
    .sum()
}

#[allow(dead_code)]
fn calculate_fuel_requirement_2(file: &str) -> u32 {
    let input = File::open(file).expect("File open fail.");
    BufReader::new(input).lines()
        .filter_map(Result::ok)
        .filter_map(|line| {
            str::parse::<u32>(&line).ok()
        })
    .map(|n| {
        let mut sum = 0;
        let mut prev = n;
        loop {
            let fuel = (prev as f32 / 3.0).floor() as u32;
            println!("n: {} prev: {} sum: {} fuel: {}", n, prev, sum, fuel);
            if fuel < 2 {
                break;
            }
            prev = fuel - 2;
            sum += prev;
        }
        sum
    })
    .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_base1() {
        assert_eq!(crate::day01::calculate_fuel_requirement_1("../../input/2019/day01_base.txt"), 2 + 2 + 654 + 33583);
    }

    #[test]
    fn test1() {
        assert_eq!(crate::day01::calculate_fuel_requirement_1("../../input/2019/day01.txt"), 3388015);
    }

    #[test]
    fn test_base2() {
        assert_eq!(crate::day01::calculate_fuel_requirement_2("../../input/2019/day01_base.txt"), 2 + 2 + 966 + 50346);
    }

    #[test]
    fn test2() {
        assert_eq!(crate::day01::calculate_fuel_requirement_2("../../input/2019/day01.txt"), 5079140);
    }
}
