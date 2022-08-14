use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Point = (i32, i32);

#[allow(dead_code)]
fn parse_wires(file: &str) -> Option<(Vec<Point>, Vec<Point>)> {
    let input = File::open(file).expect("File open fail.");
    let mut buf = String::new();
    let _ = BufReader::new(input).read_to_string(&mut buf);
    let wire_str = buf.trim();
    if let Some(i) = wire_str.find('\n') {
        let (w1, w2) = wire_str.split_at(i);
        return Some((wire_parse(w1), wire_parse(w2)));
    }
    None
}

#[allow(dead_code)]
fn wire_parse(wire: &str) -> Vec<Point> {
    let mut prev_coordinate = (0, 0);
    wire.trim().split(',')
        .map(|s| s.split_at(1))
        .filter_map(|(d, s)| {
            if let Ok(n) = str::parse::<i32>(s) {
                return Some((d, n));
            }
            None
        })
    .map(|(d, n)| {
        let (x, y) = prev_coordinate;
        let coordinate = match d {
            "R" => (x + n, y),
            "L" => (x - n, y),
            "U" => (x, y + n),
            "D" => (x, y - n),
            _ => panic!("Syntax error!"),
        };
        prev_coordinate = coordinate;
        println!("{}{} ({}, {}) -> {:?}", d, n, x, y, prev_coordinate);
        ((x, y), coordinate)
    })
    .flat_map(|((x1, y1), (x2, y2))| {
        let mut segment: Vec<(i32, i32)> = Vec::new();
        for i in x1..=x2 {
            for j in y1..=y2 {
                println!("({}, {})", i, j);
                segment.push((i, j));
            }
        }
        segment
    })
    .collect()
}

fn find_closest_wire_cross(wire1: &Vec<Point>, wire2: &Vec<Point>) -> i32 {
    let mut least_distance = i32::max_value();
    for (x1, y1) in wire1 {
        for (x2, y2) in wire2 {
            println!("({:?}, {:?}) ({:?}, {:?})", x1, y1, x2, y2);
           if x1 == x2 && y1 == y2 {
               let distance = x1.abs() + y1.abs();
               if distance < least_distance {
                   least_distance = distance;
               }
               if distance == 0 {
                   return least_distance;
               }
           }
        }
    }
    least_distance
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_base1() {
        let wire1 = crate::day03::wire_parse("R8,U5,L5,D3");
        let wire2 = crate::day03::wire_parse("U7,R6,D4,L4");
        assert_eq!(crate::day03::find_closest_wire_cross(&wire1, &wire2), 42);
    }

    //#[test]
    fn test1() {
        if let Some((wire1, wire2)) = crate::day03::parse_wires("../../input/2019/day03.txt") {
            assert_eq!(crate::day03::find_closest_wire_cross(&wire1, &wire2), 42);
        } else {
            assert!(false);
        }
    }
}
