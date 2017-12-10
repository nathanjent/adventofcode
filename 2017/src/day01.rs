use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn captcha_1(file: &str) -> u32 {
    let input = File::open(file).expect("File open fail.");
    let mut reader = BufReader::new(input);

    let mut input: Vec<u8> = Vec::new();
    let _ = reader.read_to_end(&mut input);

    let mut begin = 0u32;
    let mut end = 0u32;
    let sum = input.iter()
        .inspect(|b| print!("{:?},", b))
        .map(|&b| b)
        .filter_map(|b| {
            if b >= ('0' as u8) {
                Some(b - ('0' as u8))
            } else {
                // Drops new line character
                None
            }
        })
        .map(|b| b as u32)
        .enumerate()
        .inspect(|&(i, n)| {
            if i == 0 {
                begin = n;
            }
            end = n;
        })
    .map(|(_, n)| n)
        .fold((0, 0), |(acc, prev), next| {
            if prev == next {
                (acc + prev, next)
            } else {
                (acc, next)
            }
        }).0;
    if begin == end {
        sum + begin
    } else {
        sum
    }
}

fn captcha_2(file: &str) -> u32 {
    let input = File::open(file).expect("File open fail.");
    let mut reader = BufReader::new(input);

    let mut input: Vec<u8> = Vec::new();
    let _ = reader.read_to_end(&mut input);

    let list: Vec<u32> = input.iter()
        .map(|&b| b)
        .filter_map(|b| {
            if b >= ('0' as u8) {
                Some(b - ('0' as u8))
            } else {
                // Drops new line character
                None
            }
        })
        .map(|b| b as u32)
        .enumerate()
        .inspect(|b| print!("{:?},", b))
        .map(|(_, n)| n)
            .collect();


        let len = list.len();
        let compare_distance = list.len() / 2;
        let mut sum = 0;
        for i in 0..len {
            let mut j = i + compare_distance;
            if j >= len {
                j -= len;
            }
            if list[i] == list[j] {
                sum += list[i];
            }
        }
    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_base() {
        assert_eq!(::day01::captcha_1("input/day01_base.txt"), 10);
    }

    #[test]
    fn test1() {
        assert_eq!(::day01::captcha_1("input/day01.txt"), 1251);
    }

    #[test]
    fn test2() {
        assert_eq!(::day01::captcha_2("input/day01.txt"), 1244);
    }
}
