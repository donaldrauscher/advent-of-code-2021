use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Board {
    numbers: Vec<Vec<u32>>,
    picked: Vec<Vec<bool>>
}

impl Board {
    fn parse(input: &Vec<String>) -> Board {
        let mut numbers: Vec<Vec<u32>> = Vec::with_capacity(5);
        for i in input {
            let mut row = Vec::with_capacity(5);
            for j in 0..5 {
                row.push(i[(j*3)..(j*3+2)].trim().parse().unwrap());
            }
            numbers.push(row);
        }

        Board {
            numbers: numbers,
            picked: vec![vec![false; 5]; 5]
        }
    }

    fn pick_number(&mut self, number: &u32) {
        for row in 0..5 {
            for col in 0..5 {
                if self.numbers[row][col] == *number {
                    self.picked[row][col] = true;
                }
            }
        }
    }
}

fn main() {
    let lines = read_lines("./input.txt");
    let mut line_number: u32 = 0;

    let mut pick_numbers: Vec<u32> = Vec::new();
    let mut boards: Vec<Board> = Vec::new();

    let mut board_numbers_tmp: Vec<String> = Vec::with_capacity(5);
    for line in lines {
        if let Ok(line_text) = line {
            if line_number == 0 {
                parse_pick_numbers(&line_text, &mut pick_numbers);
            } else if line_text == "" {
                continue;
            } else {
                board_numbers_tmp.push(line_text);
            }
            // time to create a new board!
            if board_numbers_tmp.len() == 5 {
                let board: Board = Board::parse(&board_numbers_tmp);
                boards.push(board);
                board_numbers_tmp.clear();
            }
        }
        line_number += 1;
    }

    println!("{:?}", pick_numbers);
    println!("{:?}", boards[0]);
    boards[0].pick_number(&48);
    println!("{:?}", boards[0]);
}

fn parse_pick_numbers(line_text: &str, pick_numbers: &mut Vec<u32>) {
    let pick_numbers_str: Vec<&str> = line_text.split(",").collect::<Vec<&str>>();
    for p in pick_numbers_str {
        pick_numbers.push(p.trim().parse().unwrap());
    }
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where P: AsRef<Path>, {
    let file = File::open(filename).expect("Failed to load file!");
    io::BufReader::new(file).lines()
}
