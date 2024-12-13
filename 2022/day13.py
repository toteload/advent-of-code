from helpers import *
import functools

txt = day_input(13)

pairs = [tuple([eval(x) for x in line.rstrip().split('\n')]) for line in txt.split('\n\n') if line != '']

def cmp(a, b):
    if type(a) is int and type(b) is int:
        return clamp(b - a, -1, 1)

    if type(a) is list and type(b) is list:
        for (al,bl) in zip(a,b):
            c = cmp(al,bl)
            if c != 0:
                return c
        return clamp(len(b) - len(a), -1, 1)

    if type(a) is list:
        return cmp(a, [b])
    else:
        return cmp([a], b)


print(sum([i for (i,pair) in enumerate(pairs, start=1) if cmp(*pair) == 1]))

packets = [eval(line) for line in txt.split('\n') if line != '']
packets += [[[2]],[[6]]]

packets.sort(key=functools.cmp_to_key(cmp),reverse=True)

print((packets.index([[2]])+1)*(packets.index([[6]])+1))
