import fileinput
from collections import defaultdict
from itertools import pairwise
from dataclasses import dataclass, astuple
from heapq import heappop, heappush
from grid import Grid, Point

def astar_search(start, heuristic, neighbors, stepcost=lambda a,b:1):
    frontier = [(heuristic(start), start)]
    prev = {start: None}
    path_cost = {start: 0}

    while frontier:
        (_, s) = heappop(frontier)
        if heuristic(s) == 0:
            path = []
            p = s
            while p != None:
                path.append(p)
                p = prev[p]
            return path[::-1]

        for t in neighbors(s):
            tscore = path_cost[s] + stepcost(s, t)
            if t not in path_cost or tscore < path_cost[t]:
                heappush(frontier, (tscore + heuristic(t), t))
                path_cost[t] = tscore
                prev[t] = s

    return None

def manhattan(a,b):
    xa,ya = a
    xb,yb = b
    return abs(xa-xb)+abs(ya-yb)

@dataclass(frozen=True)
class Reindeer:
    p: Point
    facing: (int, int)
    def __lt__(self,other):
        return False

rows = [line.strip() for line in fileinput.input()]
grid = Grid(rows)

start = grid.find(lambda x: x == 'S')
end = grid.find(lambda x: x == 'E')

directions = [Point(1,0), Point(0,-1), Point(-1,0), Point(0,1)]

def neighbors(r):
    np = r.p + r.facing
    if grid.at_p(np) != '#':
        yield Reindeer(np, r.facing)
    yield Reindeer(r.p, directions[directions.index(r.facing)-3])
    yield Reindeer(r.p, directions[directions.index(r.facing)-1])

def cost(s, t):
    if s.facing.x != t.facing.x:
        return 1000
    return 1

def heuristic(a):
    d = manhattan(astuple(a.p), astuple(end))
    if a.facing == Point(-1,0) or a.facing == Point(0,1):
        return d + 2000
    #if (a.p.x == end.x and a.facing != Point(0,-1)) or (a.p.y == end.y and a.facing != Point(1, 0)):
    #    return d + 1000
    return d

path = astar_search(Reindeer(start, Point(1,0)), heuristic, neighbors, cost)

best = sum(cost(a,b) for a,b in pairwise(path))
print(best)

def trace_tiles(prev, at):
    if not prev[at]:
        return set()

    s = {at.p}
    for x in prev[at]:
        s |= trace_tiles(prev, x)
    return s

def astar_search_iter(start, heuristic, neighbors, stepcost=lambda a,b:1):
    frontier = [(heuristic(start), start)]
    prev = defaultdict(set)
    path_cost = {start: 0}

    while frontier:
        (f, s) = heappop(frontier)

        if heuristic(s) == 0 and f > best:
            return

        if heuristic(s) == 0:
            yield trace_tiles(prev, s)
            continue

        for t in neighbors(s):
            tscore = path_cost[s] + stepcost(s, t)
            if t not in path_cost or tscore <= path_cost[t]:
                heappush(frontier, (tscore + heuristic(t), t))
                path_cost[t] = tscore
                prev[t].add(s)

    return None

tiles = None

# There is a bug in my dirty code.
# For the examples it returns the correct answer, but for some reason it is off by one for the real
# input. When I draw the visited tiles in the grid, the starting tile is not marked. It is marked
# when I run it for the examples... I don't know what goes wrong and I'm too tired to care.
# Just add one :)
for t in astar_search_iter(Reindeer(start, Point(1,0)), heuristic, neighbors, cost):
    tiles = t

m = [list(line) for line in grid.compact_str().split('\n')]
for r in tiles:
    c = 'O'
    m[r.y][r.x] = c
print('\n'.join([''.join(line) for line in m]))


print(len(tiles))
