import itertools
import collections
from functools import reduce
from heapq import heappop, heappush

def day_input(day):
    filename = f'inputs/input{day:02}.txt'
    return open(filename).read()

def first(iterable, default=None):
    return next(iter(iterable), default)

def chunks(iterable, n, fillvalue=None):
    args = [iter(iterable)] * n
    return itertools.zip_longest(*args, fillvalue=fillvalue)

def windows(iterable, n):
    it = iter(iterable)
    window = collections.deque(itertools.islice(it, n), maxlen=n)
    if len(window) == n:
        yield tuple(window)
    for x in it:
        window.append(x)
        yield tuple(window)

def flatten(xs):
    return list(itertools.chain.from_iterable(xs))

def find_first_index(iterable, pred, default=None):
    res = next(filter(lambda x: pred(x[1]), enumerate(iter(iterable))), default)
    if res:
        return res[0]
    else:
        return None

def take(iterable, n):
    return itertools.islice(iter(iterable), n)

def clamp(x, lo, hi):
    return min(max(x, lo), hi)

def neighbors8(p):
    x, y = p
    return [
        (x-1,y+1), (x,y+1), (x+1,y+1), 
        (x-1,y),            (x+1,y), 
        (x-1,y-1), (x,y-1), (x+1,y-1)]

def astar_search(start, heuristic, neighbors, stepcost=lambda a,b:1):
    frontier = [(heuristic(start), start)]
    prev = {start: None}
    path_cost = {start: 0}

    while frontier:
        (f, s) = heappop(frontier)
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

# itertools recipe
def partition(pred, iterable):
    "Use a predicate to partition entries into false entries and true entries"
    # partition(is_odd, range(10)) --> 0 2 4 6 8   and  1 3 5 7 9
    t1, t2 = itertools.tee(iterable)
    return itertools.filterfalse(pred, t1), filter(pred, t2)

class Range:
    def __init__(self, lo, hi):
        assert lo <= hi
        self.lo = lo
        self.hi = hi

    def split(self, x):
        return [Range(self.lo, x), Range(x, self.hi)]

    def pts(self):
        return [self.lo,self.hi]

    def contains(self, x):
        return self.lo <= x < self.hi

    def size(self):
        return self.hi - self.lo

    def __repr__(self):
        return f'Range[{self.lo}, {self.hi}]'

class InclusiveRange:
    def __init__(self, lo, hi):
        assert lo <= hi
        self.lo = lo
        self.hi = hi

    def has_overlap(self, other):
        return self.lo <= other.hi and other.lo <= self.hi

    def merge(self, other):
        return InclusiveRange(min(self.lo, other.lo), max(self.hi, other.hi))

    def size(self):
        return self.hi - self.lo + 1

    def split(self, x):
        if not (self.lo <= x <= self.hi) or self.hi - self.lo == 1:
            return self

        if self.hi == x:
            return [InclusiveRange(self.lo, x-1), InclusiveRange(x, self.hi)]

        return [InclusiveRange(self.lo, x), InclusiveRange(x+1, self.hi)]

    def pts(self):
        return [self.lo, self.hi]

    def __iter__(self):
        return iter(range(self.lo,self.hi+1))

    def __repr__(self):
        return f'InclusiveRange[{self.lo}, {self.hi}]'

class Area:
    def __init__(self, xrange, yrange):
        self.xrange = xrange
        self.yrange = yrange

    def splitx(self, x):
        xs = self.xrange.split(x)
        return [Area(xr, self.yrange) for xr in xs]

    def splity(self, y):
        ys = self.yrange.split(y)
        return [Area(self.xrange, yr) for yr in ys]

    def contains(self, p):
        x,y = p
        return self.xrange.contains(x) and self.yrange.contains(y)

    def size(self):
        return self.xrange.size() * self.yrange.size()

    def lo(self):
        return (self.xrange.lo, self.yrange.lo)

    def split(self, axis, v):
        if axis == 0: return self.splitx(v)
        if axis == 1: return self.splity(v)

    def corners(self):
        return list(itertools.product(self.xrange.pts(), self.yrange.pts()))

    def subtract(self, other):
        # Other completely covers self
        if all([other.contains(c) for c in self.corners()]):
            return [Area(Range(0,0),Range(0,0))]

        # No overlap
        if not any([self.contains(c) for c in other.corners()]):
            return [self]

        # At least one of the corners of `other` is contained in `self`

        res = []
        acc = self

        if acc.xrange.contains(other.xrange.lo):
            [left,right] = acc.splitx(other.xrange.lo)
            res += [left]
            acc = right

        if acc.xrange.contains(other.xrange.hi):
            [left,right] = acc.splitx(other.xrange.hi)
            res += [right]
            acc = left

        if acc.yrange.contains(other.yrange.lo):
            [up,down] = acc.splity(other.yrange.lo)
            res += [down]
            acc = up

        if acc.yrange.contains(other.yrange.hi):
            [up,down] = acc.splity(other.yrange.hi)
            res += [up]
            acc = down

        return res

    def __repr__(self):
        return f'Area[{self.xrange}, {self.yrange}]'

