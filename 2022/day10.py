from helpers import *

txt = day_input(10)

lines = [line for line in txt.split('\n') if line != '']

tdx = []
for command in lines:
    if command == 'noop':
        tdx += [0]
    else:
        tdx += [0, int(command.split(' ')[-1])]

x,s = 1,0
for t, dx in enumerate(tdx, start=1):
    x += dx
    if t % 40 == 20: 
        s += x*t

print(s)

x, crt = 1, []
for t, dx in enumerate(tdx):
    sx = t % 40
    crt.append(x in [sx-1,sx,sx+1])
    x += dx

for row in chunks(crt, 40):
    print(''.join(['#' if c else '.' for c in row]))
