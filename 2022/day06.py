from helpers import *

buffer = day_input(6).strip()

for (i, cs) in enumerate(windows(buffer, 4)):
    if len(set(cs)) == 4:
        print(i + 4)
        break

for (i, cs) in enumerate(windows(buffer, 14)):
    if len(set(cs)) == 14:
        print(i + 14)
        break

