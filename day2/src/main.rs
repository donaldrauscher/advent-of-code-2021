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

    fn process(&self, forward: &mut i32, depth: &mut i32) {
        if self.direction == "up" {
            *depth -= self.magnitude;
        } else if self.direction == "down" {
            *depth += self.magnitude;
        } else {
            *forward += self.magnitude;
        }
    }
}

// first star
fn main() {

    let mut depth: i32 = 0;
    let mut forward: i32 = 0;

    let lines = read_lines("./input.txt");
    for line in lines {
        if let Ok(command) = line {
            let command = Command::parse(command);
            command.process(&mut forward, &mut depth);
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
