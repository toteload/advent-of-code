s = open('day01.txt', 'r').read()

ups = s.count('(')
downs = s.count(')')

print(ups-downs)

lvl = 0

for i, c in enumerate(s):
    lvl += 1 if c == '(' else -1
    if lvl == -1:
        print(i + 1)
        break
