use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Forward
}

#[derive(Debug)]
struct Command {
    direction: Direction,
    magnitude: u32
}

impl Command {
    fn parse(input: String) -> Command {
        let components = input.split(" ").collect::<Vec<&str>>();
        let magnitude: u32 = components[1].trim().parse().unwrap();

        let mut direction: Direction = Direction::Forward;
        if components[0] == "up" {
            direction = Direction::Up;
        } else if components[0] == "down" {
            direction = Direction::Down;
        }

        Command {
            direction: direction,
            magnitude: magnitude
        }
    }

    fn process_part1(&self, forward: &mut u32, depth: &mut u32) {
        match self.direction {
            Direction::Up => {
                *depth -= self.magnitude;
            },
            Direction::Down => {
                *depth += self.magnitude;
            },
            Direction::Forward => {
                *forward += self.magnitude;
            }
        }
    }

    fn process_part2(&self, forward: &mut u32, depth: &mut u32, aim: &mut u32) {
        match self.direction {
            Direction::Up => {
                *aim -= self.magnitude;
            },
            Direction::Down => {
                *aim += self.magnitude;
            },
            Direction::Forward => {
                *forward += self.magnitude;
                *depth += *aim * self.magnitude;
            }
        }
    }
}

fn first_star() {
    let mut depth: u32 = 0;
    let mut forward: u32 = 0;

    let lines = read_lines("./input.txt");
    for line in lines {
        if let Ok(command) = line {
            let command = Command::parse(command);
            command.process_part1(&mut forward, &mut depth);
        }
    }

    println!("\nFirst star solution:");
    println!("Final depth: {}", depth);
    println!("Final forward: {}", forward);
    println!("Product: {}", depth * forward);
}

fn second_star() {
    let mut depth: u32 = 0;
    let mut forward: u32 = 0;
    let mut aim: u32 = 0;

    let lines = read_lines("./input.txt");
    for line in lines {
        if let Ok(command) = line {
            let command = Command::parse(command);
            command.process_part2(&mut forward, &mut depth, &mut aim);
        }
    }

    println!("\nSecond star solution:");
    println!("Final depth: {}", depth);
    println!("Final forward: {}", forward);
    println!("Product: {}", depth * forward);
}

fn main() {
    first_star();
    second_star();
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where P: AsRef<Path>, {
    let file = File::open(filename).expect("Failed to load file!");
    io::BufReader::new(file).lines()
}
