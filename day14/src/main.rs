use std::collections::HashMap;

fn main() {
    let (template, rules) = include_str!("../input.txt").split_once("\n\n").unwrap();

    let rules = HashMap::<&str, char>::from_iter(rules
        .lines()
        .map(|r| {
            let (pair, char) = r.split_once(" -> ").unwrap();
            (pair, char.chars().nth(0).unwrap())
        }));

    let mut template: String = template.to_string();
    for _ in 0..10 {
        let mut new_template: Vec<char> = Vec::new();
        for i in 0..(template.len()-1) {
            if i == 0 {
                new_template.push(template.chars().nth(i).unwrap());
            }
            new_template.push(*rules.get(&template[i..=(i+1)]).unwrap());
            new_template.push(template.chars().nth(i+1).unwrap());
        }
        template = new_template.iter().collect();
    }

    let mut freq: Vec<usize> = template
        .chars()
        .fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        })
        .into_values()
        .collect::<Vec<_>>();
    freq.sort_unstable();
    println!("Most frequent minus least frequent = {}",
        freq.last().unwrap() - freq.first().unwrap());
}
