from helpers import *

txt = day_input(4)

lines = [l for l in txt.split('\n') if l != '']

pairs = [[[int(x) for x in elf.split('-')] for elf in line.split(',')] for line in lines]

counter = 0
for pair in pairs:
    for i,j in [(0,1), (1,0)]:
        if pair[i][0] >= pair[j][0] and pair[i][1] <= pair[j][1]:
            counter += 1
            break

print(counter)

counter = 0
for pair in pairs:
    for i,j in [(0,1), (1,0)]:
        if (pair[i][0] >= pair[j][0] and pair[i][0] <= pair[j][1]) or (pair[i][1] >= pair[j][0] and pair[i][1] <= pair[j][1]):
            counter += 1
            break

print(counter)

