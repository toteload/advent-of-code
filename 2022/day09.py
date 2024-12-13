from helpers import *
from itertools import count

txt = day_input(9)

commands = [(line.split(' ')[0], int(line.split(' ')[1])) for line in txt.split('\n') if line != '']

t = (0,0)
h = (0,0)

def neighbours_and_self(p):
    return neighbours(p) + [p]

visited = set([t])

for (m, a) in commands:
    for _ in range(a):
        if m == 'D': h = (h[0], h[1] - 1)
        if m == 'U': h = (h[0], h[1] + 1)
        if m == 'L': h = (h[0] - 1, h[1])
        if m == 'R': h = (h[0] + 1, h[1])

        if h not in neighbours_and_self(t):
            dx, dy = (clamp(h[0]-t[0], -1, 1), clamp(h[1]-t[1], -1, 1))
            t = (t[0]+dx, t[1]+dy)
            visited.add(t)

print(len(visited))

rope = [(0,0)] * 10
visited = set([(0,0)])

for (m, a) in commands:
    for _ in range(a):
        h = rope[0]

        if m == 'D': h = (h[0], h[1] - 1)
        if m == 'U': h = (h[0], h[1] + 1)
        if m == 'L': h = (h[0] - 1, h[1])
        if m == 'R': h = (h[0] + 1, h[1])

        rope[0] = h

        for (i,j) in windows(take(count(0), 10), 2):
            h, t = rope[i], rope[j]

            if h not in neighbours_and_self(t):
                dx, dy = (clamp(h[0]-t[0], -1, 1), clamp(h[1]-t[1], -1, 1))
                t = (t[0]+dx, t[1]+dy)

            rope[i], rope[j] = h, t

        visited.add(rope[-1])

print(len(visited))
