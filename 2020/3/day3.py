from functools import reduce

def main():
    with open("./input.txt", "rt") as f:
        data = f.read().splitlines()

    width = len(data[0])

    # 1 star
    def move(dx, dy):
        y = 0
        chars = []
        for row in data[::dx]:
            chars.append(row[y])
            y = (y + dy) % width

        return chars.count('#')

    print(move(1, 3))

    # 2 stars
    moves = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
    print(reduce(lambda x, y: x * y, map(lambda x: move(*x), moves)))


if __name__ == '__main__':
    main()
