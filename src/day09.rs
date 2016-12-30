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

    let mut edge_distances = HashMap::new();
    let mut edges = HashMap::new();
    for &(ref from, ref to, ref distance) in edge_cmds.iter() {
        if let Ok(from_idx) = nodes.binary_search(&&from) {
            if let Ok(to_idx) = nodes.binary_search(&&to) {
                println!("{} <-> {}", from_idx, to_idx);
                {
                    edge_distances.insert((from_idx, to_idx), *distance);
                    let mut neighbors = edges.entry(from_idx).or_insert(Vec::new());
                    neighbors.push(to_idx);
                }
                {
                    // bi-directional
                    edge_distances.insert((to_idx, from_idx), *distance);
                    let mut neighbors = edges.entry(to_idx).or_insert(Vec::new());
                    neighbors.push(from_idx);
                }
            }
        }
    }
    println!("{:?}", edge_distances);
    println!("{:?}", edges);

    //let node_list = nodes.clone();
    //let mut results = Vec::new();
    //for n in node_list {
    //    // make n node source
    //    let mut nodes = nodes.clone();
    //    if let Ok(n_index) = nodes.binary_search(&n) {
    //        let mut node = nodes.get_mut(n_index).unwrap();
    //        *node = Node::new(n.name, 0);
    //    }
    //    println!("calculating next...");
    //    results.push(dijkstra(nodes, edge_distances.clone()));
    //}

    //println!("results");
    //for result in results.clone() {
    //    println!("{}", result);
    //}
    //results.iter().cloned().min().unwrap()
    dijkstra(nodes, edges, edge_distances, 0)
}

fn dijkstra(
    nodes: Vec<&String>,
    edges: HashMap<usize, Vec<usize>>,
    edge_distances: HashMap<(usize, usize), i64>,
    source: usize) -> i64 {

    let mut q = nodes.iter().enumerate().map(|en| { let (i, _) = en; i }).collect::<Vec<usize>>();
    let mut dist = vec![i64::max_value();nodes.len()];
    let mut prev = Vec::new();
    dist[source] = 0;
    println!("{:?}", q);
    println!("{:?}", dist);
//    let mut out: Vec<Node> = Vec::new();
    loop {
        if q.is_empty() {
            break;
        }
//        println!("\nsort nodes");
//        nodes.sort_by(|a, b| b.distance.cmp(&a.distance));
//
        if let Some(min) = edge_distances.iter().map(|ed| {
            ed
        }).min() {
            let node_idx = q.remove(min).unwrap();

            if let Some(neighbors) = edges.get(&node_idx) {
                for neighbor in neighbors {
                    let edge_key = (node_idx, *neighbor);
                    if let Some(distance) = edge_distances.get(&edge_key) {
                        let alt = dist[node_idx] + distance;
                        if alt < dist[*neighbor] {
                            dist[*neighbor] = alt;
                            //prev[neighbor] = node_idx;
                            prev.push(node_idx);
                        }
                    }
                }
            }
        }
//        println!("pop min {:?}", node);
//
//        nodes.sort_by(|a, b| a.name.cmp(&b.name));
//
//        // for each neighbor of n
//        for edge in edge_distances.clone().iter() {
//            let Edge { ref a, ref b, ref len } = *edge;
//            if a == &node.name {
//                if let Ok(neighbor_index) = nodes.binary_search_by(|n| n.name.cmp(&b)) {
//                    println!("edge {:?} {:?} {:?}", a, b, len);
//                    let mut neighbor = nodes.get_mut(neighbor_index).unwrap();
//                    println!("neighbor {:?}", &neighbor);
//                    let alt = node.distance + *len;
//                    // assign new weight to neighbor
//                    if alt < neighbor.distance {
//                        println!("{:?} <- {:?}", neighbor.name, alt);
//                        // *neighbor = Node::new(b, alt);
//                    } else {
//                        println!("{:?} - {:?}", neighbor.name, neighbor.distance);
//                    }
//                }
//            }
//        }
//        out.push(node.clone());
    }
    println!("distances: {:?},", dist);
    println!("path: {:?},", prev);
//    out.iter().fold(0, |acc, ref n| acc + n.distance)
    42
}

#[derive(Debug)]
struct Edge {
    from: usize,
    to: usize,
    length: i64,
}
