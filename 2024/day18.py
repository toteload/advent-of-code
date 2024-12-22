import fileinput
from grid import Point, Grid
from heapq import heappop, heappush

bs = [Point(*(int(p) for p in line.strip().split(','))) for line in fileinput.input()]

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

def manhattanp(a,b):
    xa,ya = a.x,a.y
    xb,yb = b.x,b.y
    return abs(xa-xb)+abs(ya-yb)

def make_heuristic(end):
    def heuristic(p):
        return manhattanp(p, end)
    return heuristic

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

def part1(w, n, bs):
    rows = [['.'] * w for _ in range(w)]
    grid = Grid(rows)

    for p in bs[:n]:
        grid.setp(p, '#')

    path = astar_search(Point(0,0), make_heuristic(Point(w-1,w-1)), make_neighbors(grid))
    
    return len(path)-1

def part2(w, bs):
    def search(lo, hi):
        if hi - lo == 1:
            return bs[lo]

        rows = [['.'] * w for _ in range(w)]
        grid = Grid(rows)

        mid = lo + (hi - lo) // 2

        for p in bs[:mid]:
            grid.setp(p, '#')

        path = astar_search(Point(0,0), make_heuristic(Point(w-1,w-1)), make_neighbors(grid))

        if not path:
            return search(lo, mid)
        else:
            return search(mid, hi)

    return search(0, len(bs))

w = 71

print(part1(w, 1024, bs))
print(part2(w, bs))
