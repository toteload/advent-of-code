use std::io::{self, BufRead};

fn part_one(mut octopuses: Vec<u8>) -> usize {
    let step_count = 100;
    let n = 10;

    let mut flash_count: usize = 0;

    for _ in 0..step_count {
        let mut has_flashed = vec![false; n * n];

        for o in octopuses.iter_mut() {
            *o += 1;
        }

        let mut should_flash: Vec<usize> = octopuses
            .iter()
            .enumerate()
            .filter_map(|(i, &o)| {
                if o > 9 && !has_flashed[i] {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();

        while !should_flash.is_empty() {
            for p in should_flash {
                // Increase the energy levels of the neighbours

                // Calculate all the valid neighbours. This is ugly as hell.
                let mut neighbours = Vec::new();

                let x = p % n;
                let y = p / n;

                if y != 0 {
                    if x != 0 {
                        neighbours.push(p - n - 1);
                    }

                    neighbours.push(p - n);

                    if x != (n - 1) {
                        neighbours.push(p - n + 1);
                    }
                }

                if x != 0 {
                    neighbours.push(p - 1);
                }

                if x != (n - 1) {
                    neighbours.push(p + 1);
                }

                if y != (n - 1) {
                    if x != 0 {
                        neighbours.push(p + n - 1);
                    }

                    neighbours.push(p + n);

                    if x != (n - 1) {
                        neighbours.push(p + n + 1);
                    }
                }

                for nb in neighbours {
                    if nb < (n * n) {
                        octopuses[nb] += 1;
                    }
                }

                has_flashed[p] = true;
            }

            should_flash = octopuses
                .iter()
                .enumerate()
                .filter_map(|(i, &o)| {
                    if o > 9 && !has_flashed[i] {
                        Some(i)
                    } else {
                        None
                    }
                })
                .collect();
        }

        // Octopuses that flashed this round have their energy level set to 0.
        for (o, &flashed) in octopuses.iter_mut().zip(&has_flashed) {
            if flashed {
                *o = 0;
            }
        }

        flash_count += has_flashed.iter().filter(|&&x| x).count();
    }

    flash_count
}

fn part_two(mut octopuses: Vec<u8>) -> usize {
    let mut step_count = 0;
    let n = 10;

    loop {
        let mut has_flashed = vec![false; n * n];

        for o in octopuses.iter_mut() {
            *o += 1;
        }

        let mut should_flash: Vec<usize> = octopuses
            .iter()
            .enumerate()
            .filter_map(|(i, &o)| {
                if o > 9 && !has_flashed[i] {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();

        while !should_flash.is_empty() {
            for p in should_flash {
                // Increase the energy levels of the neighbours

                // Calculate all the valid neighbours. This is ugly as hell.
                let mut neighbours = Vec::new();

                let x = p % n;
                let y = p / n;

                if y != 0 {
                    if x != 0 {
                        neighbours.push(p - n - 1);
                    }

                    neighbours.push(p - n);

                    if x != (n - 1) {
                        neighbours.push(p - n + 1);
                    }
                }

                if x != 0 {
                    neighbours.push(p - 1);
                }

                if x != (n - 1) {
                    neighbours.push(p + 1);
                }

                if y != (n - 1) {
                    if x != 0 {
                        neighbours.push(p + n - 1);
                    }

                    neighbours.push(p + n);

                    if x != (n - 1) {
                        neighbours.push(p + n + 1);
                    }
                }

                for nb in neighbours {
                    if nb < (n * n) {
                        octopuses[nb] += 1;
                    }
                }

                has_flashed[p] = true;
            }

            should_flash = octopuses
                .iter()
                .enumerate()
                .filter_map(|(i, &o)| {
                    if o > 9 && !has_flashed[i] {
                        Some(i)
                    } else {
                        None
                    }
                })
                .collect();
        }

        // Octopuses that flashed this round have their energy level set to 0.
        for (o, &flashed) in octopuses.iter_mut().zip(&has_flashed) {
            if flashed {
                *o = 0;
            }
        }

        step_count += 1;

        if has_flashed.iter().all(|&x| x) {
            break;
        }
    }

    step_count
}

fn main() {
    let octopuses: Vec<u8> = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| (c as u8) - ('0' as u8))
                .collect::<Vec<u8>>()
        })
        .flatten()
        .collect();

    println!("{}", part_two(octopuses));
}
