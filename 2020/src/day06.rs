use std::collections::HashSet;

use crate::Problem;

pub struct Instance<'a> {
    input: &'a str,
}

impl<'a> Problem<'a> for Instance<'a> {
    fn new(input: &'a str) -> Self {
        Instance { input }
    }

    fn solve_part_one(&self) -> usize {
        let lines = self.input.lines().collect::<Vec<_>>();
        lines
            .split(|line| line.is_empty())
            .map(|group_lines| {
                group_lines
                    .iter()
                    .flat_map(|line| line.as_bytes().to_vec())
                    .collect::<HashSet<_>>()
            })
            .map(|s| s.len())
            .sum()
    }

    fn solve_part_two(&self) -> usize {
        let lines = self.input.lines().collect::<Vec<_>>();
        lines
            .split(|line| line.is_empty())
            .map(|group_lines| {
                group_lines
                    .iter()
                    .map(|line| line.as_bytes().iter().copied().collect::<HashSet<_>>())
                    .reduce(|acc, x| acc.intersection(&x).copied().collect::<HashSet<_>>())
                    .unwrap()
            })
            .map(|s| s.len())
            .sum()
    }
}
