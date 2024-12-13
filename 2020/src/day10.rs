use crate::{parse_number_list, Problem};
use std::collections::HashMap;

pub struct Instance {
    numbers: Vec<i32>,
}

struct PathCounter {
    connections: HashMap<i32, Vec<i32>>,
    memoize: HashMap<i32, usize>,
}

impl PathCounter {
    fn new(connections: HashMap<i32, Vec<i32>>) -> Self {
        PathCounter {
            connections,
            memoize: HashMap::from([(0, 1)]),
        }
    }

    fn count(&mut self, dst: i32) -> usize {
        if let Some(x) = self.memoize.get(&dst) {
            return *x;
        }

        let ps = self.connections.get(&dst).unwrap().clone();

        let mut acc = 0usize;

        for p in ps {
            acc += self.count(p);
        }

        self.memoize.insert(dst, acc);

        acc
    }
}

impl<'a> Problem<'a> for Instance {
    fn new(input: &'a str) -> Self {
        Instance {
            numbers: parse_number_list(input),
        }
    }

    fn solve_part_one(&self) -> usize {
        let mut xs = self.numbers.clone();
        xs.push(0);
        xs.sort();

        let one_diff_count = xs.windows(2).filter(|x| x[1] - x[0] == 1).count();
        let three_diff_count = 1 + xs.windows(2).filter(|x| x[1] - x[0] == 3).count();

        one_diff_count * three_diff_count
    }

    fn solve_part_two(&self) -> usize {
        let mut connections: HashMap<i32, Vec<i32>> = HashMap::new();

        let mut xs = self.numbers.clone();
        xs.push(0);
        xs.sort();
        xs.push(xs.last().unwrap() + 3);
        xs.reverse();

        for i in 0..xs.len() - 1 {
            let x = xs[i];
            let mut cons = Vec::new();
            for y in &xs[i + 1..] {
                if x.abs_diff(*y) > 3 {
                    break;
                }

                cons.push(*y);
            }
            connections.insert(x, cons);
        }

        let goal = xs[0];
        PathCounter::new(connections).count(goal)
    }
}
