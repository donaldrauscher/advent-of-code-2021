use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    // convert reading to integer and record frequencies in vector
    let univ = u32::pow(2, 12).try_into().unwrap();
    let mut readings: Vec<u32> = vec![0; univ];

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
    println!("Gamma = {}", gamma);
    println!("Episilon = {}", epsilon);
    println!("Gamma x epsilon = {}", gamma * epsilon);
}

fn get_oxygen_c02_rate(readings: &Vec<u32>) {
    let oxygen: u32 = get_oxygen_rate(readings);
    let c02: u32 = get_co2_rate(readings);

    println!("Oxygen = {:#b}", oxygen);
    println!("Oxygen = {}", oxygen);
    println!("CO2 = {:#b}", c02);
    println!("CO2 = {}", c02);
    println!("Oxygen x C02 = {}", oxygen * c02);
}


fn get_oxygen_rate(readings: &Vec<u32>) -> u32 {
    let mut oxygen: u32 = 0;

    let mut search_start: usize = 0;
    let mut search_end: usize = (1 << 12) - 1;
    let mut ones_count: u32 = 0;
    let mut zeros_count: u32 = 0;
    let mut last_reading: u32 = 0;

    for search_index in (0..12).rev() {
        ones_count = 0;
        zeros_count = 0;
        for (reading, count) in readings[search_start..(search_end + 1)].iter().enumerate() {
            if count > &0 {
                last_reading = (search_start + reading).try_into().unwrap();
                if ((reading >> search_index) & 0b1) > 0 {
                    ones_count += 1;
                } else {
                    zeros_count += 1;
                }
            }
        }
        if (ones_count + zeros_count) == 1 {
            oxygen = last_reading;
            break;
        }
        if ones_count >= zeros_count {
            oxygen += 1 << search_index;
        }
        search_start = oxygen.try_into().unwrap();
        search_end = (oxygen | ((1 << search_index) - 1)).try_into().unwrap();
    }

    return oxygen;
}

fn get_co2_rate(readings: &Vec<u32>) -> u32 {
    let mut c02: u32 = 0;

    let mut search_start: usize = 0;
    let mut search_end: usize = (1 << 12) - 1;
    let mut ones_count: u32 = 0;
    let mut zeros_count: u32 = 0;
    let mut last_reading: u32 = 0;

    for search_index in (0..12).rev() {
        ones_count = 0;
        zeros_count = 0;
        for (reading, count) in readings[search_start..(search_end + 1)].iter().enumerate() {
            if count > &0 {
                last_reading = (search_start + reading).try_into().unwrap();
                if ((reading >> search_index) & 0b1) > 0 {
                    ones_count += 1;
                } else {
                    zeros_count += 1;
                }
            }
        }
        if (ones_count + zeros_count) == 1 {
            c02 = last_reading;
            break;
        }
        if ones_count < zeros_count {
            c02 += 1 << search_index;
        }
        search_start = c02.try_into().unwrap();
        search_end = (c02 | ((1 << search_index) - 1)).try_into().unwrap();
    }

    return c02;
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where P: AsRef<Path>, {
    let file = File::open(filename).expect("Failed to load file!");
    io::BufReader::new(file).lines()
}
