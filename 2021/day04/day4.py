import numpy as np

BOARD_SIZE = 5
data = open("input.txt").read().strip()
data = data.split('\n', 1)
numbers = list(map(int, data[0].split(',')))
boards = map(lambda b: np.fromstring(b, dtype=int, sep=' ').reshape(BOARD_SIZE, BOARD_SIZE), data[1].split('\n\n'))

class Board:
    def __init__(self, a):
        self.bingo_combinations = []

        for i in range(BOARD_SIZE):
            self.bingo_combinations.append(set([a[(j, i)] for j in range(BOARD_SIZE)]))
            self.bingo_combinations.append(set([a[(i, j)] for j in range(BOARD_SIZE)]))

        self.numbers_in_this_board = set(a.flatten())

    def check_first_bingo(self, numbers):
        marked = set()
        for i, n in enumerate(numbers):
            if n in self.numbers_in_this_board:
                marked.add(n)

                for c in self.bingo_combinations:
                    c.discard(n)

                if any([len(s) == 0 for s in self.bingo_combinations]):
                    break

        return i, n, sum(self.numbers_in_this_board - marked)


results = [b.check_first_bingo(numbers) for b in map(lambda s: Board(s), boards)]

# one star
best = min(results, key=lambda t: t[0])
print(best[1] * best[2])

# two stars
worst = max(results, key=lambda t: t[0])
print(worst[1] * worst[2])
