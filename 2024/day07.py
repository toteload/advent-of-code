import itertools, functools

lines = [line.strip() for line in open("day07.txt", "r").readlines()]

[totals, xs] = zip(*[line.split(":") for line in lines])

totals = [int(x) for x in totals]
xs = [[int(y) for y in x.split()] for x in xs]

eqs = list(zip(totals, xs))

def f(a, op, b):
    if op == '+': return a + b
    if op == '*': return a * b

def is_valid(total, xs):
    for ops in itertools.product('+*', repeat=len(xs)-1):
        s = functools.reduce(lambda x, rest: f(x, rest[0], rest[1]), zip(ops, xs[1:]), xs[0])
        if s == total:
            return True
    return False

print(sum([e[0] for e in eqs if is_valid(*e)]))

def f2(a, op, b):
    if op == '+': return a + b
    if op == '*': return a * b
    if op == '|':
        return int(str(a) + str(b))

def is_valid2(total, xs):
    for ops in itertools.product('+*|', repeat=len(xs)-1):
        s = functools.reduce(lambda x, rest: f2(x, rest[0], rest[1]), zip(ops, xs[1:]), xs[0])
        if s == total:
            return True
    return False

print(sum([e[0] for e in eqs if is_valid2(*e)]))
