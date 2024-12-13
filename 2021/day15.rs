use std::cmp;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

// Uses Dijkstra's algorithm to find the shortest path. I'm sure that there is
// a more elegant way of creating the cave map for the second part, but this
// ugly solution works as well :).
//
// TODO

type Pos = (usize, usize);

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Pos,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(cave: &Vec<Vec<u8>>) -> usize {
    let mut distance = vec![vec![usize::MAX; cave[0].len()]; cave.len()];

    //      [y][x]
    //
    distance[0][0] = 0;

    let width = cave[0].len();
    let height = cave.len();

    let mut heap = BinaryHeap::new();
    heap.push(State {
        cost: 0,
        position: (0, 0),
    });

    while let Some(State {
        cost,
        position: position @ (x, y),
    }) = heap.pop()
    {
        if position == (height - 1, width - 1) {
            return cost;
        }

        if cost > distance[y][x] {
            continue;
        }

        let neighbours = [
            if x > 0 { Some((x - 1, y)) } else { None },
            if x < (width - 1) {
                Some((x + 1, y))
            } else {
                None
            },
            if y > 0 { Some((x, y - 1)) } else { None },
            if y < (height - 1) {
                Some((x, y + 1))
            } else {
                None
            },
        ];

        for (x, y) in neighbours.iter().filter_map(|nb| nb.as_ref()) {
            let next = State {
                cost: cost + cave[*y][*x] as usize,
                position: (*x, *y),
            };

            if next.cost < distance[*y][*x] {
                heap.push(next);
                distance[*y][*x] = next.cost;
            }
        }
    }

    unreachable!();
}

fn main() {
    let lines = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();

    let cave1 = lines
        .into_iter()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|&x| x - b'0')
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    let height = cave1.len();

    let inc = |x: &u8| if (x + 1) > 9 { 1 } else { *x + 1 };

    // 1 2 3 4 5
    // 2 3 4 5 6
    // 3 4 5 6 7
    // 4 5 6 7 8
    // 5 6 7 8 9

    let cave2 = cave1
        .iter()
        .map(|row| row.iter().map(inc).collect())
        .collect::<Vec<Vec<u8>>>();

    let cave3 = cave2
        .iter()
        .map(|row| row.iter().map(inc).collect())
        .collect::<Vec<Vec<u8>>>();

    let cave4 = cave3
        .iter()
        .map(|row| row.iter().map(inc).collect())
        .collect::<Vec<Vec<u8>>>();

    let cave5 = cave4
        .iter()
        .map(|row| row.iter().map(inc).collect())
        .collect::<Vec<Vec<u8>>>();

    let mut cave: Vec<Vec<u8>> = Vec::new();

    for i in 0..cave1.len() {
        cave.push(
            cave1[i]
                .iter()
                .chain(cave2[i].iter())
                .chain(cave3[i].iter())
                .chain(cave4[i].iter())
                .chain(cave5[i].iter())
                .copied()
                .collect::<Vec<u8>>(),
        );
    }

    for _ in 0..4 {
        let mut new_rows = cave[(cave.len() - height)..]
            .iter()
            .cloned()
            .collect::<Vec<Vec<u8>>>();

        for row in new_rows.iter_mut() {
            for x in row.iter_mut() {
                *x = if (*x + 1) > 9 { 1 } else { *x + 1 };
            }
        }

        cave.append(&mut new_rows);
    }

    println!("{}", dijkstra(&cave));
}
