from helpers import *
import copy

txt = day_input(5)

[stacks_txt, moves] = [x.strip() for x in txt.split('\n\n')]

start_stacks = [[] for i in range(10)]

for line in stacks_txt.split('\n')[:-1]:
    boxes = line.rstrip()[1::4]
    for i, box in enumerate(boxes, start=1):
        if box == ' ':
            continue
        start_stacks[i].append(box)

start_stacks = [s[::-1] for s in start_stacks]

moves = [[int(x) for x in line.split()[1::2]] for line in moves.split('\n')]

stacks = copy.deepcopy(start_stacks)
for [amount, src, dst] in moves:
    stacks[dst] += stacks[src][-amount:][::-1]
    stacks[src] = stacks[src][:-amount]

print("".join([s[-1] for s in stacks[1:]]))

stacks = copy.deepcopy(start_stacks)
for [amount, src, dst] in moves:
    stacks[dst] += stacks[src][-amount:]
    stacks[src] = stacks[src][:-amount]

print("".join([s[-1] for s in stacks[1:]]))
