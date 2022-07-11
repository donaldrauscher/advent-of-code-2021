use std::fs;

#[derive(Debug)]
struct Lanternfish {
    timer: u32
}

impl Lanternfish {
    fn age(&mut self) -> Option<Lanternfish> {
        if self.timer == 0 {
            self.timer = 6;
            return Some(Lanternfish {
                timer: 8
            })
        } else {
            self.timer -= 1;
            return None
        }
    }
}

fn main() {
    let mut lanternfish: Vec<Lanternfish> = fs::read_to_string("input.txt")
        .expect("Unable to read file")
        .trim()
        .split(',')
        .map(|x| Lanternfish{ timer: x.parse::<u32>().unwrap() })
        .collect();

    for _ in 0..80 {
        for i in 0..lanternfish.len() {
            if let Some(new_fish) = lanternfish[i].age() {
                lanternfish.push(new_fish);
            }
        }
    }

    println!("Number of lanternfish: {}", lanternfish.len());
}
