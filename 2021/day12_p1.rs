use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

#[derive(PartialEq)]
enum CaveSize {
    Small,
    Big,
}

fn dfs(
    is_connected: &Vec<bool>,
    cave_sizes: &Vec<CaveSize>,
    visited: &mut Vec<bool>,
    at: usize,
    end: usize,
) -> usize {
    if at == end {
        return 1;
    }

    if visited[at] && cave_sizes[at] == CaveSize::Small {
        return 0;
    }

    let n = cave_sizes.len();
    let neighbours: Vec<usize> = is_connected
        .iter()
        .skip(at * n)
        .take(n)
        .enumerate()
        .filter_map(|(i, &x)| if x { Some(i) } else { None })
        .collect();

    visited[at] = true;

    let mut sum = 0;
    for nb in neighbours {
        sum += dfs(is_connected, cave_sizes, visited, nb, end);
    }

    visited[at] = false;

    sum
}

fn main() {
    let lines: Vec<(String, String)> = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let l = line.unwrap();
            let mut parts = l.split('-');
            (
                parts.next().unwrap().to_string(),
                parts.next().unwrap().to_string(),
            )
        })
        .collect();

    let names: Vec<&String> = lines
        .iter()
        .map(|(a, b)| vec![a, b])
        .flatten()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    let lookup: HashMap<&String, usize> = names
        .iter()
        .enumerate()
        .map(|(i, &name)| (name, i))
        .collect();

    let cave_sizes: Vec<CaveSize> = names
        .iter()
        .map(|&name| {
            if name.chars().next().unwrap().is_lowercase() {
                CaveSize::Small
            } else {
                CaveSize::Big
            }
        })
        .collect();

    let mut is_connected: Vec<bool> = vec![false; names.len().pow(2)];

    let stride = names.len();

    for (from, to) in &lines {
        let a = lookup[from];
        let b = lookup[to];

        is_connected[a + b * stride] = true;
        is_connected[b + a * stride] = true;
    }

    let start = lookup[&"start".to_string()];
    let end = lookup[&"end".to_string()];

    let mut visited = vec![false; names.len()];

    println!(
        "{}",
        dfs(&is_connected, &cave_sizes, &mut visited, start, end)
    );
}
