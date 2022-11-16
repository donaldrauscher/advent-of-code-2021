use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn first_star() {

    let mut last_measurement: Option<u32> = None;
    let mut num_increases: u32 = 0;

    let lines = read_lines("./input.txt");
    for line in lines {
        if let Ok(measurement) = line {
            let measurement: u32 = measurement.trim().parse().unwrap();

            num_increases += match last_measurement {
                None => 0,
                Some(m) => if measurement > m { 1 } else { 0 }
            };

            last_measurement = Some(measurement);
        }
    }

    println!("Number of measurement increases: {}", num_increases);

}


fn second_star(rolling_window: usize) {

    let mut last_measurements: Vec<u32> = Vec::with_capacity(rolling_window + 1);
    let mut num_increases: u32 = 0;

    let lines = read_lines("./input.txt");
    for line in lines {
        if let Ok(measurement) = line {
            let measurement: u32 = measurement.trim().parse().unwrap();

            last_measurements.push(measurement);

            if last_measurements.len() == (rolling_window + 1) {
                let comp1 = vector_sum(&last_measurements[..rolling_window]);
                let comp2 = vector_sum(&last_measurements[1..]);
                if comp2 > comp1 {
                    num_increases += 1;
                }
                last_measurements.remove(0);
            }

        }
    }

    println!("Number of measurement increases with rolling window of {}: {}",
        rolling_window, num_increases);

}


fn main() {
    first_star();
    second_star(3);
    // this should match first_star()
    second_star(1);
}


fn vector_sum(v: &[u32]) -> u32 {
    let mut sum: u32 = 0;
    for vv in v {
        sum += vv;
    }
    return sum;
}


fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where P: AsRef<Path>, {
    let file = File::open(filename).expect("Failed to load file!");
    io::BufReader::new(file).lines()
}
