from __future__ import annotations
import fileinput
import itertools
import collections
from dataclasses import dataclass
from heapq import heappop, heappush

@dataclass(frozen=True)
class Point:
    x: int
    y: int

    def neighbors4(self):
        x, y = self.x, self.y
        return [Point(x,y+1), Point(x-1,y), Point(x+1,y), Point(x,y-1)]

    def neighbors8(self):
        x, y = self.x, self.y
        return [
            Point(x-1,y+1), Point(x,y+1), Point(x+1,y+1), 
            Point(x-1,y  ),               Point(x+1,y  ), 
            Point(x-1,y-1), Piont(x,y-1), Point(x+1,y-1),
        ]

    def manhattan_distance(self, other: Point) -> int:
        return abs(self.x - other.x) + abs(self.y - other.y)

@dataclass(frozen=True)
class Interval:
    start: int
    end: int

    def overlaps(self, other: Interval) -> bool:
        return other.start < self.end and self.start < other.end

    def contains(self, x: int) -> bool:
        return self.start <= x < self.end

@dataclass(frozen=True)
class Rectangle:
    x: int
    y: int
    width: int
    height: int

    def contains_point(self, p: Point) -> bool:
        return Interval(x, x+width).contains(p.x) and Interval(y, y+height).contains(p.y)

    def corners(self):
        return [
            Point(self.x,                  self.y),
            Point(self.x + self.width - 1, self.y),
            Point(self.x,                  self.y + self.height - 1),
            Point(self.x + self.width - 1, self.y + self.height - 1),
        ]

def readlines():
    return [line.strip() for line in fileinput.input()]

def floodfill(start, neighbors):
    dist = {}
    q = [(0, start)]

    while q:
        (f, u) = q.pop()

        if u in dist:
            continue

        dist[u] = f
        q.extend((f + 1, n) for n in neighbors(u))

    return dist

def flatten(xs):
    return list(itertools.chain.from_iterable(xs))

def clamp(x, lo, hi):
    return min(max(x, lo), hi)

def windows(iterable, n):
    # I yanked this code from my 2022 helpers.py.
    # I don't understand it and I don't know where I got it from :P
    it = iter(iterable)
    window = collections.deque(itertools.islice(it, n), maxlen=n)
    if len(window) == n:
        yield tuple(window)
    for x in it:
        window.append(x)
        yield tuple(window) 

# - `start` is the starting position. This can be any type.
# - `heuristic` is a function that takes a position and returns a number.
# - `neighbors` is a function that takes a position and returns the neighbors of that position.
# - `stepcost` is a function that takes two positions and returns the cost of moving from the first
#   position to the second position.

# Returns the shortest path.
def astar(start, heuristic, neighbors, stepcost=lambda a,b:1):
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
