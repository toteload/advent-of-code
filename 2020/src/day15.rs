use crate::Problem;
use nom::{
    character::complete::{char, u64},
    multi::separated_list0,
    IResult,
};
use std::collections::HashMap;

pub struct Instance {
    starting_numbers: Vec<u64>,
}

impl<'a> Problem<'a> for Instance {
    fn new(input: &'a str) -> Instance {
        let res: IResult<_, _, nom::error::Error<&str>> = separated_list0(char(','), u64)(input);
        let starting_numbers = res.unwrap().1;
        Instance { starting_numbers }
    }

    fn solve_part_one(&self) -> usize {
        let mut history = self
            .starting_numbers
            .iter()
            .take(self.starting_numbers.len() - 1)
            .copied()
            .enumerate()
            .map(|(i, x)| (x, i))
            .collect::<HashMap<u64, usize>>();

        let mut last_number = *self.starting_numbers.last().unwrap();

        const N: usize = 2020;

        for idx in self.starting_numbers.len() - 1..(N - 1) {
            let x;
            if let Some(prev_idx) = history.get(&last_number) {
                x = idx - prev_idx;
            } else {
                x = 0;
            }
            history.insert(last_number, idx);
            last_number = x as u64;
        }

        last_number as usize
    }

    fn solve_part_two(&self) -> usize {
        let mut history = self
            .starting_numbers
            .iter()
            .take(self.starting_numbers.len() - 1)
            .copied()
            .enumerate()
            .map(|(i, x)| (x, i))
            .collect::<HashMap<u64, usize>>();

        let mut last_number = *self.starting_numbers.last().unwrap();

        const N: usize = 30_000_000;

        for idx in self.starting_numbers.len() - 1..(N - 1) {
            let x;
            if let Some(prev_idx) = history.get(&last_number) {
                x = idx - prev_idx;
            } else {
                x = 0;
            }
            history.insert(last_number, idx);
            last_number = x as u64;
        }

        last_number as usize
    }
}
