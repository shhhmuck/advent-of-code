use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::petgraph::graphmap::UnGraphMap;
use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

const INPUT: &str = include_str!("./input.txt");

#[derive(Debug)]
struct Graph {
    nodes: HashMap<String, HashSet<String>>,
    time: usize,
    low: HashMap<String, usize>,
    disc: HashMap<String, usize>,
    visited: HashSet<String>,
    count: HashMap<String, usize>,
    bridges: Vec<(String, String)>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            time: 0,
            low: HashMap::new(),
            disc: HashMap::new(),
            visited: HashSet::new(),
            count: HashMap::new(),
            bridges: Vec::new(),
        }
    }

    fn add_node(&mut self, node: String) {
        let c = self.count.entry(node.clone()).or_insert(0);
        *c += 1;
        self.nodes.entry(node).or_insert_with(HashSet::new);
    }

    fn add_edge(&mut self, from: String, to: String) {
        self.nodes
            .entry(from)
            .or_insert_with(HashSet::new)
            .insert(to.clone());

        self.nodes.entry(to).or_insert_with(HashSet::new);
    }

    fn remove_edge(&mut self, from: String, to: String) {
        if let Some(edges) = self.nodes.get_mut(&from) {
            edges.remove(&to);
        }
    }

    fn bridge_util(&mut self, start_node: String) {
        let mut stack = vec![(start_node, None)];

        while let Some((node, parent)) = stack.pop() {
            if !self.visited.contains(&node) {
                self.visited.insert(node.clone());
                self.disc.insert(node.clone(), self.time);
                self.low.insert(node.clone(), self.time);
                self.time += 1;

                if let Some(neighbors) = self.nodes.get(&node).cloned() {
                    for v in neighbors {
                        if !self.visited.contains(&v) {
                            stack.push((v.clone(), Some(node.clone())));
                        } else if Some(&v) != parent.as_ref() {
                            let u_low = *self.low.get(&node).unwrap();
                            let v_disc = *self.disc.get(&v).unwrap();
                            self.low.insert(node.clone(), u_low.min(v_disc));
                        }
                    }
                }
            }

            if let Some(parent) = parent {
                let u_low = *self.low.get(&parent).unwrap();
                let v_low = *self.low.get(&node).unwrap();
                self.low.insert(parent.clone(), u_low.min(v_low));
                if v_low > *self.disc.get(&parent).unwrap() {
                    self.bridges.push((parent.clone(), node.clone()));
                }
            }
        }
    }

    fn find_bridges(&mut self) {
        let nodes = self.nodes.keys().cloned().collect::<Vec<_>>();
        for node in nodes {
            if !self.visited.contains(&node) {
                self.bridge_util(node.clone());
            }
        }
    }
}

fn main() {
    let s = Instant::now();
    println!(
        "Part 1: {} in {:?}",
        part_1_lib(INPUT),
        Instant::now().duration_since(s)
    );
}

fn part_1_lib(input: &str) -> usize {
    let mut edges: HashSet<(&str, &str)> = HashSet::new();
    input.lines().for_each(|l| {
        let (n, e) = l.split_once(": ").unwrap();
        e.split_ascii_whitespace().for_each(|e| {
            edges.insert((n, e));
        });
    });
    let graph = UnGraphMap::<&str, ()>::from_edges(edges);
    let Ok(Some((_, cut))) = stoer_wagner_min_cut(&graph, |_| Ok::<_, ()>(1))
    else {
        panic!("No cut found");
    };
    cut.len() * (graph.node_count() - cut.len())
}

fn part_1(input: &str) -> usize {
    let mut graph = Graph::new();
    for line in input.lines() {
        let (node, edges) = line.split_once(": ").unwrap();
        let edges = edges.split(" ").map(String::from).collect::<Vec<_>>();

        graph.add_node(node.to_string());
        for edge in edges {
            graph.add_edge(node.to_string(), edge.clone());

            graph.add_node(edge.clone());
            graph.add_edge(edge, node.to_string());
        }
    }

    // let mut visited = HashSet::new();
    // let mut connections = Vec::new();

    // for node in graph.nodes.keys() {
    //     if !visited.contains(node) {
    //         let connected_nodes = dfs(&graph.nodes, node.clone());
    //         visited.extend(connected_nodes.clone());
    //         connections.push(connected_nodes);
    //     }
    // }

    // for v in visited {
    //     println!("{}", v);
    // }

    // println!("{:?}", connections.len());

    graph.find_bridges();
    for (u, v) in &graph.bridges {
        println!("{} -> {}", u, v);
    }

    0
}

fn dfs(graph: &HashMap<String, HashSet<String>>, start: String) -> HashSet<String> {
    let mut visited = HashSet::new();
    dfs_helper(graph, start, &mut visited);
    visited
}

fn dfs_helper(
    graph: &HashMap<String, HashSet<String>>,
    node: String,
    visited: &mut HashSet<String>,
) {
    if !visited.contains(&node) {
        visited.insert(node.clone());
        if let Some(neighbors) = graph.get(&node) {
            for neighbor in neighbors {
                dfs_helper(graph, neighbor.clone(), visited);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = "\
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1_lib(TEST), 54);
    }
}
