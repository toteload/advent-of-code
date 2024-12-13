from helpers import *

txt = day_input(25)

def snafu_to_decimal(s):
    acc = 0
    for c in s:
        x = None
        if c == '=': x = -2
        if c == '-': x = -1
        if c == '0': x = 0
        if c == '1': x = 1
        if c == '2': x = 2
        acc *= 5
        acc += x
    return acc

def decimal_to_snafu(d):
    acc = d
    chars = []
    while acc != 0:
        r = acc % 5
        acc //= 5

        if r >= 3:
            acc += 1
            r -= 5

        if r >= 0:
            chars.append(str(r))
        else:
            chars.append('-' if r == -1 else '=')

    return ''.join(reversed(chars))

print(decimal_to_snafu(sum([snafu_to_decimal(s) for s in txt.splitlines()])))
