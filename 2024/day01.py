lines = open("day01.txt", "r").readlines()
pairs = [[int(x) for x in line.split()] for line in lines]
ls = [sorted(xs) for xs in zip(*pairs)]
print(sum([abs(a - b) for [a, b] in zip(*ls)]))
[left, right] = ls
print(sum([x * right.count(x) for x in left]))
