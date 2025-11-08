import fileinput
import sys
from itertools import cycle

# Usage: python cutie.py input.txt

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

def read_cells():
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

def pretty_print_cells(cells):
    h = len(cells[0])
    w = h // 2
    rows = [w * [-1] for _ in range(h)]

    start = topleft(cells)

    for y in range(0, h, 2):
        for x in range(w):
            s = start
            s = step(s, 'south', y // 2)
            s = step(s, 'east', x)

            sx, sy = s
            rows[y][x] = cells[sy][sx]

            s = step(s, 'southeast')

            sx, sy = s
            rows[y+1][x] = cells[sy][sx]

    def cellstr(c):
        if c == -1:
            return ' '
        return str(c)

    lines = [''.join([cellstr(c) for c in row]) for row in rows]

    print(' ' + w * '__    ')
    for i, line in enumerate(lines):
        pre = '/' if i % 2 == 0 else '\\__/'
        post = ' \\__/' if i % 2 == 0 else ' \\'
        print(pre + ' \\__/'.join(line) + post)
    print('/  ' + w * '\\__/  ')

def create_tree_shadow_map(cells, lamp_direction):
    shadows = [len(cells[0]) * [None] for _ in range(len(cells))]

    if lamp_direction in ['north', 'south']:
        steps = ['southeast', 'northeast']
    else:
        steps = ['southeast', 'southwest']

    w = len(cells[0])
    start = topleft(cells)

    if lamp_direction == 'north':
        start = step(start, 'south', w // 2 - 1)
    elif lamp_direction == 'west':
        start = step(start, 'east', w // 2 - 1)

    p = start
    for _, s in zip(range(w), cycle(steps)):
        shadow_height = 0
        for i in range(w // 2):
            x, y = step(p, lamp_direction, i)
            shadows[y][x] = shadow_height
            shadow_height = max(shadow_height, cells[y][x])

        p = step(p, s)

    return shadows

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

def create_fluorescent_light_map(tree_shadows, cells, lamp_direction):
    width  = len(cells[0])
    height = len(cells)

    light = [width * [5 * [False]] for _ in range(height)]

    # Find all trees that receive some light
    lit_trees = []
    for y in range(height):
        for x in range(width):
            if cells[y][x] == None:
                light[y][x] = None

            if tree_shadows[y][x] == None or tree_shadows[y][x] >= cells[y][x]:
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

    return light

def update_forest(cells, lamp_direction):
    ncells = [row[:] for row in cells]

    tree_shadows   = create_tree_shadow_map(cells, lamp_direction)
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
            if tree_shadows[y][x] > 0:
                continue

            ncells[y][x] = 0

    # Grow the trees
    for y in range(height):
        for x in range(width):
            # Spot must be valid and there must be a tree or seed
            if cells[y][x] == None or cells[y][x] == -1:
                continue

            # Special case for seeds. Kinda ugly :/
            if cells[y][x] == 0 and tree_shadows[y][x] == 0:
                ncells[y][x] = cells[y][x] + 1
                continue

            # The tree must have some light and cannot be fully in the shadow
            if tree_shadows[y][x] >= cells[y][x]:
                continue

            ncells[y][x] = cells[y][x] + 1

    cut_tree_count = 0

    for y in range(height):
        for x in range(width):
            if ncells[y][x] == 5:
                ncells[y][x] = -1
                cut_tree_count += 1

    return (cut_tree_count, ncells)

def update_forest_with_fluorescence(cells, lamp_direction):
    ncells = [row[:] for row in cells]

    tree_shadows      = create_tree_shadow_map(cells, lamp_direction)
    tall_neighbors    = create_tall_neighbor_map(cells)
    fluorescent_light = create_fluorescent_light_map(tree_shadows, cells, lamp_direction)

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
            if tree_shadows[y][x] > 0 and not fluorescent_light[y][x][0]:
                continue

            ncells[y][x] = 0

    # Grow the trees
    for y in range(height):
        for x in range(width):
            # Spot must be valid and there must be a tree or seed
            if cells[y][x] == None or cells[y][x] == -1:
                continue

            # Special case for seeds. Kinda ugly :/
            if cells[y][x] == 0 and (tree_shadows[y][x] == 0 or fluorescent_light[y][x][0]):
                ncells[y][x] = cells[y][x] + 1
                continue

            # The tree must have some light
            if tree_shadows[y][x] >= cells[y][x] and not any([fluorescent_light[y][x][z] for z in range(cells[y][x])]):
                continue

            ncells[y][x] = cells[y][x] + 1

    cut_tree_count = 0

    for y in range(height):
        for x in range(width):
            if ncells[y][x] == 5:
                ncells[y][x] = -1
                cut_tree_count += 1

    return (cut_tree_count, ncells)


def simulate(cells, update):
    cells = [row[:] for row in cells]

    lamp_directions = cycle(['south', 'west', 'north', 'east'])

    count = 0
    for i, d in zip(range(256), lamp_directions):
        #pretty_print_cells(cells)
        cutcount, cells = update(cells, d)
        count += cutcount

    return count

cells = read_cells()

print(simulate(cells, update_forest))
print(simulate(cells, update_forest_with_fluorescence))
