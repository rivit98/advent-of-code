import numpy as np

data = open("input.txt").read().strip().splitlines()
b = np.array([list(map(int, [c for c in line])) for line in data])
step_mat = np.ones_like(b)
W, H = b.shape


def inc_neigh(i, j, flashed):
    if b[i][j] != 9 or (i, j) in flashed:
        return

    flashed.add((i, j))

    for dx, dy in set([(x, y) for x in range(-1, 2, 1) for y in range(-1, 2, 1)]) - {(0, 0)}:
        nx, ny = i + dx, j + dy
        if nx < 0 or nx >= W or ny < 0 or ny >= H: continue
        b[nx][ny] = min(b[nx][ny] + 1, 9)
        if b[nx][ny] == 9:
            inc_neigh(nx, ny, flashed)


part1 = 0
step = 0
while b.any():
    flashed = set()
    for i in range(W):
        for j in range(H):
            inc_neigh(i, j, flashed)

    b += step_mat
    if step < 100:
        c = (b == 10).sum()
        part1 += c

    b[b == 10] = 0

    step += 1

# one star
print(part1)

# two stars
print(step)
