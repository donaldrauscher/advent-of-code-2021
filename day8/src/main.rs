use std::collections::{HashSet, HashMap};
use std::hash::{Hash, Hasher};


#[derive(Eq, PartialEq, Debug)]
struct Digit {
    letters: HashSet<char>
}

impl Digit {
    fn new(s: &str) -> Self {
        let mut letters: HashSet<char> = HashSet::new();
        for c in s.chars() {
            letters.insert(c);
        }
        Digit {
            letters: letters
        }
    }

    fn size(&self) -> usize {
        return self.letters.len()
    }

    fn intersection_size(&self, d: &Digit) -> usize {
        self.letters.intersection(&d.letters).count()
    }
}

impl Hash for Digit {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut s = self.letters.iter().collect::<Vec<&char>>();
        s.sort();
        for c in s {
            c.hash(state);
        }
    }
}

impl Clone for Digit {
    fn clone(&self) -> Self {
        let str = self.letters
            .iter()
            .map(|l| l.to_string())
            .collect::<Vec<String>>()
            .join("");
        Self::new(&str)
    }
}


fn main() {
    let part1_soln = include_str!("../input.txt")
        .lines()
        .map(|x| {
            let iter: Vec<&str> = x.splitn(2, "|").map(|y| y.trim()).collect::<Vec<&str>>();
            (iter[0], iter[1])
        })
        .map(|(_, o)| {
            o
                .split(' ')
                .map(|x| [2,3,4,7].iter().any(|y| x.chars().count() == *y) as u32)
                .sum::<u32>()
        })
        .sum::<u32>();

    let part2_soln = include_str!("../input.txt")
        .lines()
        .map(|x| {
            let iter: Vec<&str> = x.splitn(2, "|").map(|y| y.trim()).collect::<Vec<&str>>();
            (iter[0], iter[1])
        })
        .map(|(s, o)| decode(s, o))
        .sum::<u32>();

    println!("Part 1 solution: {}", part1_soln);
    println!("Part 2 solution: {}", part2_soln);
}

fn decode(digits: &str, output: &str) -> u32{
    let digits_vec: Vec<Digit> = digits
        .split(' ')
        .map(|d| Digit::new(d))
        .collect::<Vec<Digit>>();

    let mut decoder: HashMap<Digit, u32> = HashMap::new();

    // first pass: identify 1, 4, 7, and 8 (unique length)
    let first_pass = digits_vec
        .iter()
        .filter(|&d| [2,3,4,7].iter().any(|l| d.size() == *l))
        .map(|d| {
            (d, match d.size() {
                2 => 1,
                3 => 7,
                4 => 4,
                7 => 8,
                _ => unreachable!()
            })
        });

    let mut four: Option<Digit> = None;
    let mut one: Option<Digit> = None;
    for (d, n) in first_pass {
        decoder.insert(d.clone(), n);
        if n == 4 {
            four = Some(d.clone());
        } else if n == 1 {
            one = Some(d.clone());
        }
    }
    let four = four.unwrap();
    let one = one.unwrap();

    // second pass: identify remaining numbers
    let second_pass = digits_vec
        .iter()
        .filter(|&d| ![2,3,4,7].iter().any(|l| d.size() == *l))
        .map(|d| {
            (d, match (
                d.size(),
                d.intersection_size(&four),
                d.intersection_size(&one)
            ) {
                (6, 3, 2) => 0,
                (5, 2, 1) => 2,
                (5, 3, 2) => 3,
                (5, 3, 1) => 5,
                (6, 3, 1) => 6,
                (6, 4, 2) => 9,
                _ => unreachable!()
            })
        });

    for (d, n) in second_pass {
        decoder.insert(d.clone(), n);
    }

    // solution
    return output
        .split(' ')
        .rev()
        .enumerate()
        .map(|(i,d)| {
            10_u32.pow(i.try_into().unwrap()) * decoder[&Digit::new(d)]
        })
        .sum::<u32>();
}
