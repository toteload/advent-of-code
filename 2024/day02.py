def is_safe(r):
    ds = [b-a for [a, b] in zip(r, r[1:])]
    is_monotonic = all([x < 0 for x in ds]) or all([x > 0 for x in ds])
    valid_step_sizes = all([1 <= abs(x) <= 3 for x in ds])
    return is_monotonic and valid_step_sizes

lines = open("day02.txt", "r").readlines()
reports = [[int(x) for x in line.split()] for line in lines]
print(len([x for x in reports if is_safe(x)]))

def is_safe2(r):
    for i in range(len(r)):
        xs = r[:]
        del xs[i]
        if is_safe(xs):
            return True
    return is_safe(r)

print(len([x for x in reports if is_safe2(x)]))
