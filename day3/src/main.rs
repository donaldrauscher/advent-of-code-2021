use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {

    let mut diagnostics: Vec<i32> = vec![0; 12];

    let lines = read_lines("./input.txt");
    for line in lines {
        if let Ok(reading) = line {
            for (i, r) in reading.chars().enumerate() {
                if r == '1' {
                    diagnostics[i] += 1;
                } else {
                    diagnostics[i] -= 1;
                }
            }
        }
    }

    diagnostics.reverse();

    let gamma = get_gamma_rate(&diagnostics);
    let epsilon = get_epsilon_rate(&diagnostics);

    println!("Gamma rate: {}", gamma);
    println!("Epsilon rate: {}", epsilon);
    println!("Power consumption: {}", gamma * epsilon);

}

fn get_gamma_rate(diagnostics: &Vec<i32>) -> u32 {
    let mut gamma: u32 = 0;
    let base: u32 = 2;
    for (i, d) in diagnostics.iter().enumerate() {
        if d > &0 {
            gamma += base.pow(i.try_into().unwrap());
        }
    }
    gamma
}

fn get_epsilon_rate(diagnostics: &Vec<i32>) -> u32 {
    let mut epsilon: u32 = 0;
    let base: u32 = 2;
    for (i, d) in diagnostics.iter().enumerate() {
        if d < &0 {
            epsilon += base.pow(i.try_into().unwrap());
        }
    }
    epsilon
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where P: AsRef<Path>, {
    let file = File::open(filename).expect("Failed to load file!");
    io::BufReader::new(file).lines()
}
