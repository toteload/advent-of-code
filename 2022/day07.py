from helpers import *

txt = day_input(7)

lines = [line for line in txt.split('\n') if line != '']

root = {}
wd = []

def get(root, path):
    d = root
    for p in path:
        d = d[p]
    return d

for line in lines:
    if line == '$ ls':
        continue
    elif line.startswith('$ cd'):
        d = line.split(' ')[-1]
        if d == '/':
            wd = []
        elif d == '..':
            wd.pop()
        else:
            wd.append(d)
    else:
        [a, b] = line.split(' ')
        if a == 'dir':
            get(root, wd)[b] = {}
        else:
            get(root, wd)[b] = int(a)

sizes = []
def dirsize(d):
    s = 0
    for (name, c) in d.items():
        if type(c) is dict:
            s += dirsize(c)
        else:
            s += c
    sizes.append(s)
    return s

space_used = dirsize(root)

print(sum([s for s in sizes if s <= 100_000]))

total_size = 70_000_000
size_needed = 30_000_000
unused_space = total_size - space_used
to_delete = size_needed - unused_space

print(min([s for s in sizes if s >= to_delete]))
