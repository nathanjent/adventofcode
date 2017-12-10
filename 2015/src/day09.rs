use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

pub fn single_night_1(file: &str) -> i64 {
    let graph = generate_graph(file);
    (0..graph.nodes.len()).map(|i| salesman(&graph, i, true))
        .inspect(|distance| println!("{}", distance))
        .min().unwrap()
}

pub fn single_night_2(file: &str) -> i64 {
    let graph = generate_graph(file);
    (0..graph.nodes.len()).map(|i| salesman(&graph, i, false))
        .inspect(|distance| println!("{}", distance))
        .max().unwrap()
}

fn generate_graph(file: &str) -> Graph {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let mut nodes = Vec::new();
    let edge_cmds = reader.lines()
        .filter_map(Result::ok)
        .map(|line| {
            let words = line.split_whitespace().collect::<Vec<&str>>();
            let from = words[0].to_owned();
            let to = words[2].to_owned();
            let distance = words[4].parse::<i64>().expect("parse error");

            nodes.push(from.clone());
            nodes.push(to.clone());
            (from, to, distance)
        })
        .collect::<Vec<(String, String, i64)>>();
//    println!("{:?}", edge_cmds);
    nodes.sort();
    nodes.dedup();
    println!("nodes: {:?}", nodes);

    let mut edge_lengths = HashMap::new();
    let mut edges = HashMap::new();
    for &(ref from, ref to, ref distance) in edge_cmds.iter() {
        if let Ok(from_idx) = nodes.binary_search(&&from) {
            if let Ok(to_idx) = nodes.binary_search(&&to) {
//                println!("{} <-> {}", from_idx, to_idx);
                {
                    edge_lengths.insert((from_idx, to_idx), *distance);
                    let neighbors = edges.entry(from_idx).or_insert(Vec::new());
                    neighbors.push(to_idx);
                }
                {
                    // bi-directional
                    edge_lengths.insert((to_idx, from_idx), *distance);
                    let neighbors = edges.entry(to_idx).or_insert(Vec::new());
                    neighbors.push(from_idx);
                }
            }
        }
    }
    println!("edges: {:?}", edges);
    println!("edge lengths: {:?}", edge_lengths);
    Graph { nodes: nodes, edges: edges, edge_lengths: edge_lengths }
}

fn salesman(graph: &Graph, source: usize, min: bool) -> i64 {
    let node_idxs = graph.nodes.iter()
        .enumerate()
        .map(|(i, _)| i)
    .collect::<Vec<usize>>();

    let mut visited = vec![false; graph.nodes.len()];
    let mut curr = source;
    visited[curr] = true;
    println!("visit: {:?}", graph.nodes[source]);
    let mut sum = 0;

    loop {
        if visited.iter().all(|&b| b) {
            break;
        }
        let node_idx = node_idxs[curr];
        if let Some(neighbors) = graph.edges.get(&node_idx) {
            if min {
                if let Some((min_idx, &length)) = neighbors.iter()
                    .filter(|&&n| !visited[n])
                    .map(|&neighbor| {
                        let edge_key = (node_idx, neighbor);
                        let length = graph.edge_lengths.get(&edge_key).unwrap();
                        (neighbor, length)
                    })
                    .inspect(|n| println!("{:?}", n))
                    .min_by_key(|&(_, &length)| length) {
                    println!("visit: {:?}", graph.nodes[min_idx]);
                    visited[min_idx] = true;
                    curr = min_idx;
                    sum += length;
                }
            } else {
                if let Some((max_idx, &length)) = neighbors.iter()
                    .filter(|&&n| !visited[n])
                        .map(|&neighbor| {
                            let edge_key = (node_idx, neighbor);
                            let length = graph.edge_lengths.get(&edge_key).unwrap();
                            (neighbor, length)
                        })
                .inspect(|n| println!("{:?}", n))
                    .max_by_key(|&(_, &length)| length) {
                        println!("visit: {:?}", graph.nodes[max_idx]);
                        visited[max_idx] = true;
                        curr = max_idx;
                        sum += length;
                    }
            }
        }
    }
    sum
}

struct Graph {
    nodes: Vec<String>,
    edges: HashMap<usize, Vec<usize>>,
    edge_lengths: HashMap<(usize, usize), i64>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1base() {
        assert_eq!(::day09::single_night_1("input/day09_base.txt"), 605);
    }

    #[test]
    fn test2base() {
        assert_eq!(::day09::single_night_2("input/day09_base.txt"), 982);
    }

    #[test]
    fn test1() {
        assert_eq!(::day09::single_night_1("input/day09.txt"), 251);
    }

    #[test]
    fn test2() {
        assert_eq!(::day09::single_night_2("input/day09.txt"), 0);
    }
}
