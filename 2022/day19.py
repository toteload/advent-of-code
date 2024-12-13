from helpers import *
import re, functools
from dataclasses import dataclass

txt = day_input(19)
#txt = open('example.txt').read()

lines = [re.findall('\d+', line)[1:] for line in txt.splitlines()]

blueprints = []
for [a,b,c,d,e,f] in lines:
    blueprints.append((
        [int(a),0,0,0],
        [int(b),0,0,0],
        [int(c),int(d),0,0],
        [int(e),0,int(f),0],
    ))

for [a,b,c,d,e,f] in lines:
    s=f"[[{int(a)},0,0,0], [{int(b)},0,0,0], [{int(c)},{int(d)},0,0], [{int(e)},0,{int(f)},0]],"
    print(s)

def ceil_div(a,b):
    return (a+b-1) // b

def recursive_search(blueprint, max_robots, ores, robots, t, best):
    searched_deeper = False

    for i in range(4):
        if robots[i] >= max_robots[i]:
            continue

        recipe = blueprint[i]

        wait_time = -1
        for j in range(4):
            if recipe[j] == 0:
                continue

            if recipe[j] <= ores[j]:
                wait_time = max(wait_time, 0)
            elif robots[j] == 0:
                wait_time = 9999
            else:
                wait_time = max(wait_time, ceil_div(recipe[j] - ores[j], robots[j]))

        if t-wait_time-1 <= 0:
            continue

        next_ores = ores[:]
        for j in range(4):
            next_ores[j] += robots[j] * (wait_time + 1) - recipe[j]

        next_robots = robots[:]
        next_robots[i] += 1

        remaining_time = t - (wait_time + 1)
        rt = remaining_time
        
        max_geodes = next_ores[3] + rt * next_robots[3] + ((rt-1) * rt) / 2
        if max_geodes < best:
            continue

        searched_deeper = True

        best = max(best, recursive_search(blueprint, max_robots, next_ores, next_robots, rt, next_ores[3]))

    if not searched_deeper:
        best = max(best, ores[3] + robots[3] * t)

    return best

def search(blueprint, t):
    max_robots = [max(*x) for x in zip(*blueprint)]
    max_robots[3] = 99999
    return recursive_search(blueprint, max_robots, [0,0,0,0], [1,0,0,0], t, 0)

print(sum([i * search(blueprint, 24) for (i,blueprint) in enumerate(blueprints,start=1)]))
print(functools.reduce(lambda a,b: a*b, [search(b, 32) for b in take(blueprints,3)]))
