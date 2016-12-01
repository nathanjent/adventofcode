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
        let from = words[0];
        let to = words[2];
        let distance = words[4].parse::<i64>().expect("parse error");

        nodes.push(Node::new(from, i64::max_value()));
        nodes.push(Node::new(to, i64::max_value()));
        edges.push(Edge::new(from, to, distance));
        edges.push(Edge::new(to, from, distance)); // bi-directed
    }
    // remove duplicate nodes
    nodes.sort_by(|a, b| a.name.cmp(&b.name));
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
            *node = Node::new(n.name, 0);
        }
        println!("calculating next...");
        results.push(dijkstra(nodes, edges.clone()));
    }

    println!("results");
    for result in results.clone() {
        println!("{}", result);
    }
    results.iter().cloned().min().unwrap()
}

fn dijkstra(mut nodes: Vec<Node>, edges: Vec<Edge>) -> i64 {
    let mut out: Vec<Node> = Vec::new();
    loop {
        if nodes.is_empty() {
            break;
        }
        println!("\nsort nodes");
        nodes.sort_by(|a, b| b.distance.cmp(&a.distance));

        let node = nodes.pop().unwrap();
        println!("pop min {:?}", node);

        nodes.sort_by(|a, b| a.name.cmp(&b.name));

        // for each neighbor of n
        for edge in edges.clone().iter() {
            let Edge { ref a, ref b, ref len } = *edge;
            if a == &node.name {
                if let Ok(neighbor_index) = nodes.binary_search_by(|n| n.name.cmp(&b)) {
                    println!("edge {:?} {:?} {:?}", a, b, len);
                    let mut neighbor = nodes.get_mut(neighbor_index).unwrap();
                    println!("neighbor {:?}", &neighbor);
                    let alt = node.distance + *len;
                    // assign new weight to neighbor
                    if alt < neighbor.distance {
                        println!("{:?} <- {:?}", neighbor.name, alt);
                        //*neighbor = Node::new(b, alt);
                    } else {
                        println!("{:?} - {:?}", neighbor.name, neighbor.distance);
                    }
                }
            }
        }
        out.push(node.clone());
    }
    print!("Out: ");
    for node in out.clone() {
        print!("{:?},", node);
    }
    out.iter().fold(0, |acc, ref n| acc + n.distance)
}

#[derive(Debug, Eq, Ord, PartialOrd)]
struct Node {
    name: String,
    distance: i64,
}

#[derive(Debug)]
struct Edge {
    a: String,
    b: String,
    len: i64,
}

impl Node {
    fn new<S>(name: S, distance: i64) -> Self where S: Into<String> {
        Node { name: name.into(), distance: distance }
    }
}

impl Edge {
    fn new<S>(a: S, b: S, len: i64) -> Self where S: Into<String> {
        Edge { a: a.into(), b: b.into(), len: len }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.name.eq(&other.name)
    }
    fn ne(&self, other: &Node) -> bool {
        !self.name.eq(&other.name)
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Node { name: self.name.clone(), distance: self.distance }
    }
}

impl Clone for Edge {
    fn clone(&self) -> Self {
        Edge { a: self.a.clone(), b: self.b.clone(), len: self.len }
    }
}

