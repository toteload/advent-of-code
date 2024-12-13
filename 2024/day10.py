import itertools

grid = [[int(v) for v in line.strip()] for line in open("demo10.txt").readlines()]
grid = [[int(v) for v in line.strip()] for line in open("day10.txt").readlines()]

def find_all_in_grid2d(grid, pred):
    w = len(grid[0])
    h = len(grid)

    hits = []

    for (y, x) in itertools.product(range(h), range(w)):
        if pred(grid[y][x]):
            hits.append((x, y))

    return hits

def neighbors4(p):
    (x, y) = p
    return [(x+1,y), (x-1,y), (x,y+1), (x,y-1)]

def count_peaks_reachable_from(grid, start):
    w = len(grid[0])
    h = len(grid)

    frontier = [start]

    peaks = set()

    while len(frontier) > 0:
        p = frontier.pop()

        (px,py) = p

        if grid[py][px] == 9:
            peaks.add(p)
            continue

        for k in neighbors4(p):
            (x,y) = k

            if x < 0 or x >= w or y < 0 or y >= h:
                continue

            if grid[y][x] - grid[py][px] != 1:
                continue

            frontier.append(k)

    return len(peaks)

def rating(grid, start):
    w = len(grid[0])
    h = len(grid)

    frontier = [start]

    s = 0

    while len(frontier) > 0:
        p = frontier.pop()

        (px,py) = p

        if grid[py][px] == 9:
            s += 1
            continue

        for k in neighbors4(p):
            (x,y) = k

            if x < 0 or x >= w or y < 0 or y >= h:
                continue

            if grid[y][x] - grid[py][px] != 1:
                continue

            frontier.append(k)

    return s


trailheads = find_all_in_grid2d(grid, lambda v: v == 0)

print(sum([count_peaks_reachable_from(grid, p) for p in trailheads]))
print(sum([rating(grid, p) for p in trailheads]))
