
use std::hash::{Hash, Hasher};
use std::collections::HashSet;
use priority_queue::PriorityQueue;


#[derive(Debug)]
struct Node {
    location: usize,
    distance: isize
}

impl Node {
    fn add_neighbors(&self,
                     pq: &mut PriorityQueue<Node, isize>,
                     visited: &HashSet<Node>,
                     risk_levels: &Vec<isize>,
                     r: usize,
                     c: usize) {
        let x = self.location % c;
        let y = self.location / c;
        if x > 0 {
            let n = y*c + x - 1;
            self.add_neighbor(n, pq, visited, risk_levels);
        }
        if y > 0 {
            let n = (y-1)*c + x;
            self.add_neighbor(n, pq, visited, risk_levels);
        }
        if x < (c - 1) {
            let n = y*c + x + 1;
            self.add_neighbor(n, pq, visited, risk_levels);
        }
        if y < (r - 1) {
            let n = (y+1)*c + x;
            self.add_neighbor(n, pq, visited, risk_levels);
        }
    }

    fn add_neighbor(&self,
                    n: usize,
                    pq: &mut PriorityQueue<Node, isize>,
                    visited: &HashSet<Node>,
                    risk_levels: &Vec<isize>) {
        let d = self.distance + risk_levels[n];
        let node = Node { location: n, distance: d };
        if !visited.contains(&node) {
            pq.push_increase(node, -d);
        }
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.location.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.location == other.location
    }
}
impl Eq for Node {}


fn main() {
    // load data
    let n_row = include_str!("../input.txt").lines().count();
    let n_col = include_str!("../input.txt").split_once("\n").unwrap().0.chars().count();
    let risk_levels: Vec<isize> = include_str!("../input.txt")
        .lines()
        .flat_map(|r| {
            r
                .chars()
                .map(|c| c.to_string().parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect::<Vec<_>>();

    // // multiple board
    // let n_row = n_row * 5;
    // let n_col = n_col * 5;
    // let risk_levels = (
    //     (0..n_row*n_col*25)
    //     .map(|l| {
    //         let x = (l % (n_col * 5)) % 5;
    //         let y = (l / (n_col * 5)) % 5;
    //         risk_levels[y*n_col + x]
    //     })
    //     .(0..max_pos)
    //         .map(|p| {
    //             crabs
    //                 .iter()
    //                 .map(|x|{ let n = (x - p).abs(); n*(n+1)/2 })
    //                 .sum::<i32>()
    //         })
    // )

    // set up priority queue
    let mut pq = PriorityQueue::new();
    let mut visited: HashSet<Node> = HashSet::new();

    let origin = Node{ location: 0, distance: 0 };
    pq.push(origin, 0);

    // run dijikstras
    let mut distance: Option<isize> = None;
    while let Some((node, _)) = pq.pop() {
        if node.location == (n_row * n_col - 1) {
            distance = Some(node.distance);
            break;
        }
        node.add_neighbors(&mut pq, &visited, &risk_levels, n_row, n_col);
        visited.insert(node);
    }
    if let Some(d) = distance {
        println!("Distance: {:?}", d);
    }
}
