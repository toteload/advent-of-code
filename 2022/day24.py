from helpers import *

left = set()
right = set()
up = set()
down = set()

txt = day_input(24)
#txt = open('example.txt').read()
lines = [row[1:-1] for row in txt.splitlines()[1:-1]]

for y,row in enumerate(lines):
    for x,c in enumerate(row):
        p = x,y
        if c == '>': right.add(p)
        if c == '<': left.add(p)
        if c == 'v': down.add(p)
        if c == '^': up.add(p)

w = len(lines[0])
h = len(lines)

states = []

for _ in range(w*h):
    states.append(left|right|up|down)

    left = {((x-1)%w,y) for x,y in left}
    right = {((x+1)%w,y) for x,y in right}
    up = {(x,(y-1)%h) for x,y in up}
    down = {(x,(y+1)%h) for x,y in down}

GOAL = w-1,h
START = 0,-1

def heuristic(state):
    p,t = state
    return manhattan(p, GOAL)

def ns(state):
    p,t = state
    next_state = states[(t+1) % len(states)]

    ps = [p]
    if p == START:
        ps.append((0,0))
    else:
        x,y = p
        if x > 0: ps.append((x-1,y))
        if x < w-1: ps.append((x+1,y))
        if y > 0: ps.append((x,y-1))
        if y < h-1: ps.append((x,y+1))
        if p == (w-1,h-1): ps.append(GOAL)

    return [(p,t+1) for p in ps if p not in next_state]

#print(astar_search((START, 0), heuristic, ns)[-1][1])

ONEWAYCOST = manhattan(START,GOAL)
phasegoal = [GOAL,START,GOAL]

def heuristic(state):
    p,z,t = state
    if z == 3: return 0
    return (2-z)*ONEWAYCOST + manhattan(p,phasegoal[z])

def ns(state):
    p,z,t = state
    next_state = states[(t+1) % len(states)]

    ps = [p]

    if p == START: ps.append((0,0))
    if p == GOAL: ps.append((w-1,h-1))
    if p == (w-1,h-1): ps.append(GOAL)
    if p == (0,0): ps.append(START)

    if p != START and p != GOAL:
        x,y = p
        if x > 0: ps.append((x-1,y))
        if x < w-1: ps.append((x+1,y))
        if y > 0: ps.append((x,y-1))
        if y < h-1: ps.append((x,y+1))

    return [(q,z+int(q==phasegoal[z]),t+1) for q in ps if q not in next_state]

print(astar_search((START,0,0), heuristic, ns)[-1][-1])
