use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


#[derive(Debug)]
struct Command {
    direction: String,
    magnitude: i32
}

impl Command {
    fn parse(input: String) -> Command {
        let components = input.split(" ").collect::<Vec<&str>>();
        let direction: String = String::from(components[0]);
        let magnitude: i32 = components[1].trim().parse().unwrap();
        Command {
            direction: direction,
            magnitude: magnitude
        }
    }

    #[allow(dead_code)]
    fn process_part1(&self, forward: &mut i32, depth: &mut i32) {
        if self.direction == "up" {
            *depth -= self.magnitude;
        } else if self.direction == "down" {
            *depth += self.magnitude;
        } else {
            *forward += self.magnitude;
        }
    }

    fn process_part2(&self, forward: &mut i32, depth: &mut i32, aim: &mut i32) {
        if self.direction == "up" {
            *aim -= self.magnitude;
        } else if self.direction == "down" {
            *aim += self.magnitude;
        } else {
            *forward += self.magnitude;
            *depth += *aim * self.magnitude;
        }
    }
}

fn main() {

    let mut depth: i32 = 0;
    let mut forward: i32 = 0;
    let mut aim: i32 = 0;

    let lines = read_lines("./input.txt");
    for line in lines {
        if let Ok(command) = line {
            let command = Command::parse(command);
            //command.process_part1(&mut forward, &mut depth);
            command.process_part2(&mut forward, &mut depth, &mut aim);
        }
    }

    println!("Final depth: {}", depth);
    println!("Final forward: {}", forward);
    println!("Product: {}", depth * forward);

}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where P: AsRef<Path>, {
    let file = File::open(filename).expect("Failed to load file!");
    io::BufReader::new(file).lines()
}
