from itertools import groupby

def main():
    with open("./input.txt", "rt") as f:
        data = f.read().splitlines()

    chunks = [list(group) for k, group in groupby(data, lambda k: k == '') if not k ]

    # 1 star
    counts = list(map(lambda group: len(set(list(''.join(group)))), chunks))
    print(sum(counts))

    # 2 stars
    counts2 = list(map(lambda group: len(set.intersection(*(map(lambda x: set(list(x)), group)))), chunks))
    print(sum(counts2))


if __name__ == '__main__':
    main()
