import fileinput
import itertools
from itertools import pairwise, permutations
from functools import cache

BIG = 9999999999999999999

dirchar = { (-1,0): '<', (1,0): '>', (0,-1): '^', (0,1): 'v' }

# Starting position for the below keypad is on the A button.
#
# +---+---+---+
# | 7 | 8 | 9 |
# +---+---+---+
# | 4 | 5 | 6 |
# +---+---+---+
# | 1 | 2 | 3 |
# +---+---+---+
#     | 0 | A |
#     +---+---+

numpad = { '7': (0,0), '8': (1,0), '9': (2,0),
           '4': (0,1), '5': (1,1), '6': (2,1),
           '1': (0,2), '2': (1,2), '3': (2,2),
                       '0': (1,3), 'A': (2,3), }

def numpad_paths(s,t):
    ax,ay = numpad[s]
    bx,by = numpad[t]

    dx = abs(bx-ax)
    dy = abs(by-ay)

    cx = (-1,0) if bx < ax else (1,0)
    cy = (0,-1) if by < ay else (0,1)

    paths = []

    for steps in permutations(dx*[cx] + dy*[cy], dx+dy):
        x,y = ax,ay
        is_valid = True

        for (sx,sy) in steps:
            if (x,y) == (0,3):
                is_valid = False
                break
            x,y = x+sx,y+sy

        if not is_valid:
            continue

        paths.append([dirchar[s] for s in steps] + ['A'])

    return paths

def len_shortest_numpad_seq(s,t,depth):
    ps = numpad_paths(s,t)

    l = BIG
    for p in ps:
        s = sum(len_shortest_dirpad_seq(a,b,depth) for a,b in pairwise(['A'] + p))
        l = min(s, l)

    return l

# Starting position for the below keypad is on the A button.
#
#     +---+---+
#     | ^ | A |
# +---+---+---+
# | < | v | > |
# +---+---+---+

dirpad = {             '^': (1,0), 'A': (2,0),
           '<': (0,1), 'v': (1,1), '>': (2,1), }

def dirpad_paths(s,t):
    ax,ay = dirpad[s]
    bx,by = dirpad[t]

    dx = abs(bx-ax)
    dy = abs(by-ay)

    cx = (-1,0) if bx < ax else (1,0)
    cy = (0,-1) if by < ay else (0,1)

    paths = []

    for steps in permutations(dx*[cx] + dy*[cy], dx+dy):
        x,y = ax,ay
        is_valid = True

        for (sx,sy) in steps:
            if (x,y) == (0,0):
                is_valid = False
                break
            x,y = x+sx,y+sy

        if not is_valid:
            continue

        paths.append([dirchar[s] for s in steps] + ['A'])

    return paths

@cache
def len_shortest_dirpad_seq(s,t,depth):
    ps = dirpad_paths(s,t)

    if depth == 1:
        return min(len(p) for p in ps)

    l = BIG
    for p in ps:
        s = sum(len_shortest_dirpad_seq(a,b,depth-1) for a,b in pairwise(['A'] + p))
        l = min(s, l)

    return l

def shortest_seq_len(code, depth):
    return sum(len_shortest_numpad_seq(s,t,depth) for (s,t) in pairwise(['A'] + list(code)))

codes = [line.strip() for line in fileinput.input()]

print(sum(int(code[:-1]) * shortest_seq_len(code, 2) for code in codes))
print(sum(int(code[:-1]) * shortest_seq_len(code, 25) for code in codes))
