def main():
    with open("./input.txt", "rt") as f:
        data = list(map(lambda s: int(s), f.read().splitlines()))
        data.sort()
        data.append(max(data) + 3)

    cnt = [0, 0, 0, 0]
    for prev, current in zip([0] + data[:-1], data):
        cnt[current - prev] += 1

    print(cnt[1] * cnt[3])

    accumulators = [1] + [0] * len(data)
    data = [0] + data
    for i, d in enumerate(data[1:], start=1):
        for j in range(0, i):
            if d - data[j] <= 3:
                accumulators[i] += accumulators[j]

    print(accumulators[-1])


if __name__ == '__main__':
    main()

