use std::cmp::{max, min};
use std::io::{self, BufRead};

struct Point {
    x: usize,
    y: usize,
}

fn parse_point(s: &str) -> Point {
    let mut n = s.split(',').map(|x| x.trim().parse::<usize>().unwrap());

    Point {
        x: n.next().unwrap(),
        y: n.next().unwrap(),
    }
}

fn parse_line(line: &str) -> (Point, Point) {
    let mut points = line.split("->").map(|p| parse_point(p));

    (points.next().unwrap(), points.next().unwrap())
}

fn main() {
    let lines = io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .map(|line| parse_line(&line))
        .collect::<Vec<_>>();

    let maxx = lines
        .iter()
        .map(|l| max(l.0.x, l.1.x))
        .fold(0, |acc, x| max(acc, x));

    let maxy = lines
        .iter()
        .map(|l| max(l.0.y, l.1.y))
        .fold(0, |acc, y| max(acc, y));

    let mut bitmap: Vec<usize> = vec![0; (maxx + 1) * (maxy + 1)];

    let stride = maxx;

    for line in lines {
        if line.0.x == line.1.x {
            // Horizontal lines

            let start = min(line.0.y, line.1.y);
            let end = max(line.0.y, line.1.y);

            for i in start..=end {
                bitmap[line.0.x + i * stride] += 1;
            }
        } else if line.0.y == line.1.y {
            // Vertical lines

            let start = min(line.0.x, line.1.x);
            let end = max(line.0.x, line.1.x);

            for i in start..=end {
                bitmap[i + line.0.y * stride] += 1;
            }
        } else {
            //Diagonal lines

            let dx: isize = if line.0.x > line.1.x { -1 } else { 1 };
            let dy: isize = if line.0.y > line.1.y { -1 } else { 1 };

            let count = max(line.0.x, line.1.x) - min(line.0.x, line.1.x);

            for i in 0..=count {
                let x = (line.0.x as isize + (i as isize * dx)) as usize;
                let y = (line.0.y as isize + (i as isize * dy)) as usize;

                bitmap[x + y * stride] += 1;
            }
        }
    }

    let answer = bitmap.iter().filter(|&&x| x >= 2).count();

    println!("{}", answer);
}
