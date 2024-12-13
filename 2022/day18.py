from helpers import *

txt = day_input(18)

cubes = {eval(line) for line in txt.split('\n') if line != ''}

def cube_neighbors(p):
    x,y,z = p
    return {
        (x-1,y,z),(x+1,y,z),
        (x,y-1,z),(x,y+1,z),
        (x,y,z-1),(x,y,z+1),
    }

area = 0
for c in cubes:
    area += 6 - len(cubes & cube_neighbors(c))

print(area)

# For part two, I create a virtual casting mold using a flood fill.
# A side of a cube is on the outside, if the cube that would share that side
# is part of the mold. Alternatively, you can find the area of the mold and
# subtract the outside area of the mold, which is easy to calculate (the six
# sides of the bounding box).

xs,ys,zs = [list(s) for s in zip(*cubes)]

xmin,xmax = min(xs),max(xs)
ymin,ymax = min(ys),max(ys)
zmin,zmax = min(zs),max(zs)

xrange = InclusiveRange(xmin-2,xmax+2)
yrange = InclusiveRange(ymin-2,ymax+2)
zrange = InclusiveRange(zmin-2,zmax+2)

# Create a shell to constrain the flood fill.
# This shell is a box that is large enough to fully contain the droplet.
shell = set()
for (x,y) in itertools.product(xrange,yrange):
    shell.add((x,y,zrange.lo))
    shell.add((x,y,zrange.hi))

for (x,z) in itertools.product(xrange,zrange):
    shell.add((x,yrange.lo,z))
    shell.add((x,yrange.hi,z))

for (y,z) in itertools.product(yrange,zrange):
    shell.add((xrange.lo,y,z))
    shell.add((xrange.hi,y,z))

border = cubes | shell
start = (xmin-1,ymin-1,zmin-1)

mold = {start}
candidates = cube_neighbors(start) - border

# Flood fill
while len(candidates) > 0:
    c = candidates.pop()
    mold.add(c)
    candidates |= cube_neighbors(c) - border - mold

exterior_area = 0
for c in cubes:
    exterior_area += len(cube_neighbors(c) & mold)
print(exterior_area)

# Alternative way of calculating the exterior area
mold_area = 0
for c in mold:
    mold_area += 6 - len(mold & cube_neighbors(c))

a = (xrange.size()-2)*(yrange.size()-2)
b = (xrange.size()-2)*(zrange.size()-2)
c = (yrange.size()-2)*(zrange.size()-2)

assert mold_area - (2*a + 2*b + 2*c) == exterior_area

