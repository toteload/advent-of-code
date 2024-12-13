use std::collections::HashSet;
use std::io::{self, BufRead};
use std::ops::Range;

enum SwitchState {
    On,
    Off,
}

struct Command {
    turn: SwitchState,
    cuboid: Cuboid,
}

#[derive(Clone)]
struct Cuboid {
    xs: Range<i32>,
    ys: Range<i32>,
    zs: Range<i32>,
}

fn has_overlap(a: &Range<i32>, b: &Range<i32>) -> bool {
    !(a.end <= b.start || b.end <= a.start)
}

struct SplitResult {
    overlap: Option<Range<i32>>,
    non_overlap: [Option<Range<i32>>; 2],
}

fn split(a: &Range<i32>, b: &Range<i32>) -> SplitResult {
    if !has_overlap(a, b) {
        return SplitResult {
            overlap: None,
            non_overlap: [Some(a.clone()), None],
        };
    }

    match (a.contains(&b.start), a.contains(&b.end)) {
        (true, true) => SplitResult {
            overlap: Some(b.clone()),
            non_overlap: [Some(a.start..b.start), Some(b.end..a.end),],
        },
        (false, true) => SplitResult {
            overlap: Some(a.start..b.end),
            non_overlap: [Some(b.end..a.end), None],
        },
        (true, false) => SplitResult {
            overlap: Some(b.start..a.end),
            non_overlap: [Some(a.start..b.start), None],
        },
        (false, false) => SplitResult {
            overlap: Some(a.clone()),
            non_overlap: [None, None],
        },
    }
}

impl Cuboid {
    fn has_intersection(&self, other: &Cuboid) -> bool {
        has_overlap(&self.xs, &other.xs)
            && has_overlap(&self.ys, &other.ys)
            && has_overlap(&self.zs, &other.zs)
    }

    fn subtract(&self, other: &Cuboid) -> Vec<Cuboid> {
        if !self.has_intersection(other) {
            return vec![self.clone()];
        }

        let mut res = Vec::new();

        let splx = split(&self.xs, &other.xs);

        for x in &splx.non_overlap {
            if let Some(r) = x {
                res.push(Cuboid {
                    xs: r.clone(),
                    ys: self.ys.clone(),
                    zs: self.zs.clone(),
                });
            }
        }

        if let Some(xr) = splx.overlap {
            let sply = split(&self.ys, &other.ys);

            for y in &sply.non_overlap {
                if let Some(r) = y {
                    res.push(Cuboid {
                        xs: xr.clone(),
                        ys: r.clone(),
                        zs: self.zs.clone(),
                    });
                }
            }

            if let Some(yr) = sply.overlap {
                let splz = split(&self.zs, &other.zs);

                for z in &splz.non_overlap {
                    if let Some(r) = z {
                        res.push(Cuboid {
                            xs: xr.clone(),
                            ys: yr.clone(),
                            zs: r.clone(),
                        });
                    }
                }
            }
        }

        res
    }

    fn volume(&self) -> u64 {
        [&self.xs, &self.ys, &self.zs]
            .iter()
            .map(|r| (r.end - r.start).abs() as u64)
            .product()
    }
}

impl Command {
    fn parse_from_line(line: &str) -> Command {
        let mut ps = line.split(' ');

        let turn = match ps.next().unwrap() {
            "on" => SwitchState::On,
            "off" => SwitchState::Off,
            _ => unreachable!(),
        };

        let mut rs = ps.next().unwrap().split(',');

        let mut xs = std::str::from_utf8(&rs.next().unwrap().as_bytes()[2..])
            .unwrap()
            .split("..");

        let x0 = xs.next().unwrap().parse::<i32>().unwrap();
        let x1 = xs.next().unwrap().parse::<i32>().unwrap();

        let mut ys = std::str::from_utf8(&rs.next().unwrap().as_bytes()[2..])
            .unwrap()
            .split("..");

        let y0 = ys.next().unwrap().parse::<i32>().unwrap();
        let y1 = ys.next().unwrap().parse::<i32>().unwrap();

        let mut zs = std::str::from_utf8(&rs.next().unwrap().as_bytes()[2..])
            .unwrap()
            .split("..");

        let z0 = zs.next().unwrap().parse::<i32>().unwrap();
        let z1 = zs.next().unwrap().parse::<i32>().unwrap();

        Command {
            turn,
            cuboid: Cuboid {
                xs: x0..(x1 + 1),
                ys: y0..(y1 + 1),
                zs: z0..(z1 + 1),
            },
        }
    }
}

fn part_one(commands: &[Command]) -> usize {
    let mut cubes = HashSet::new();

    for cmd in commands {
        for x in cmd.cuboid.xs.clone() {
            for y in cmd.cuboid.ys.clone() {
                for z in cmd.cuboid.zs.clone() {
                    match cmd.turn {
                        SwitchState::On => cubes.insert((x, y, z)),
                        SwitchState::Off => cubes.remove(&(x, y, z)),
                    };
                }
            }
        }
    }

    cubes.len()
}

fn part_two(commands: &[Command]) -> u64 {
    let mut cuboids: Vec<Cuboid> = Vec::new();

    for cmd in commands {
        cuboids = cuboids
            .into_iter()
            .flat_map(|c| c.subtract(&cmd.cuboid))
            .collect();

        match cmd.turn {
            SwitchState::On => cuboids.push(cmd.cuboid.clone()),
            _ => {}
        }
    }

    cuboids.iter().map(|c| c.volume()).sum()
}

fn main() {
    // I don't do any filtering on the input lines. For part one I only feed
    // the lines that are within range. Which in my case were the first 10
    // lines of "input.txt".
    let commands = io::stdin()
        .lock()
        .lines()
        .map(|x| Command::parse_from_line(x.unwrap().as_str()))
        .collect::<Vec<_>>();

    let timer = std::time::Instant::now();
    let answer = part_two(&commands);
    let elapsed_time = timer.elapsed();
    println!("time: {}ms, answer: {}", elapsed_time.as_millis(), answer);
}
