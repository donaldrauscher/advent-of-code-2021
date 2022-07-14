fn main() {

    // part 1
    let select_digits = include_str!("../input.txt")
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
                .map(|x| [2,3,4,7].iter().any(|y| x.chars().count() == *y) as u32)
                .sum::<u32>()
        })
        .sum::<u32>();

    println!("Number of 1/4/7/8 = {}", select_digits);
}
