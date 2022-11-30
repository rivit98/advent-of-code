import numpy as np
from queue import PriorityQueue

data = open("input.txt").read().strip().splitlines()
b = np.array([list(map(int, [c for c in line])) for line in data])
w, h = b.shape
START, END = (0, 0), (w - 1, h - 1)


def get_neigbors(v):
    vx, vy = v
    out = []
    for dx, dy in [(1, 0), (0, 1), (-1, 0), (0, -1)]:
        nx, ny = vx + dx, vy + dy
        if nx < 0 or nx >= w or ny < 0 or ny >= h: continue
        out.append((nx, ny))

    return out


def dij(matrix, start_v, end_v):
    D = np.full(matrix.shape, 10 ** 9, dtype=int)
    D[start_v] = 0
    visited = set()

    pq = PriorityQueue()
    pq.put((0, start_v))

    while not pq.empty():
        dist, current_v = pq.get()
        visited.add(current_v)

        for neighbor in get_neigbors(current_v):
            if neighbor in visited: continue

            new_cost = D[current_v] + matrix[neighbor]
            if new_cost < D[neighbor]:
                pq.put((new_cost, neighbor))
                D[neighbor] = new_cost

    return D[end_v]


# one star
print(dij(b, START, END))

# two stars
row = np.hstack([(b + i - 1) % 9 + 1 for i in range(5)])
big_map = np.vstack([(row + i - 1) % 9 + 1 for i in range(5)])
w, h = big_map.shape
START, END = (0, 0), (w - 1, h - 1)
print(dij(big_map, START, END))
