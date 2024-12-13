use crate::Problem;
use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_while};
use nom::character::complete::line_ending;
use nom::multi::separated_list0;
use nom::IResult;

pub struct Instance {
    code: Vec<Instruction>,
}

#[derive(Clone, Copy)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

fn parse_instruction(s: &str) -> IResult<&str, Instruction> {
    let (s, instruction_name) = alt((tag("nop"), tag("acc"), tag("jmp")))(s)?;
    let (s, _) = take(1usize)(s)?;
    let (s, amount_str) = take_while(|c: char| c == '-' || c == '+' || c.is_ascii_digit())(s)?;
    let amount = amount_str.parse::<i32>().unwrap();

    let instruction = match instruction_name {
        "nop" => Instruction::Nop(amount),
        "acc" => Instruction::Acc(amount),
        "jmp" => Instruction::Jmp(amount),
        _ => unreachable!(),
    };

    Ok((s, instruction))
}

fn parse(input: &str) -> Vec<Instruction> {
    separated_list0(line_ending, parse_instruction)(input)
        .unwrap()
        .1
}

enum BootCodeResult {
    Terminated(i32),
    InfiniteLoop(i32),
}

fn test_boot_code(code: &[Instruction]) -> BootCodeResult {
    let mut visited = vec![false; code.len()];
    let mut acc = 0i32;
    let mut pc = 0;
    let end = code.len();

    while pc < end && !visited[pc] {
        use Instruction::*;

        visited[pc] = true;

        match code[pc] {
            Nop(_) => pc += 1,
            Acc(x) => {
                acc += x;
                pc += 1;
            }
            Jmp(x) => pc = ((pc as i32) + x) as usize,
        }
    }

    if pc >= end {
        BootCodeResult::Terminated(acc)
    } else {
        BootCodeResult::InfiniteLoop(acc)
    }
}

impl<'a> Problem<'a> for Instance {
    fn new(input: &'a str) -> Self {
        let code = parse(input);
        Instance { code }
    }

    fn solve_part_one(&self) -> usize {
        match test_boot_code(&self.code) {
            BootCodeResult::InfiniteLoop(acc) => acc as usize,
            _ => unreachable!(),
        }
    }

    fn solve_part_two(&self) -> usize {
        // Brute force search.
        use Instruction::*;

        let mut code_fix = self.code.clone();

        for i in 0..self.code.len() {
            match code_fix[i] {
                Nop(x) => code_fix[i] = Jmp(x),
                Jmp(x) => code_fix[i] = Nop(x),
                Acc(_) => continue,
            }

            if let BootCodeResult::Terminated(acc) = test_boot_code(&code_fix) {
                return acc as usize;
            }

            match code_fix[i] {
                Nop(x) => code_fix[i] = Jmp(x),
                Jmp(x) => code_fix[i] = Nop(x),
                Acc(_) => unreachable!(),
            }
        }

        unreachable!()
    }
}
