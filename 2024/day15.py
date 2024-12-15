import fileinput
import itertools

class Grid:
    def __init__(self, rows):
        self.width = len(rows[0])
        self.height = len(rows)
        self.rows = rows

    def find(self, pred):
        for y in range(self.height):
            for x in range(self.width):
                if pred(self.rows[y][x]):
                    return (x,y)
        return None

    def at_xy(self, x, y):
        return self.rows[y][x]

    def set_xy(self, x, y, v):
        self.rows[y][x] = v

    def row(self, y):
        return self.rows[y][:]

    def col(self, x):
        return [self.rows[y][x] for y in range(self.height)]

    def write_col(self, x, col):
        for y, v in enumerate(col):
            self.rows[y][x] = v

    def write_row(self, y, row):
        for x, v in enumerate(row):
            self.rows[y][x] = v

    def compact_str(self):
        return '\n'.join([''.join(row) for row in self.rows])

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

def update(row):
    row = row[:]
    last_free = None
    p = None

    for i, v in enumerate(row):
        if v == '@':
            p = i
            break

        if v == '.':
            last_free = i

        if v == '#':
            last_free = None

    if last_free == None:
        return row

    patch = row[last_free+1:p+1] + ['.']
    row[last_free:p+1] = patch

    return row

for move in moves:
    (x,y) = grid.find(lambda x: x == '@')

    match move:
        case '^': grid.write_col(x, update(grid.col(x)))
        case 'v': grid.write_col(x, reversed(update(list(reversed(grid.col(x))))))
        case '<': grid.write_row(y, update(grid.row(y)))
        case '>': grid.write_row(y, reversed(update(list(reversed(grid.row(y))))))

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
        return

    match grid.at_xy(x, y+dy):
        case '[': update_box_vertical(grid, x,   y+dy, dy)
        case ']': update_box_vertical(grid, x-1, y+dy, dy)

    grid.set_xy(x, y,    '.')
    grid.set_xy(x, y+dy, '@')

rows = [list(itertools.chain.from_iterable([widen(c) for c in row])) for row in rows]
grid = Grid(rows)

for move in moves:
    (x,y) = grid.find(lambda x: x == '@')

    match move:
        case '<': grid.write_row(y, update(grid.row(y)))
        case '>': grid.write_row(y, reversed(update(list(reversed(grid.row(y))))))
        case '^': update_robot_vertical(grid, x, y, -1)
        case 'v': update_robot_vertical(grid, x, y,  1)

print(score(grid, '['))
