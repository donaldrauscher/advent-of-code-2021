use std::collections::HashSet;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Coord {
    x: usize,
    y: usize
}

#[derive(Eq, PartialEq)]
enum DirectionX {
    LEFT,
    RIGHT,
    NONE
}

#[derive(Eq, PartialEq)]
enum DirectionY {
    UP,
    DOWN,
    NONE
}

impl Coord {
    fn index(&self) -> usize {
        return self.x*10 + self.y;
    }

    fn new(i: usize) -> Self {
        return Coord {
            x: i / 10,
            y: i % 10
        }
    }

    fn shift(&self, dir_x: &DirectionX, dir_y: &DirectionY) -> Option<Self> {
        let mut x = self.x;
        let mut y = self.y;

        if (dir_x == &DirectionX::NONE) && (dir_y == &DirectionY::NONE) {
            return None;
        }

        match dir_x {
            DirectionX::LEFT => {
                if x == 0 {
                    return None;
                } else {
                    x = x.checked_sub(1).unwrap();
                }
            },
            DirectionX::RIGHT => {
                if x == 9 {
                    return None;
                } else {
                    x += 1;
                }
            },
            DirectionX::NONE => {}
        }

        match dir_y {
            DirectionY::DOWN => {
                if y == 0 {
                    return None;
                } else {
                    y = y.checked_sub(1).unwrap();
                }
            },
            DirectionY::UP => {
                if y == 9 {
                    return None;
                } else {
                    y += 1;
                }
            },
            DirectionY::NONE => {}
        }

        return Some(Coord { x: x, y: y });
    }
}

fn main() {
    let mut energy_levels = include_str!("../input.txt")
        .lines()
        .flat_map(|row| {
            row
                .chars()
                .into_iter()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();

    println!("Initial:");
    print_state(&energy_levels);

    let mut num_flashes = 0;
    let mut num_step = 0;
    let mut stop: bool = false;
    while !stop {
        num_flashes += step(&mut energy_levels);
        num_step += 1;

        let energy_sum = energy_levels.iter().sum::<usize>();
        if energy_sum == 0 {
            stop = true;
        }

        if num_step == 100 {
            println!("After 100 flashes:");
            print_state(&energy_levels);
            println!("Number of flashes: {}", num_flashes);
        }
    }

    println!("After {} flashes:", num_step);
    print_state(&energy_levels);
}

fn step(energy_levels: &mut Vec<usize>) -> usize {
    let mut flash_queue: Vec<Coord> = Vec::new();
    let mut flashed: HashSet<Coord> = HashSet::new();

    // first, incremental all the energy_levels and record flashes
    for i in 0..100 {
        energy_levels[i] += 1;
        if energy_levels[i] > 9 {
            flash_queue.push(Coord::new(i));
        }
    }

    // process flash queue and any ripples
    while flash_queue.len() > 0 {
        let c = flash_queue.pop().unwrap();

        if flashed.contains(&c) {
            continue;
        }

        for dx in vec![DirectionX::LEFT, DirectionX::RIGHT, DirectionX::NONE] {
            for dy in vec![DirectionY::UP, DirectionY::DOWN, DirectionY::NONE] {
                if let Some(new_c) = c.shift(&dx, &dy) {
                    energy_levels[new_c.index()] += 1;
                    if (energy_levels[new_c.index()] > 9) && !flashed.contains(&new_c) {
                        flash_queue.push(new_c);
                    }
                }
            }
        }

        flashed.insert(c);
    }

    // reset anything that has flashed to 0
    let n_flashed = flashed.len();
    for c in flashed.drain() {
        energy_levels[c.index()] = 0;
    }

    return n_flashed;
}

fn print_state(energy_levels: &Vec<usize>) {
    for i in 0..10 {
        println!("{}", energy_levels[(i*10)..((i+1)*10)]
            .iter()
            .map(|c| c.to_string())
            .collect::<String>());
    }
}
