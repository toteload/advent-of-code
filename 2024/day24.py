import fileinput

lines = [line.strip() for line in fileinput.input()]
i = lines.index('')

xs = {}
for line in lines[:i]:
    [x,v] = line.split(': ')
    xs[x] = int(v)

ys = {}
for line in lines[i+1:]:
    [x, op, y, _, z] = line.split()
    ys[z] = (op, x, y)

zs = reversed(sorted(z for z in ys.keys() if z[0] == 'z'))

def val(xs, ys, z):
    if z[0] == 'x' or z[0] == 'y':
        return xs[z]

    (op, x, y) = ys[z]

    match op:
        case 'XOR': return val(xs,ys,x) ^ val(xs,ys,y)
        case 'OR':  return val(xs,ys,x) | val(xs,ys,y)
        case 'AND': return val(xs,ys,x) & val(xs,ys,y)

acc = 0
for z in zs:
    acc <<= 1
    acc += val(xs,ys,z)

print(acc)

# I solved part 2 by generating a DOT graph file, creating a graph and manually inspecting it for
# the mistakes. This is not as painful as it might sound :P 

edges = []
nodes = []

for i in range(45):
    nodes.append(f'x{i:02} []')

for i in range(45):
    nodes.append(f'y{i:02} []')

for (z, (op, x, y)) in ys.items():
    op_node = z + '_' + op

    nodes.append(f'{op_node} [label="{op}"]')

    edges.append(f'{x} -> {op_node}')
    edges.append(f'{y} -> {op_node}')
    edges.append(f'{op_node} -> {z}')

dot = f'digraph {{\n{"\n".join(nodes)}\n{"\n".join(edges)}}}'

open('graph.dot','w').write(dot)
