use std::convert::identity;
use std::io::{self, BufRead};

#[derive(Clone)]
struct Board {
    nums: Vec<usize>,
    marks: Vec<bool>,
}

impl Board {
    fn has_bingo(&self) -> bool {
        let has_horizontal_bingo = self
            .marks
            .chunks(5)
            .map(|row| row.iter().all(|&x| x))
            .any(identity);

        if has_horizontal_bingo {
            return true;
        }

        for i in 0..5 {
            let is_bingo = self.marks.iter().skip(i).step_by(5).all(|&x| x);
            if is_bingo {
                return true;
            }
        }

        false
    }

    fn mark_number(&mut self, x: usize) {
        if let Some(idx) = self.nums.iter().position(|&y| x == y) {
            self.marks[idx] = true;
        }
    }

    fn unmarked_numbers_sum(&self) -> usize {
        self.nums
            .iter()
            .zip(self.marks.iter())
            .filter_map(|(x, mark)| if !mark { Some(x) } else { None })
            .sum()
    }

    fn parse_from_lines(lines: &[String]) -> Board {
        let nums = lines
            .iter()
            .flat_map(|line| line.trim().split_whitespace())
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        assert_eq!(nums.len(), 25);

        Board {
            nums,
            marks: vec![false; 25],
        }
    }
}

fn part_one(picks: &[usize], boards: &[Board]) -> usize {
    let mut bs = Vec::new();
    bs.extend_from_slice(boards);

    for pick in picks {
        for board in &mut bs {
            board.mark_number(*pick);

            if board.has_bingo() {
                return pick * board.unmarked_numbers_sum();
            }
        }
    }

    unreachable!();
}

fn part_two(picks: &[usize], boards: &[Board]) -> usize {
    let mut bs = Vec::new();
    bs.extend_from_slice(boards);

    for pick in picks {
        for board in &mut bs {
            board.mark_number(*pick);
        }

        if bs.len() == 1 && bs[0].has_bingo() {
            return pick * bs[0].unmarked_numbers_sum();
        }

        bs = bs.into_iter().filter(|b| !b.has_bingo()).collect();
    }

    unreachable!()
}

fn main() {
    let lines = io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>()
        .into_boxed_slice();

    let mut parts = lines.split(|line| line.is_empty());

    let pick_line = parts.next().unwrap();

    let picks = pick_line[0]
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let boards = parts
        .map(|x| Board::parse_from_lines(x))
        .collect::<Vec<Board>>();

    println!("{}", part_two(&picks, &boards));
}
