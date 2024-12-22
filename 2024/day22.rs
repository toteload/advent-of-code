use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::str::FromStr;

pub fn parse_whitespace_separated_items<T: FromStr>(s: &str) -> Vec<T> {
    s.trim()
        .split_ascii_whitespace()
        .map(|s| s.parse::<T>().ok().unwrap())
        .collect()
}

#[derive(Copy, Clone)]
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
    let prices = xs.iter().map(|x| {
        (SecretGenerator { seed: *x })
            .map(|x| (x % 10) as i8)
            .take(2001) // <- This needs to be 2001, not 2000 >:(
    });

    let seqs = prices.clone().map(|xs| {
        let dt = xs
            .clone()
            .zip(xs.skip(1))
            .map(|(a, b)| b - a)
            .collect::<Vec<_>>();

        dt.windows(4)
            .map(|s| [s[0], s[1], s[2], s[3]])
            .collect::<Vec<_>>()
    });

    let buyers = prices.zip(seqs).map(|(p, s)| p.skip(4).zip(s));

    let mut seen: HashSet<[i8; 4]> = HashSet::new();
    let mut bananas: HashMap<[i8; 4], u64> = HashMap::new();

    for buyer in buyers {
        seen.clear();

        for (price, seq) in buyer {
            if seen.contains(&seq) {
                continue;
            }

            *bananas.entry(seq).or_insert(0) += price as u64;
            seen.insert(seq);
        }
    }

    *bananas.values().max().unwrap()
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let input = fs::read_to_string(filename).unwrap();
    let xs = parse_whitespace_separated_items::<u64>(&input);

    println!("{}", part1(&xs));
    println!("{}", part2(&xs));
}
