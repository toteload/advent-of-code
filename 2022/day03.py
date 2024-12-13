from helpers import *

txt = day_input(3)

lines = [l for l in txt.split('\n') if l != '']

def priority(c):
    if c >= 'a' and c <= 'z':
        return 1 + ord(c) - ord('a')
    else:
        return 27 + ord(c) - ord('A')

s = 0
for line in lines:
    mid = len(line) // 2
    c0, c1 = line[:mid], line[mid:]
    s += priority(first(set(c0) & set(c1))) 

print(s)

s = 0
for group in chunks(lines, 3):
    group_sets = [set(x) for x in group]
    s += priority(first(reduce(lambda x,y: x & y, group_sets)))

print(s)

