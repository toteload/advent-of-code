use crate::Problem;
use itertools::Itertools;
use std::ops::BitXor;

struct Entry<'a> {
    at_least: usize,
    at_most: usize,
    letter: u8,
    password: &'a [u8],
}

impl<'a> Entry<'a> {
    fn from_line(s: &str) -> Entry {
        let mut parts = s.split_ascii_whitespace();

        let range = parts
            .next()
            .unwrap()
            .split('-')
            .map(|s| s.parse::<usize>().unwrap());
        let Some((at_least, at_most)) = range.tuples().next() else { unreachable!(); };

        let letter = parts.next().unwrap().as_bytes()[0];

        let password = parts.next().unwrap().as_bytes();

        Entry {
            at_least,
            at_most,
            letter,
            password,
        }
    }
}

pub struct Instance<'a> {
    entries: Vec<Entry<'a>>,
}

impl<'a> Problem<'a> for Instance<'a> {
    fn new(input: &'a str) -> Self {
        let entries = input.lines().map(Entry::from_line).collect::<Vec<_>>();
        Instance { entries }
    }

    fn solve_part_one(&self) -> usize {
        let mut valid_count = 0;
        for entry in &self.entries {
            let Entry {
                at_least,
                at_most,
                letter,
                password,
            } = entry;

            let count: usize = password
                .iter()
                .fold(0, |acc, c| acc + usize::from(c == letter));
            if (at_least..=at_most).contains(&&count) {
                valid_count += 1;
            }
        }

        valid_count
    }

    fn solve_part_two(&self) -> usize {
        let mut valid_count = 0;
        for entry in &self.entries {
            let Entry {
                at_least: i,
                at_most: j,
                letter,
                password,
            } = entry;

            let is_valid = (password[i - 1] == *letter).bitxor(password[j - 1] == *letter);
            valid_count += usize::from(is_valid);
        }

        valid_count
    }
}
