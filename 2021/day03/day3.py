from collections import defaultdict, Counter

numbers = open("input.txt").read().strip().splitlines()

# one star
cnt = defaultdict(Counter)
for n in numbers:
    for i, b in enumerate(n):
        cnt[i].update(b)

gamma = int(''.join([v.most_common(1)[0][0] for k, v in cnt.items()]), 2)
epsilon = gamma ^ (2 ** (int.bit_length(gamma)) - 1)  # flip bytes
print(gamma * epsilon)


# two stars
def f(candidates, bit, level=0):
    if len(candidates) == 1:
        return int(''.join(candidates[0]), 2)

    new_candids = [[], []]
    for n in candidates:
        if n[level] == bit:
            new_candids[int(bit)].append(n)
        else:
            new_candids[int(bit) ^ 1].append(n)

    if len(new_candids[0]) == len(new_candids[1]):
        new_candidates = int(bit)
    elif bit == '1':
        new_candidates = 0 if len(new_candids[0]) > len(new_candids[1]) else 1
    else:
        new_candidates = 0 if len(new_candids[0]) < len(new_candids[1]) else 1

    return f(
        new_candids[new_candidates],
        bit,
        level + 1
    )


print(f(numbers, '1') * f(numbers, '0'))
