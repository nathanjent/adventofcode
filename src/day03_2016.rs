use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn square_triangles_1(file: &str) -> usize {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    reader.lines()
        .filter_map(Result::ok)
        .map(|line| {
            line.split_whitespace()
            .filter_map(|side| {
                side.parse::<i32>().ok()
            })
            .collect::<Vec<i32>>()
        })
        //.inspect(|v| {
        //    for n in v {
        //        print!("{} ", n);
        //    }
        //    println!("");
        //})
        .filter_map(|mut t| {
            t.sort();
            let a = t[0];
            let b = t[1];
            let c = t[2];
            if a + b > c {
                Some(c)
            } else {
                None
            }
        })
        //.inspect(|n| println!("{} ", n))
        .count()
}

pub fn square_triangles_2(file: &str) -> usize {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let mut triple_iter = reader.lines()
        .filter_map(Result::ok)
        .map(|line| {
            line.split_whitespace()
            .filter_map(|side| {
                side.parse::<i32>().ok()
            })
            .collect::<Vec<i32>>()
        });
        let mut rotated = Vec::new();
    loop {
      if let Some(top) = triple_iter.next() {
          if let Some(middle) = triple_iter.next() {
              if let Some(bottom) = triple_iter.next() {
                  let mut left = vec![top[0], middle[0], bottom[0]];
                  let mut center = vec![top[1], middle[1], bottom[1]];
                  let mut right = vec![top[2], middle[2], bottom[2]];
                  left.sort();
                  center.sort();
                  right.sort();
                  rotated.push(left);
                  rotated.push(center);
                  rotated.push(right);
              } else {
                  break;
              }
          } else {
              break;
          }
      } else {
          break;
      }
    }


    rotated.iter().filter_map(|t| {
            let a = t[0];
            let b = t[1];
            let c = t[2];
            if a + b > c {
                Some(c)
            } else {
                None
            }
        })
        .count()
}
