use std::collections::HashMap;

fn pair_int(pair: &str, char_map: &HashMap<char, usize>) -> (usize, usize) {
    let p1 = *char_map.get(&pair.chars().nth(0).unwrap()).unwrap();
    let p2 = *char_map.get(&pair.chars().nth(1).unwrap()).unwrap();
    return (p1, p2);
}

fn main() {
    let (template, rules) = include_str!("../input.txt").split_once("\n\n").unwrap();

    let rules = HashMap::<&str, char>::from_iter(rules
        .lines()
        .map(|r| {
            let (pair, char) = r.split_once(" -> ").unwrap();
            (pair, char.chars().nth(0).unwrap())
        }));

    // map characters to integers
    let mut chars: Vec<char> = rules
        .iter()
        .flat_map(|(k, _)| k.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    chars.sort_unstable();
    chars.dedup();

    let n = chars.len();
    let char_map: HashMap<char, usize> = HashMap::from_iter(chars
        .iter()
        .enumerate()
        .map(|(i, c)| (*c, i)));

    // new rules that include new pairs created by insertion
    let rules: Vec<(usize, usize, usize)> = rules
        .iter()
        .fold(vec![(0, 0, 0); n*n], |mut m, (p, c)| {
            let (p1, p2) = pair_int(p, &char_map);
            let c = *char_map.get(c).unwrap();
            m[p1*n + p2] = (c, p1*n + c, c*n + p2);
            m
        });

    // initialize based on template
    let mut pair_counts: Vec<usize> = (0..(template.len()-1))
        .into_iter()
        .fold(vec![0; n*n], |mut map, i| {
            let (p1, p2) = pair_int(&template[i..=(i+1)], &char_map);
            map[p1*n + p2] += 1;
            map
        });

    let mut char_counts: Vec<usize> = template
        .chars()
        .fold(vec![0; n], |mut map, c| {
            map[*char_map.get(&c).unwrap()] += 1;
            map
        });

    // run steps
    for _ in 0..40 {
        let mut new_pair_counts = vec![0; n*n];
        for (p, n) in pair_counts.iter().enumerate() {
            let (c, p1, p2) = rules[p];
            char_counts[c] += n;
            new_pair_counts[p1] += n;
            new_pair_counts[p2] += n;
        }
        pair_counts = new_pair_counts;
    }

    char_counts.sort_unstable();
    println!("Most frequent minus least frequent = {}",
        char_counts.last().unwrap() - char_counts.first().unwrap());
}
