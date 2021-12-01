numbers = list(map(int, open("input.txt", "rt").read().strip().splitlines()))

# one star
print(sum([x < y for x, y in zip(numbers, numbers[1:])]))

# two stars
print(sum([sum(x) < sum(y) for x, y in zip(zip(numbers, numbers[1:], numbers[2:]), zip(numbers[1:], numbers[2:], numbers[3:]))]))
