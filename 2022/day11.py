from helpers import *
from dataclasses import dataclass
from collections import deque
import copy

@dataclass
class Monkey:
    items: 'deque[int]'
    op: 'Callable[[int],int]'
    next_monkey: 'Callable[[int],int]'

init_monkeys = [
    Monkey(
        deque([85,77,77]),
        lambda x: x * 7,
        lambda x: 6 if x % 19 == 0 else 7,
    ),
    Monkey(
        deque([80,99]),
        lambda x: x * 11,
        lambda x: 3 if x % 3 == 0 else 5,
    ),
    Monkey(
        deque([74,60,74,63,86,92,80]),
        lambda x: x + 8,
        lambda x: 0 if x % 13 == 0 else 6,
    ),
    Monkey(
        deque([71,58,93,65,80,68,54,71]),
        lambda x: x + 7,
        lambda x: 2 if x % 7 == 0 else 4,
    ),
    Monkey(
        deque([97,56,79,65,58]),
        lambda x: x + 5,
        lambda x: 2 if x % 5 == 0 else 0,
    ),
    Monkey(
        deque([77]),
        lambda x: x + 4,
        lambda x: 4 if x % 11 == 0 else 3,
    ),
    Monkey(
        deque([99,90,84,50]),
        lambda x: x * x,
        lambda x: 7 if x % 17 == 0 else 1,
    ),
    Monkey(
        deque([50,66,61,92,64,78]),
        lambda x: x + 3,
        lambda x: 5 if x % 2 == 0 else 1,
    ),
]

counters = [0]*len(init_monkeys)
monkeys = copy.deepcopy(init_monkeys)

for _ in range(20):
    for i, m in enumerate(monkeys):
        while len(m.items) > 0:
            counters[i] += 1
            x = m.items.popleft()
            y = m.op(x) // 3
            monkeys[m.next_monkey(y)].items.append(y)

counters.sort()
print(counters[-2]*counters[-1])

counters = [0]*len(init_monkeys)
monkeys = copy.deepcopy(init_monkeys)

M = 19 * 3 * 13 * 7 * 5 * 11 * 17 * 2

for _i in range(10_000):
    for i, m in enumerate(monkeys):
        while len(m.items) > 0:
            counters[i] += 1
            x = m.items.popleft()
            y = m.op(x) % M
            monkeys[m.next_monkey(y)].items.append(y)

counters.sort()
print(counters[-2]*counters[-1])

