use crate::Problem;
use itertools::Itertools;

pub struct Instance {
    entries: Vec<i32>,
}

impl<'a> Problem<'a> for Instance {
    fn new(input: &str) -> Self {
        let entries = input
            .lines()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        Self { entries }
    }

    fn solve_part_one(&self) -> usize {
        for (a, b) in self.entries.iter().tuple_combinations() {
            if a + b == 2020 {
                return (a * b) as usize;
            }
        }

        unreachable!();
    }

    fn solve_part_two(&self) -> usize {
        for (a, b, c) in self.entries.iter().tuple_combinations() {
            if a + b + c == 2020 {
                return (a * b * c) as usize;
            }
        }

        unreachable!();
    }
}
