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
    println!("{:?}", edge_cmds);

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
                println!("{} <-> {}", from_idx, to_idx);
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
    println!("{:?}", edge_lengths);
    println!("{:?}", edges);
    (0..nodes.len()).map(|i| dijkstra(&nodes, &edges, &edge_lengths, i)).min().unwrap()
}

fn dijkstra(nodes: &Vec<&String>,
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
    let mut dist = vec![i64::max_value(); nodes.len()];
    let mut prev = vec![None; nodes.len()];

    dist[source] = 0;

    loop {
        if q.iter().all(Option::is_none) {
            break;
        }
        if let Some((min_idx, _)) =
            dist.iter()
                .enumerate()
                .filter(|d| q[d.0].is_some())
                .inspect(|&e| println!("{:?}", e))
                .min_by_key(|&(_, v)| {
                    v
                }) {
            println!("");
            println!("q: {:?}", q);
            println!("distances: {:?},", dist);
            println!("dist[{}] = {}", min_idx, dist[min_idx]);
            let node_idx = q[min_idx];
            q[min_idx] = None;
            if let Some(node_idx) = node_idx {
                if let Some(neighbors) = edges.get(&node_idx) {
                    for neighbor in neighbors.iter().filter(|&&n| q[n].is_some()) {
                        let edge_key = (node_idx, *neighbor);
                        if let Some(length) = edge_lengths.get(&edge_key) {
                            let alt = dist[node_idx] + length;
                            println!("neighbor = {:?}", neighbor);
                            println!("alt = {:?}", alt);
                            if alt < dist[*neighbor] {
                                dist[*neighbor] = alt;
                                println!("dist[{}] = {}", neighbor, dist[*neighbor]);
                                prev[*neighbor] = Some(node_idx);
                            }
                        }
                    }
                }
            }
        }
    }
    println!("distances: {:?},", dist);
    println!("path: {:?},", prev);
    dist.iter().sum()
}

#[derive(Debug)]
struct Edge {
    from: usize,
    to: usize,
    length: i64,
}
