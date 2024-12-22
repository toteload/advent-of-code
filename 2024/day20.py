import fileinput
from grid import Grid, Point
from collections import defaultdict

def flood(grid, s):
    dist = {}
    visited = set()
    q = [(0, s)]

    neighbors = make_neighbors(grid)

    while q:
        (f, u) = q.pop()

        if u in visited:
            continue

        dist[u] = f
        visited.add(u)
        q.extend((f+1, n) for n in neighbors(u))

    return dist

def make_neighbors(grid):
    def neighbors(p):
        (x, y) = p.x,p.y
        if x+1 < grid.width and grid.at_xy(x+1,y) != '#':
            yield Point(x+1,y)
        if x-1 >= 0 and grid.at_xy(x-1,y) != '#':
            yield Point(x-1,y)
        if y+1 < grid.height and grid.at_xy(x,y+1) != '#':
            yield Point(x,y+1)
        if y-1 >= 0 and grid.at_xy(x,y-1) != '#':
            yield Point(x,y-1)
    return neighbors

rows = [list(line.strip()) for line in fileinput.input()]

grid = Grid(rows)

start = grid.find(lambda x: x == 'S')
end = grid.find(lambda x: x == 'E')

grid.setp(start, '.')
grid.setp(end, '.')

dist = flood(grid, end)

baseline = dist[start]

skips = defaultdict(list)

for y in range(1,grid.height-1):
    for x in range(1,grid.width-1):
        if grid.at_xy(x,y) != '#':
            continue

        left  = Point(x-1,y)
        right = Point(x+1,y)

        if grid.at_p(left) == grid.at_p(right) == '.':
            skips[left].append(right)
            skips[right].append(left)

        top    = Point(x,y-1)
        bottom = Point(x,y+1)

        if grid.at_p(top) == grid.at_p(bottom) == '.':
            skips[top].append(bottom)
            skips[bottom].append(top)

cheats = 0
for (s,ts) in skips.items():
    for t in ts:
        score = dist[t] - dist[s] - 2
        if score >= 100:
            cheats += 1

print(cheats)

cheats = defaultdict(int)

for y in range(1,grid.height-1):
    for x in range(1,grid.width-1):
        if grid.at_xy(x,y) != '.':
            continue

        s = Point(x,y)

        for dy in range(-20,21):
            for dx in range(-20,21):
                l = abs(dy) + abs(dx)
                if l > 20:
                    continue

                p = Point(x+dx, y+dy)

                if 0 <= p.y < grid.height and 0 <= p.x < grid.width and grid.at_p(p) == '.':
                    score = dist[p] - dist[s] - l
                    cheats[score] += 1

print(sum(v for k,v in cheats.items() if k >= 100))
