
use std::hash::{Hash, Hasher};
use priority_queue::PriorityQueue;


#[derive(Debug)]
struct Node {
    location: usize,
    distance: isize
}

impl Node {
    fn add_neighbors(&self, pq: &mut PriorityQueue<Node, isize>, risk_levels: &Vec<isize>, r: usize, c: usize) {
        let x = self.location % c;
        let y = self.location / c;
        if x < (c - 1) {
            let n = y*c + x + 1;
            let d = self.distance + risk_levels[n];
            pq.push_increase(Node { location: n, distance: d }, -d);
        }
        if y < (r - 1) {
            let n = (y+1)*c + x;
            let d = self.distance + risk_levels[n];
            pq.push_increase(Node { location: n, distance: d }, -d);
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

    // insert origin
    let origin = Node{ location: 0, distance: 0 };
    let mut pq = PriorityQueue::new();
    pq.push(origin, 0);

    // run dijikstras
    let mut distance: Option<isize> = None;
    while let Some((node, _)) = pq.pop() {
        if node.location == (n_row * n_col - 1) {
            distance = Some(node.distance);
            break;
        }
        node.add_neighbors(&mut pq, &risk_levels, n_row, n_col);
    }
    if let Some(d) = distance {
        println!("Distance: {:?}", d);
    }
}
