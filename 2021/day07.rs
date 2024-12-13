use std::io;

fn abs_diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn bruteforce_part_one(crabs: &[usize]) -> usize {
    let mut fuel_cost = crabs.iter().sum();

    for pos in 1..(crabs.len()) {
        let fuel = crabs.iter().map(|&x| abs_diff(x, pos)).sum();

        if fuel > fuel_cost {
            break;
        }

        fuel_cost = fuel;
    }

    fuel_cost
}

fn move_cost(n: usize) -> usize {
    (n * (n + 1)) / 2
}

fn bruteforce_part_two(crabs: &[usize]) -> usize {
    let mut fuel_cost = crabs.iter().map(|&x| move_cost(x)).sum();

    for pos in 1..(crabs.len()) {
        let fuel = crabs.iter().map(|&x| move_cost(abs_diff(x, pos))).sum();

        if fuel > fuel_cost {
            break;
        }

        fuel_cost = fuel;
    }

    fuel_cost
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let crabs = line
        .split(',')
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    println!("{}", bruteforce_part_two(&crabs));
}
