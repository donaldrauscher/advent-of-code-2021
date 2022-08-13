
fn get_neighbors(node: usize, r: usize, c: usize) -> Vec<usize> {
    let x = node % c;
    let y = node / c;
    let mut neighbors: Vec<usize> = Vec::new();
    if x > 0 {
        neighbors.push(y*c + x - 1);
    }
    if y > 0 {
        neighbors.push((y-1)*c + x);
    }
    if x < (c - 1) {
        neighbors.push(y*c + x + 1);
    }
    if y < (r - 1) {
        neighbors.push((y+1)*c + x);
    }
    return neighbors;
}

fn main() {
    // load data
    let n_row = include_str!("../input.txt").lines().count();
    let n_col = include_str!("../input.txt").split_once("\n").unwrap().0.chars().count();
    let risk: Vec<usize> = include_str!("../input.txt")
        .lines()
        .flat_map(|r| {
            r
                .chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();

    // // initialize djikstras
    let start = 0;
    let end = n_row*n_col - 1;
    let mut dist: Vec<usize> = vec![usize::max_value(); n_row*n_col];
    dist[start] = 0;

    // run dijikstras
    let mut unvisited: Vec<usize> = vec![0];

    while unvisited.len() > 0 {
        let n = unvisited.remove(0);
        for nn in get_neighbors(n, n_row, n_col) {
            if (dist[n] + risk[nn]) < dist[nn] {
                dist[nn] = dist[n] + risk[nn];
                if !unvisited.contains(&nn) {
                    unvisited.push(nn);
                }
            }
        }
    }
    println!("Minimum path to destination: {}", dist[end]);
}
