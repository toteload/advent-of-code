use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let starting_population = line
        .split(',')
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut population = [0; 9];

    for x in starting_population {
        population[x] += 1;
    }

    let days = 256;

    for _ in 0..days {
        let fuckers = population[0];

        for i in 0..8 {
            population[i] = population[i + 1];
        }

        population[8] = 0;

        population[6] += fuckers;
        population[8] += fuckers;
    }

    let total: usize = population.iter().sum();

    println!("{}", total);
}
