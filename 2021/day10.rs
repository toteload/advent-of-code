use std::io::{self, BufRead};

fn match_char(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!(),
    }
}

fn find_corruption(line: &str) -> Option<char> {
    let mut stack = Vec::new();

    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                if let Some(&last) = stack.last() {
                    if last != match_char(c) {
                        return Some(c);
                    } else {
                        stack.pop();
                    }
                } else {
                    // A closing character without a matching opening
                    // character. This shouldn't happen with our input.
                    unreachable!();
                }
            }
            _ => unreachable!(),
        }
    }

    None
}

fn part_one(lines: &[String]) -> usize {
    let mut score = 0;

    for line in lines {
        if let Some(c) = find_corruption(&line) {
            score += match c {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => unreachable!(),
            };
        }
    }

    score
}

fn part_two(lines: &[String]) -> usize {
    let mut scores = Vec::new();

    for line in lines.iter().filter(|line| find_corruption(line).is_none()) {
        let mut stack = Vec::new();

        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                // I assume that there will be no illegal characters that are
                // not a opening or closing character. Also all the corrupted
                // lines have been filtered out, so the closing characters will
                // be matched with their opening counterpart. For these reasons
                // you can just pop the last character.
                _ => {
                    stack.pop();
                }
            }
        }

        let score = stack
            .iter()
            .map(|&c| match_char(c))
            .enumerate()
            .map(|(i, c)|
                 match c {
                     ')' => 1,
                     ']' => 2,
                     '}' => 3,
                     '>' => 4,
                     _ => unreachable!(),
                 } * 5usize.pow(i as u32))
            .sum();

        scores.push(score);
    }

    scores.sort();

    scores[scores.len() / 2]
}

fn main() {
    let lines: Vec<String> = io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .collect();

    println!("{}", part_two(&lines));
}
