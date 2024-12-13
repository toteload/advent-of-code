from helpers import *
import sys

txt = day_input(20)
xs = [int(x) for x in txt.splitlines()]

def mixlist(xs, nrounds=1):
    n = len(xs)

    inext = list(range(1,n)) + [0]
    iprev = [n-1] + list(range(n-1))

    for _ in range(nrounds):
        for i,x in enumerate(xs):
            d = xs[i] % (n - 1)

            if d == 0: continue

            j = i
            for _ in range(d):
                j = inext[j]

            iprev[inext[i]] = iprev[i]
            inext[iprev[i]] = inext[i]

            k = inext[j]
            iprev[k] = i
            inext[j] = i

            inext[i] = k
            iprev[i] = j

    ys = []
    i = 0
    for _ in range(n):
        ys.append(xs[i])
        i = inext[i]

    return ys

def answer(xs):
    n = len(xs)
    iz = xs.index(0)
    return sum([xs[(iz+d)%n] for d in [1000,2000,3000]])

print(answer(mixlist(xs[:])))
print(answer(mixlist([x*811589153 for x in xs],10)))
