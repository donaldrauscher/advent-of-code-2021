use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    // convert reading to integer and record frequencies in vector
    let mut readings: Vec<u32> = vec![0; usize::pow(2, 12)];

    let lines = read_lines("./input.txt");
    for line in lines {
        if let Ok(reading) = line {
            let reading_int: usize = usize::from_str_radix(&reading, 2).unwrap();
            readings[reading_int] += 1;
        }
    }

    get_gamma_epsilon_rate(&readings);
    get_oxygen_c02_rate(&readings);
}

fn get_gamma_epsilon_rate(readings: &Vec<u32>) {
    let gamma: usize = _get_rate(readings, true, false);
    let epsilon: usize = !gamma & 0b111111111111;

    println!("Gamma = {}", gamma);
    println!("Episilon = {}", epsilon);
    println!("Gamma x epsilon = {}", gamma * epsilon);
}

fn get_oxygen_c02_rate(readings: &Vec<u32>) {
    let oxygen: usize = _get_rate(readings, true, true);
    let c02: usize = _get_rate(readings, false, true);

    println!("Oxygen = {:#b}", oxygen);
    println!("Oxygen = {}", oxygen);
    println!("CO2 = {:#b}", c02);
    println!("CO2 = {}", c02);
    println!("Oxygen x C02 = {}", oxygen * c02);
}

fn _get_rate(readings: &Vec<u32>, keep_ones: bool, update_search_univ: bool) -> usize {
    let mut last_reading: usize = 0;

    let mut search_prefix: usize = 0;
    let mut search_start: usize = 0;
    let mut search_end: usize = (1 << 12) - 1;

    for search_index in (0..12).rev() {
        let mut ones_count: u32 = 0;
        let mut zeros_count: u32 = 0;
        for (reading, count) in readings[search_start..(search_end + 1)].iter().enumerate() {
            if count > &0 {
                last_reading = search_start + reading;
                if ((reading >> search_index) & 0b1) > 0 {
                    ones_count += 1;
                } else {
                    zeros_count += 1;
                }
            }
        }
        if (ones_count + zeros_count) == 1 {
            break;
        }
        if keep_ones {
            if ones_count >= zeros_count {
                search_prefix += 1 << search_index;
            }
        } else {
            if ones_count < zeros_count {
                search_prefix += 1 << search_index;
            }
        }
        if update_search_univ {
            search_start = search_prefix;
            search_end = search_prefix | ((1 << search_index) - 1);
        }
    }

    if update_search_univ {
        return last_reading;
    } else {
        return search_prefix;
    }
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where P: AsRef<Path>, {
    let file = File::open(filename).expect("Failed to load file!");
    io::BufReader::new(file).lines()
}
