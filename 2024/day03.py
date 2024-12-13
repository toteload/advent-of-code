import re

mem = open("day03.txt", "r").read()

muls = re.findall(r"mul\((\d{1,3}),(\d{1,3})\)", mem)
print(sum([int(a) * int(b) for [a,b] in muls]))

ops = re.findall(r"mul\((\d{1,3}),(\d{1,3})\)|(do)\(\)|(don't)\(\)", mem)

s = 0
on = True
for (a, b, x, y) in ops:
    if x:
        on = True
        continue
    if y:
        on = False
        continue

    if on:
        s += int(a) * int(b)

print(s)
