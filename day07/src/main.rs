use std::fs;

fn main() {
    let mut crabs: Vec<i32> = fs::read_to_string("input.txt")
        .expect("Unable to read file")
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    // optimal point will be the median
    let mid = crabs.len() / 2;
    let med: i32 = *crabs.select_nth_unstable(mid).1;
    let part1_fuel: i32 = crabs.iter().map(|x| (x - med).abs()).sum::<i32>();

    // don't know the best place so iterate over all positions
    let max_pos: i32 = *crabs.iter().max_by(|a, b| a.cmp(b)).unwrap();
    let part2_fuel = (0..max_pos)
        .map(|p| {
            crabs
                .iter()
                .map(|x|{ let n = (x - p).abs(); n*(n+1)/2 })
                .sum::<i32>()
        })
        .min_by(|a, b| a.cmp(b))
        .unwrap();

    println!("Part 1 fuel: {}", part1_fuel);
    println!("Part 2 fuel: {}", part2_fuel);

}
