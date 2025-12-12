import fileinput
import re
from functools import cache

lines = [*fileinput.input(encoding="utf-8")]

re_buttons = re.compile(r'(?:\(([0-9,]+)\))')
re_joltage = re.compile(r'(?:\{([0-9,]+)\})')

machines = []

for line in lines:
    buttons = [[int(x) for x in text.split(',')] for text in re_buttons.findall(line)]
    joltage = re_joltage.findall(line)[0]
    machines.append((buttons, joltage));

BIG = 9999999999999999999

def pack(xs):
    return ','.join(str(x) for x in xs)

def unpack(s):
    return [int(x) for x in s.split(',')]

for [buttons, joltage] in machines:
    @cache
    def find_min_press(s):
        state = unpack(s)
        if any(x < 0 for x in state):
            return BIG

        if all(x == 0 for x in state):
            return 0

        bestest = BIG

        for button in buttons:
            max_press_count = min(state[i] for i in button)

            best = BIG
            for count in range(max_press_count, 0, -1):
                acc = state[:]
                for i in button:
                    acc[i] -= count
                best = min(best, count + find_min_press(pack(acc)))
                if best < BIG:
                    bestest = min(bestest, best)
                    break
        
        return bestest

    print(find_min_press(joltage))
