import fileinput
import re
from z3 import *

def parse_machines(lines):
    re_buttons = re.compile(r'(?:\(([0-9,]+)\))')
    re_joltage = re.compile(r'(?:\{([0-9,]+)\})')

    machines = []

    for line in lines:
        buttons = [[int(x) for x in text.split(',')] for text in re_buttons.findall(line)]
        req = [int(x) for x in re_joltage.findall(line)[0].split(',')]
        machines.append((buttons, req));

    return machines

# Using Z3 feels a bit like cheating though :P 
def solve_machine_with_z3(machine):
    buttons,req = machine
    s = Optimize()

    size = len(req)

    x = [Int(f'x_{i}') for i in range(len(buttons))]
    for v in x:
        s.add(v >= 0)

    lights = [[] for _ in range(size)]

    for i, button in enumerate(buttons):
        for j in button:
            lights[j].append(i)

    for light, y in zip(lights, req):
        s.add(Sum([x[i] for i in light]) == y)

    s.minimize(Sum(x))

    assert(s.check() == sat)

    m = s.model()

    return sum([m[v].as_long() for v in x])

if __name__ == "__main__":
    lines = [*fileinput.input(encoding="utf-8")]
    machines = parse_machines(lines)
    print(sum(solve_machine_with_z3(m) for m in machines))
