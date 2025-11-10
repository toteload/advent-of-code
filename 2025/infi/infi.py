import fileinput
import copy
import itertools

# Usage: python infi.py input.txt

class HexGrid:
    def __init__(self, grid):
        self.cells = grid
        self.width = len(grid[0])
        self.height = len(grid)

    def copy(self):
        return self.copy_with(lambda x: x)

    def copy_with(self, f):
        ncells = copy.deepcopy(self.cells)

        for y in range(len(ncells)):
            for x in range(len(ncells[0])):
                if ncells[y][x] == None:
                    continue

                ncells[y][x] = f(ncells[y][x])

        return HexGrid(ncells)

    def topleft(self):
        return (0, (self.height - 1) // 2)

    def is_valid_position(self, x, y):
        return x >= 0 and x < self.width and y >= 0 and y < self.height and self.cells[y][x] != None

    def __getitem__(self, i):
        return self.cells[i]

def step(p, d, n=1):
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

    x, y = p
    dx, dy = directions[d]

    return (x + n * dx, y + n * dy)

def rows_to_grid(rows):
    w = len(rows[0])
    h = len(rows)

    # For simplicity we assume the input is always of this ratio.
    # This is the case for the input and examples.
    assert w * 2 == h

    grid = [h * [None] for _ in range(h-1)]
    start = (0, (h - 1) // 2)

    for y in range(0, h, 2):
        for x in range(w):
            s = start
            s = step(s, 'south', y // 2)
            s = step(s, 'east', x)

            sx, sy = s
            grid[sy][sx] = rows[y][x]

            s = step(s, 'southeast')

            sx, sy = s
            grid[sy][sx] = rows[y+1][x]

    return grid

def grid_to_rows(grid):
    h = grid.width
    w = h // 2
    rows = [w * [None] for _ in range(h)]

    start = grid.topleft()

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

    return rows

def read_input():
    # Skip the first and last line, because they don't contain any numbers
    lines = [*fileinput.input(encoding="utf-8")][1:-1]

    def parse(v):
        return int(v) if v != ' ' else -1

    # Extract all the tree heights from the hexagonal grid text input
    rows = [[parse(v) for v in line.strip()[1+3*(i%2)::6]] for (i, line) in enumerate(lines)]

    return HexGrid(rows_to_grid(rows))

def pretty_print_grid(grid, f):
    rows  = grid_to_rows(grid)
    lines = [''.join([f(c) for c in row]) for row in rows]
    w = len(rows[0])

    print(' ' + w * '__    ')
    for i, line in enumerate(lines):
        pre = '/' if i % 2 == 0 else '\\__/'
        post = ' \\__/' if i % 2 == 0 else ' \\'
        print(pre + ' \\__/'.join(line) + post)
    print('/  ' + w * '\\__/  ')

def pretty_print_tree_heights(tree_heights):
    def tree_as_str(c):
        if c == -1:
            return ' '
        return str(c)

    pretty_print_grid(tree_heights, tree_as_str)

def pretty_print_light_map(light):
    for z in range(4):
        pretty_print_grid(light, lambda c: ' ' if c[z] else 'x')

def has_light(tree_height, light):
    return any([light[z] for z in range(tree_height)])

def add_lamp_light(light, tree_heights, lamp_direction):
    if lamp_direction in ['north', 'south']:
        steps = ['southeast', 'northeast']
    else:
        steps = ['southeast', 'southwest']

    start = tree_heights.topleft()
    width = tree_heights.width

    if lamp_direction == 'north':
        start = step(start, 'south', width // 2 - 1)
    elif lamp_direction == 'west':
        start = step(start, 'east', width // 2 - 1)

    p = start
    for _, s in zip(range(width), itertools.cycle(steps)):
        shadow_height = 0
        for i in range(width // 2):
            x, y = step(p, lamp_direction, i)
            for z in range(shadow_height, 4):
                light[y][x][z] = True
            shadow_height = max(shadow_height, tree_heights[y][x])
        p = step(p, s)

def add_fluorescent_light(light, tree_heights, lamp_direction):
    # Find all trees that receive some light
    lit_trees = []
    for y in range(light.height):
        for x in range(light.width):
            # Must be a valid position
            if not tree_heights.is_valid_position(x, y):
                continue

            # There must be a tree of at least height 1
            if tree_heights[y][x] <= 0:
                continue

            # The tree must have some light to absorb
            if not has_light(tree_heights[y][x], light[y][x]):
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
        light_height = tree_heights[ty][tx]
        for d in fluorescent_directions[lamp_direction]:
            shadow_height = 0
            for i in itertools.count(1):
                x, y = step((tx, ty), d, i)

                if not tree_heights.is_valid_position(x, y):
                    break

                for z in range(shadow_height, light_height):
                    light[y][x][z] = True

                shadow_height = max(shadow_height, tree_heights[y][x])

def create_tall_neighbor_map(tree_heights):
    ts = tree_heights.copy_with(lambda _: 0)

    for y in range(ts.height):
        for x in range(ts.width):
            if not tree_heights.is_valid_position(x, y):
                continue

            count = 0
            ds = [ 'north', 'northeast', 'southeast', 'south', 'southwest', 'northwest' ]
            for d in ds:
                nx, ny = step((x, y), d)

                if not tree_heights.is_valid_position(nx, ny):
                    continue
                
                if tree_heights[ny][nx] < 2:
                    continue

                count += 1

            ts[y][x] = count

    return ts

def update_forest(tree_heights, lamp_direction, add_light):
    ntree_heights = tree_heights.copy()

    light = tree_heights.copy_with(lambda _: 4 * [False])
    add_light(light, tree_heights, lamp_direction)

    tall_neighbors = create_tall_neighbor_map(tree_heights)

    width  = tree_heights.width
    height = tree_heights.height

    # Find all spots where a seed will land
    for y in range(height):
        for x in range(width):
            # Spot must be valid
            if not tree_heights.is_valid_position(x, y): 
                continue

            # Spot must be empty
            if tree_heights[y][x] != -1:
                continue

            # Spot must have enough tall trees around
            if tall_neighbors[y][x] < 2:
                continue

            # The spot must be in the light
            if not light[y][x][0]:
                continue

            ntree_heights[y][x] = 0

    # Grow the trees
    for y in range(height):
        for x in range(width):
            # Spot must be valid
            if not tree_heights.is_valid_position(x, y):
                continue

            # Spot must have a tree or seed
            if tree_heights[y][x] == -1:
                continue

            # The tree must have some light and cannot be fully in the shadow
            # There is a minimum of 1 light level that we check otherwise we would skip over seeds.
            if not has_light(max(1, tree_heights[y][x]), light[y][x]):
                continue

            ntree_heights[y][x] = tree_heights[y][x] + 1

    cut_tree_count = 0

    # Cut down fully grown trees
    for y in range(height):
        for x in range(width):
            if ntree_heights[y][x] == 5:
                ntree_heights[y][x] = -1
                cut_tree_count += 1

    return (cut_tree_count, ntree_heights)

def simulate(tree_heights, light_update):
    tree_heights = tree_heights.copy()

    lamp_directions = itertools.cycle(['south', 'west', 'north', 'east'])

    count = 0
    for _, d in zip(range(256), lamp_directions):
        cutcount, tree_heights = update_forest(tree_heights, d, light_update)
        count += cutcount

    return count

def main():
    def add_lamp_light_and_fluorescence(light, tree_heights, lamp_direction):
        add_lamp_light(light, tree_heights, lamp_direction)
        # add_lamp_light must be called first, because we need the light of the lamp to determine
        # if fluorescence is going to be added.
        add_fluorescent_light(light, tree_heights, lamp_direction)

    tree_heights = read_input()
    print(simulate(tree_heights, add_lamp_light))
    print(simulate(tree_heights, add_lamp_light_and_fluorescence))

main()
