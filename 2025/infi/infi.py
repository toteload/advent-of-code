import fileinput
import sys
from itertools import cycle

# Usage: python infi.py input.txt

directions = {
    'north':     (-1, -1),
    'east':      (+1, -1),
    'south':     (+1, +1),
    'west':      (-1, +1),
    'northeast': ( 0, -1),
    'northwest': (-1,  0),
    'southeast': (+1,  0),
    'southwest': ( 0, +1),
}

def step(p, d, n=1):
    x, y = p
    dx, dy = directions[d]
    return (x + n * dx, y + n * dy)

def topleft(cells):
    h = len(cells[0])
    return (0, (h - 1) // 2)

def read_input():
    # Skip the first and last line, because they don't contain any numbers
    lines = [*fileinput.input(encoding="utf-8")][1:-1]

    def cell(v):
        return int(v) if v != ' ' else -1

    # Extract all the cells from the hexagonal grid text input
    rows = [[cell(v) for v in line.strip()[1+3*(i%2)::6]] for (i, line) in enumerate(lines)]

    w = len(rows[0])
    h = len(rows)

    # For simplicity we assume the input is always of this ratio.
    # This is the case for the input and examples.
    assert w * 2 == h

    cells = [h * [None] for _ in range(h-1)]
    start = topleft(cells)

    for y in range(0, h, 2):
        for x in range(w):
            s = start
            s = step(s, 'south', y // 2)
            s = step(s, 'east', x)

            sx, sy = s
            cells[sy][sx] = rows[y][x]

            s = step(s, 'southeast')

            sx, sy = s
            cells[sy][sx] = rows[y+1][x]

    return cells

def pretty_print_grid(grid, f):
    h = len(grid[0])
    w = h // 2
    rows = [w * [None] for _ in range(h)]

    start = topleft(grid)

    for y in range(0, h, 2):
        for x in range(w):
            s = start
            s = step(s, 'south', y // 2)
            s = step(s, 'east', x)

            sx, sy = s
            rows[y][x] = grid[sy][sx]

            s = step(s, 'southeast')

            sx, sy = s
            rows[y+1][x] = grid[sy][sx]

    lines = [''.join([f(c) for c in row]) for row in rows]

    print(' ' + w * '__    ')
    for i, line in enumerate(lines):
        pre = '/' if i % 2 == 0 else '\\__/'
        post = ' \\__/' if i % 2 == 0 else ' \\'
        print(pre + ' \\__/'.join(line) + post)
    print('/  ' + w * '\\__/  ')

def pretty_print_tree_heights(tree_heights):
    def tree(c):
        if c == -1:
            return ' '
        return str(c)

    pretty_print_grid(tree_heights, tree)

def pretty_print_light_map(light):
    for z in range(4):
        pretty_print_grid(light, lambda c: ' ' if c[z] else 'x')

def create_light_map(cells):
    light = [[4 * [False] for _ in range(len(cells[0]))] for _ in range(len(cells))]
    for y in range(len(cells)):
        for x in range(len(cells[0])):
            if cells[y][x] == None:
                light[y][x] = None
    return light

def add_lamp_light(light, tree_heights, lamp_direction):
    if lamp_direction in ['north', 'south']:
        steps = ['southeast', 'northeast']
    else:
        steps = ['southeast', 'southwest']

    width = len(tree_heights[0])

    start = topleft(tree_heights)
    if lamp_direction == 'north':
        start = step(start, 'south', width // 2 - 1)
    elif lamp_direction == 'west':
        start = step(start, 'east', width // 2 - 1)

    p = start
    for _, s in zip(range(width), cycle(steps)):
        shadow_height = 0
        for i in range(width // 2):
            x, y = step(p, lamp_direction, i)
            for z in range(4 - shadow_height):
                light[y][x][shadow_height + z] = True
            shadow_height = max(shadow_height, tree_heights[y][x])
        p = step(p, s)

def add_fluorescent_light(light, cells, lamp_direction):
    width  = len(cells[0])
    height = len(cells)

    # Find all trees that receive some light
    lit_trees = []
    for y in range(height):
        for x in range(width):
            if cells[y][x] == None:
                continue

            if cells[y][x] <= 0 or not any([light[y][x][z] for z in range(cells[y][x])]):
                continue

            lit_trees.append((x, y))

    fluorescent_directions = {
        'south' : ['southwest', 'southeast'],
        'west'  : ['northwest', 'southwest'],
        'north' : ['northwest', 'northeast'],
        'east'  : ['northeast', 'southeast'],
    }

    # For all the trees that received some light, shoot fluorescent rays
    for (tx, ty) in lit_trees:
        light_height = cells[ty][tx]
        shadow_height = 0
        for d in fluorescent_directions[lamp_direction]:
            x, y = tx, ty
            while True:
                x, y = step((x, y), d)

                if x < 0 or x >= width or y < 0 or y >= height or cells[y][x] == None:
                    break

                for z in range(shadow_height, light_height):
                    light[y][x][z] = True

                shadow_height = max(shadow_height, cells[y][x])

def create_tall_neighbor_map(cells):
    width  = len(cells[0])
    height = len(cells)

    ts = [width * [None] for _ in range(height)]

    for y in range(height):
        for x in range(width):
            if cells[y][x] == None:
                continue

            count = 0
            ds = [ 'north', 'northeast', 'southeast', 'south', 'southwest', 'northwest' ]
            for d in ds:
                nx, ny = step((x, y), d)
                if nx < 0 or nx >= width or ny < 0 or ny >= height or cells[ny][nx] == None:
                    continue
                
                if cells[ny][nx] < 2:
                    continue

                count += 1

            ts[y][x] = count

    return ts

def update_forest(cells, lamp_direction, light_update):
    ncells = [row[:] for row in cells]

    light = create_light_map(cells)
    light_update(light, cells, lamp_direction)

    #pretty_print_tree_heights(cells)
    #pretty_print_light_map(light)

    tall_neighbors = create_tall_neighbor_map(cells)

    width  = len(cells[0])
    height = len(cells)

    # Find all spots where a seed will land
    for y in range(height):
        for x in range(width):
            # Spot must be valid and empty
            if cells[y][x] == None or cells[y][x] != -1:
                continue

            # Spot must have enough tall trees around
            if tall_neighbors[y][x] < 2:
                continue

            # The spot must be in the light
            if not light[y][x][0]:
                continue

            ncells[y][x] = 0

    # Grow the trees
    for y in range(height):
        for x in range(width):
            # Spot must be valid and there must be a tree or seed
            if cells[y][x] == None or cells[y][x] == -1:
                continue

            # The tree must have some light and cannot be fully in the shadow
            # There is a minimum of 1 light level that we check otherwise we would skip over seeds.
            if not any([light[y][x][z] for z in range(max(1, cells[y][x]))]):
                continue

            ncells[y][x] = cells[y][x] + 1

    cut_tree_count = 0

    for y in range(height):
        for x in range(width):
            if ncells[y][x] == 5:
                ncells[y][x] = -1
                cut_tree_count += 1

    return (cut_tree_count, ncells)

def simulate(cells, light_update):
    cells = [row[:] for row in cells]

    lamp_directions = cycle(['south', 'west', 'north', 'east'])

    count = 0
    for i, d in zip(range(256), lamp_directions):
        #pretty_print_tree_heights(cells)
        cutcount, cells = update_forest(cells, d, light_update)
        count += cutcount

    return count

def main():
    def only_lamp_light(light, cells, lamp_direction):
        add_lamp_light(light, cells, lamp_direction)

    def lamp_light_and_fluorescence(light, cells, lamp_direction):
        add_lamp_light(light, cells, lamp_direction)
        # add_lamp_light must be called first, because we need the light of the lamp to determine
        # if fluorescence is going to be added.
        add_fluorescent_light(light, cells, lamp_direction)

    cells = read_input()
    print(simulate(cells, only_lamp_light))
    print(simulate(cells, lamp_light_and_fluorescence))

main()
