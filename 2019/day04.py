from utils import *

[line] = readlines()
[lo, hi] = [int(x) for x in line.split('-')]

print(lo, hi)

# Ugly bruteforce
hits = 0
for x in range(lo, hi):
    s = str(x)

    has_doubles = any([a == b for (a, b) in windows(s, 2)])
    monotonically_increasing = all([b >= a for (a, b) in windows(s, 2)])

    if not (has_doubles and monotonically_increasing):
        continue

    hits += 1

print(hits)

# Ugly bruteforce
hits = 0
for x in range(lo, hi):
    s = str(x)

    a, b, c, *_ = s
    start_double = a == b and b != c

    *_, c, b, a = s
    end_double = a == b and b != c

    mid_double = any([a != b and b == c and c != d for (a, b, c, d) in windows(s, 4)])

    monotonically_increasing = all([b >= a for [a, b] in windows(s, 2)])

    if not ((start_double or end_double or mid_double) and monotonically_increasing):
        continue

    hits += 1

print(hits)
