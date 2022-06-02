use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// first star
// fn main() {
//
//     let mut last_measurement: i32 = -1;
//     let mut num_increases: i32 = 0;
//
//     let lines = read_lines("./input.txt");
//     for line in lines {
//         if let Ok(measurement) = line {
//             let measurement: i32 = measurement.trim().parse().unwrap();
//
//             if last_measurement > 0 {
//                 if measurement > last_measurement {
//                     num_increases += 1;
//                 }
//             }
//
//             last_measurement = measurement;
//         }
//     }
//
//     println!("Number of measurement increases: {}", num_increases);
//
// }

// second star
fn main() {

    let mut last_measurements: Vec<i32> = Vec::with_capacity(4);
    let mut num_increases: i32 = 0;

    let lines = read_lines("./input.txt");
    for line in lines {
        if let Ok(measurement) = line {
            let measurement: i32 = measurement.trim().parse().unwrap();

            last_measurements.push(measurement);

            if last_measurements.len() == 4 {
                let comp1 = vector_sum(&last_measurements[..3]);
                let comp2 = vector_sum(&last_measurements[1..]);
                if comp2 > comp1 {
                    num_increases += 1;
                }
                last_measurements.remove(0);
            }

        }
    }

    println!("Number of measurement increases: {}", num_increases);

}


fn vector_sum(v: &[i32]) -> i32 {
    let mut sum: i32 = 0;
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
