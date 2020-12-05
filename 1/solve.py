def task1():
    SUM = 2020
    with open("D:\\adventOfCode\\1\\input.txt", "rt") as f:
        numbers = list(map(int, f.read().splitlines()))

    d = {}
    for n in numbers:
        if n in d:
            a = n
            b = d[n]
            print(a, b, a * b)

        d[SUM - n] = n


    for a in numbers:
        for b in numbers:
            for c in numbers:
                if a + b + c == SUM:
                    print(a * b * c)
                    return


if __name__ == '__main__':
    task1()
