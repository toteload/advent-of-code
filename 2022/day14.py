from helpers import *

txt = day_input(14)
#txt = open('example.txt').read()

rockpaths = [[eval(p) for p in line.split('->')] for line in txt.split('\n') if line != '']
xs,ys = [flatten(x) for x in zip(*[zip(*path) for path in rockpaths])]

minx,maxx = min(xs),max(xs)
miny,maxy = min(ys),max(ys)

lo = minx,miny
hi = maxx,maxy

print(lo, hi)

# The first rock in the x direction is at 480 for me, so everything before that we don't have to
# keep in a map. I'm still going to do it otherwise I have to mess with coordinates and subtract
# `minx` from all the coordinates.

w = maxx+1
h = maxy+1

source = 500,0

occupied = [[False]*w for _ in range(h)]
for path in rockpaths:
    for (a,b) in windows(path, 2):
        x,y = a
        dx = b[0] - a[0]
        dy = b[1] - a[1]
        if dx == 0:
            step = clamp(dy,-1,1)
            for i in range(abs(dy)+1):
                occupied[y+i*step][x] = True
        else:
            step = clamp(dx,-1,1)
            for i in range(abs(dx)+1):
                occupied[y][x+i*step] = True

sand = source

def is_out_of_bounds(p):
    x,y = p
    return not (x >= minx and x <= maxx and y <= maxy)

counter = 0
while True:
    x,y = sand
    nexts = [(x,y+1),(x-1,y+1),(x+1,y+1)]
    if any([is_out_of_bounds(x) for x in nexts]):
        break

    i = find_first_index(nexts, lambda p: not occupied[p[1]][p[0]])
    if i == None:
        occupied[y][x] = True
        sand = source
        counter += 1
    else:
        sand = nexts[i]

print(counter)

# I am sure that there is also a solution that does not involve simulating all the sand.
# The final configuration will have a triangle of sand (unless there are parts of the rock that
# fall outside of this triangle). You can compute the volume of this triangle. You would then
# need to subtract the rocks from it and any upside down triangles projected from rock paths.
# This could be faster, but also requires more work from me compared to making the `occupied` map
# larger and just simulating it all :)

occupied = [[False]*1500 for _ in range(maxy+3)]
for path in rockpaths:
    for (a,b) in windows(path, 2):
        x,y = a
        dx = b[0] - a[0]
        dy = b[1] - a[1]
        if dx == 0:
            step = clamp(dy,-1,1)
            for i in range(abs(dy)+1):
                occupied[y+i*step][x] = True
        else:
            step = clamp(dx,-1,1)
            for i in range(abs(dx)+1):
                occupied[y][x+i*step] = True

for i in range(1500):
    occupied[maxy+2][i] = True

counter = 0
while not occupied[0][500]:
    x,y = sand
    nexts = [(x,y+1),(x-1,y+1),(x+1,y+1)]

    i = find_first_index(nexts, lambda p: not occupied[p[1]][p[0]])
    if i == None:
        occupied[y][x] = True
        sand = source
        counter += 1
    else:
        sand = nexts[i]

print(counter)

