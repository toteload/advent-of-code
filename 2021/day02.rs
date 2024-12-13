use std::io::{self, BufRead};

enum Instruction {
    Forward(isize),
    Down(isize),
    Up(isize),
}

fn parse_instruction(s: &str) -> Instruction {
    let parts = s.split(' ').collect::<Vec<_>>();
    let units = parts[1].parse::<isize>().unwrap();
    match parts[0] {
        "forward" => Instruction::Forward(units),
        "down" => Instruction::Down(units),
        "up" => Instruction::Up(units),
        _ => unreachable!(),
    }
}

fn part_one(instructions: &[Instruction]) -> isize {
    let mut depth: isize = 0;
    let mut position: isize = 0;

    for inst in instructions {
        match inst {
            Instruction::Forward(x) => position += x,
            Instruction::Up(x) => depth -= x,
            Instruction::Down(x) => depth += x,
        }
    }

    depth * position
}

fn part_two(instructions: &[Instruction]) -> isize {
    let mut depth: isize = 0;
    let mut position: isize = 0;
    let mut aim: isize = 0;

    for inst in instructions {
        match inst {
            Instruction::Forward(x) => {
                position += x;
                depth += aim * x;
            },
            Instruction::Up(x) => aim -= x,
            Instruction::Down(x) => aim += x,
        }
    }

    depth * position
}

fn main() {
    let instructions = io::stdin()
        .lock()
        .lines()
        .map(|s| parse_instruction(&s.unwrap()))
        .collect::<Vec<_>>();

    println!("{}", part_two(&instructions));
}
