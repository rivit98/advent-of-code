numbers = list(map(int, open("input.txt").read().strip().split(',')))

part1, part2 = [], []
for i in range(min(numbers), max(numbers)+1):
    s1 = list(map(lambda x: abs(x-i), numbers))
    s2 = sum(map(lambda x: (x * (x + 1)) // 2, s1))
    part1.append(sum(s1))
    part2.append(s2)

# one star
print(min(part1))

# two sars
print(min(part2))

