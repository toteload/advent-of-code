from helpers import *

txt = day_input(21)

monkeys = {}
for line in txt.splitlines():
    [name, e] = line.split(': ')

    if e.isdecimal():
        monkeys[name] = int(e)
    else:
        [lhs, op, rhs] = e.split(' ')
        monkeys[name] = (op, lhs, rhs)

def evaluate(monkeys, i):
    m = monkeys[i]

    if type(m) is int:
        return m

    [op, lhs, rhs] = m
    a = evaluate(monkeys, lhs)
    b = evaluate(monkeys, rhs)

    if op == '-': return a - b
    if op == '+': return a + b
    if op == '*': return a * b
    if op == '/': return a // b

print(evaluate(monkeys, 'root'))

# A recursive tree search to test if it contains the symbol `x`
def contains_x(monkeys, i, x):
    if i == x:
        return True

    m = monkeys[i]

    if type(m) is int:
        return False

    [op, lhs, rhs] = m

    return contains_x(monkeys, lhs, x) or contains_x(monkeys, rhs, x)

# At every recursion step I do a tree search to find out in which side of the
# node `x` is. This is redundant work and would really only need to be done
# once. After which all the traversed nodes can be marked. This would turn
# checking if a side contains `x` into a constant time table lookup.

def solve_x(monkeys, s, i, x):
    if i == x: return s

    [op, lhs, rhs] = monkeys[i] 
    xleft = contains_x(monkeys, lhs, x)

    cside, xside = (rhs,lhs) if xleft else (lhs,rhs)
    d = evaluate(monkeys, cside)

    if op == '+': return solve_x(monkeys, s - d, xside, x)
    if op == '*': return solve_x(monkeys, s // d, xside, x)
    if op == '/':
        y = s*d if xleft else d // s
        return solve_x(monkeys, y, xside, x)
    if op == '-':
        y = s+d if xleft else d-s
        return solve_x(monkeys, y, xside, x)

x = 'humn'
[_, lhs, rhs] = monkeys['root']
xside, cside = (lhs,rhs) if contains_x(monkeys, lhs, x) else (rhs,lhs)
y = evaluate(monkeys, cside)

print(solve_x(monkeys, y, lhs, x))
