use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

#[derive(PartialEq)]
enum CaveSize {
    Small,
    Big,
}

fn dfs(
    is_connected: &Vec<bool>,
    max_visits: &Vec<usize>,
    visit_count: &mut Vec<usize>,
    path: &mut Vec<usize>,
    out_paths: &mut Vec<Vec<usize>>,
    at: usize,
    end: usize,
) {
    if visit_count[at] == max_visits[at] {
        return;
    }

    path.push(at);

    if at == end {
        out_paths.push(path.clone());
    } else {
        let n = max_visits.len();
        let neighbours: Vec<usize> = is_connected
            .iter()
            .skip(at * n)
            .take(n)
            .enumerate()
            .filter_map(|(i, &x)| if x { Some(i) } else { None })
            .collect();

        visit_count[at] += 1;

        for nb in neighbours {
            dfs(
                is_connected,
                max_visits,
                visit_count,
                path,
                out_paths,
                nb,
                end,
            );
        }

        visit_count[at] -= 1;
    }

    path.pop();
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

    let small_caves: Vec<usize> = cave_sizes
        .iter()
        .enumerate()
        .filter_map(|(i, size)| {
            if size == &CaveSize::Small && i != start {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    let mut paths = Vec::new();

    for &i in &small_caves {
        let mut visit_count = vec![0; names.len()];

        let mut max_visits: Vec<usize> = cave_sizes
            .iter()
            .map(|size| {
                if size == &CaveSize::Small {
                    1
                } else {
                    usize::MAX
                }
            })
            .collect();

        max_visits[i] = 2;

        let mut acc = Vec::new();
        dfs(
            &is_connected,
            &max_visits,
            &mut visit_count,
            &mut acc,
            &mut paths,
            start,
            end,
        );
    }

    println!("{}", paths.iter().collect::<HashSet<_>>().len());
}
