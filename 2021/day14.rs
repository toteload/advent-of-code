use std::cmp;
use std::collections::HashMap;
use std::io::{self, BufRead};

struct Rule {
    pair: (u8, u8),
    insert: u8,
}

impl Rule {
    fn parse_from_line(line: &str) -> Rule {
        let mut parts = line.split("->");
        let pair = parts.next().unwrap().trim().as_bytes();
        let insert = parts.next().unwrap().trim().as_bytes();

        Rule {
            pair: (pair[0], pair[1]),
            insert: insert[0],
        }
    }
}

struct PolymerState {
    counts: HashMap<u8, isize>,
    pairs: HashMap<(u8, u8), isize>,
}

impl PolymerState {
    fn create(initial_polymer: &[u8], rules: &[Rule]) -> PolymerState {
        let mut counts = HashMap::new();

        // I assume here that each unique character that can occur is present
        // in the initial polymer. I am not sure if this is true though :P.
        for x in initial_polymer {
            counts.insert(*x, 0);
        }

        // The above assumption was indeed not true :), but adding the loop
        // below fixed it.
        for Rule { insert, .. } in rules {
            counts.insert(*insert, 0);
        }

        for x in initial_polymer {
            *counts.get_mut(x).unwrap() += 1;
        }

        let mut pairs = HashMap::new();

        for Rule { pair, .. } in rules {
            pairs.insert(*pair, 0);
        }

        for pair in initial_polymer.windows(2) {
            *pairs.get_mut(&(pair[0], pair[1])).unwrap() += 1;
        }

        PolymerState { counts, pairs }
    }

    fn step(&mut self, rules: &[Rule]) {
        let mut pair_changes: HashMap<(u8, u8), isize> = HashMap::new();

        for Rule {
            pair: pair @ (a, b),
            insert,
        } in rules
        {
            pair_changes.insert((*a, *insert), 0);
            pair_changes.insert((*insert, *b), 0);
            pair_changes.insert(*pair, 0);
        }

        for Rule {
            pair: pair @ (a, b),
            insert,
        } in rules
        {
            if let Some((pair, count)) = self.pairs.get_key_value(&pair) {
                *pair_changes.get_mut(&(*a, *insert)).unwrap() += count;
                *pair_changes.get_mut(&(*insert, *b)).unwrap() += count;
                *pair_changes.get_mut(pair).unwrap() -= count;

                *self.counts.get_mut(insert).unwrap() += count;
            }
        }

        for (pair, dt) in pair_changes {
            *self.pairs.get_mut(&pair).unwrap() += dt;
        }
    }
}

fn main() {
    let mut lines = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let initial_polymer: Vec<u8> = lines.swap_remove(0).into_bytes();

    lines.swap_remove(1);

    let rules = lines
        .iter()
        .map(|line| Rule::parse_from_line(&line))
        .collect::<Vec<Rule>>();

    let mut polymer = PolymerState::create(&initial_polymer, &rules);

    for _ in 0..40 {
        polymer.step(&rules);
    }

    let counts = polymer
        .counts
        .iter()
        .map(|(_, x)| x)
        .copied()
        .collect::<Vec<_>>();

    let most_common = counts
        .iter()
        .copied()
        .reduce(|acc, x| cmp::max(acc, x))
        .unwrap();

    let least_common = counts
        .iter()
        .copied()
        .reduce(|acc, x| cmp::min(acc, x))
        .unwrap();

    println!(
        "{} - {} = {}",
        most_common,
        least_common,
        most_common - least_common
    );
}
