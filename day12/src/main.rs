use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Node {
    idx: usize,
    key: String,
    start: bool,
    end: bool,
    small: bool
}

impl Node {
    fn new(idx: usize, key: &str) -> Self {
        Node {
            idx: idx,
            key: key.to_string(),
            start: (key == "start"),
            end: (key == "end"),
            small: (key == key.to_lowercase()) && (key != "start") && (key != "end")
        }
    }
}

#[derive(Debug, Clone)]
struct Path {
    path: Vec<usize>,
    small_visited: HashSet<usize>
}

impl Path {
    fn new(graph: &Graph) -> Self {
        return Path {
            path: vec![graph.node_key_map["start"]],
            small_visited: HashSet::new()
        };
    }

    fn extend(&self, graph: &Graph, idx: usize) -> Option<Self> {
        if graph.nodes[idx].start {
            return None;
        } else if graph.nodes[idx].small {
            if !self.small_visited.contains(&idx) {
                let mut out = self.clone();
                out.path.push(idx);
                out.small_visited.insert(idx);
                return Some(out);
            } else {
                return None;
            }
        } else {
            let mut out = self.clone();
            out.path.push(idx);
            return Some(out);
        }
    }

    fn last_visited(&self) -> usize {
        return *self.path.last().unwrap();
    }

    fn print(&self, graph: &Graph) {
        let mut out = String::from("start");
        for i in &self.path[1..] {
            out.push(',');
            out.push_str(&graph.nodes[*i].key);
        }
        println!("{}", out);
    }
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Node>,
    node_key_map: HashMap<String, usize>,
    edges: Vec<Vec<usize>>
}

impl Graph {
    fn load() -> Self {
        let mut nodes = include_str!("../input.txt")
            .lines()
            .flat_map(|s| s.splitn(2, "-").collect::<Vec<&str>>())
            .collect::<Vec<_>>();
        nodes.sort_unstable();
        nodes.dedup();

        let nodes: Vec<Node> = nodes
            .iter()
            .enumerate()
            .map(|(i, n)| Node::new(i, n))
            .collect::<Vec<_>>();

        let node_key_map = HashMap::<String, usize>::from_iter(nodes
            .iter()
            .map(|n| (n.key.clone(), n.idx))
        );

        let edges = include_str!("../input.txt")
            .lines()
            .map(|s| {
                let iter: Vec<&str> = s.splitn(2, "-").collect::<Vec<&str>>();
                (node_key_map[iter[0]], node_key_map[iter[1]])
            })
            .fold({
                (0..nodes.len())
                    .into_iter()
                    .map(|_| vec![])
                    .collect::<Vec<Vec<usize>>>()
            }, |mut e, (n1, n2)| {
                e[n1].push(n2);
                e[n2].push(n1);
                e
            });

        Graph{
            nodes: nodes,
            node_key_map: node_key_map,
            edges: edges
        }
    }

    fn initial_paths(&self) -> Vec<Path> {
        self.edges[self.node_key_map["start"]]
            .iter()
            .map(|n| Path::new(self).extend(self, *n).unwrap() )
            .collect::<Vec<_>>()
    }
}


fn main() {
    // load graph
    let graph = Graph::load();

    // build through graph
    let mut paths: Vec<Path> = graph.initial_paths();
    let mut completed_paths: Vec<Path> = Vec::new();
    while paths.len() > 0 {
        let path = paths.remove(0);
        for n in &graph.edges[path.last_visited()] {
            if let Some(new_path) = path.extend(&graph, *n) {
                if graph.nodes[*n].end {
                    completed_paths.push(new_path);
                } else {
                    paths.push(new_path);
                }
            }
        }
    }

    // print paths
    // for p in &completed_paths {
    //     p.print(&graph);
    // }
    println!("Number of completed_paths: {}", completed_paths.len());
}
