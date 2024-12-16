import fileinput
import itertools
from grid import Grid, Point

def list_index_where(it, pred):
    for i, x in enumerate(it):
        if pred(x):
            return i
    return None

lines = [line.strip() for line in fileinput.input()]
idx = list_index_where(lines, lambda x: not x)
rows = [list(line) for line in lines[:idx]]
moves = ''.join(lines[idx+1:])

grid = Grid([row[:] for row in rows])

def update(row, x):
    row = row[:]
    last_free = None

    for i, v in enumerate(row[:x]):
        if v == '.':
            last_free = i

        if v == '#':
            last_free = None

    if last_free == None:
        return (row, False)

    row[last_free:x] = row[last_free+1:x] + ['.']

    return (row, True)

p = grid.find(lambda x: x == '@')
grid.setp(p, '.')

offset = {'^': Point(0,-1), 'v': Point(0,1), '<': Point(-1,0), '>': Point(1,0)}

for move in moves:
    has_moved = None

    match move:
        case '^':
            col, has_moved = update(grid.col(p.x), p.y)
            grid.write_col(p.x, col)
        case 'v':
            col, has_moved = update(list(reversed(grid.col(p.x))), grid.height - p.y - 1)
            grid.write_col(p.x, reversed(col))
        case '<': 
            row, has_moved = update(grid.row(p.y), p.x)
            grid.write_row(p.y, row)
        case '>':
            row, has_moved = update(list(reversed(grid.row(p.y))), grid.width - p.x - 1)
            grid.write_row(p.y, reversed(row))

    if has_moved:
        p += offset[move]

def score(grid, c):
    s = 0
    for y in range(grid.height):
        for x in range(grid.width):
            if grid.at_xy(x, y) == c:
                s += x + y * 100
    return s

print(score(grid, 'O'))

def widen(c):
    match c:
        case '#': return ['#', '#']
        case 'O': return ['[', ']']
        case '.': return ['.', '.']
        case '@': return ['@', '.']

def can_move_box_vertical(grid, x, y, dy):
    can_move = True

    match grid.at_xy(x, y+dy):
        case '#': can_move = False
        case '[': can_move = can_move_box_vertical(grid, x,   y+dy, dy)
        case ']': can_move = can_move_box_vertical(grid, x-1, y+dy, dy)

    match grid.at_xy(x+1, y+dy):
        case '#': can_move = False
        case '[': can_move = can_move and can_move_box_vertical(grid, x+1, y+dy, dy)

    return can_move

def update_box_vertical(grid, x, y, dy):
    match grid.at_xy(x, y+dy):
        case '[': update_box_vertical(grid, x,   y+dy, dy)
        case ']': update_box_vertical(grid, x-1, y+dy, dy)

    if grid.at_xy(x+1, y+dy) == '[':
        update_box_vertical(grid, x+1, y+dy, dy)

    grid.set_xy(x,   y, '.')
    grid.set_xy(x+1, y, '.')

    grid.set_xy(x,   y+dy, '[')
    grid.set_xy(x+1, y+dy, ']')

    return True

def update_robot_vertical(grid, x, y, dy):
    can_move = True

    match grid.at_xy(x, y+dy):
        case '#': can_move = False 
        case '[': can_move = can_move_box_vertical(grid, x,   y+dy, dy)
        case ']': can_move = can_move_box_vertical(grid, x-1, y+dy, dy)

    if not can_move:
        return False

    match grid.at_xy(x, y+dy):
        case '[': update_box_vertical(grid, x,   y+dy, dy)
        case ']': update_box_vertical(grid, x-1, y+dy, dy)

    return True

rows = [list(itertools.chain.from_iterable([widen(c) for c in row])) for row in rows]
grid = Grid(rows)

p = grid.find(lambda x: x == '@')
grid.setp(p, '.')

for move in moves:
    has_moved = None

    match move:
        case '<': 
            row, has_moved = update(grid.row(p.y), p.x)
            grid.write_row(p.y, row)
        case '>':
            row, has_moved = update(list(reversed(grid.row(p.y))), grid.width - p.x - 1)
            grid.write_row(p.y, reversed(row))
        case '^': 
            has_moved = update_robot_vertical(grid, p.x, p.y, -1)
        case 'v':
            has_moved = update_robot_vertical(grid, p.x, p.y,  1)

    if has_moved:
        p += offset[move]

print(score(grid, '['))
