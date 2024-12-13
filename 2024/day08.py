import collections, itertools

m = [line.strip() for line in open("day08.txt", "r").readlines()]

antennas = collections.defaultdict(set)

w = len(m[0])
h = len(m)

for y in range(h):
    for x in range(w):
        c = m[y][x]
        if c != '.':
            antennas[c].add((x,y))

antinodes = set()

for group in antennas.values():
    for ((ax,ay), (bx,by)) in itertools.combinations(group, 2):
        dx = bx - ax
        dy = by - ay

        x = ax - dx
        y = ay - dy

        if 0 <= x < w and 0 <= y < h:
            antinodes.add((x,y))

        x = bx + dx
        y = by + dy
        
        if 0 <= x < w and 0 <= y < h:
            antinodes.add((x,y))

print(len(antinodes))

antinodes.clear()

for group in antennas.values():
    for ((ax,ay), (bx,by)) in itertools.combinations(group, 2):
        dx = bx - ax
        dy = by - ay

        x = ax
        y = ay

        while 0 <= x < w and 0 <= y < h:
            antinodes.add((x,y))
            x -= dx
            y -= dy

        x = bx
        y = by

        while 0 <= x < w and 0 <= y < h:
            antinodes.add((x,y))
            x += dx
            y += dy

print(len(antinodes))

