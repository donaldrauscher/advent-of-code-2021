use std::collections::HashSet;

struct HeightMap {
    heights: Vec<Vec<usize>>,
    n_rows: usize,
    n_cols: usize
}

impl HeightMap {
    fn new(heights: Vec<Vec<usize>>) -> Self {
        let n_rows = heights.len();
        let n_cols = heights.iter().next().unwrap().len();
        HeightMap {
            heights: heights,
            n_rows: n_rows,
            n_cols: n_cols
        }
    }

    fn get_height(&self, c: &Coord) -> usize {
        return self.heights[c.x][c.y];
    }

    fn low_point(&self, c: &Coord) -> usize {
        let row_start = if c.x == 0 { 0 } else { c.x - 1 };
        let row_end = if c.x == (self.n_rows - 1) { c.x } else { c.x + 1 };
        let col_start = if c.y == 0 { 0 } else { c.y - 1 };
        let col_end = if c.y == (self.n_cols - 1) { c.y } else { c.y + 1 };

        return *self.heights[row_start..=row_end]
            .into_iter()
            .map(|r| r[col_start..=col_end].into_iter().min().unwrap())
            .min()
            .unwrap();
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize
}

enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN
}

impl Coord {
    fn shift(&self, dir: Direction, hmap: &HeightMap) -> Option<Self> {
        match dir {
            Direction::LEFT => {
                if self.x == 0 {
                    return None;
                } else {
                    return Some(Coord {
                        x: self.x.checked_sub(1).unwrap(),
                        y: self.y
                    });
                }
            },
            Direction::RIGHT => {
                if self.x == (hmap.n_rows - 1) {
                    return None;
                } else {
                    return Some(Coord {
                        x: self.x + 1,
                        y: self.y
                    });
                }
            },
            Direction::DOWN => {
                if self.y == 0 {
                    return None;
                } else {
                    return Some(Coord {
                        x: self.x,
                        y: self.y.checked_sub(1).unwrap()
                    });
                }
            },
            Direction::UP => {
                if self.y == (hmap.n_cols - 1) {
                    return None;
                } else {
                    return Some(Coord {
                        x: self.x,
                        y: self.y + 1
                    });
                }
            }
        }
    }
}

fn main() {
    let hmap_raw = include_str!("../input.txt")
        .lines()
        .map(|row| {
            row
                .chars()
                .into_iter()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();

    let hmap = HeightMap::new(hmap_raw);

    let risk_level =(0..(hmap.n_rows * hmap.n_cols))
        .map(|i| {
            let c = Coord {
                x: i / hmap.n_cols,
                y: i % hmap.n_cols
            };
            if hmap.low_point(&c) == hmap.get_height(&c) {
                hmap.get_height(&c) + 1
            } else {
                0
            }
        })
        .sum::<usize>();

    let mut basin_sizes: Vec<usize> = (0..(hmap.n_rows * hmap.n_cols))
        .filter_map(|i| {
            let c = Coord {
                x: i / hmap.n_cols,
                y: i % hmap.n_cols
            };
            if hmap.low_point(&c) == hmap.get_height(&c) {
                Some(basin_size(&c, &hmap))
            } else {
                None
            }
        })
        .collect::<Vec<usize>>();

    basin_sizes.sort_by(|a, b| b.cmp(a));
    let top_3_basin_product = basin_sizes
        .iter()
        .take(3)
        .fold(1, |prod, x| prod * x);

    println!("Total risk level: {}", risk_level);
    println!("Top 3 basin product: {}", top_3_basin_product);
}

fn basin_size(c: &Coord, hmap: &HeightMap) -> usize {
    let mut points: Vec<Coord> = Vec::new();
    points.push(c.clone());

    let mut basin_size: usize = 0;
    let mut visited: HashSet<Coord> = HashSet::new();

    while points.len() > 0 {
        let c = points.pop().unwrap();
        if (hmap.get_height(&c) == 9) || visited.contains(&c) {
            continue;
        }

        for d in vec![Direction::LEFT, Direction::RIGHT, Direction::UP, Direction::DOWN] {
            if let Some(new_c) = c.shift(d, hmap) {
                if (hmap.get_height(&new_c) > hmap.get_height(&c)) && !visited.contains(&new_c) {
                    points.push(new_c);
                }
            }
        }

        basin_size += 1;
        visited.insert(c);
    }
    return basin_size;
}
