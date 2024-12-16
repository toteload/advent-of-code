import fileinput
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
    res = []
    np = r.p + r.facing
    if grid.at_p(np) != '#':
        res.append(Reindeer(np, r.facing))
    res.append(Reindeer(r.p, directions[directions.index(r.facing)-3]))
    res.append(Reindeer(r.p, directions[directions.index(r.facing)-1]))
    return res

def cost(s, t):
    if s.facing.x != t.facing.x:
        return 1000
    return 1

path = astar_search(Reindeer(start, Point(1,0)), lambda x: manhattan(astuple(x.p), astuple(end)), neighbors, cost)

print(sum(cost(a,b) for a,b in pairwise(path)))
