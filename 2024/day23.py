import fileinput
from collections import defaultdict
from itertools import combinations

lines = (line.strip() for line in fileinput.input())
conns = [(l[:2], l[3:]) for l in lines]

e = defaultdict(set)
v = set()

for a,b in conns:
    e[a].add(b)
    e[b].add(a)

    v.add(a)
    v.add(b)

lans = []

# Each 3-size LAN will be added three times with the computers in the LAN in a different order.
for x in v:
    for y,z in combinations(e[x],2):
        if y in e[z] and z in e[y]:
            lans.append((x,y,z))

lans_with_t = [x for x in lans if x[0][0] == 't' or x[1][0] == 't' or x[2][0] == 't']

print(len(lans_with_t)//3)

best = set()

for x in v:
    clique = {x}

    for y in e[x]:
        if clique <= e[y]:
            clique.add(y)

    if len(clique) > len(best):
        best = clique

print(','.join(sorted(best)))
