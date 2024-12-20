import fileinput
from functools import cache
from collections import defaultdict

lines = [line.strip() for line in fileinput.input()]
patterns = lines[0].split(', ')
designs = lines[2:]

towels = defaultdict(list)

for p in patterns:
    towels[p[0]].append(p)

@cache
def is_possible(design):
    if not design:
        return True

    for p in towels[design[0]]:
        if not design.startswith(p):
            continue

        if is_possible(design[len(p):]):
            return True

    return False

print(len([1 for d in designs if is_possible(d)]))

@cache
def number_of_ways(design):
    if not design:
        return 1

    return sum(number_of_ways(design[len(p):]) for p in towels[design[0]] if design.startswith(p))

print(sum(number_of_ways(d) for d in designs))
