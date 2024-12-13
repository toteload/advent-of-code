use std::collections::HashSet;
use std::io::{self, BufRead};
use std::iter::FromIterator;

fn part_one(numbers: &[u32], width: usize, height: usize) -> u32 {
    let stride = width;

    let mut sum = 0;
    for y in 0..height {
        for x in 0..width {
            let at = numbers[x + y * stride];

            let is_minimum = !((y > 0 && numbers[x + (y - 1) * stride] <= at)
                || (y < (height - 1) && numbers[x + (y + 1) * stride] <= at)
                || (x > 0 && numbers[(x - 1) + y * stride] <= at)
                || (x < (width - 1) && numbers[(x + 1) + y * stride] <= at));

            if is_minimum {
                sum += at + 1;
            }
        }
    }

    sum
}

fn neighbours(pos: &(usize, usize), width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut res = Vec::new();

    if pos.0 > 0 {
        res.push((pos.0 - 1, pos.1))
    }
    if pos.1 > 0 {
        res.push((pos.0, pos.1 - 1))
    }
    if pos.0 < (width - 1) {
        res.push((pos.0 + 1, pos.1))
    }
    if pos.1 < (height - 1) {
        res.push((pos.0, pos.1 + 1))
    }

    res
}

fn part_two(numbers: &[u32], width: usize, height: usize) -> usize {
    let stride = width;

    let mut unvisited = HashSet::new();

    for y in 0..height {
        for x in 0..width {
            if numbers[x + y * stride] != 9 {
                unvisited.insert((x, y));
            }
        }
    }

    let mut basins: Vec<usize> = Vec::new();

    while !unvisited.is_empty() {
        let start: (usize, usize) = *unvisited.iter().next().unwrap();
        unvisited.remove(&start);

        let mut basin = vec![start];

        loop {
            let basin_and_border =
                &HashSet::from_iter(basin.iter().map(|p| neighbours(p, width, height)).flatten());

            let growth = &unvisited & &basin_and_border;

            if growth.is_empty() {
                break;
            }

            for x in growth.iter() {
                unvisited.remove(x);
            }

            basin.extend(growth.into_iter());
        }

        basins.push(basin.len());
    }

    basins.sort();

    basins.iter().rev().take(3).product()
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines().map(|x| x.unwrap()).collect();

    let width = lines[0].len();
    let height = lines.len();

    let numbers: Vec<u32> = lines
        .join("")
        .chars()
        .map(|c| (c as u32) - ('0' as u32))
        .collect();

    assert_eq!(width * height, numbers.len());

    println!("{}", part_two(&numbers, width, height));
}
