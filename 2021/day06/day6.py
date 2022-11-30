lant = list(map(int, open("input.txt").read().strip().split(',')))


def calc(max_days):
    buckets = [0] * 9

    for i in lant:
        buckets[i] += 1

    for i in range(max_days):
        to_create = buckets[0]
        buckets[7] += to_create
        buckets.append(to_create)
        buckets.pop(0)

    return sum(buckets)


# one star
print(calc(80))

# two stars
print(calc(256))

