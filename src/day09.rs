use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn single_night(file: &str) -> i64 {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        // println!("{}", s);

        let words: Vec<&str> = line.split_whitespace().collect();
        let from = Node(String::from(words[0]), i64::max_value());
        let to = Node(String::from(words[2]), i64::max_value());
        let distance = words[4]
            .parse::<i64>()
            .expect("parse error");

        nodes.push(from.clone());
        nodes.push(to.clone());
        edges.push((from.clone(), to.clone(), distance));
        edges.push((to, from, distance)); // bi-directed
    }
    // remove duplicate nodes
    nodes.sort_by(|a, b| a.0.cmp(&b.0));
    nodes.dedup();


    println!("nodes");
    for node in nodes.clone() {
        println!("{:?}", node);
    }

    let node_list = nodes.clone();
    let mut results = Vec::new();
    for n in node_list {
        // make n node source
        let mut nodes = nodes.clone();
        if let Ok(n_index) = nodes.binary_search(&n) {
            let mut node = nodes.get_mut(n_index).unwrap();
            *node = Node(n.0.clone(), 0);
        }
        println!("calculating next...");
        results.push(dijkstra(nodes, edges.clone()));
    }

    for result in results.clone() {
        let (route, _) = result;
        for node in route {
            println!("{:?}->", node);
        }
    }
    // sort reversed and pop min
    results.sort_by(|a, b| b.1.cmp(&a.1));
    let (_, min) = results.pop().unwrap();
    min
}

fn dijkstra(mut nodes: Vec<Node>, edges: Vec<(Node, Node, i64)>) -> (Vec<Node>, i64) {
    let mut out: Vec<Node> = Vec::new();
    loop {
        if nodes.is_empty() {
            let mut sum = 0;
            for node in out.clone() {
                sum += node.1;
                println!("{:?}", node);
            }
            return (out, sum);
        }
        println!("sorted nodes");
        nodes.sort_by(|a, b| b.1.cmp(&a.1));
        for node in nodes.clone() {
            println!("{:?}", node);
        }

        let node = nodes.pop().unwrap();
        println!("pop min {:?}", node);
        println!("edges length {:?}", edges.len());

        // for each neighbor of n
        for edge in edges.clone().iter() {
            let (ref from, ref to, ref dist) = *edge;
            if from == &node {
                if let Ok(neighbor_index) = nodes.binary_search_by(|n| n.0.cmp(&to.0)) {
                    println!("edge {:?} {:?} {:?}", from.0, to.0, dist);
                    let mut neighbor = nodes.get_mut(neighbor_index).unwrap();
                    let alt = node.1 + *dist;
                    // assign new weight to neighbor
                    if alt < neighbor.1 {
                        println!("{:?} <- {:?}", neighbor.0, alt);
                        *neighbor = Node(to.0.clone(), alt);
                    }
                }
            }
        }
        out.push(node);
    }
}

#[derive(Debug, Eq, Ord, PartialOrd)]
struct Node(String, i64);

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.0.eq(&other.0)
    }
    fn ne(&self, other: &Node) -> bool {
        !self.0.eq(&other.0)
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Node(self.0.clone(), self.1)
    }
}

