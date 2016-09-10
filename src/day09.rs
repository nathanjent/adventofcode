use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::cmp::{Ordering, };
use std::collections::{HashMap, HashSet, BinaryHeap};

pub fn single_night(file: &str) -> i64 {
    let input = File::open(file).expect("File open fail.");
    let reader = BufReader::new(input);

    #[derive(Debug, Hash)]
    struct Node(String, i64);

    impl PartialEq for Node {
        fn eq(&self, other: &Node) -> bool {
            self.0.eq(&other.0)
        }
    }

    impl Eq for Node {}

    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    
    impl Ord for Node {
        fn cmp(&self, other: &Node) -> Ordering {
            self.1.cmp(&other.1)
        }
    }

    impl Clone for Node {
        fn clone(&self) -> Self {
            Node(self.0.clone(), self.1)
        }
    }

    let mut nodes = HashSet::new();
    let mut edges = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        // println!("{}", s);

        let words: Vec<&str> = line.split_whitespace().collect();
        let from = Node(String::from(words[0]), i64::min_value());
        let to = Node(String::from(words[2]), i64::min_value());
        let distance = words[4]
            .parse::<i64>().expect("parse error");

        nodes.insert(from.clone());
        nodes.insert(to.clone());
        // negate distance because <BinaryHeap>.pop() returns highest
        edges.insert((from, to), -distance);
    }
    let mut nodes: Vec<Node> = nodes.iter().cloned().collect();
    let mut p_queue = BinaryHeap::new();
    let source = nodes.pop().unwrap();
    let source = Node(source.0, 0);
    nodes.push(source);
    println!("nodes");
    for node in nodes.clone() {
        println!("{:?}", node);
        p_queue.push(node);
    }
    println!("queue");
    for n in p_queue.clone().into_vec() {
        println!("{:?}", n);
    }
    let mut nodes = nodes;
    println!("calculating...");
    loop {
        if p_queue.is_empty() { break; }
        
        let node = p_queue.pop().unwrap();

        println!("Pop {:?}", node);
        // for each neighbor of n
        for (from_to, edge) in edges.iter() {
            let (ref from, ref to) = *from_to;
            if from == &node {
                let alt = node.1 - edge;
                if let Some(neighbor) 
                    = p_queue.clone().iter().find(|&n| n == to) {
                    if alt > neighbor.1 {
                        p_queue.push(Node(node.0.clone(), alt));
                    }
                    println!("Push {:?} {:?}", neighbor, alt);
                }
                //println!("{:?} {:?} {:?} {:?}", from, to, edge, node);
            }
        }
    }
    -1
}
