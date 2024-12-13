from helpers import *
from itertools import repeat
import copy

txt = day_input(8)

trees = [[int(x) for x in line] for line in txt.split('\n') if line != '']

w = len(trees[0])
h = len(trees)

visible = [[False]*w for _ in range(h)]

def look_right(y, start=0):
    return zip(range(start, w), repeat(y))

def look_left(y, start=w-1):
    return zip(range(start, -1, -1), repeat(y))

def look_down(x, start=0):
    return zip(repeat(x), range(start, h))

def look_up(x, start=h-1):
    return zip(repeat(x), range(start, -1, -1))

for it in itertools.chain(
        [look_right(y) for y in range(h)], 
        [look_left(y) for y in range(h)], 
        [look_down(x) for x in range(w)],
        [look_up(x) for x in range(w)]):
    heighest = -1
    for (xi, yi) in it:
        z = trees[yi][xi]
        if z > heighest:
            visible[yi][xi] = True
            heighest = z

print(flatten(visible).count(True))

scores = [[None]*w for _ in range(h)]

for y in range(h):
    for x in range(w):
        s = 1
        for (it, default) in [
                (look_right(y, x+1), w-x-1), 
                (look_left(y, x-1), x), 
                (look_down(x, y+1), h-y-1), 
                (look_up(x, y-1), y)]:
            idx = find_first_index((trees[yi][xi] for (xi, yi) in it), lambda z: z >= trees[y][x])
            view = idx+1 if idx != None else default
            s *= view

        scores[y][x] = s

print(max(flatten(scores)))
