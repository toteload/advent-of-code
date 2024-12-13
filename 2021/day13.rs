use std::cmp::max;
use std::collections::HashSet;
use std::io::{self, BufRead};
use std::iter::FromIterator;

enum Fold {
    AlongX(usize),
    AlongY(usize),
}

impl Fold {
    fn from_line(line: &str) -> Fold {
        let parts: Vec<&str> = line.split('=').collect();
        let axis = *parts[0].as_bytes().last().unwrap();
        let val: usize = parts[1].parse().unwrap();

        if axis == b'x' {
            Fold::AlongX(val)
        } else {
            Fold::AlongY(val)
        }
    }
}

fn fold_paper(dots: HashSet<(usize, usize)>, fold: &Fold) -> HashSet<(usize, usize)> {
    let mut set = HashSet::new();

    for dot @ (x, y) in dots {
        set.insert(match fold {
            Fold::AlongX(val) => {
                if x > *val {
                    (2 * val - x, y)
                } else {
                    dot
                }
            }
            Fold::AlongY(val) => {
                if y > *val {
                    (x, 2 * val - y)
                } else {
                    dot
                }
            }
        });
    }

    set
}

fn main() {
    let lines: Vec<String> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let mut lines_iter = lines.iter();

    let dots: HashSet<(usize, usize)> = lines_iter
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split(',');
            (
                parts.next().unwrap().parse::<usize>().unwrap(),
                parts.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect();

    let folds: Vec<Fold> = lines_iter.map(|line| Fold::from_line(&line)).collect();

    let res = folds.iter().fold(dots, |acc, fold| fold_paper(acc, fold));

    let (width, height) = res.iter().fold((0, 0), |(ax, ay), (x, y)| {
        (max(ax, *x + 1), max(ay, *y + 1))
    });

    let mut bitmap = vec!['.'; width * height];
    let stride = width;

    for (x, y) in res.iter() {
        bitmap[x + y * stride] = '#';
    }

    for row in bitmap.chunks(width) {
        println!("{}", String::from_iter(row));
    }
}
