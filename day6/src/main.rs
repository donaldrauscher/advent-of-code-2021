use std::fs;

fn main() {
    let lanternfish_initial: Vec<usize> = fs::read_to_string("input.txt")
        .expect("Unable to read file")
        .trim()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut lanternfish: Vec<u64> = vec![0; 9];
    for l in lanternfish_initial {
        lanternfish[l] += 1;
    }

    for day in 0..256 {
        let birth = lanternfish.remove(0);
        lanternfish.resize(9, 0);
        lanternfish[8] = birth;
        lanternfish[6] += birth;

        if day == 79 {
            population_update(&lanternfish);
        }
    }

    population_update(&lanternfish);
}

fn population_update(v: &[u64]) {
    let mut sum: u64 = 0;
    for vv in v {
        sum += vv;
    }
    println!("Number of lanternfish: {}", sum);
}
