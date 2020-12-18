from itertools import product

with open("./input.txt") as f:
    data = list(map(lambda s: list(s), f.read().splitlines()))


def init_cube(dim):
    neighbours = list(product([-1, 0, 1], repeat=dim))
    neighbours.remove(tuple(0 for _ in range(dim)))
    dim -= 2
    s = [(0, len(data[0])), (0, len(data)), *[(0, 1) for _ in range(dim)]]
    init = set()
    for i, row in enumerate(data):
        for j, y in enumerate(row):
            if y == '#':
                init.add((i, j, *([0] * dim)))

    return s, init, neighbours


def count_neighbours(points, size, position):
    cnt = 0
    for p in neighbours_to_check:
        new_pos = tuple(i + j for i, j in zip(p, position))
        if any([x < c[0] or x >= c[1] for x, c in zip(new_pos, size)]):
            continue

        if new_pos in points:
            cnt += 1

    return cnt


def iteration(points, size):
    new_cube = set()
    for pos in product(*[list(range(x - 1, y + 1)) for x, y in size]):
        n = count_neighbours(points, size, pos)
        if n == 3 or (pos in points and n == 2):
            new_cube.add(pos)

    return new_cube, tuple([(x - 1, y + 1) for x, y in size])


size, cube, neighbours_to_check = init_cube(3)
for _ in range(6):
    cube, size = iteration(cube, size)

print(len(cube))

# 2 star
size, cube, neighbours_to_check = init_cube(4)
for _ in range(6):
    cube, size = iteration(cube, size)

print(len(cube))
