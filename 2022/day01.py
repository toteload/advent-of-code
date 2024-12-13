from helpers import *

txt = day_input(1)

elves = txt.split('\n\n')
foods = [[int(y) for y in x.strip().split('\n')] for x in elves]
total_calories = [sum(x) for x in foods]
total_calories.sort(reverse=True)
print(total_calories[0])
print(sum(total_calories[:3]))

