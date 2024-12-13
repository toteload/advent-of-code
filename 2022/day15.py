from helpers import *
import re

txt = day_input(15)
YROW = 2000000

lines = [line for line in txt.split('\n') if line != '']

p = re.compile('-?\d+', re.ASCII)

pairs = []
for line in lines:
    [a,b,c,d] = [int(x.group()) for x in p.finditer(line)]
    pairs += [((a,b), (c,d))]

beacons_on_row = set()
for _,b in pairs:
    x,y = b
    if y != YROW:
        continue
    beacons_on_row.add((x,y))

exclusions = []
for sensor,beacon in pairs:
    x,y = sensor
    d = manhattan(sensor, beacon) - abs(y - YROW)
    if d <= 0:
        continue

    ex = InclusiveRange(x-d, x+d)
    # Updating the overlaps can probably be done faster. This is just linear, but there are at most
    # 24 ranges so that's not a lot.
    other, overlaps = partition(lambda r: r.has_overlap(ex), exclusions)
    exclusions = list(other) + [reduce(lambda a,b: a.merge(b), overlaps, ex)]

print(sum([r.size() for r in exclusions]) - len(beacons_on_row))

# I assume that the undiscovered beacon is constrained by four beacon-sensor diamonds, and that
# the search area border is not necessary for the constrainment of the beacon. If this is true,
# then there are two pairs of diamonds that leave a gap of precisely 1 in between them.
# I search for these pairs, then I walk along the border of one of the diamonds, where we know the
# beacon has to be. 
# The last part is a relatively slow linear search, and there is probably a more efficient solution.
ps = []
for (a,b) in itertools.combinations(pairs,2):
    if (manhattan(*a) + manhattan(*b) + 2) == manhattan(a[0],b[0]):
        ps += [(a,b)]

assert len(ps) == 2

ps = flatten(ps)

sensor,beacon = ps[0]
sx,sy = sensor
d = manhattan(sensor,beacon)+1

ps = [(s,manhattan(s,b)) for (s,b) in ps]

for c in itertools.chain(
        ((sx-d+i,sy+i) for i in range(d)),
        ((sx+i,sy-d+i) for i in range(d)),
        ((sx+d-i,sy-i) for i in range(d)),
        ((sx-i,sy+d-i) for i in range(d))):
    if all([manhattan(c,s) == d+1 for s,d in ps]):
        x,y = c
        print(x*4000000+y)
        break
