use std::fs;

fn main() {
    let mut lanternfish: [u64; 9] = fs::read_to_string("input.txt")
        .expect("Unable to read file")
        .trim()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .fold([0; 9], |mut map, n| {
            map[n] += 1;
            map
        });

    for day in 0..256 {
        lanternfish[(day + 7) % 9] += lanternfish[day % 9];
        if day == 79 {
            println!("Number of lanternfish on day {}: {}", day + 1, lanternfish.iter().sum::<u64>());
        }
    }

    println!("Number of lanternfish on day 256: {}", lanternfish.iter().sum::<u64>());
}
