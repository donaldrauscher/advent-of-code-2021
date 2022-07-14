fn main() {

    // part 1
    let digits = include_str!("../input.txt")
        .lines()
        .map(|x| {
            let mut iter = x.splitn(2, "|");
            let signals = iter.next().unwrap().trim();
            let digits = iter.next().unwrap().trim();
            (signals, digits)
        })
        .map(|(_,d)| {
            d
                .split(' ')
                .fold([0; 8], |mut map, n| {
                    map[n.chars().count()] += 1;
                    map
                })
        })
        .collect::<Vec<_>>();

    let select_digits = digits.iter().map(|x| x[2]+x[4]+x[3]+x[7]).sum::<u32>();

    println!("Number of 1/4/7/8 = {}", select_digits);
}
