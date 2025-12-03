from utils import *
import re

lines = readlines()
prog = re.compile(r'#\d+ @ (\d+),(\d+): (\d+)x(\d+)')
orders = [Rectangle(*[int(x) for x in prog.match(line).groups()]) for line in lines]

corners = flatten([order.corners() for order in orders])
xs, ys = list(zip(*[(p.x, p.y) for p in corners]))
lox, hix = min(xs), max(xs)
loy, hiy = min(ys), max(ys)

print(lox, hix, loy, hiy)
