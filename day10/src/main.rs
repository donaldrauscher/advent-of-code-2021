use std::collections::HashMap;

#[derive(Debug)]
enum ChunkChar {
    Open(char),
    Close(char)
}

#[derive(Debug)]
enum LineStatus {
    Incomplete(Vec<char>),
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
                LineStatus::Incomplete(_) => 0
            }
        })
        .sum::<usize>();

    let mut autocomplete_score = include_str!("../input.txt")
        .lines()
        .map(|line| check_line(line, &open_close_match))
        .filter_map(|status| {
            match status {
                LineStatus::Corrupted(_) => None,
                LineStatus::Incomplete(c) => {
                    Some(c
                        .iter()
                        .fold(0, |total, cc| {
                            total*5 + match cc {
                                ')' => 1,
                                ']' => 2,
                                '}' => 3,
                                '>' => 4,
                                _ => unreachable!()
                            }
                        }))
                }
            }
        })
        .collect::<Vec<usize>>();

    autocomplete_score.sort();
    let auto_complete_score_mid = autocomplete_score[autocomplete_score.len()/2];

    println!("Total error score: {:?}", error_score);
    println!("Middle autocompletion score: {:?}", auto_complete_score_mid);
}

fn check_line(line: &str, open_close_match: &HashMap<char,char>) -> LineStatus {
    let mut open_chunks: Vec<char> = Vec::new();
    let mut line_status: Option<LineStatus> = None;

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
                    line_status = Some(LineStatus::Corrupted(i));
                    break;
                }
            }
        }
    }

    if let Some(ls) = line_status {
        return ls;
    } else {
        let mut completion_chars: Vec<char> = Vec::new();
        while let Some(c) = open_chunks.pop() {
            completion_chars.push(open_close_match[&c]);
        }
        return LineStatus::Incomplete(completion_chars);
    }
}
