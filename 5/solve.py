from math import ceil

def main():
    with open("./input.txt", "rt") as f:
        data = f.read().splitlines()

    ROW_MIN, ROW_MAX = 0, 127
    COL_MIN, COL_MAX = 0, 7

    t = {
        "F": lambda x, y: (x, x + (y-x)//2),
        "B": lambda x, y: (ceil(x+(y-x)/2), y),
        "L": lambda x, y: t['F'](x, y),
        "R": lambda x, y: t['B'](x, y)
    }

    def mapper(line):
        x, y = (ROW_MIN, ROW_MAX), (COL_MIN, COL_MAX)
        for c in line[:-3]:
            x = t[c](*x)

        for c in line[-3:]:
            y = t[c](*y)

        return x[0] * 8 + y[0]

    seats = list(map(mapper, data))
    print(max(seats))

    # all_seats = {x * 8 + y for x in range(ROW_MAX+1) for y in range(COL_MAX+1)}
    all_seats = {x for x in range(min(seats), max(seats))}
    free = all_seats.difference(set(seats))
    print(free)


if __name__ == '__main__':
    main()
