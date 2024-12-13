m = [line.strip() for line in open("day06.txt", "r").readlines()]
#rows = [line.strip() for line in open("demo.txt", "r").readlines()]

w = len(m[0])
h = len(m)

def turn_right(d):
    if d == (0,-1): return (1,0)
    if d == (1,0): return (0,1)
    if d == (0,1): return (-1,0)
    if d == (-1,0): return (0,-1)

def find_guard(m, w, h):
    for y in range(h):
        for x in range(w):
            if m[y][x] == '^':
                return (x, y)

def count_steps(m, w, h):
    d = (0, -1)
    (x,y) = find_guard(m, w, h)

    steps = {(x,y)}

    while True:
        (dx, dy) = d

        nx = x + dx
        ny = y + dy

        if not (0 <= nx < w and 0 <= ny < h):
            break

        if m[ny][nx] == '#':
            d = turn_right(d)
        else:
            x, y = nx, ny
            steps.add((x,y))

    return len(steps)

print(count_steps(m, w, h))

start = find_guard(m, w, h)

def has_loop(m):
    s = 0
    d = (0, -1)
    (x,y) = start
    while s < 100000:
        (dx, dy) = d

        nx = x + dx
        ny = y + dy

        if not (0 <= nx < w and 0 <= ny < h):
            return False

        if m[ny][nx] == '#':
            d = turn_right(d)
        else:
            x, y = nx, ny
            s += 1
    return True

s = 0
for y in range(h):
    for x in range(w):
        if m[y][x] != '.':
            continue

        m[y] = m[y][:x] + '#' + m[y][x+1:]

        if has_loop(m):
            s += 1

        m[y] = m[y][:x] + '.' + m[y][x+1:]

print(s)
