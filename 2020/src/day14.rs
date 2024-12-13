use std::{collections::HashMap, hash::Hash};

use crate::Problem;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{line_ending, u64},
    multi::separated_list0,
    sequence::delimited,
    IResult,
};

enum Instruction {
    // Terrible variable names, too lazy to change.
    // `mask` is a mask of all the Xs in the mask.
    // `val` is a mask of all the 1s in the mask.
    SetMask { mask: u64, val: u64 },
    Write { address: u64, val: u64 },
}

pub struct Instance {
    program: Vec<Instruction>,
}

fn parse_instruction(s: &str) -> IResult<&str, Instruction> {
    fn write_instruction(s: &str) -> IResult<&str, Instruction> {
        let (s, _) = tag("mem")(s)?;
        let (s, address) = delimited(tag("["), u64, tag("]"))(s)?;
        let (s, _) = tag(" = ")(s)?;
        let (s, val) = u64(s)?;

        Ok((s, Instruction::Write { address, val }))
    }

    fn mask_instruction(s: &str) -> IResult<&str, Instruction> {
        let (s, _) = tag("mask = ")(s)?;
        let (s, mask_str) = take(36usize)(s)?;

        let mask = mask_str
            .as_bytes()
            .iter()
            .rev()
            .enumerate()
            .fold(
                0u64,
                |acc, (i, c)| {
                    if *c == b'X' {
                        acc | 1 << i
                    } else {
                        acc
                    }
                },
            );

        let val = mask_str
            .as_bytes()
            .iter()
            .rev()
            .enumerate()
            .fold(
                0u64,
                |acc, (i, c)| {
                    if *c == b'1' {
                        acc | 1 << i
                    } else {
                        acc
                    }
                },
            );

        Ok((s, Instruction::SetMask { mask, val }))
    }

    alt((write_instruction, mask_instruction))(s)
}

fn parse(s: &str) -> Vec<Instruction> {
    separated_list0(line_ending, parse_instruction)(s)
        .unwrap()
        .1
}

impl<'a> Problem<'a> for Instance {
    fn new(input: &'a str) -> Instance {
        let program = parse(input);
        for inst in &program {
            let Instruction::SetMask { mask, val } = inst else { continue; };
            println!("{}", mask.count_ones());
        }

        Instance { program }
    }

    fn solve_part_one(&self) -> usize {
        let mut mem: HashMap<u64, u64> = HashMap::new();
        let mut mask = 0u64;
        let mut maskv = 0u64;

        for inst in &self.program {
            use Instruction::*;
            match inst {
                SetMask { mask: m, val: v } => {
                    mask = *m;
                    maskv = *v;
                }
                Write { address, val } => {
                    let x = (*val & mask) | maskv;
                    mem.insert(*address, x);
                }
            }
        }

        mem.into_values().sum::<u64>() as usize
    }

    fn solve_part_two(&self) -> usize {
        let mut mem: HashMap<u64, u64> = HashMap::new();
        let mut mask_unspecified = 0u64; // All the Xs in the mask
        let mut mask_set_to_one = 0u64; // All the 1s in the mask
        let mut unspecified_bit_index = Vec::new(); // Indices of the 1 positions in `mask_unspecified`

        for inst in &self.program {
            use Instruction::*;

            match inst {
                SetMask { mask: m, val: v } => {
                    unspecified_bit_index.clear();
                    let mut acc = *m;
                    while acc != 0 {
                        let i = acc.trailing_zeros();
                        unspecified_bit_index.push(i);
                        acc &= !(1u64 << i);
                    }

                    mask_unspecified = *m;
                    mask_set_to_one = *v;
                }

                Write { address, val } => {
                    let base_address = (address & !mask_unspecified) | mask_set_to_one;

                    for bits in 0..(1 << unspecified_bit_index.len()) {
                        let mut address = base_address;
                        for (i, j) in unspecified_bit_index.iter().enumerate() {
                            let x = ((bits >> i) & 1) << j;
                            address |= x;
                        }

                        mem.insert(address, *val);
                    }
                }
            }
        }

        mem.into_values().sum::<u64>() as usize
    }
}
