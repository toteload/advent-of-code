from helpers import *
from collections import defaultdict
import re

txt = day_input(16)

lines = [line for line in txt.split('\n') if line != '']
p = re.compile('Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z]{2}(?:, [A-Z]{2})*)', re.ASCII)

pressure = {}
travelcost = defaultdict(dict)
rooms = set()

for line in lines:
    room, flow, tunnels = p.match(line).groups()
    tunnels = [x.strip() for x in tunnels.split(',')]
    pressure[room] = int(flow)
    for adj in tunnels:
        travelcost[room][adj] = 1
    rooms.add(room)

for room in rooms:
    others = rooms - {room}
    travelcost[room] = {x: -1 for x in others} | travelcost[room]

# This feels like a very ugly way of calculating path costs. 
# Could be nice to research neater ways.
for _ in range(3):
    keys = travelcost.keys()
    for (a,b,c) in itertools.permutations(keys, 3):
        assert travelcost[a][b] == travelcost[b][a]
        assert travelcost[c][b] == travelcost[b][c]

        cost_ab = travelcost[a][b]
        cost_bc = travelcost[b][c]

        if cost_ab != -1 and cost_bc != -1:
            d = cost_ab + cost_bc
            dd = d if travelcost[a][c] == -1 else min(travelcost[a][c], d)
            travelcost[a][c] = dd
            travelcost[c][a] = dd

candidates = {room for room in rooms if pressure[room] > 0}

# Returning the path is not necessary, but I return it for debugging purposes.
# I actually had it working correctly first try, but I thought that the starting
# room was the first room in the input not room AA... This made me think something
# was wrong and I added the path to debug. Oh well...
#def search(score, room, time_remaining, candidates):
#    if len(candidates) == 0:
#        return score, []
#
#    best = -1 
#    path = None
#
#    for c in candidates:
#        d = travelcost[room][c]
#
#        if (d+1) >= time_remaining:
#            continue
#
#        t = time_remaining-(d+1)
#        new_score = score + pressure[c] * t
#        cs = candidates - {c}
#
#        s, rest = search(new_score, c, t, cs)
#        if s > best:
#            best = s
#            path = [(c,t)] + rest
#
#    if best == -1:
#        return score, []
#
#    return best, path

def search(room, time_remaining, candidates):
    if len(candidates) == 0:
        return 0

    best = 0
    for c in candidates:
        d = travelcost[room][c]

        if (d+1) >= time_remaining:
            continue

        t = time_remaining-(d+1)
        cs = candidates - {c}

        best = max(best, pressure[c] * t + search(c, t, cs))

    return best

score = search('AA', 30, candidates)
print(score)

def elephant_search(you, elephant, you_time, elephant_time, candidates):
    if len(candidates) == 0:
        return 0

    best = 0
    for c in candidates:
        # Either you go to c, or elephant goes to c

        d = travelcost[you][c]
        if (d+1) < you_time:
            t = you_time - (d+1)
            cs = candidates - {c}
            score = pressure[c] * t + elephant_search(c, elephant, t, elephant_time, cs)
            best = max(best, score)

        d = travelcost[elephant][c]
        if (d+1) < elephant_time:
            t = elephant_time - (d+1)
            cs = candidates - {c}
            score = pressure[c] * t + elephant_search(you, c, you_time, t, cs)
            best = max(best, score)

    return best

print(elephant_search('AA', 'AA', 26, 26, candidates))
