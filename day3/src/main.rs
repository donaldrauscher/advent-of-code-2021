use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    // convert reading to integer and record frequencies in vector
    let univ = (u32::pow(2, 12) - 1).try_into().unwrap();
    let mut readings: Vec<u32> = vec![0; univ];

    let lines = read_lines("./input.txt");
    for line in lines {
        if let Ok(reading) = line {
            let reading_int: usize = usize::from_str_radix(&reading, 2).unwrap();
            readings[reading_int] += 1;
        }
    }

    get_gamma_epsilon_rate(&readings);
}

fn get_gamma_epsilon_rate(readings: &Vec<u32>) {
    let mut digits: Vec<u32> = vec![0; 12];
    let mut num_readings: u32 = 0;

    for (reading, count) in readings.iter().enumerate() {
        if count > &0 {
            num_readings += count;
            for digit in 0..12 {
                if (reading & (1 << digit)) > 0 {
                    digits[digit] += count;
                }
            }
        }
    }

    let mut mode: u32 = 0;
    for (i, &d) in digits.iter().enumerate() {
        if d > (num_readings - d) {
            mode += 1 << i
        }
    }

    let gamma: u32 = mode;
    let epsilon: u32 = !mode & 0b111111111111;
    println!("Gamma x epsilon = {}", gamma * epsilon);
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where P: AsRef<Path>, {
    let file = File::open(filename).expect("Failed to load file!");
    io::BufReader::new(file).lines()
}
