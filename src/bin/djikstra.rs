use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::usize::MAX;

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
    weight: usize,
}

impl Node {
    fn new(key:String) -> Self {
        Self {
            key,
        }
    }
}

impl Edge {
    fn new(from:Node, to:Node, weight:usize) -> Self {
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
    dist: usize,
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

fn djikstra(graph:&Graph, src_node:String) -> HashMap<String, usize> {
    let mut shortest_distances:HashMap<String, usize> = HashMap::new();
    let mut ordered_nodes:BinaryHeap<NodeDist> = BinaryHeap::new();

    for node in graph.nodes.iter() {
        if node.key == src_node {
            shortest_distances.insert(node.key.clone(), 0);
            ordered_nodes.push(NodeDist { node_key: node.key.clone(), dist: 0 });
        }

        else {
            shortest_distances.insert(node.key.clone(), MAX);
        }
    }

    let mut neighbors:HashMap<String, Vec<(String, usize)>> = HashMap::new();

    for edge in graph.edges.iter() {
        let u = &edge.from;
        let v = &edge.to;
        let w = edge.weight;
        
        let mut nb = neighbors.entry(u.key.clone()).or_insert(vec![]);
        nb.push((v.key.clone(), w));
        nb = neighbors.entry(v.key.clone()).or_insert(vec![]);
        nb.push((u.key.clone(), w));
    }

    while ordered_nodes.len() > 0 {
        let min_wt = ordered_nodes.pop();
        match min_wt {
            Some(min_node_dist) => {
                let u = min_node_dist.node_key;
                let d = min_node_dist.dist;
                let nbs = neighbors.get(&u);
                match nbs {
                    Some(nb) => {
                        for v in nb.iter() {
                            let d_curr = shortest_distances.get(&v.0);
                            match d_curr {
                                Some(x) => {
                                    if d + v.1 < *x {
                                        shortest_distances.insert(v.0.clone(), d+v.1);
                                        ordered_nodes.push(NodeDist { node_key: v.0.clone(), dist: d+v.1 });
                                    }
                                }
                                None => {}
                            }
                        }
                    }
                    None => {}
                }
            }
            None => {}
        }
    }
    return shortest_distances;
}

fn main() {
    let mut graph = Graph::new();

    let node_0 = Node::new(String::from("0"));
    let node_1 = Node::new(String::from("1"));
    let node_2 = Node::new(String::from("2"));
    let node_3 = Node::new(String::from("3"));
    let node_4 = Node::new(String::from("4"));
    let node_5 = Node::new(String::from("5"));
    let node_6 = Node::new(String::from("6"));

    let nodes = vec![Box::new(node_0.clone()), Box::new(node_1.clone()), Box::new(node_2.clone()), Box::new(node_3.clone()), Box::new(node_4.clone()), Box::new(node_5.clone()), Box::new(node_6.clone())];
    
    let edge1 = Edge::new(node_0.clone(), node_1.clone(), 2);
    let edge2 = Edge::new(node_0.clone(), node_2.clone(), 6);
    let edge3 = Edge::new(node_1.clone(), node_3.clone(), 5);
    let edge4 = Edge::new(node_2.clone(), node_3.clone(), 8);
    let edge5 = Edge::new(node_3.clone(), node_4.clone(), 10);
    let edge6 = Edge::new(node_3.clone(), node_5.clone(), 15);
    let edge7 = Edge::new(node_4.clone(), node_6.clone(), 2);
    let edge8 = Edge::new(node_5.clone(), node_6.clone(), 6);

    let edges = vec![Box::new(edge1), Box::new(edge2), Box::new(edge3), Box::new(edge4), Box::new(edge5), Box::new(edge6), Box::new(edge7), Box::new(edge8)];
    
    graph.nodes = nodes;
    graph.edges = edges;

    let shortest = djikstra(&graph, String::from("0"));
    println!("Shortest distances = {:?}", shortest);
}