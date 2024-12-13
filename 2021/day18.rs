use std::cmp;
use std::io::{self, BufRead};
use std::ops::Add;

#[derive(Debug, Clone)]
enum Pair {
    Int(u64),
    Pair(Box<Pair>, Box<Pair>),
}

struct ExplodeProbe {
    candidate: *mut Pair,
    left: Option<*mut Pair>,
    right: Option<*mut Pair>,
}

impl Pair {
    fn from_line(line: &str) -> Self {
        Pair::parse_pair(line.trim()).0
    }

    fn parse_atom(s: &str) -> (Self, usize) {
        if let Some(x) = (s.as_bytes()[0] as char).to_digit(10) {
            (Pair::Int(x.into()), 1)
        } else {
            Pair::parse_pair(s)
        }
    }

    fn parse_pair(s: &str) -> (Self, usize) {
        let (lhs, lhs_len) = Pair::parse_atom(&s[1..]);
        let (rhs, rhs_len) = Pair::parse_atom(&s[(1 + lhs_len + 1)..]);

        (
            Pair::Pair(Box::new(lhs), Box::new(rhs)),
            1 + lhs_len + 1 + rhs_len + 1,
        )
    }

    fn split(&mut self) {
        *self = match self {
            Pair::Int(x) => {
                let lhs = *x / 2;
                let rhs = *x - lhs;
                Pair::Pair(Box::new(Pair::Int(lhs)), Box::new(Pair::Int(rhs)))
            }
            _ => unreachable!(),
        }
    }

    fn find_split_candidate(&mut self) -> Option<&mut Self> {
        match self {
            Pair::Int(x) => {
                if *x >= 10 {
                    Some(self)
                } else {
                    None
                }
            }
            Pair::Pair(lhs, rhs) => lhs
                .find_split_candidate()
                .or_else(move || rhs.find_split_candidate()),
        }
    }

    fn explode(&mut self, lhs: Option<*mut Pair>, rhs: Option<*mut Pair>) {
        if let Pair::Pair(self_lhs, self_rhs) = self {
            if let Pair::Int(bomb_lhs) = **self_lhs {
                if let Some(lhs) = lhs {
                    unsafe {
                        match &mut *lhs {
                            Pair::Int(x) => *x += bomb_lhs,
                            _ => unreachable!(),
                        }
                    }
                }
            } else {
                unreachable!();
            }

            if let Pair::Int(bomb_rhs) = **self_rhs {
                if let Some(rhs) = rhs {
                    unsafe {
                        match &mut *rhs {
                            Pair::Int(x) => *x += bomb_rhs,
                            _ => unreachable!(),
                        }
                    }
                }
            } else {
                unreachable!();
            }
        } else {
            unreachable!();
        }

        *self = Pair::Int(0);
    }

    fn find_explode_candidate(&mut self, depth: usize) -> Option<ExplodeProbe> {
        if depth < 4 {
            if let Pair::Pair(lhs, rhs) = self {
                if let Some(probe) = lhs.find_explode_candidate(depth + 1) {
                    if probe.right.is_none() {
                        return Some(ExplodeProbe {
                            candidate: probe.candidate,
                            left: probe.left,
                            right: rhs.find_leftmost_number(),
                        });
                    } else {
                        return Some(probe);
                    }
                } else {
                    if let Some(probe) = rhs.find_explode_candidate(depth + 1) {
                        if probe.left.is_none() {
                            return Some(ExplodeProbe {
                                candidate: probe.candidate,
                                left: lhs.find_rightmost_number(),
                                right: probe.right,
                            });
                        } else {
                            return Some(probe);
                        }
                    }
                }
            }

            return None;
        } else {
            match self {
                Pair::Pair(..) => Some(ExplodeProbe {
                    candidate: self,
                    left: None,
                    right: None,
                }),
                _ => None,
            }
        }
    }

    fn find_rightmost_number(&mut self) -> Option<*mut Self> {
        match self {
            Pair::Int(_) => Some(&mut *self),
            Pair::Pair(lhs, rhs) => rhs
                .find_rightmost_number()
                .or_else(move || lhs.find_rightmost_number()),
        }
    }

    fn find_leftmost_number(&mut self) -> Option<*mut Self> {
        match self {
            Pair::Int(_) => Some(&mut *self),
            Pair::Pair(lhs, rhs) => lhs
                .find_leftmost_number()
                .or_else(move || rhs.find_leftmost_number()),
        }
    }

    fn reduce(&mut self) {
        loop {
            if let Some(ExplodeProbe {
                candidate,
                left,
                right,
            }) = self.find_explode_candidate(0)
            {
                unsafe {
                    (&mut *candidate).explode(left, right);
                }
                continue;
            }

            if let Some(x) = self.find_split_candidate() {
                x.split();
                continue;
            }

            break;
        }
    }

    fn print(&self) {
        self.print_helper();
        println!("");
    }

    fn print_helper(&self) {
        match self {
            Pair::Int(x) => print!("{}", x),
            Pair::Pair(lhs, rhs) => {
                print!("[");
                lhs.print_helper();
                print!(",");
                rhs.print_helper();
                print!("]");
            }
        }
    }

    fn magnitude(&self) -> u64 {
        match self {
            Pair::Int(x) => *x,
            Pair::Pair(lhs, rhs) => 3 * lhs.magnitude() + 2 * rhs.magnitude(),
        }
    }
}

impl Add for Pair {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut res = Pair::Pair(Box::new(self), Box::new(other));
        res.reduce();
        res
    }
}

fn main() {
    let pairs = io::stdin()
        .lock()
        .lines()
        .map(|line| Pair::from_line(&line.unwrap()))
        .collect::<Vec<_>>();

    // Part one
    println!(
        "{}",
        pairs
            .iter()
            .cloned()
            .reduce(|acc, x| acc + x)
            .unwrap()
            .magnitude()
    );

    // Part two
    let mut largest = 0;
    for i in 0..(pairs.len()) {
        for j in 0..(pairs.len()) {
            if i == j {
                continue;
            }

            largest = cmp::max(largest, (pairs[i].clone() + pairs[j].clone()).magnitude());
        }
    }

    println!("{}", largest);
}
