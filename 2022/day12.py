from helpers import *

txt = day_input(12)

def letter_to_height(c):
    if c == 'S': return 0
    if c == 'E': return 25
    return ord(c) - ord('a')

lines = [line for line in txt.split('\n') if line != '']
heightmap = [[letter_to_height(c) for c in line] for line in lines]

start, end = (0,20), (52,20)
w = len(lines[0])
h = len(lines)

def is_upstep_valid(s,t):
    x,y = s
    tx,ty = t
    h = heightmap[y][x]
    th = heightmap[ty][tx]
    return (th - h) <= 1

def upward_neighbors(p):
    x,y = p
    ns = []
    if x > 0: ns.append((x-1,y))
    if x < w-1: ns.append((x+1,y))
    if y > 0: ns.append((x,y-1))
    if y < h-1: ns.append((x,y+1))
    return [q for q in ns if is_upstep_valid(p,q)]

def heuristic(p):
    x,y = p
    ex,ey = end
    return abs(x-ex) + abs(y-ey)

path = astar_search(
    start,
    heuristic,
    upward_neighbors,
)

def print_path(path):
    pathmap = [['.']*w for _ in range(h)]

    for [(sx,sy), (tx,ty)] in windows(path, 2):
        if sx == tx:
            if sy > ty: pathmap[sy][sx] = '^'
            else: pathmap[sy][sx] = 'v'
        else:
            if sx > tx: pathmap[sy][sx] = '<'
            else: pathmap[sy][sx] = '>'

    for row in pathmap:
        print(''.join(row))

print(len(path)-1)

def find_lowground_heuristic(p):
    x,y = p
    if heightmap[y][x] == 0:
        return 0

    return 1

def is_downstep_valid(s,t):
    x,y = s
    tx,ty = t
    h = heightmap[y][x]
    th = heightmap[ty][tx]
    return (h - th) <= 1

def downward_neighbors(p):
    x,y = p
    ns = []
    if x > 0: ns.append((x-1,y))
    if x < w-1: ns.append((x+1,y))
    if y > 0: ns.append((x,y-1))
    if y < h-1: ns.append((x,y+1))
    return [q for q in ns if is_downstep_valid(p,q)]

path = astar_search(
    end, 
    find_lowground_heuristic, 
    downward_neighbors,
)

print(len(path)-1)
