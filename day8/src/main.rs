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
    let mut digits_vec: Vec<Digit> = digits
        .split(' ')
        .map(|d| Digit::new(d))
        .collect::<Vec<Digit>>();

    let mut decoder: HashMap<Digit, u32> = HashMap::new();
    let mut encoder: HashMap<u32, Digit> = HashMap::new();

    // first pass: identify 1, 4, 7, and 8 (unique length)
    let mut idx: usize = 0;
    'outer: while idx < digits_vec.len() {
        let d = &digits_vec[idx];
        for (l, n) in vec![(2,1), (3,7), (4,4), (7,8)] {
            if d.size() == l {
                decoder.insert(d.clone(), n);
                encoder.insert(n, d.clone());
                digits_vec.remove(idx);
                continue 'outer;
            }
        }
        idx = idx + 1;
    }

    // second pass:
    // - identify 2 and 9 (intersect with 4)
    // - identify 0 and 6 (length + intersect with 1)
    let mut idx: usize = 0;
    'outer2: while idx < digits_vec.len() {
        let d = &digits_vec[idx];
        for (l, n) in vec![(2,2), (4,9)] {
            if d.intersection_size(&encoder[&4]) == l {
                decoder.insert(d.clone(), n);
                encoder.insert(n, d.clone());
                digits_vec.remove(idx);
                continue 'outer2;
            }
        }
        for (l, n) in vec![(2,0), (1,6)] {
            if d.size() == 6 && d.intersection_size(&encoder[&1]) == l {
                decoder.insert(d.clone(), n);
                encoder.insert(n, d.clone());
                digits_vec.remove(idx);
                continue 'outer2;
            }
        }
        idx = idx + 1;
    }

    // third pass: identify 3 and 5 (intersect with 7)
    let mut idx: usize = 0;
    'outer3: while idx < digits_vec.len() {
        let d = &digits_vec[idx];
        for (l, n) in vec![(3,3), (2,5)] {
            if d.intersection_size(&encoder[&7]) == l {
                decoder.insert(d.clone(), n);
                encoder.insert(n, d.clone());
                digits_vec.remove(idx);
                continue 'outer3;
            }
        }
        idx = idx + 1;
    }

    // solution
    assert_eq!(digits_vec.len(), 0);
    return output
        .split(' ')
        .rev()
        .enumerate()
        .map(|(i,d)| {
            10_u32.pow(i.try_into().unwrap()) * decoder[&Digit::new(d)]
        })
        .sum::<u32>();
}
