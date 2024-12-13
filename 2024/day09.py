from dataclasses import dataclass, astuple
from itertools import pairwise

@dataclass
class FilePos:
    i: int
    offset: int
    size: int

def compress(files):
    fs = files[1:]
    out = [files[0]]

    while fs:
        end_of_last_file = out[-1].offset + out[-1].size

        if end_of_last_file < fs[0].offset:
            gap_size = fs[0].offset - end_of_last_file

            f = fs.pop()

            s = min(f.size, gap_size)

            out.append(FilePos(f.i, end_of_last_file, s))

            if f.size > gap_size:
                fs.append(FilePos(f.i, f.offset, f.size - s))
        else:
            out.append(fs.pop(0))

    return out

def compress2(files):
    out = files[:]

    n = len(out)

    # Don't look at this loop. Avert your eyes!
    for i in range(n-1, 0, -1):
        idx = None 
        for j, f in enumerate(reversed(out)):
            if f.i == i:
                idx = n - 1 - j
                break

        f = out[idx]

        for j, (a,b) in enumerate(pairwise(out[:idx+1])):
            gap_size = b.offset - (a.offset + a.size)

            if gap_size >= f.size:
                out.pop(idx)
                out.insert(j+1, FilePos(f.i, a.offset + a.size, f.size))
                break

    return out

def checksum(files):
    s = 0
    for fp in files:
        for j in range(fp.size):
            s += (fp.offset + j) * fp.i
    return s

diskmap = [int(x) for x in open("day09.txt").read().strip()]

files = []
offset = 0

for i, size in enumerate(diskmap):
    is_file = i % 2 == 0

    if is_file:
        files.append(FilePos(len(files), offset, size))

    offset += size

print(checksum(compress(files)))
print(checksum(compress2(files)))
