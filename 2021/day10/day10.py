from functools import reduce

words = open("input.txt").read().strip().splitlines()

match = {k: v for k, v in zip(')]}>', '([{<')}
score = {k: v for k, v in zip(match.keys(), [3, 57, 1197, 25137])}
score2 = {k: v for k, v in zip(match.values(), range(1, 5))}

part1 = 0
scores = []
for word in words:
    bad = False
    stack = []
    for c in word:
        if c in match.values():
            stack.append(c)
        elif stack.pop() != match[c]:
            part1 += score[c]
            bad = True
            break

    if not bad:
        scores.append(reduce(lambda acc, v: acc*5+score2[v], reversed(stack), 0))

# one star
print(part1)

# two stars
print(sorted(scores)[len(scores)//2])
