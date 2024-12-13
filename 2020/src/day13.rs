use nom::{
    branch::alt,
    character::complete::{char, i32, line_ending},
    multi::separated_list0,
    sequence::separated_pair,
    IResult, Parser,
};

use crate::Problem;

pub struct Instance {
    timestamp: i32,
    bus: Vec<Option<i32>>,
}

fn parse_bus(s: &str) -> IResult<&str, Option<i32>> {
    alt((char('x').map(|_| None), i32.map(Some)))(s)
}

fn parse_bus_list(s: &str) -> IResult<&str, Vec<Option<i32>>> {
    separated_list0(char(','), parse_bus)(s)
}

fn parse(s: &str) -> (i32, Vec<Option<i32>>) {
    separated_pair(i32, line_ending, parse_bus_list)(s)
        .unwrap()
        .1
}

fn div_ceil(a: i32, b: i32) -> i32 {
    (a + b - 1) / b
}

// Returns Bezout's coefficients and the Greatest Common Divisor.
fn extended_gcd(a: &i64, b: &i64) -> (i64, i64, i64) {
    let (mut old_r, mut r) = (*a, *b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
        (old_t, t) = (t, old_t - q * t);
    }

    (old_s, old_t, old_r)
}

fn chinese_remainder_theorem(cs: &[(i64,i64)]) -> (i64, i64) {
    // Direct construction algorithm from Wikipedia
    let p: i64 = cs.iter().map(|x| x.1).product();

    let x = cs.iter().map(|(a, n)| {
        let i = p / n; // N / n_i
        a * extended_gcd(&i,n).0 * i
    }).sum::<i64>();

    (x.rem_euclid(p), p)

    /*
    let (mut a, mut n) = (cs[0].0 as i128, cs[0].1 as i128);
    for (ap, np) in &cs[1..] {
        let (m, mp, _) = extended_gcd(&(n as i64), &np);
        let (m, mp) = (m as i128, mp as i128);
        let per = n*(*np as i128);
        // b0 = ap * m * n
        let b0 = ((*ap as i128)*m*n).rem_euclid(per);
        // b1 = a * mp * np;
        let b1 = ((a*(mp.rem_euclid(per))).rem_euclid(per)*(*np as i128)).rem_euclid(per);
        a = (b0 + b1).rem_euclid(per);
        n = per;
    }
    (a as i64, n as i64)
    */
}

#[cfg(test)]
mod tests {
    use super::{extended_gcd, chinese_remainder_theorem};

    #[test]
    fn test_extended_gcd() {
        let a = 240;
        let b = 46;
        let (c0, c1, gcd) = extended_gcd(&a, &b);
        assert_eq!(gcd, 2);
        assert_eq!(c0 * a + c1 * b, gcd);
    }

    #[test]
    fn test_chinese_remainder_theorem() {
        let cs = [(0,3), (3,4), (4,5)];
        let (a, n) = chinese_remainder_theorem(&cs);

        assert_eq!(n, 60);
        assert_eq!(a, 39);
    }
}

impl<'a> Problem<'a> for Instance {
    fn new(input: &'a str) -> Instance {
        let (timestamp, bus) = parse(input);
        Instance { timestamp, bus }
    }

    fn solve_part_one(&self) -> usize {
        let (id, departure) = self
            .bus
            .iter()
            .filter_map(|&x| x)
            .map(|x| (x, x * div_ceil(self.timestamp, x)))
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap();

        (id * (departure - self.timestamp)) as usize
    }

    fn solve_part_two(&self) -> usize {
        // Make use of Chinese Remainder Theorem with Bezout's identity.
        
        // Vec<(offset, mod)>
        let busses = self
            .bus
            .iter()
            .enumerate()
            .filter_map(|(i, &x)| x.map(|x| (-(i as i64), x as i64)))
            .collect::<Vec<(i64, i64)>>();

        let (offset, period) = chinese_remainder_theorem(&busses);

        println!("{}, {}", offset, period);

        assert_eq!(offset, 780601154795940);

        offset as usize
    }
}
