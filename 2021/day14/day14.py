from collections import Counter
from functools import lru_cache

word, prod = open("input.txt").read().strip().split('\n\n')
word = word.strip()

productions = {}
for p in prod.strip().splitlines():
    rhs, lhs = p.split(' -> ')
    productions[rhs] = lhs


@lru_cache(maxsize=None)
def count(level, maxlevel, pair):
    if level == maxlevel:
        return Counter(pair)

    new_str = pair[0] + productions[pair] + pair[1]
    return count(level + 1, maxlevel, new_str[:2]) + count(level + 1, maxlevel, new_str[1:]) - Counter(new_str[1])


def solve(maxlevel, word):
    res = Counter()
    for i in range(0, len(word) - 1):
        res += count(0, maxlevel, word[i:i + 2])

    # correction
    res -= Counter(word[1:-1])

    counts = list(map(lambda t: t[1], res.most_common()))
    return counts[0] - counts[-1]


# one star
print(solve(10, word))

# two stars
print(solve(40, word))
