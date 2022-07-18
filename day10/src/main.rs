use std::collections::HashMap;

#[derive(Debug)]
enum ChunkChar {
    Open(char),
    Close(char)
}

#[derive(Debug)]
enum LineStatus {
    Incomplete,
    Corrupted(char)
}

fn main() {
    let mut open_close_match: HashMap<char, char> = HashMap::new();
    open_close_match.insert('(', ')');
    open_close_match.insert('[', ']');
    open_close_match.insert('{', '}');
    open_close_match.insert('<', '>');

    let error_score = include_str!("../input.txt")
        .lines()
        .map(|line| check_line(line, &open_close_match))
        .map(|status| {
            match status {
                LineStatus::Corrupted(c) => match c {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => unreachable!()
                },
                LineStatus::Incomplete => 0
            }
        })
        .sum::<usize>();

    println!("Total error score: {}", error_score);
}

fn check_line(line: &str, open_close_match: &HashMap<char,char>) -> LineStatus {
    let mut open_chunks: Vec<char> = Vec::new();
    let mut line_status: LineStatus = LineStatus::Incomplete;

    let chars = line
        .chars()
        .into_iter()
        .map(|c| {
            if (c == '(') || (c == '[') || (c == '{') || (c == '<') {
                ChunkChar::Open(c)
            } else {
                ChunkChar::Close(c)
            }
        })
        .collect::<Vec<ChunkChar>>();

    for c in chars {
        match c {
            ChunkChar::Open(i) => { open_chunks.push(i); },
            ChunkChar::Close(i) => {
                let correct_close = open_close_match[open_chunks.last().unwrap()];
                if i == correct_close {
                    open_chunks.pop();
                } else {
                    line_status = LineStatus::Corrupted(i);
                    break;
                }
            }
        }
    }

    return line_status;
}
