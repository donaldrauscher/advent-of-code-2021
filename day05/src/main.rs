use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Vent {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize
}

#[derive(Debug)]
struct Diagram {
    points: Vec<Vec<u32>>
}

impl Vent {
    fn parse(input: String) -> Vent {
        let points = input.split(" -> ").collect::<Vec<&str>>();

        let mut x1: usize = 0;
        let mut y1: usize = 0;
        let mut x2: usize = 0;
        let mut y2: usize = 0;

        let mut split = points[0].splitn(2, ",");
        if let Some(num) = split.next() {
            x1 = num.parse().unwrap();
        }
        if let Some(num) = split.next() {
            y1 = num.parse().unwrap();
        }

        let mut split = points[1].splitn(2, ",");
        if let Some(num) = split.next() {
            x2 = num.parse().unwrap();
        }
        if let Some(num) = split.next() {
            y2 = num.parse().unwrap();
        }

        Vent {
            x1: x1,
            y1: y1,
            x2: x2,
            y2: y2
        }
    }

    fn is_horizontal_or_vertical(&self) -> bool {
        return (self.x1 == self.x2) | (self.y1 == self.y2);
    }

    fn is_diagonal(&self) -> bool {
        return (self.x1 as i32 - self.x2 as i32).abs() == (self.y1 as i32 - self.y2 as i32).abs();
    }

    fn get_update_range(&self) -> Vec<(usize, usize)> {
        let xrange: Vec<usize>;
        if self.x1 <= self.x2 {
            xrange = (self.x1..=self.x2).into_iter().collect::<Vec<usize>>();
        } else {
            xrange = (self.x2..=self.x1).into_iter().rev().collect::<Vec<usize>>()
        }

        let yrange: Vec<usize>;
        if self.y1 <= self.y2 {
            yrange = (self.y1..=self.y2).into_iter().collect::<Vec<usize>>();
        } else {
            yrange = (self.y2..=self.y1).into_iter().rev().collect::<Vec<usize>>()
        }

        let mut update_range: Vec<(usize, usize)> = Vec::new();
        if self.is_horizontal_or_vertical() {
            for x in &xrange {
                for y in &yrange {
                    update_range.push((*x, *y));
                }
            }
        } else if self.is_diagonal() {
            let it = xrange.into_iter().zip(yrange.into_iter());
            for (x, y) in it {
                update_range.push((x, y));
            }
        }
        return update_range;
    }
}

impl Diagram {
    fn create() -> Diagram {
        Diagram {
            points: vec![vec![0; 1000]; 1000]
        }
    }

    fn update(&mut self, vent: &Vent) {
        let update_range: Vec<(usize, usize)> = vent.get_update_range();
        for (x, y) in update_range {
            self.points[x][y] += 1;
        }
    }

    fn num_danger_points(&self) -> u32 {
        let mut n: u32 = 0;
        for r in &self.points {
            for c in r {
                if c >= &2 {
                    n += 1;
                }
            }
        }
        return n;
    }
}

fn main() {
    let lines = read_lines("./input.txt");

    let mut diagram: Diagram = Diagram::create();
    for line in lines {
        if let Ok(line_text) = line {
            let vent = Vent::parse(line_text);
            diagram.update(&vent);
        }
    }

    println!("Number of dangerous points: {}", diagram.num_danger_points());
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where P: AsRef<Path>, {
    let file = File::open(filename).expect("Failed to load file!");
    io::BufReader::new(file).lines()
}
