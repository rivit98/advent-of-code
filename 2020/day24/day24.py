from collections import defaultdict

with open("./input.txt") as f:
    data = f.read().splitlines()

data = list(map(lambda s: s.replace("se", "x").replace("sw", "z").replace("nw", "h").replace("ne", "j"), data))

black_tiles = defaultdict(bool)

for line in data:
    current_tile = (0, 0)
    for m in list(line):
        if m == "e":   current_tile = (current_tile[0] + 1, current_tile[1])
        if m == "w": current_tile = (current_tile[0] - 1, current_tile[1])
        if m == "x": current_tile = (current_tile[0] + 1, current_tile[1] - 1)
        if m == "z": current_tile = (current_tile[0], current_tile[1] - 1)
        if m == "h": current_tile = (current_tile[0] - 1, current_tile[1] + 1)
        if m == "j": current_tile = (current_tile[0], current_tile[1] + 1)

    black_tiles[current_tile] = not black_tiles[current_tile]

print(sum(black_tiles.values()))

neighbours = [(1, 0), (-1, 0), (1, -1), (0, -1), (-1, 1), (0, 1)]
for i in range(100):
    to_flip = set()
    to_check = set()
    for pos in black_tiles.keys():
        to_check.add(pos)
        for neighbour in [(pos[0] + x[0], pos[1] + x[1]) for x in neighbours]:
            to_check.add(neighbour)

    for pos in to_check:
        black_cnt = 0
        for neighbour in [(pos[0] + x[0], pos[1] + x[1]) for x in neighbours]:
            if black_tiles[neighbour] is True:
                black_cnt += 1

        if black_tiles[pos] and (black_cnt == 0 or black_cnt > 2):
            to_flip.add(pos)
        elif not black_tiles[pos] and black_cnt == 2:
            to_flip.add(pos)

    for tile in to_flip:
        black_tiles[tile] = not black_tiles[tile]

print(sum(black_tiles.values()))