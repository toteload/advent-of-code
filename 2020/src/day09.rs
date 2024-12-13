use crate::{parse_number_list, Problem};
use itertools::Itertools;

pub struct Instance {
    numbers: Vec<i32>,
}

impl<'a> Problem<'a> for Instance {
    fn new(input: &'a str) -> Self {
        Instance {
            numbers: parse_number_list(input),
        }
    }

    fn solve_part_one(&self) -> usize {
        for xs in self.numbers.windows(26) {
            let [history @ .., target] = xs else { unreachable!() };

            let has_sum = history
                .iter()
                .tuple_combinations()
                .any(|(a, b)| a + b == *target);

            if !has_sum {
                return *target as usize;
            }
        }

        unreachable!()
    }

    fn solve_part_two(&self) -> usize {
        let target = self.solve_part_one() as i32;

        for start in 0..self.numbers.len() {
            for end in (start + 1)..self.numbers.len() {
                let group = &self.numbers[start..end];
                let sum: i32 = group.iter().sum();

                if sum == target {
                    return (group.iter().min().unwrap() + group.iter().max().unwrap()) as usize;
                }

                if sum > target {
                    break;
                }
            }
        }

        unreachable!()
    }
}
