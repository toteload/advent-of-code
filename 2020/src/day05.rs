use crate::Problem;

pub struct Instance {
    seats: Vec<u16>,
}

impl<'a> Problem<'a> for Instance {
    fn new(input: &'a str) -> Self {
        let seats = input
            .lines()
            .map(|line| {
                line.as_bytes()
                    .iter()
                    .map(|x| *x == b'B' || *x == b'R')
                    .rev()
                    .enumerate()
                    .fold(0, |acc, (i, x)| acc | (u16::from(x) << i))
            })
            .collect::<Vec<_>>();
        Instance { seats }
    }

    fn solve_part_one(&self) -> usize {
        *self.seats.iter().max().unwrap() as usize
    }

    fn solve_part_two(&self) -> usize {
        let mut bitmap: Vec<bool> = vec![false; 128 * 8];

        for seat in &self.seats {
            bitmap[*seat as usize] = true;
        }

        for i in 1..bitmap.len() - 1 {
            if bitmap[i - 1] && !bitmap[i] && bitmap[i + 1] {
                return i;
            }
        }

        unreachable!();
    }
}
