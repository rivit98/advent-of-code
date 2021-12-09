import numpy as np
from functools import reduce

data = open("input.txt").read().strip().splitlines()
W, H = len(data), len(data[0])
b = np.fromiter(''.join(data), dtype=int).reshape(W, H)
b = np.pad(b, 1, mode='constant', constant_values=9)

# one star
def check_cell(i, j):
    out = []
    for dx, dy in [(1, 0), (0, 1), (-1, 0), (0, -1)]:
        out.append(b[i + dx][j + dy])

    return out


lps = []
for i in range(1, W + 1):
    for j in range(1, H + 1):
        if min(check_cell(i, j)) > b[i][j]:
            lps.append((i, j))

print(reduce(lambda x, y: x+y, map(lambda t: 1+b[t[0]][t[1]], lps)))

# two stars
def check_basin(i, j, visited):
    visited.add((i, j))

    for dx, dy in [(1, 0), (0, 1), (-1, 0), (0, -1)]:
        nx, ny = i + dx, j + dy
        if b[nx][ny] == 9: continue
        if (nx, ny) in visited: continue

        check_basin(nx, ny, visited)

    return len(visited)


sizes = [check_basin(lpx, lpy, set()) for lpx, lpy in lps]
print(reduce(lambda x, y: x * y, sorted(sizes, reverse=True)[:3]))
