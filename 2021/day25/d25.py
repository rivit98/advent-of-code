import numpy as np

data = open("input.txt").read().strip().splitlines()
b = np.array([[c for c in line] for line in data])


xlim, ylim = b.shape
moved = True
step = 0
while moved:
    moved = False
    new_b = np.full(b.shape, '.')
    down_facing = {}
    for x in range(xlim):
        for y in range(ylim):
            c = b[x][y]
            if c == '.': continue

            if c == '>':
                ny = (y + 1) % ylim
                if b[x][ny] == '.':
                    new_b[x][ny] = '>'
                    moved = True
                else:
                    new_b[x][y] = '>'
            else:
                nx = (x + 1) % xlim
                down_facing[(x, y)] = (nx, y)

    for k, v in down_facing.items():
        nx, y = v
        ox, oy = k
        if new_b[nx][y] == '.' and b[nx][y] != 'v':
            new_b[nx][y] = 'v'
            moved = True
        else:
            new_b[ox][oy] = 'v'

    step += 1
    b = new_b

print(step)