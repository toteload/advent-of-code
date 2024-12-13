from helpers import *
import re

txt = day_input(22)
#txt = open('example.txt').read()
lines = txt.splitlines()

walls = set()
onboard = set()

BIG = 999999
boardlines = lines[:-2]
w = len(boardlines[0])
h = len(boardlines)

rows = []
cols = []

for y,row in enumerate(boardlines):
    rowmin,rowmax = BIG,-BIG

    for x,c in enumerate(row):
        p = (x,y)
        if c == '#': walls.add(p)
        if c != ' ': 
            rowmin = min(rowmin, x)
            rowmax = max(rowmax, x)

            onboard.add(p)

    rows.append((rowmin,rowmax+1))

for x in range(w):
    colmin,colmax = BIG, -BIG
    for y in range(h):
        row = boardlines[y]
        if x >= len(row):
            break
        c = row[x]
        if c != ' ':
            colmin = min(colmin, y)
            colmax = max(colmax, y)

    cols.append((colmin, colmax+1))

def dir_to_char(d):
    if d == (1,0): return '>'
    if d == (0,1): return 'v'
    if d == (-1,0): return '<'
    if d == (0,-1): return '^'

moves = re.findall('\d+|[RL]', lines[-1])

p = rows[0][0],0
d = (1,0)

dirs = [(1,0),(0,1),(-1,0),(0,-1)]

steps = {}

for m in moves:
    if m == 'L' or m == 'R':
        dd = -1 if m == 'L' else 1
        d = dirs[(dirs.index(d) + dd) % 4]
    else:
        i = int(m)

        for _ in range(i):
            steps[p] = dir_to_char(d)

            x,y = p
            dx,dy = d
            np = (x+dx,y+dy)

            if np not in onboard:
                nx,ny = np

                if d[0] != 0:
                    rmin,rmax = rows[ny]
                    w = rmax-rmin
                    nx = ((nx - rmin) % w) + rmin
                else:
                    cmin,cmax = cols[nx]
                    h = cmax-cmin
                    ny = ((ny-cmin) % h) + cmin

                np = nx,ny

            if np in walls:
                break

            p = np

x,y = p
f = dirs.index(d)
print(1000*(y+1) + 4*(x+1) + f)
