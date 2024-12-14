import fileinput
import re
import sys
import copy
from dataclasses import dataclass, astuple
from functools import reduce

def count(it, pred):
    s = 0
    for x in it:
        if pred(x):
            s += 1
    return s

@dataclass
class Robot:
    x: int
    y: int
    vx: int
    vy: int

def is_point_in_area(x,y, bx,by,bw,bh):
    return bx <= x < (bx+bw) and by <= y < (by+bh)

lines = fileinput.input()
entries = [(int(x) for x in re.match(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)", line).groups()) for line in lines]
robots_start = [Robot(*xs) for xs in entries]

robots = [copy.copy(r) for r in robots_start]

# !!! WARNING !!!
#
# The dimensions of the area are not part of the read input and different between the example
# and the real problem!
# 
# Don't forget to adjust them accordingly!

w = 101
h = 103

steps = 100
for _ in range(steps):
    for robot in robots:
        robot.x = (robot.x + robot.vx) % w
        robot.y = (robot.y + robot.vy) % h

quadrants = [
    [     0,      0, w//2, h//2],
    [1+w//2,      0, w//2, h//2],
    [     0, 1+h//2, w//2, h//2],
    [1+w//2, 1+h//2, w//2, h//2],
]

qs = [count(robots, lambda r: is_point_in_area(r.x,r.y, *quadrant)) for quadrant in quadrants]
print(reduce(lambda a, b: a * b, qs))

robots = [copy.copy(r) for r in robots_start]

i = 0

while True:
    for robot in robots:
        robot.x = (robot.x + robot.vx) % w
        robot.y = (robot.y + robot.vy) % h

    i += 1

    ps = set([(r.x,r.y) for r in robots])

    for y in range(h):
        for x in range(w):
            if (x,y) in ps:
                sys.stdout.write('█')
            else:
                sys.stdout.write('.')
        sys.stdout.write('\n')

    sys.stdout.flush()

    input(f"The above picture was drawn after {i} seconds have elapsed. Press Return to continue...")

