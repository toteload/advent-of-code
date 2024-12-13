use std::io::{self, BufRead};

fn main() {
    // Part one
    // let increments: usize = io::stdin()
    //     .lock()
    //     .lines()
    //     .map(|s| s.unwrap().parse::<usize>().unwrap())
    //     .collect::<Vec<usize>>()
    //     .windows(2)
    //     .map(|x| if x[1] > x[0] { 1 } else { 0 })
    //     .sum();

    // Part two
    let increments: usize = io::stdin()
        .lock()
        .lines()
        .map(|s| s.unwrap().parse::<usize>().unwrap())
        .collect::<Vec<usize>>()

        .windows(3)
        .map(|x| x.iter().sum())
        .collect::<Vec<usize>>()

        .windows(2)
        .map(|x| if x[1] > x[0] { 1 } else { 0 })
        .sum();

    println!("{}", increments);


}
