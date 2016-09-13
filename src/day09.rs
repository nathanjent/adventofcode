use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::cmp::Ordering;

pub fn single_night(file: &str) -> i64 {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

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
        edges.push((from, to, distance));
    }
    // remove duplicate nodes
    nodes.sort_by(|a, b| a.0.cmp(&b.0));
    nodes.dedup();

    // make last node source
    let source = nodes.pop().unwrap();
    let source = Node(source.0, 0);
    nodes.push(source);

    println!("nodes");
    for node in nodes.clone() {
        println!("{:?}", node);
    }

    println!("calculating...");
    let mut out: Vec<Node> = Vec::new();
    loop {
        if nodes.is_empty() {
            let mut sum = 0;
            for node in out {
                sum += node.1;
                println!("{:?}", node);
            }
            return sum;
        }
        println!("sorted nodes");
        nodes.sort_by(|a, b| b.1.cmp(&a.1));
        for node in nodes.clone() {
            println!("{:?}", node);
        }


        let node = nodes.pop().unwrap();
        println!("pop min {:?}", node);

        // for each neighbor of n
        for edge in edges.iter() {
            let (ref from, ref to, ref dist) = *edge;
            if from == &node {
                println!("edge {:?} {:?} {:?}", from.0, to.0, dist);
                let alt = node.1 + *dist;
                if let Ok(neighbor_index) = nodes.binary_search(to) {
                    let mut neighbor = nodes.get_mut(neighbor_index).unwrap();
                    // assign new weight to neighbor
                    if alt < neighbor.1 {
                        println!("New weight {:?} {:?}", neighbor, alt);
                        *neighbor = Node(to.0.clone(), alt);
                    }
                }
            }
        }
        out.push(node);
    }
}
