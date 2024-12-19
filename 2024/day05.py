import itertools
from functools import cmp_to_key

lines = [line.strip() for line in open("demo.txt","r").readlines()]
lines = [line.strip() for line in open("input.txt","r").readlines()]
idx = lines.index('')
rules,updates = lines[:idx], lines[idx+1:]
rules = [[int(x) for x in line.split('|')] for line in rules]
updates = [[int(x) for x in line.split(',')] for line in updates]

def is_correct(rules, xs):
    for (a, b) in itertools.combinations(xs, 2):
        for (lo, hi) in rules:
            if a == hi and b == lo:
                return False
    return True

print(sum([xs[len(xs)//2] for xs in updates if is_correct(rules, xs)]))

incorrect = [xs for xs in updates if not is_correct(rules, xs)]

def comes_before(a,b):
    for (lo,hi) in rules:
        if b == lo and a == hi:
            return False
    return True

def find_biggest(xs):
    for x in xs:
        zs = xs[:]
        zs.remove(x)
        if all([comes_before(x, z) for z in zs]):
            return x

def sorted2(xs):
    ys=[]
    zs = xs[:]
    while len(zs) > 0:
        x = find_biggest(zs)
        ys.append(x)
        zs.remove(x)

    return ys

mods = [sorted2(xs) for xs in incorrect]

print(sum([xs[len(xs)//2] for xs in mods]))
