type Direction = 'U' | 'D' | 'L' | 'R';
type Step = [ Direction, number ];

function parseStep(s: string): Step {
    const d = s[0];
    const n = parseInt(s.slice(1), 10);
    return [d as Direction, n];
}

function parseLine(line: string): Step[] {
    return line.split(",").map(parseStep);
}

type Point = [number, number];
type Segment = [Point, Point];

function segments(steps: Step[]): Segment[] {
    const res = [];

    let at: Point = [0, 0];
    for (const [d, n] of steps) {
        const next: Point = [...at];
        switch (d) {
        case "U": next[1] += n; break;
        case "D": next[1] -= n; break;
        case "L": next[0] -= n; break;
        case "R": next[0] += n; break;
        }
        
        res.push([at, next] as Segment);
        at = next;
    }

    return res;
}

function orientation(a: Segment): 'horizontal' | 'vertical' {
    if (a[0][0] === a[1][0]) {
        return 'vertical';
    }

    return 'horizontal';
}

function intersection(a: Segment, b: Segment): Point | undefined {
    if (orientation(a) === orientation(b)) {
        return undefined;
    }

    const [h, v] = orientation(a) === 'horizontal' ? [a, b] : [b, a];

    const x = v[0][0];
    const loX = Math.min(h[0][0], h[1][0]);
    const hiX = Math.max(h[0][0], h[1][0]);
    if (x < loX || x > hiX) {
        return undefined;
    }

    const y = h[0][1];
    const loY = Math.min(v[0][1], v[1][1]);
    const hiY = Math.max(v[0][1], v[1][1]);
    if (y < loY || y > hiY) {
        return undefined;
    }

    return [x, y];
}

function manhattan([x, y]: Point): number {
    return x + y;
}

const text = Deno.readTextFileSync('day03.txt');
const steps = text.split('\n').map(parseLine);
const paths = steps.map(segments);

const BigNumber = 99999999999;
let best = BigNumber;

for (const a of paths[0]) {
    for (const b of paths[1]) {
        const p = intersection(a, b);
        if (p === undefined || (p[0] === 0 && p[1] === 0)) {
            continue;
        }

        best = Math.min(best, manhattan(p));
    }
}

console.log(best);

function distanceOfPointAlongPath(p: Point, steps: Step[]): number {
    let s = 0;
    let at: Point = [0, 0];

    for (const [d, n] of steps) {
        const next: Point = [...at];
        switch (d) {
        case "U": next[1] += n; break;
        case "D": next[1] -= n; break;
        case "L": next[0] -= n; break;
        case "R": next[0] += n; break;
        }
 
        if ((d === 'U' || d === 'D') && at[0] === p[0]) {
            const ds = Math.abs(p[1] - at[1]);
            console.log(p, at, next, ds, n, d);
            if (ds <= n) {
                return s + ds;
            }
        }

        if ((d === 'L' || d === 'R') && at[1] === p[1]) {
            const ds = Math.abs(p[0] - at[0]);
            console.log(p, at, next, ds, n, d);
            if (ds <= n) {
                return s + ds;
            }
        }

        s += n;

       at = next;
    }

    throw new Error('oh oh');
}

const intersections = [];
for (const a of paths[0]) {
    for (const b of paths[1]) {
        const p = intersection(a, b);
        if (p === undefined || (p[0] === 0 && p[1] === 0)) {
            continue;
        }

        intersections.push(p);
    }
}

console.log(intersections);

best = BigNumber;

for (const p of intersections) {
    best = Math.min(best, distanceOfPointAlongPath(p, steps[0]) + distanceOfPointAlongPath(p, steps[1]));
}

console.log(best);