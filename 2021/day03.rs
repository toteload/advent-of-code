use std::io::{self, BufRead};

fn part_one(bitwidth: usize, readings: &[usize]) -> usize {
    let mut counts = vec![0; bitwidth];

    for x in readings {
        for i in 0..bitwidth {
            let mask = 1 << i;
            let v = (x & mask) >> i;
            counts[i] += v;
        }
    }

    let gamma = {
        let mut gamma = 0;
        let n2 = readings.len() / 2;

        for i in 0..bitwidth {
            if counts[i] > n2 {
                gamma |= 1 << i;
            }
        }

        gamma
    };

    let mask = (1 << bitwidth) - 1;
    let epsilon = (!gamma) & mask;

    gamma * epsilon
}

fn oxy_rating(bitwidth: usize, idx: usize, readings: &[usize]) -> usize {
    let one_count = {
        let mut count = 0;
        let mask = 1 << (bitwidth - (idx + 1));

        for x in readings {
            if (x & mask) != 0 {
                count += 1;
            }
        }

        count
    };

    let residu: Vec<usize> = {
        let mask = 1 << (bitwidth - (idx + 1));
        let n2 = readings.len() / 2;

        if one_count == n2 && (n2 * 2) == readings.len() {
            readings
                .iter()
                .cloned()
                .filter(|x| (x & mask) != 0)
                .collect()
        } else {
            let key = if one_count > n2 {
                mask
            } else {
                0
            };

            readings
                .iter()
                .cloned()
                .filter(|x| (x & mask) == key)
                .collect()
        }
    };

    if residu.len() == 1 {
        residu[0]
    } else {
        oxy_rating(bitwidth, idx + 1, &residu)
    }
}

fn co2_rating(bitwidth: usize, idx: usize, readings: &[usize]) -> usize {
    let one_count = {
        let mut count = 0;
        let mask = 1 << (bitwidth - (idx + 1));

        for x in readings {
            if (x & mask) != 0 {
                count += 1;
            }
        }

        count
    };

    let residu: Vec<usize> = {
        let mask = 1 << (bitwidth - (idx + 1));
        let n2 = readings.len() / 2;

        if one_count == n2 && (n2 * 2) == readings.len() {
            readings
                .iter()
                .cloned()
                .filter(|x| (x & mask) == 0)
                .collect()
        } else {
            let key = if one_count > n2 {
                0
            } else {
                mask
            };

            readings
                .iter()
                .cloned()
                .filter(|x| (x & mask) == key)
                .collect()
        }
    };

    if residu.len() == 1 {
        residu[0]
    } else {
        co2_rating(bitwidth, idx + 1, &residu)
    }
}

fn part_two(bitwidth: usize, readings: &[usize]) -> usize {
    // The functions for calculating these two ratings are for the most part
    // the same, and could be merged into one parameterizable function, but eh.
    // Simply copying the function and changing what was necessary was easier.

    oxy_rating(bitwidth, 0, readings) * co2_rating(bitwidth, 0, readings)
}

fn main() {
    let readings = io::stdin()
        .lock()
        .lines()
        .map(|s| s.unwrap())
        .collect::<Vec<_>>();

    let bitwidth = readings[0].len();

    assert!(bitwidth <= 64);

    let numbers = readings
        .iter()
        .map(|s| usize::from_str_radix(s, 2).unwrap())
        .collect::<Vec<_>>();

    println!("{}", part_two(bitwidth, &numbers));
}
