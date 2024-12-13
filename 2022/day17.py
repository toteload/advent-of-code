from helpers import *
import sys

txt = day_input(17)
#txt = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"

jetpattern = []
for c in txt:
    if c == '<': jetpattern.append(-1)
    if c == '>': jetpattern.append(1)

rockshapes = [
    [[1,1,1,1]],

    [[0,1,0],
     [1,1,1],
     [0,1,0]],

    [[0,0,1],
     [0,0,1],
     [1,1,1]],

    [[1],
     [1],
     [1],
     [1]],

    [[1,1],
     [1,1]],
]

jet = itertools.cycle(jetpattern)
rocks = itertools.cycle(rockshapes)

chamber = [[False]*7 for _ in range(3)]

def has_overlap(a, b, bx, by):
    w = len(b[0])
    h = len(b)

    for (x,y) in itertools.product(range(w),range(h)):
        if a[by+y][bx+x] and b[y][x]:
            return True

    return False

def print_chamber(chamber):
    for row in chamber:
        print(''.join(['#' if x else '.' for x in row]))
    print('')

for i in range(2022):
    rock = next(rocks)
    w = len(rock[0])
    h = len(rock)

    chamber = [[False]*7 for _ in range(h)] + chamber

    x = 2 # Left side of the rock
    y = 0

    while True:
        dx = next(jet)
        nx = clamp(x+dx,0,7-w)

        if not has_overlap(chamber, rock, nx, y):
            x = nx

        if (y+h) < len(chamber) and not has_overlap(chamber, rock, x, y+1):
            y += 1
            continue

        break

    for (rx,ry) in itertools.product(range(w),range(h)):
        if rock[ry][rx]:
            chamber[y+ry][x+rx] = True

    first_non_empty = None
    for y in range(len(chamber)):
        if any([bool(x) for x in chamber[y]]):
            first_non_empty = y
            break

    if first_non_empty > 3:
        chamber = chamber[first_non_empty-3:]
    elif first_non_empty < 3:
        chamber = [[False]*7 for _ in range(3-first_non_empty)] + chamber

    #print_chamber(chamber)

print_chamber(chamber)

first_non_empty = None
for y in range(len(chamber)):
    if any([bool(x) for x in chamber[y]]):
        first_non_empty = y
        break

print(first_non_empty)

print(len(chamber) - first_non_empty)
