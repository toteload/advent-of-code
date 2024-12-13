use std::ops;
use std::cmp;

struct Target {
    x: ops::RangeInclusive<isize>,
    y: ops::RangeInclusive<isize>,
}

impl Target {
    fn contains(&self, x: isize, y: isize) -> bool {
        self.x.contains(&x) && self.y.contains(&y)
    }
}

#[derive(Debug)]
struct Pos {
    x: isize,
    y: isize,
}

fn shoot(target: &Target, mut vx: isize, mut vy: isize) -> Option<Vec<Pos>> {
    let mut pts = Vec::new();

    let mut x = 0;
    let mut y = 0;

    loop {
        if x > *target.x.end()
            || (vx == 0 && !target.x.contains(&x))
            || (vy < 0 && y < *target.y.start())
        {
            return None;
        }

        pts.push(Pos { x, y });

        if target.contains(x, y) {
            return Some(pts);
        }

        x += vx;
        y += vy;

        vx += vx.signum() * -1;
        vy -= 1;
    }
}

fn main() {
    let target = Target { x: 288..=330, y: -96..=-50 };

    let max_vx = target.x.end() + 1;

    let mut max_height = 0;

    // Just try up to a maximum y velocity of 1000.
    //for vy in 0..1000 {
    //    for vx in 0..=max_vx {
    //        if let Some(pts) = shoot(&target, vx, vy) {
    //            max_height = pts.iter().map(|p| p.y).fold(max_height, |acc, y| cmp::max(acc, y));
    //        }
    //    }
    //}
    //
    //println!("{}", max_height);

    let mut count = 0;

    for vy in -100..1000 {
        for vx in 0..=max_vx {
            if let Some(_) = shoot(&target, vx, vy) {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
