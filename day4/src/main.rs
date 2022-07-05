use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Board {
    numbers: Vec<Vec<u32>>,
    marked: Vec<Vec<bool>>,
    won: bool
}

impl Board {
    fn create(numbers: Vec<Vec<u32>>) -> Board {
        Board {
            numbers: numbers,
            marked: vec![vec![false; 5]; 5],
            won: false
        }
    }

    fn mark_number(&mut self, number: &u32) {
        'outer: for row in 0..5 {
            for col in 0..5 {
                if self.numbers[row][col] == *number {
                    self.marked[row][col] = true;
                    self.won = self.has_won(row, col);
                    break 'outer;
                }
            }
        }
    }

    fn has_won(&self, row: usize, col: usize) -> bool {
        let mut row_count: u32 = 0;
        for c in 0..5 {
            if self.marked[row][c] {
                row_count += 1;
            }
        }
        let mut col_count: u32 = 0;
        for r in 0..5 {
            if self.marked[r][col] {
                col_count += 1;
            }
        }
        return (row_count == 5) || (col_count == 5);
    }

    fn unmarked_sum(&self) -> u32 {
        let mut sum: u32 = 0;
        for row in 0..5 {
            for col in 0..5 {
                if !self.marked[row][col] {
                    sum += self.numbers[row][col];
                }
            }
        }
        return sum;
    }
}

fn main() {
    let lines = read_lines("./input.txt");
    let mut line_number: u32 = 0;

    let mut drawn_numbers: Vec<u32> = Vec::new();
    let mut board_numbers: Vec<Vec<u32>> = Vec::with_capacity(5);
    let mut boards: Vec<Board> = Vec::new();

    for line in lines {
        if let Ok(line_text) = line {
            if line_number == 0 {
                parse_drawn_numbers(&line_text, &mut drawn_numbers);
            } else if line_text == "" {
                continue;
            } else {
                board_numbers.push(parse_board_numbers(&line_text));
            }
            // time to create a new board!
            if board_numbers.len() == 5 {
                let board: Board = Board::create(board_numbers);
                boards.push(board);
                board_numbers = Vec::with_capacity(5);
            }
        }
        line_number += 1;
    }

    'outer: for n in drawn_numbers {
        println!("Drawing number {}...", n);
        for i in 0..boards.len() {
            let board = &mut boards[i];
            board.mark_number(&n);
            if board.won {
                println!("Final score: {}", board.unmarked_sum() * n);
                println!("{:?}", board);
                break 'outer;
            }
        }
    }
}

fn parse_drawn_numbers(line_text: &str, drawn_numbers: &mut Vec<u32>) {
    let drawn_numbers_str: Vec<&str> = line_text.split(",").collect::<Vec<&str>>();
    for p in drawn_numbers_str {
        drawn_numbers.push(p.trim().parse().unwrap());
    }
}

fn parse_board_numbers(line_text: &str) -> Vec<u32> {
    let mut row = Vec::with_capacity(5);
    for i in 0..5 {
        row.push(line_text[(i*3)..(i*3+2)].trim().parse().unwrap());
    }
    return row;
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where P: AsRef<Path>, {
    let file = File::open(filename).expect("Failed to load file!");
    io::BufReader::new(file).lines()
}
