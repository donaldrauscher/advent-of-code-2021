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

    fn update_range(&self) -> (usize, usize, usize, usize) {
        let xstart;
        let xend;
        let ystart;
        let yend;

        if self.x1 <= self.x2 {
            xstart = self.x1;
            xend = self.x2 + 1;
        } else {
            xstart = self.x2;
            xend = self.x1 + 1;
        }
        if self.y1 <= self.y2 {
            ystart = self.y1;
            yend = self.y2 + 1;
        } else {
            ystart = self.y2;
            yend = self.y1 + 1;
        }

        return (xstart, xend, ystart, yend);
    }
}

impl Diagram {
    fn create() -> Diagram {
        Diagram {
            points: vec![vec![0; 1000]; 1000]
        }
    }

    fn update(&mut self, vent: &Vent) {
        if vent.is_horizontal_or_vertical() {
            let (xstart, xend, ystart, yend) = vent.update_range();
            for x in xstart..xend {
                for y in ystart..yend {
                    self.points[x][y] += 1;
                }
            }
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
