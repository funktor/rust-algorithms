use std::collections::HashMap;
use std::cmp::Ordering;
use std::isize::MAX;

struct Graph {
    nodes: Vec<Box<Node>>,
    edges: Vec<Box<Edge>>,
}

#[derive(Eq, PartialEq, Clone, Hash)]
struct Node {
    key: String,
}

#[derive(Eq, PartialEq, Clone)]
struct Edge {
    from: Box<Node>,
    to: Box<Node>,
    weight: isize,
}

impl Node {
    fn new(key:String) -> Self {
        Self {
            key,
        }
    }
}

impl Edge {
    fn new(from:Node, to:Node, weight:isize) -> Self {
        Self {
            from: Box::new(from),
            to: Box::new(to),
            weight,
        }
    }
}

impl Graph {
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
}

#[derive(Eq, PartialEq, Clone)]
struct NodeDist {
    node_key: String,
    dist: isize,
}

impl Ord for NodeDist {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.cmp(&other.dist)
    }
}

impl PartialOrd for NodeDist {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn bellman_ford(graph:&Graph, src_node:String) -> (HashMap<String, isize>, bool) {
    let mut shortest_distances:HashMap<String, isize> = HashMap::new();

    for node in graph.nodes.iter() {
        if node.key == src_node {
            shortest_distances.insert(node.key.clone(), 0);
        }

        else {
            shortest_distances.insert(node.key.clone(), MAX);
        }
    }

    let num_nodes = graph.nodes.len();
    let mut contains_negative_cycle:bool = false;

    for i in 0..num_nodes {
        for edge in graph.edges.iter() {
            let u = &edge.from;
            let v = &edge.to;
            let w = edge.weight;
    
            let du = shortest_distances.get(&u.key);
            let dv = shortest_distances.get(&v.key);
    
            match du {
                Some(x) => {
                    match dv {
                        Some(y) => {
                            if *x + w < *y {
                                if i == num_nodes-1 {
                                    contains_negative_cycle = true;
                                }
                                shortest_distances.insert(v.key.clone(), *x+w);
                            }
                        }
                        None => {}
                    }
                }
                None => {}
            }
        }
    }

    return (shortest_distances, contains_negative_cycle);
}

fn main() {
    let mut graph = Graph::new();

    let node_0 = Node::new(String::from("A"));
    let node_1 = Node::new(String::from("B"));
    let node_2 = Node::new(String::from("C"));
    let node_3 = Node::new(String::from("D"));
    let node_4 = Node::new(String::from("E"));
    let node_5 = Node::new(String::from("F"));

    let nodes = vec![Box::new(node_0.clone()), Box::new(node_1.clone()), Box::new(node_2.clone()), Box::new(node_3.clone()), Box::new(node_4.clone()), Box::new(node_5.clone())];
    
    let edge1 = Edge::new(node_0.clone(), node_1.clone(), 5);
    let edge2 = Edge::new(node_1.clone(), node_2.clone(), 1);
    let edge3 = Edge::new(node_1.clone(), node_3.clone(), 2);
    let edge4 = Edge::new(node_2.clone(), node_4.clone(), 1);
    let edge5 = Edge::new(node_4.clone(), node_3.clone(), -1);
    let edge6 = Edge::new(node_3.clone(), node_5.clone(), 2);
    let edge7 = Edge::new(node_5.clone(), node_4.clone(), -3);

    let edges = vec![Box::new(edge1), Box::new(edge2), Box::new(edge3), Box::new(edge4), Box::new(edge5), Box::new(edge6), Box::new(edge7)];
    
    graph.nodes = nodes;
    graph.edges = edges;

    let (shortest, neg_cycle) = bellman_ford(&graph, String::from("A"));
    println!("Shortest distances = {:?}", shortest);
    println!("Contains negative cycle = {:?}", neg_cycle);
}