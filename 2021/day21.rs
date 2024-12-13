use std::cmp;

fn part_one(p1: usize, p2: usize) -> usize {
    let mut scores = [0; 2];
    let mut positions = [p1, p2];

    let mut p = 0;

    let mut roll_count = 0;

    loop {
        let roll_sum = {
            let x = (roll_count + 1) % 100;

            let a = x;
            let b = ((x + 1) - 1) % 100 + 1;
            let c = ((x + 2) - 1) % 100 + 1;

            a + b + c
        };

        roll_count += 3;

        positions[p] = ((positions[p] + (roll_sum % 10)) - 1) % 10 + 1;
        scores[p] += positions[p];

        if scores[p] >= 1000 {
            break;
        }

        p = (p + 1) % 2;
    }

    scores[(p + 1) % 2] * roll_count
}

#[derive(Clone, Copy)]
struct State {
    current: u8,
    positions: [u8; 2],
    scores: [u8; 2],
}

impl State {
    fn next(&self, step_size: u8) -> State {
        let mut res = self.clone();

        let p = self.current as usize;

        res.positions[p] = ((res.positions[p] + (step_size % 10)) - 1) % 10 + 1;
        res.scores[p] += res.positions[p];
        res.current = (res.current + 1) % 2;

        res
    }
}

// I also tried out a memoized version of this, but it wasn't noticably faster, if at all.
fn compute(state: &State) -> (u64, u64) {
    if state.scores[0] >= 21 {
        return (1, 0);
    }

    if state.scores[1] >= 21 {
        return (0, 1);
    }

    // Pairs of (step_size, frequency)
    let rolls: [(u8, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

    let (wins1, wins2): (Vec<_>, Vec<_>) = rolls
        .iter()
        .map(|(step, n)| {
            let (wins1, wins2) = compute(&state.next(*step));
            (n * wins1, n * wins2)
        })
        .unzip();

    (wins1.iter().sum(), wins2.iter().sum())
}

fn part_two(p1: usize, p2: usize) -> u64 {
    let (wins1, wins2) = compute(&State {
        current: 0,
        positions: [p1 as u8, p2 as u8],
        scores: [0, 0],
    });
    cmp::max(wins1, wins2)
}

fn main() {
    let p1 = 1;
    let p2 = 3;

    println!("{}", part_one(p1, p2));
    println!("{}", part_two(p1, p2));
}
