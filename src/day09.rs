use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

pub fn single_night(file: &str) -> i64 {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let edge_cmds = reader.lines()
        .filter_map(Result::ok)
        .map(|line| {
            let words = line.split_whitespace().collect::<Vec<&str>>();
            let from = words[0].to_owned();
            let to = words[2].to_owned();
            let distance = words[4].parse::<i64>().expect("parse error");

            (from, to, distance)
        })
        .collect::<Vec<(String, String, i64)>>();
//    println!("{:?}", edge_cmds);

    let mut nodes = Vec::new();
    for &(ref from, ref to, _) in edge_cmds.iter() {
        nodes.push(from);
        nodes.push(to);
    }
    nodes.sort();
    nodes.dedup();
    println!("{:?}", nodes);

    let mut edge_lengths = HashMap::new();
    let mut edges = HashMap::new();
    for &(ref from, ref to, ref distance) in edge_cmds.iter() {
        if let Ok(from_idx) = nodes.binary_search(&&from) {
            if let Ok(to_idx) = nodes.binary_search(&&to) {
//                println!("{} <-> {}", from_idx, to_idx);
                {
                    edge_lengths.insert((from_idx, to_idx), *distance);
                    let mut neighbors = edges.entry(from_idx).or_insert(Vec::new());
                    neighbors.push(to_idx);
                }
                {
                    // bi-directional
                    edge_lengths.insert((to_idx, from_idx), *distance);
                    let mut neighbors = edges.entry(to_idx).or_insert(Vec::new());
                    neighbors.push(from_idx);
                }
            }
        }
    }
    println!("{:?}", edges);
    println!("{:?}", edge_lengths);
    (0..nodes.len()).map(|i| salesman(&nodes, &edges, &edge_lengths, i))
        .inspect(|distance| println!("{}", distance))
        .min().unwrap()
}

fn salesman(nodes: &Vec<&String>,
            edges: &HashMap<usize, Vec<usize>>,
            edge_lengths: &HashMap<(usize, usize), i64>,
            source: usize)
            -> i64 {

    let mut q = nodes.iter()
        .enumerate()
        .map(|en| {
            let (i, _) = en;
            Some(i)
        })
    .collect::<Vec<Option<usize>>>();

    let mut visited = vec![false; nodes.len()];
    visited[source] = true;
    let mut curr = source;
    let mut sum = 0;
    let mut count = 0;

    loop {
        if visited.iter().all(|&b| b) {
            break;
        }
        count += 1;
        if count == 500 {
            return i64::max_value();
        }
        if let Some(node_idx) = q[curr] {
            if let Some(neighbors) = edges.get(&node_idx) {
                if let Some((&min_idx, &length)) = neighbors.iter()
                    .filter(|&&n| q[n].is_some())
                        .map(|neighbor| {
                            let edge_key = (node_idx, *neighbor);
                            let length = edge_lengths.get(&edge_key).unwrap();
                            (neighbor, length)
                        })
                .min_by_key(|&(_, &length)| length)
                {
                    println!("min: {:?}", min_idx);
                    visited[min_idx] = true;
                    curr = min_idx;
                    sum += length;
                    count = 0;
                }
            }
        }
    }
    sum
}

#[derive(Debug)]
struct Edge {
    from: usize,
    to: usize,
    length: i64,
}
