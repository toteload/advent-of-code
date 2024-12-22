use std::collections::HashMap;
use std::env;
use std::fs;
use std::str::FromStr;

pub fn parse_whitespace_separated_items<T: FromStr>(s: &str) -> Vec<T> {
    s.trim()
        .split_ascii_whitespace()
        .map(|s| s.parse::<T>().ok().unwrap())
        .collect()
}

struct SecretGenerator {
    seed: u64,
}

impl Iterator for SecretGenerator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let x = self.seed;
        self.seed = step(self.seed);
        Some(x)
    }
}

fn mixprune(a: u64, x: u64) -> u64 {
    (a ^ x) % 16777216
}

fn step(x: u64) -> u64 {
    let x = mixprune(x, x * 64);
    let x = mixprune(x, x / 32);
    let x = mixprune(x, x * 2048);
    x
}

fn part1(xs: &[u64]) -> u64 {
    xs.iter()
        .map(|x| (SecretGenerator { seed: *x }).nth(2000).unwrap())
        .sum()
}

fn part2(xs: &[u64]) -> u64 {
    let prices: Vec<Vec<i8>> = xs
        .iter()
        .map(|x| {
            (SecretGenerator { seed: *x })
                .map(|x| (x % 10) as i8)
                .take(2001) // <- This needs to be 2001, not 2000 >:(
                .collect()
        })
        .collect();

    let seqs: Vec<Vec<[i8; 4]>> = prices
        .iter()
        .map(|xs| {
            let dt: Vec<i8> = xs
                .iter()
                .zip(xs.iter().skip(1))
                .map(|(a, b)| b - a)
                .collect();
            dt.windows(4).map(|s| [s[0], s[1], s[2], s[3]]).collect()
        })
        .collect();

    let entries: Vec<Vec<(i8, [i8; 4])>> = prices
        .into_iter()
        .zip(seqs.into_iter())
        .map(|(p, s)| p.into_iter().skip(4).zip(s.into_iter()).collect())
        .collect();

    let mut memo: HashMap<[i8; 4], u64> = HashMap::new();

    for entry in entries.iter() {
        for (_, s) in entry.iter() {
            if memo.contains_key(s) {
                continue;
            }

            let score: u64 = entries
                .iter()
                .filter_map(|e| e.iter().find(|&&x| x.1 == *s))
                .map(|&x| x.0 as u64)
                .sum();

            memo.insert(*s, score);
        }
    }

    *memo.values().max().unwrap()
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let input = fs::read_to_string(filename).unwrap();
    let xs = parse_whitespace_separated_items::<u64>(&input);

    println!("{}", part1(&xs));
    println!("{}", part2(&xs));
}
