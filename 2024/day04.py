import itertools

def flatten(xs):
    return list(itertools.chain.from_iterable(xs))

def count_xmas_in_line(s):
    c = 0
    for i in range(len(s)):
        t = s[i:i+4]
        if t == 'XMAS':
            c += 1
    return c

def rotl(xs, n):
    assert n <= len(xs)
    return xs[n:] + xs[:n]

def rotr(xs, n):
    assert n <= len(xs)
    return xs[len(xs)-n:] + xs[:len(xs)-n]

def diags(xs):
    n = len(xs[0]) - len(xs) + 1
    ys = list(zip(*[rotr(x, i) for (i,x) in enumerate(xs)]))
    ys = ys[-n:] + flatten([[y[:i+1], y[i+1:]] for (i,y) in enumerate(ys[:-n])])
    ys = [''.join(y) for y in ys if len(y) >= 4]

    zs = list(zip(*[rotl(x, i) for (i,x) in enumerate(xs)]))
    zs = zs[:n] + flatten([[z[:-i-1], z[-i-1:]] for (i,z) in enumerate(zs[n:])])
    zs = [''.join(z) for z in zs if len(z) >= 4]

    return ys + zs

rows = [line.strip() for line in open("demo.txt", "r").readlines()]
rows = [line.strip() for line in open("day04.txt", "r").readlines()]
cols = [''.join(col) for col in zip(*rows)]
diags = diags(rows)

lines = rows + cols + diags
reversed_lines = [s[::-1] for s in lines]
all_lines = lines + reversed_lines
s = sum([count_xmas_in_line(s) for s in all_lines])
print(s)

def part2(rows):
    w = len(rows[0])
    h = len(rows)
    s = 0
    for x in range(1, w-1):
        for y in range(1, h-1):
            c = rows[y][x]
            if c != 'A':
                continue

            if ((rows[y-1][x-1] == 'M' and rows[y+1][x+1] == 'S') or (rows[y-1][x-1] == 'S' and rows[y+1][x+1] == 'M')) and ((rows[y-1][x+1] == 'M' and rows[y+1][x-1] == 'S') or (rows[y-1][x+1] == 'S' and rows[y+1][x-1] == 'M')):
                s += 1

    return s

print(part2(rows))
