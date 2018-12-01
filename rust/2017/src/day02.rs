use csv;

fn corruption_check_1(file: &str) -> u32 {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_path(file)
        .expect("File open fail.");

    let mut sum = 0;
    for result in reader.deserialize() {
        let record: Vec<Option<u32>> =  result.expect("Record error");
        let record: Vec<u32> = record.into_iter().filter_map(|r| r).collect();
        println!("{:?}", &record);
        if let Some(min) = record.iter().min() {
            if let Some(max) = record.iter().max() {
                println!("{:?},{:?}", max, min);
                sum += max - min;
            }
        }
    }
    sum
}

fn corruption_check_2(file: &str) -> u32 {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_path(file)
        .expect("File open fail.");

    let mut sum = 0;
    for result in reader.deserialize() {
        let record: Vec<Option<u32>> =  result.expect("Record error");
        let record: Vec<u32> = record.into_iter().filter_map(|r| r).collect();
        println!("{:?}", &record);

        'greedy: for n in record.iter() {
            for m in record.iter()
                .filter(|&m| n != m)
                .filter(|&m| n % m == 0)
            {
                sum += n / m;
                break 'greedy;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_base1() {
        assert_eq!(::day02::corruption_check_1("input/day02_base.txt"), 18);
    }

    #[test]
    fn test_base2() {
        assert_eq!(::day02::corruption_check_2("input/day02_base.txt"), 9);
    }

    #[test]
    fn test1() {
        assert_eq!(::day02::corruption_check_1("input/day02.txt"), 42378);
    }

    #[test]
    fn test2() {
        assert_eq!(::day02::corruption_check_2("input/day02.txt"), 246);
    }
}
