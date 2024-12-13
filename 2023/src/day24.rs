fn parse_line(line: &str) -> ([f64; 3], [f64; 3]) {
    let mut parts = line.split(" @ ");

    let mut ps = parts
        .next()
        .unwrap()
        .split(", ")
        .map(|p| p.trim().parse::<i64>().unwrap() as f64);

    let pos = [ps.next().unwrap(), ps.next().unwrap(), ps.next().unwrap()];

    let mut vs = parts
        .next()
        .unwrap()
        .split(", ")
        .map(|v| v.trim().parse::<i64>().unwrap() as f64);

    let vel = [vs.next().unwrap(), vs.next().unwrap(), vs.next().unwrap()];

    (pos, vel)
}

pub fn part1(input: &str) -> u32 {
    let stones = input.lines().map(parse_line).collect::<Vec<_>>();

    let lo = 200000000000000.0;
    let hi = 400000000000000.0;

    let mut answer = 0;

    for (i, a) in stones[..(stones.len() - 1)].iter().enumerate() {
        for b in &stones[(i + 1)..] {
            let ([ax, ay, _], [avx, avy, _]) = a;
            let ([bx, by, _], [bvx, bvy, _]) = b;

            let s = avx / avy;
            let q = (bx - ax - (s * (by - ay))) / (s * bvy - bvx);

            let s = bvx / bvy;
            let r = (ax - bx - (s * (ay - by))) / (s * avy - avx);

            // q is negative if the intersection is in the past for stone b.
            // if one of them is infinite, then both are infinite (I think maybe).
            if q.is_infinite() || q < 0.0 || r < 0.0 {
                continue;
            }

            let x = bx + bvx * q;
            let y = by + bvy * q;

            if x < lo || x > hi || y < lo || y > hi {
                continue;
            }

            answer += 1;
        }
    }

    answer
}

fn parse_line_part2(line: &str) -> (Vector3, Vector3) {
    let mut parts = line.split(" @ ");

    let mut ps = parts
        .next()
        .unwrap()
        .split(", ")
        .map(|p| p.trim().parse::<i128>().unwrap());

    let pos = [ps.next().unwrap(), ps.next().unwrap(), ps.next().unwrap()];

    let mut vs = parts
        .next()
        .unwrap()
        .split(", ")
        .map(|v| v.trim().parse::<i128>().unwrap());

    let vel = [vs.next().unwrap(), vs.next().unwrap(), vs.next().unwrap()];

    (pos, vel)
}

type Vector3 = [i128; 3];

fn dot(a: Vector3, b: Vector3) -> i128 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: Vector3, b: Vector3) -> Vector3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn sub(a: Vector3, b: Vector3) -> Vector3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn add(a: Vector3, b: Vector3) -> Vector3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn div(a: Vector3, b: Vector3) -> Vector3 {
    [a[0] / b[0], a[1] / b[1], a[2] / b[2]]
}

fn scale(s: i128, b: Vector3) -> Vector3 {
    [s * b[0], s * b[1], s * b[2]]
}

pub fn part2(input: &str) -> i128 {
    let mut stones = input.lines().map(parse_line_part2).collect::<Vec<_>>();
    let (_, v_base) = stones[0];

    for (_, v) in stones.iter_mut() {
        *v = sub(*v, v_base);
    }

    let (p0, d) = stones[1];
    let (p1, _) = stones[0];
    let p2 = add(p0, d);

    let p01 = sub(p1, p0);
    let p02 = sub(p2, p0);

    let n = cross(p01, p02);

    let mut intersection_times = Vec::new();

    for (b, h) in &stones[2..] {
        let t = dot(sub(p1, *b), n) / dot(*h, n);
        intersection_times.push(t);
    }

    // OPTIMIZE Pretty sure you only have to use 3 lines in total

    let t0 = intersection_times[0];
    let t1 = intersection_times[1];
    let (b0, v0) = stones[2];
    let (b1, v1) = stones[3];
    let p0 = add(b0, scale(t0, add(v0, v_base)));
    let p1 = add(b1, scale(t1, add(v1, v_base)));

    let v = div(sub(p1, p0), [t1 - t0; 3]);
    let p = sub(p0, scale(t0, v));

    p[0] + p[1] + p[2]
}
