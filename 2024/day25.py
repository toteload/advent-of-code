from fileinput import input
import itertools

lines = [line.strip() for line in input()]

chunks = [c[:7] for c in itertools.batched(lines, 8)]

locks = []
keys = []

for rows in chunks:
    cols = list(zip(*rows))
    cs = [col.count('#') for col in cols]
    is_lock = rows[0] == '#####'

    if is_lock:
        locks.append(cs)
    else:
        keys.append(cs)

s = 0
for lock,key in itertools.product(locks, keys):
    fits = all(a + b <= 7 for a,b in zip(lock,key))
    if fits:
        s += 1

print(s)
