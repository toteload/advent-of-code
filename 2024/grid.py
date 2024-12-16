from dataclasses import dataclass

@dataclass(frozen=True)
class Point:
    x: int
    y: int
    def __add__(self, other):
        return Point(self.x + other.x, self.y + other.y)

class Grid:
    def __init__(self, rows):
        self.width = len(rows[0])
        self.height = len(rows)
        self.rows = rows

    def find(self, pred):
        for y in range(self.height):
            for x in range(self.width):
                if pred(self.rows[y][x]):
                    return Point(x,y)
        return None

    def at_p(self, p):
        return self.rows[p.y][p.x]

    def at_xy(self, x, y):
        return self.rows[y][x]

    def setp(self, p, v):
        self.rows[p.y][p.x] = v

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


