import itertools
from collections import defaultdict

grid = [line.strip() for line in open("day12.txt").readlines()]

def neighbors4(p):
    (x, y) = p
    return [(x+1,y), (x-1,y), (x,y+1), (x,y-1)]

def floodfill(grid, start, pred):
    w = len(grid[0])
    h = len(grid)

    frontier = [start]
    visited = set()

    while len(frontier) > 0:
        p = frontier.pop()

        (px,py) = p

        if not pred(grid[py][px]) or p in visited:
            continue

        visited.add(p)

        for k in neighbors4(p):
            (x,y) = k

            if x < 0 or x >= w or y < 0 or y >= h:
                continue

            frontier.append(k)

    return visited

def find_regions(grid):
    regions = []

    w = len(grid[0])
    h = len(grid)

    unvisited = set(itertools.product(range(w), range(h)))

    while len(unvisited) > 0:
        (x,y) = unvisited.pop()
        region = floodfill(grid, (x,y), lambda v: v == grid[y][x])
        unvisited -= region
        regions.append(region)

    return regions

def price(r):
    area = len(r)
    perimeter = sum([4 - len(set(neighbors4(p)) & r) for p in r])
    return area * perimeter

def price_sides(r):
    area = len(r)

    top    = defaultdict(list)
    bottom = defaultdict(list)
    left   = defaultdict(list)
    right  = defaultdict(list)

    for (x,y) in r:
        north = (x,y-1)
        east  = (x+1,y)
        south = (x,y+1)
        west  = (x-1,y)

        if north not in r:
            top[y].append(x)

        if south not in r:
            bottom[y].append(x)

        if east not in r:
            right[x].append(y)

        if west not in r:
            left[x].append(y)

    s = 0

    for side in [top, bottom, left, right]:
        for sides in side.values():
            sides.sort()

            if len(sides) == 1:
                s += 1
            else:
                s += 1 + len([1 for (a,b) in itertools.pairwise(sides) if b-a != 1])

    return area * s

print(sum([price(r) for r in find_regions(grid)]))
print(sum([price_sides(r) for r in find_regions(grid)]))
