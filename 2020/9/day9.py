from itertools import product

def main():
    with open("./input.txt", "rt") as f:
        data = f.read().splitlines()
        data = list(map(lambda s: int(s), data))

    # 1 star
    PREAMBLE_LEN = 25
    last_numbers = data[:PREAMBLE_LEN]
    invalid_number = None
    for x in data[PREAMBLE_LEN:]:
        if not any(a + b == x for a, b in product(last_numbers, repeat=2)):
            invalid_number = x
            break

        last_numbers = last_numbers[1:] + [x]

    print(invalid_number)

    # 2 star
    for i in range(len(data)):
        for j in range(i, len(data)):
            slice = data[i:j]
            s = sum(slice)
            if s == invalid_number and len(slice) > 1:
                print(min(slice) + max(slice))
                return


if __name__ == '__main__':
    main()
