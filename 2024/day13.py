import re
from itertools import product

text = open("day13.txt").read()

machines = re.findall(r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)", text, re.ASCII)

def lowest_cost_bruteforce(ax, ay, bx, by, px, py):
    best = None

    for (i,j) in product(range(100),range(100)):
        if i*ax + j*bx == px and i*ay + j*by == py:
            toks = 3*i + j
            best = toks if not best else min(best, toks)

    return 0 if not best else best

def with_higher_price(ax,ay,bx,by,px,py):
    return (ax,ay,bx,by,px+10000000000000,py+10000000000000)

def lowest_cost(ax, ay, bx, by, px, py):
    if (px*by-bx*py) % (ax*by-bx*ay) != 0:
        return 0

    i = (px*by-bx*py) // (ax*by-bx*ay)
    j = (px-i*ax) // bx

    return 3*i + j

print(sum([lowest_cost_bruteforce(*[int(v) for v in m]) for m in machines]))
print(sum([lowest_cost(*with_higher_price(*[int(v) for v in m])) for m in machines]))
