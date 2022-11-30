import re
import math
from typing import List, Optional
from functools import reduce


with open("./input.txt") as f:
    data = f.read().strip()
    chunks_raw = data.split('\n\n')

class Chunk:
    def __init__(self, data):
        data = data.split('\n')
        self.title_nr = int(re.match(r"Tile (\d+):", data[0]).group(1))
        data = list(map(lambda x: list(x), data[1:]))
        self.top = ''.join(data[0])
        self.bottom = ''.join(data[-1])
        self.right = ''.join([row[-1] for row in data])
        self.left = ''.join([row[0] for row in data])
        self.data = [''.join(row[1:-1]) for row in data[1:-1]]
        self.state_idx = 0
        self.states = []
        self.data_states = []
        self.build_state_array()

    def get_border_part(self, idx):
        return self.states[self.state_idx][idx]

    def get_data(self):
        return self.data_states[self.state_idx]

    def set_state(self, i):
        self.state_idx = i
        self.top, self.right, self.bottom, self.left = self.states[self.state_idx]

    def build_state_array(self):
        for i in range(4):
            self.states.append((self.top, self.right, self.bottom, self.left))
            self.data_states.append(self.data)
            self.add_flips()
            self.rotateRight()

    def add_flips(self):
        self.flipVertically()
        self.states.append((self.top, self.right, self.bottom, self.left))
        self.data_states.append(self.data)
        self.flipVertically()

    def rotateRight(self):
        self.right, self.left = self.right[::-1], self.left[::-1]
        self.top, self.right, self.bottom, self.left = self.left, self.top, self.right, self.bottom
        self.data = list(map(''.join, zip(*reversed(self.data))))

    def flipVertically(self):
        self.top, self.bottom = self.bottom, self.top
        self.right, self.left = self.right[::-1], self.left[::-1]
        self.data = [x for x in reversed(self.data)]


chunks = [Chunk(chunk) for chunk in chunks_raw]
N = int(round(math.sqrt(len(chunks))))
grid: List[List[Optional[Chunk]]] = [[None] * N for _ in range(N)]

dirs = [(1, 0, 1, 3), (-1, 0, 3, 1), (0, 1, 2, 0), (0, -1, 0, 2)]  # dx, dy, current_border, matching_border
def tile_matches_to_grid(tile, grid_idx):
    x, y = grid_idx // N, grid_idx % N
    for dy, dx, tile_border_idx, border_idx in dirs:
        nx, ny = x + dx, y + dy
        if nx < 0 or nx >= N or ny < 0 or ny >= N or grid[nx][ny] is None: continue

        current_tile_border = tile.get_border_part(tile_border_idx)
        neighbour_tile_border = grid[nx][ny].get_border_part(border_idx)
        if current_tile_border != neighbour_tile_border:
            return False

    return True


def solve_puzzle(available_tiles, grid_idx=0):
    if grid_idx >= N * N:
        return True

    x, y = grid_idx // N, grid_idx % N  # x - row, y - column
    for tile in available_tiles.copy():
        available_tiles.remove(tile)

        for i in range(len(tile.states)):
            tile.set_state(i)
            if tile_matches_to_grid(tile, grid_idx):
                grid[x][y] = tile
                if solve_puzzle(available_tiles, grid_idx + 1):
                    return True

                grid[x][y] = None

        tile.set_state(0)  # clear state
        available_tiles.append(tile)

    return False


solve_puzzle(chunks)
print(reduce(lambda v, a: a * v, (grid[x][y].title_nr for x in [0, N-1] for y in [0, N-1]), 1))

# 2 star
final_image = []
chunk_h = len(grid[0][0].get_data())
for x in range(N):
    for h in range(chunk_h):
        final_image.append(''.join([grid[x][y].get_data()[h] for y in range(N)]))


final_permutations = []
for i in range(4):
    final_permutations.append(final_image)
    final_image = [x for x in reversed(final_image)]  # flip
    final_permutations.append(final_image)
    final_image = list(map(''.join, zip(*reversed(final_image))))  # rotate right


image_size = chunk_h * N
sea_monster = """
                  # 
#    ##    ##    ###
 #  #  #  #  #  #   """.strip('\n').split('\n')

sea_monster_len = len(sea_monster[0])
sea_monster_h = len(sea_monster)

res = []
for f in final_permutations:
    hash_cnt = '\n'.join(f).count('#')
    for x in range(image_size - sea_monster_h + 1):
        for y in range(image_size - sea_monster_len + 1):
            good = True
            for sx in range(sea_monster_h):
                for sy in range(sea_monster_len):
                    sc = sea_monster[sx][sy]
                    if sc != '#': continue

                    if f[x+sx][y+sy] != '#':
                        good = False
                        break
            if good:
                hash_cnt -= ''.join(sea_monster).count('#')

    res.append(hash_cnt)

print(min(list(set(res))))


