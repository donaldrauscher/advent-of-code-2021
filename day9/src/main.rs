use std::collections::HashSet;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
    max_x: usize,
    max_y: usize
}

enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN
}

impl Coord {
    fn shift(&self, dir: Direction) -> Option<Self> {
        match dir {
            Direction::LEFT => {
                if self.x == 0 {
                    return None;
                } else {
                    return Some(Coord {
                        x: self.x.checked_sub(1).unwrap(),
                        y: self.y,
                        max_x: self.max_x,
                        max_y: self.max_y
                    });
                }
            },
            Direction::RIGHT => {
                if self.x == self.max_x {
                    return None;
                } else {
                    return Some(Coord {
                        x: self.x + 1,
                        y: self.y,
                        max_x: self.max_x,
                        max_y: self.max_y
                    });
                }
            },
            Direction::DOWN => {
                if self.y == 0 {
                    return None;
                } else {
                    return Some(Coord {
                        x: self.x,
                        y: self.y.checked_sub(1).unwrap(),
                        max_x: self.max_x,
                        max_y: self.max_y
                    });
                }
            },
            Direction::UP => {
                if self.y == self.max_y {
                    return None;
                } else {
                    return Some(Coord {
                        x: self.x,
                        y: self.y + 1,
                        max_x: self.max_x,
                        max_y: self.max_y
                    });
                }
            }
        }
    }

    fn get_height(&self, height_map: &Vec<Vec<usize>>) -> usize {
        return height_map[self.x][self.y];
    }
}

fn main() {
    let height_map = include_str!("../input_sample.txt")
        .lines()
        .map(|row| {
            row
                .chars()
                .into_iter()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();

    let n_rows = height_map.len();
    let n_cols = height_map.iter().next().unwrap().len();

    let risk_level =(0..n_rows)
        .into_iter()
        .map(|r| {
            (0..n_cols)
                .into_iter()
                .map(|c| {
                    let lp = low_point(r, c, &height_map);
                    if lp == height_map[r][c] {
                        let bs = basin_size(r, c, &height_map);
                        println!("({}, {}): {}", r, c, bs);
                        bs
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("Total risk level: {}", risk_level);
}

fn low_point(r: usize, c: usize, height_map: &Vec<Vec<usize>>) -> usize {
    let n_rows = height_map.len();
    let n_cols = height_map.iter().next().unwrap().len();

    let row_start = if r == 0 { 0 } else { r - 1 };
    let row_end = if r == (n_rows - 1) { r } else { r + 1 };
    let col_start = if c == 0 { 0 } else { c - 1 };
    let col_end = if c == (n_cols - 1) { c } else { c + 1 };

    let low_point = height_map[row_start..=row_end]
        .into_iter()
        .map(|r| r[col_start..=col_end].into_iter().min().unwrap())
        .min()
        .unwrap();

    return *low_point;
}

fn basin_size(r: usize, c: usize, height_map: &Vec<Vec<usize>>) -> usize {
    let n_rows = height_map.len();
    let n_cols = height_map.iter().next().unwrap().len();

    let mut points: Vec<Coord> = Vec::new();
    points.push(Coord{
        x: r,
        y: c,
        max_x: n_rows - 1,
        max_y: n_cols - 1
    });

    let mut basin_size: usize = 0;
    let mut visited: HashSet<Coord> = HashSet::new();

    while points.len() > 0 {
        let c = points.pop().unwrap();
        if (c.get_height(height_map) == 9) || visited.contains(&c) {
            continue;
        }

        for d in vec![Direction::LEFT, Direction::RIGHT, Direction::UP, Direction::DOWN] {
            if let Some(new_c) = c.shift(d) {
                if (new_c.get_height(height_map) > c.get_height(height_map)) && !visited.contains(&new_c) {
                    points.push(new_c);
                }
            }
        }

        basin_size += 1;
        visited.insert(c);
    }
    return basin_size;
}
