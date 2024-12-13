from functools import cache

@cache
def stone_count(stone, step_count):
    if step_count == 0:
        return 1

    match stone:
        case '0':
            return stone_count('1', step_count - 1)
        case _ if len(stone) % 2 == 0:
            mid = len(stone) // 2
            (l,r) = stone[:mid], stone[mid:]
            return stone_count(str(int(l)), step_count - 1) + stone_count(str(int(r)), step_count - 1)
        case _:
            return stone_count(str(2024 * int(stone)), step_count - 1)

stones = ['125', '17']
stones = open("day11.txt").read().strip().split()

print(sum([stone_count(s, 25) for s in stones]))
print(sum([stone_count(s, 75) for s in stones]))
